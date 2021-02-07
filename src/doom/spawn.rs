use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		geometry::{Interval, AABB2, AABB3},
		quadtree::Quadtree,
		spawn::{SpawnContext, SpawnMerger, SpawnMergerHandlerSet},
		time::{GameTime, Timer},
	},
	doom::{
		components::{SpawnPoint, Transform},
		data::DOOMEDNUMS,
		exit::NextMap,
		map::{
			AnimState, LinedefDynamic, LinedefRef, Map, MapDynamic, SectorDynamic, SectorRef,
			SidedefDynamic, Thing, ThingFlags,
		},
		physics::BoxCollider,
		template::EntityTemplate,
	},
};
use anyhow::bail;
use arrayvec::ArrayString;
use legion::{
	any, systems::ResourceSet, world::EntityHasher, Entity, IntoQuery, Read, Resources, World,
	Write,
};
use nalgebra::{Vector2, Vector3};
use std::{collections::HashMap, fmt::Write as _};

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
		let template_handle: AssetHandle<EntityTemplate> = {
			let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);

			match DOOMEDNUMS.get(&thing.r#type) {
				Some(template_name) => asset_storage.load::<EntityTemplate>(template_name),
				None => {
					log::warn!("Thing {} has invalid thing type {}", i, thing.r#type);
					continue;
				}
			}
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
		let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
		asset_storage.load::<EntityTemplate>("player.entity")
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
	let (map_entity, num_linedefs, num_sectors) = {
		let (asset_storage, game_time) = <(Read<AssetStorage>, Read<GameTime>)>::fetch(resources);
		let map = asset_storage.get(&map_handle).unwrap();

		// Create map entity
		let anim_states = map
			.anims
			.iter()
			.map(|(k, v)| {
				(
					k.clone(),
					AnimState {
						frame: 0,
						timer: Timer::new(*game_time, v.frame_time),
					},
				)
			})
			.collect();

		let num_linedefs = map.linedefs.len();
		let num_sectors = map.sectors.len();
		let map_entity = world.push((MapDynamic {
			anim_states,
			map: map_handle.clone(),
			linedefs: Vec::with_capacity(num_linedefs),
			sectors: Vec::with_capacity(num_sectors),
		},));

		(map_entity, num_linedefs, num_sectors)
	};

	resources.insert(SpawnContext(NextMap("e1m2".into())));
	let mut query = <&mut MapDynamic>::query();

	// Create linedef entities

	for i in 0..num_linedefs {
		// Load the entity template handle
		let special_type = {
			let asset_storage = <Read<AssetStorage>>::fetch(resources);
			asset_storage.get(&map_handle).unwrap().linedefs[i].special_type
		};

		let template_handle = {
			let mut template_name = ArrayString::<[u8; 20]>::new();
			write!(template_name, "linedef{}.entity", special_type)?;
			let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
			asset_storage.load::<EntityTemplate>(&template_name)
		};

		// Spawn the entity
		resources.insert(SpawnContext(LinedefRef {
			map_entity,
			index: i,
		}));
		resources.insert(SpawnContext(template_handle.clone()));

		let linedef_entity = {
			let asset_storage = <Read<AssetStorage>>::fetch(resources);
			let template = asset_storage.get(&template_handle).unwrap();
			let entity_map = spawn_helper(&template.world, world, resources);
			entity_map.into_iter().map(|(_, to)| to).next().unwrap()
		};

		resources.remove::<SpawnContext<LinedefRef>>();
		resources.remove::<SpawnContext<AssetHandle<EntityTemplate>>>();

		// Add to MapDynamic
		let map_dynamic = query.get_mut(world, map_entity).unwrap();
		let asset_storage = <Read<AssetStorage>>::fetch(resources);
		let linedef = &asset_storage.get(&map_handle).unwrap().linedefs[i];
		let sidedefs = [
			linedef.sidedefs[0].as_ref().map(|sidedef| SidedefDynamic {
				textures: sidedef.textures.clone(),
			}),
			linedef.sidedefs[1].as_ref().map(|sidedef| SidedefDynamic {
				textures: sidedef.textures.clone(),
			}),
		];
		map_dynamic.linedefs.push(LinedefDynamic {
			entity: linedef_entity,
			sidedefs,
			texture_offset: Vector2::new(0.0, 0.0),
		});
	}

	// Create sector entities
	for i in 0..num_sectors {
		// Load the entity template handle
		let special_type = {
			let asset_storage = <Read<AssetStorage>>::fetch(resources);
			asset_storage.get(&map_handle).unwrap().sectors[i].special_type
		};

		let template_handle = {
			let mut template_name = ArrayString::<[u8; 20]>::new();
			write!(template_name, "sector{}.entity", special_type)
				.expect("Insufficient capacity for name");
			let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
			asset_storage.load::<EntityTemplate>(&template_name)
		};

		// Find midpoint of sector for sound purposes
		let transform = {
			let mut bbox = AABB2::empty();
			let asset_storage = <Read<AssetStorage>>::fetch(resources);
			let map = asset_storage.get(&map_handle).unwrap();

			for linedef in map.linedefs.iter() {
				for sidedef in linedef.sidedefs.iter().flatten() {
					if sidedef.sector_index == i {
						bbox.add_point(linedef.line.point);
						bbox.add_point(linedef.line.end_point());
					}
				}
			}

			let midpoint = (bbox.min() + bbox.max()) / 2.0;

			Transform {
				position: Vector3::new(midpoint[0], midpoint[1], 0.0),
				rotation: Vector3::new(0.into(), 0.into(), 0.into()),
			}
		};

		// Spawn the entity
		resources.insert(SpawnContext(transform));
		resources.insert(SpawnContext(SectorRef {
			map_entity,
			index: i,
		}));
		resources.insert(SpawnContext(template_handle.clone()));

		let sector_entity = {
			let asset_storage = <Read<AssetStorage>>::fetch(resources);
			let template = asset_storage.get(&template_handle).unwrap();
			let entity_map = spawn_helper(&template.world, world, resources);
			entity_map.into_iter().map(|(_, to)| to).next().unwrap()
		};

		resources.remove::<SpawnContext<SectorRef>>();
		resources.remove::<SpawnContext<AssetHandle<EntityTemplate>>>();

		// Add to MapDynamic
		let map_dynamic = query.get_mut(world, map_entity).unwrap();
		let asset_storage = <Read<AssetStorage>>::fetch(resources);
		let sector = &asset_storage.get(&map_handle).unwrap().sectors[i];
		map_dynamic.sectors.push(SectorDynamic {
			entity: sector_entity,
			light_level: sector.light_level,
			interval: sector.interval,
		});
	}

	resources.remove::<SpawnContext<NextMap>>();
	Ok(())
}
