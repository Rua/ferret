use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		geometry::AABB2,
		time::Timer,
	},
	doom::{
		components::{SpawnOnCeiling, SpawnPoint, Transform},
		entitytemplate::{EntityTemplate, EntityTemplateRef, EntityTypeId},
		map::{
			AnimState, LinedefDynamic, LinedefRef, Map, MapDynamic, SectorDynamic, SectorRef,
			SidedefDynamic, Thing, ThingFlags,
		},
	},
};
use anyhow::bail;
use legion::{
	systems::{CommandBuffer, ResourceSet},
	Entity, IntoQuery, Read, Resources, World,
};
use nalgebra::{Vector2, Vector3};

pub fn spawn_things(
	things: Vec<Thing>,
	world: &mut World,
	resources: &mut Resources,
	map_handle: &AssetHandle<Map>,
) -> anyhow::Result<()> {
	let asset_storage = <Read<AssetStorage>>::fetch_mut(resources);

	let mut command_buffer = CommandBuffer::new(world);

	for (i, thing) in things.into_iter().enumerate() {
		if thing.flags.intersects(ThingFlags::DMONLY) {
			continue;
		}

		if !thing.flags.intersects(ThingFlags::EASY) {
			continue;
		}

		// Fetch entity template
		let (handle, template) = match asset_storage
			.iter::<EntityTemplate>()
			.find(|(_, template)| template.type_id == Some(EntityTypeId::Thing(thing.r#type)))
		{
			Some(entry) => entry,
			None => {
				log::warn!("Thing {} has invalid thing type {}", i, thing.r#type);
				continue;
			}
		};

		// Create entity and add components
		let entity = command_buffer.push(());
		template
			.components
			.add_to_entity(entity, &mut command_buffer);

		// Set entity template reference
		command_buffer.add_component(entity, EntityTemplateRef(handle.clone()));

		// Set entity transform
		let z = {
			let map = asset_storage.get(&map_handle).unwrap();
			let ssect = map.find_subsector(thing.position);
			map.sectors[ssect.sector_index].interval.min
		};

		command_buffer.add_component(
			entity,
			Transform {
				position: Vector3::new(thing.position[0], thing.position[1], z),
				rotation: Vector3::new(0.into(), 0.into(), thing.angle),
			},
		);
	}

	command_buffer.flush(world);

	// TODO very ugly way to do it
	for (entity, spawn_on_ceiling, transform) in
		<(Entity, &SpawnOnCeiling, &mut Transform)>::query().iter_mut(world)
	{
		command_buffer.remove_component::<SpawnOnCeiling>(*entity);

		let map = asset_storage.get(&map_handle).unwrap();
		let position = Vector2::new(transform.position[0], transform.position[1]);
		let ssect = map.find_subsector(position);
		let sector = &map.sectors[ssect.sector_index];
		transform.position[2] = sector.interval.max - spawn_on_ceiling.offset;
	}

	command_buffer.flush(world);
	Ok(())
}

pub fn spawn_player(world: &mut World, resources: &mut Resources) -> anyhow::Result<Entity> {
	let mut command_buffer = CommandBuffer::new(world);

	// Get spawn point transform
	let transform = <(&Transform, &SpawnPoint)>::query()
		.iter(world)
		.find_map(|(t, s)| if s.player_num == 1 { Some(*t) } else { None })
		.unwrap();

	// Fetch entity template
	let asset_storage = <Read<AssetStorage>>::fetch_mut(resources);
	let handle = match asset_storage.handle_for::<EntityTemplate>("player") {
		Some(template) => template,
		None => bail!("Entity type not found: {}", "player"),
	};
	let template = asset_storage.get(&handle).unwrap();

	// Create entity and add components
	let entity = command_buffer.push(());

	template
		.components
		.add_to_entity(entity, &mut command_buffer);
	command_buffer.add_component(entity, transform);

	// Set entity template reference
	command_buffer.add_component(entity, EntityTemplateRef(handle.clone()));

	command_buffer.flush(world);

	Ok(entity)
}

pub fn spawn_map_entities(
	world: &mut World,
	resources: &Resources,
	map_handle: &AssetHandle<Map>,
) -> anyhow::Result<()> {
	let mut command_buffer = CommandBuffer::new(world);
	let asset_storage = <Read<AssetStorage>>::fetch(resources);
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
					timer: Timer::new(v.frame_time),
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
		// Create entity and set reference
		let entity = command_buffer.push(());
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

		if let Some(special_type) = linedef.special_type {
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

			template
				.components
				.add_to_entity(entity, &mut command_buffer);

			// Set entity template reference
			command_buffer.add_component(entity, EntityTemplateRef(handle.clone()));

			if template.components.len() == 0 {
				log::debug!(
					"Linedef {} has special type {} with empty template",
					i,
					special_type
				);
			}
		}
	}

	// Create sector entities
	for (i, sector) in map.sectors.iter().enumerate() {
		// Create entity and set reference
		let entity = command_buffer.push(());
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

		if let Some(special_type) = sector.special_type {
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

			template
				.components
				.add_to_entity(entity, &mut command_buffer);

			// Set entity template reference
			command_buffer.add_component(entity, EntityTemplateRef(handle.clone()));

			if template.components.len() == 0 {
				log::debug!(
					"Sector {} has special type {} with empty template",
					i,
					special_type
				);
			}
		}
	}

	command_buffer.add_component(map_entity, map_dynamic);
	command_buffer.flush(world);
	Ok(())
}
