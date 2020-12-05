use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		frame::FrameState,
		geometry::{Interval, AABB2, AABB3},
		quadtree::Quadtree,
		spawn::{SpawnContext, SpawnMerger, SpawnMergerHandlerSet},
		time::Timer,
	},
	doom::{
		components::{SpawnPoint, Transform},
		map::{
			AnimState, LinedefDynamic, LinedefRef, Map, MapDynamic, SectorDynamic, SectorRef,
			SidedefDynamic, Thing, ThingFlags,
		},
		physics::BoxCollider,
		template::{EntityTemplate, EntityTemplateRef, EntityTypeId},
	},
};
use anyhow::bail;
use legion::{
	any,
	systems::{CommandBuffer, ResourceSet},
	world::EntityHasher,
	Entity, IntoQuery, Read, Resources, World, Write,
};
use nalgebra::{Vector2, Vector3};
use std::collections::HashMap;

pub fn spawn_helper(
	src_world: &World,
	dst_world: &mut World,
	resources: &Resources,
) -> HashMap<Entity, Entity, EntityHasher> {
	let handler_set = <Read<SpawnMergerHandlerSet>>::fetch(resources);
	let mut merger = SpawnMerger::new(&handler_set, &resources);
	dst_world.clone_from(&src_world, &any(), &mut merger)
}

pub fn spawn_entity(
	world: &mut World,
	resources: &mut Resources,
	template_handle: &AssetHandle<EntityTemplate>,
	transform: Transform,
) -> Entity {
	// Create spawn context and insert into resources, for SpawnFrom implementations to read
	resources.insert(SpawnContext(transform));
	resources.insert(SpawnContext(template_handle.clone()));

	// Create the entity
	let entity = {
		let asset_storage = <Read<AssetStorage>>::fetch(resources);
		let template = asset_storage.get(template_handle).unwrap();

		if template.world.is_empty() {
			world.push(())
		} else {
			let entity_map = spawn_helper(&template.world, world, resources);
			entity_map.into_iter().map(|(_, to)| to).next().unwrap()
		}
	};

	// Add entity to quadtree
	{
		let mut quadtree = <Write<Quadtree>>::fetch_mut(resources);
		if let Ok((&entity, box_collider, transform)) =
			<(Entity, &BoxCollider, &Transform)>::query().get(world, entity)
		{
			let bbox = AABB3::from_radius_height(box_collider.radius, box_collider.height);
			quadtree.insert(entity, &AABB2::from(bbox.offset(transform.position)));
		}
	}

	resources.remove::<SpawnContext<Transform>>();
	resources.remove::<SpawnContext<AssetHandle<EntityTemplate>>>();

	entity
}

pub fn spawn_things(
	things: Vec<Thing>,
	world: &mut World,
	resources: &mut Resources,
) -> anyhow::Result<()> {
	for (i, thing) in things.into_iter().enumerate() {
		if thing.flags.intersects(ThingFlags::DMONLY) {
			continue;
		}

		if !thing.flags.intersects(ThingFlags::EASY) {
			continue;
		}

		// Find entity template
		let template_handle = {
			let asset_storage = <Read<AssetStorage>>::fetch(resources);

			let template_handle = match asset_storage
				.iter::<EntityTemplate>()
				.find(|(_, template)| template.type_id == Some(EntityTypeId::Thing(thing.r#type)))
			{
				Some((x, _)) => x.clone(),
				None => {
					log::warn!("Thing {} has invalid thing type {}", i, thing.r#type);
					continue;
				}
			};

			template_handle
		};

		// Use NAN to use the default spawn height based on the sector interval
		let transform = Transform {
			position: Vector3::new(thing.position[0], thing.position[1], f32::NAN),
			rotation: Vector3::new(0.into(), 0.into(), thing.angle),
		};

		let sector_interval = {
			let asset_storage = <Read<AssetStorage>>::fetch(resources);
			let map_dynamic = <&MapDynamic>::query().iter(world).next().unwrap();
			let map = asset_storage.get(&map_dynamic.map).unwrap();
			let ssect = map.find_subsector(transform.position.fixed_resize(0.0));
			SpawnContext(map_dynamic.sectors[ssect.sector_index].interval)
		};
		resources.insert(sector_interval);

		spawn_entity(world, resources, &template_handle, transform);

		resources.remove::<SpawnContext<Interval>>();
	}

	Ok(())
}

pub fn spawn_player(
	world: &mut World,
	resources: &mut Resources,
	player_num: usize,
) -> anyhow::Result<Entity> {
	let template_handle = {
		let asset_storage = <Read<AssetStorage>>::fetch(resources);

		match asset_storage.handle_for::<EntityTemplate>("player") {
			Some(template) => template.clone(),
			None => bail!("Entity type not found: {}", "player"),
		}
	};

	// Get spawn point transform
	let transform = match <(&Transform, &SpawnPoint)>::query()
		.iter(world)
		.find_map(|(t, s)| {
			if s.player_num == player_num {
				Some(*t)
			} else {
				None
			}
		}) {
		Some(x) => x,
		None => bail!("Spawn point for player {} not found", player_num),
	};

	Ok(spawn_entity(world, resources, &template_handle, transform))
}

pub fn spawn_map_entities(
	world: &mut World,
	resources: &mut Resources,
	map_handle: &AssetHandle<Map>,
) -> anyhow::Result<()> {
	let mut command_buffer = CommandBuffer::new(world);

	{
		let (asset_storage, frame_state, handler_set) = <(
			Read<AssetStorage>,
			Read<FrameState>,
			Read<SpawnMergerHandlerSet>,
		)>::fetch(resources);
		let mut merger = SpawnMerger::new(&handler_set, &resources);

		let map = asset_storage.get(&map_handle).unwrap();

		// Create map entity
		let map_entity = command_buffer.push(());
		let anim_states = map
			.anims
			.iter()
			.map(|(k, v)| {
				(
					k.clone(),
					AnimState {
						frame: 0,
						timer: Timer::new(frame_state.time, v.frame_time),
					},
				)
			})
			.collect();

		let mut map_dynamic = MapDynamic {
			anim_states,
			map: map_handle.clone(),
			linedefs: Vec::with_capacity(map.linedefs.len()),
			sectors: Vec::with_capacity(map.sectors.len()),
		};

		// Create linedef entities
		for (i, linedef) in map.linedefs.iter().enumerate() {
			let entity = if let Some(special_type) = linedef.special_type {
				// Fetch and add entity template
				let (handle, template) =
					match asset_storage
						.iter::<EntityTemplate>()
						.find(|(_, template)| {
							template.type_id == Some(EntityTypeId::Linedef(special_type))
						}) {
						Some(entry) => entry,
						None => {
							log::warn!("Linedef {} has invalid special type {}", i, special_type);
							continue;
						}
					};

				let entity = if template.world.is_empty() {
					log::debug!(
						"Linedef {} has special type {} with empty template world",
						i,
						special_type
					);

					command_buffer.push(())
				} else {
					let entity_map = world.clone_from(&template.world, &any(), &mut merger);
					entity_map.into_iter().map(|(_, to)| to).next().unwrap()
				};

				// Set entity template reference
				command_buffer.add_component(entity, EntityTemplateRef(handle.clone()));

				entity
			} else {
				command_buffer.push(())
			};

			let sidedefs = [
				linedef.sidedefs[0].as_ref().map(|sidedef| SidedefDynamic {
					textures: sidedef.textures.clone(),
				}),
				linedef.sidedefs[1].as_ref().map(|sidedef| SidedefDynamic {
					textures: sidedef.textures.clone(),
				}),
			];
			map_dynamic.linedefs.push(LinedefDynamic {
				entity,
				sidedefs,
				texture_offset: Vector2::new(0.0, 0.0),
			});
			command_buffer.add_component(
				entity,
				LinedefRef {
					map_entity,
					index: i,
				},
			);
		}

		// Create sector entities
		for (i, sector) in map.sectors.iter().enumerate() {
			let entity = if let Some(special_type) = sector.special_type {
				// Fetch and add entity template
				let (handle, template) =
					match asset_storage
						.iter::<EntityTemplate>()
						.find(|(_, template)| {
							template.type_id == Some(EntityTypeId::Sector(special_type))
						}) {
						Some(entry) => entry,
						None => {
							log::warn!("Sector {} has invalid special type {}", i, special_type);
							continue;
						}
					};

				let entity = if template.world.is_empty() {
					log::debug!(
						"Sector {} has special type {} with empty template world",
						i,
						special_type
					);

					command_buffer.push(())
				} else {
					let entity_map = world.clone_from(&template.world, &any(), &mut merger);
					entity_map.into_iter().map(|(_, to)| to).next().unwrap()
				};

				// Set entity template reference
				command_buffer.add_component(entity, EntityTemplateRef(handle.clone()));

				entity
			} else {
				command_buffer.push(())
			};

			map_dynamic.sectors.push(SectorDynamic {
				entity,
				light_level: sector.light_level,
				interval: sector.interval,
			});
			command_buffer.add_component(
				entity,
				SectorRef {
					map_entity,
					index: i,
				},
			);

			// Find midpoint of sector for sound purposes
			let mut bbox = AABB2::empty();

			for linedef in map.linedefs.iter() {
				for sidedef in linedef.sidedefs.iter().flatten() {
					if sidedef.sector_index == i {
						bbox.add_point(linedef.line.point);
						bbox.add_point(linedef.line.end_point());
					}
				}
			}

			let midpoint = (bbox.min() + bbox.max()) / 2.0;

			command_buffer.add_component(
				entity,
				Transform {
					position: Vector3::new(midpoint[0], midpoint[1], 0.0),
					rotation: Vector3::new(0.into(), 0.into(), 0.into()),
				},
			);
		}

		command_buffer.add_component(map_entity, map_dynamic);
	}

	command_buffer.flush(world, resources);
	Ok(())
}
