use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		frame::FrameState,
		geometry::{Interval, AABB2},
		spawn::{SpawnMerger, SpawnMergerHandlerSet},
		time::Timer,
	},
	doom::{
		components::{SpawnPoint, Transform},
		entitytemplate::{EntityTemplate, EntityTemplateRef, EntityTypeId},
		map::{
			AnimState, LinedefDynamic, LinedefRef, Map, MapDynamic, SectorDynamic, SectorRef,
			SidedefDynamic, Thing, ThingFlags,
		},
	},
};
use anyhow::bail;
use legion::{
	any,
	systems::{CommandBuffer, ResourceSet},
	Entity, IntoQuery, Read, Resources, World,
};
use nalgebra::{Vector2, Vector3};

#[derive(Clone, Debug)]
pub struct SpawnContext {
	pub template_handle: AssetHandle<EntityTemplate>,
	pub transform: Transform,
	pub sector_interval: Interval,
}

pub fn spawn_things(
	things: Vec<Thing>,
	world: &mut World,
	resources: &mut Resources,
) -> anyhow::Result<()> {
	let mut command_buffer = CommandBuffer::new(world);

	for (i, thing) in things.into_iter().enumerate() {
		if thing.flags.intersects(ThingFlags::DMONLY) {
			continue;
		}

		if !thing.flags.intersects(ThingFlags::EASY) {
			continue;
		}

		let spawn_context = {
			let asset_storage = <Read<AssetStorage>>::fetch(resources);

			// Find entity template
			let (template_handle, _) = match asset_storage
				.iter::<EntityTemplate>()
				.find(|(_, template)| template.type_id == Some(EntityTypeId::Thing(thing.r#type)))
			{
				Some(entry) => entry,
				None => {
					log::warn!("Thing {} has invalid thing type {}", i, thing.r#type);
					continue;
				}
			};

			let sector_interval = {
				let map_dynamic = <&MapDynamic>::query().iter(world).next().unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let ssect = map.find_subsector(thing.position);
				map_dynamic.sectors[ssect.sector_index].interval
			};

			let transform = Transform {
				position: Vector3::new(thing.position[0], thing.position[1], f32::NAN),
				rotation: Vector3::new(0.into(), 0.into(), thing.angle),
			};

			SpawnContext {
				template_handle: template_handle.clone(),
				transform,
				sector_interval,
			}
		};

		resources.insert(spawn_context);

		let (asset_storage, handler_set, spawn_context) = <(
			Read<AssetStorage>,
			Read<SpawnMergerHandlerSet>,
			Read<SpawnContext>,
		)>::fetch(resources);
		let template = asset_storage.get(&spawn_context.template_handle).unwrap();

		if template.world.is_empty() {
			log::debug!("Thing {} has empty template world", i);
			command_buffer.push(());
		} else {
			// Create entity and add components
			let mut merger = SpawnMerger::new(&handler_set, &resources);
			world.clone_from(&template.world, &any(), &mut merger);
		}
	}

	resources.remove::<SpawnContext>();
	command_buffer.flush(world);
	Ok(())
}

pub fn spawn_player(world: &mut World, resources: &mut Resources) -> anyhow::Result<Entity> {
	let mut command_buffer = CommandBuffer::new(world);

	let spawn_context = {
		// Get spawn point transform
		let transform = match <(&Transform, &SpawnPoint)>::query()
			.iter(world)
			.find_map(|(t, s)| if s.player_num == 1 { Some(*t) } else { None })
		{
			Some(x) => x,
			None => bail!("Spawn point for player 1 not found"),
		};

		let asset_storage = <Read<AssetStorage>>::fetch(resources);

		let template_handle = match asset_storage.handle_for::<EntityTemplate>("player") {
			Some(template) => template,
			None => bail!("Entity type not found: {}", "player"),
		};

		let sector_interval = {
			let map_dynamic = <&MapDynamic>::query().iter(world).next().unwrap();
			let map = asset_storage.get(&map_dynamic.map).unwrap();
			let ssect = map.find_subsector(transform.position.fixed_resize(0.0));
			map_dynamic.sectors[ssect.sector_index].interval
		};

		SpawnContext {
			template_handle: template_handle.clone(),
			transform,
			sector_interval,
		}
	};

	resources.insert(spawn_context);

	let entity = {
		// Fetch entity template
		let (asset_storage, handler_set, spawn_context) = <(
			Read<AssetStorage>,
			Read<SpawnMergerHandlerSet>,
			Read<SpawnContext>,
		)>::fetch(resources);
		let template = asset_storage.get(&spawn_context.template_handle).unwrap();

		// Create entity and add components
		let mut merger = SpawnMerger::new(&handler_set, &resources);
		let entity_map = world.clone_from(&template.world, &any(), &mut merger);
		entity_map.into_iter().map(|(_, to)| to).next().unwrap()
	};

	resources.remove::<SpawnContext>();
	command_buffer.flush(world);

	Ok(entity)
}

pub fn spawn_map_entities(
	world: &mut World,
	resources: &mut Resources,
	map_handle: &AssetHandle<Map>,
) -> anyhow::Result<()> {
	let mut command_buffer = CommandBuffer::new(world);
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
			let (handle, template) = match asset_storage
				.iter::<EntityTemplate>()
				.find(|(_, template)| template.type_id == Some(EntityTypeId::Linedef(special_type)))
			{
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
			let (handle, template) = match asset_storage
				.iter::<EntityTemplate>()
				.find(|(_, template)| template.type_id == Some(EntityTypeId::Sector(special_type)))
			{
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
					bbox.add_point(linedef.line.point + linedef.line.dir);
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
	command_buffer.flush(world);
	Ok(())
}
