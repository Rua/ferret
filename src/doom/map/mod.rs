pub mod load;
pub mod meshes;
pub mod textures;

use crate::{
	assets::{AssetHandle, AssetStorage},
	doom::{
		components::{SpawnOnCeiling, SpawnPoint, Transform},
		data::{LinedefTypes, MobjTypes, SectorTypes},
		map::{
			load::LinedefFlags,
			textures::{Flat, TextureType, Wall},
		},
		physics::SolidMask,
	},
	geometry::{Angle, Interval, Line2, Plane2, Plane3, Side, AABB2},
	timer::Timer,
};
use anyhow::anyhow;
use bitflags::bitflags;
use fnv::FnvHashMap;
use legion::prelude::{
	CommandBuffer, Entity, IntoQuery, Read, ResourceSet, Resources, World, Write,
};
use nalgebra::{Vector2, Vector3};
use serde::Deserialize;
use std::{fmt::Debug, time::Duration};

#[derive(Debug)]
pub struct Map {
	pub anims_flat: FnvHashMap<AssetHandle<Flat>, Anim<Flat>>,
	pub anims_wall: FnvHashMap<AssetHandle<Wall>, Anim<Wall>>,
	pub bbox: AABB2,
	pub linedefs: Vec<Linedef>,
	pub nodes: Vec<Node>,
	pub sectors: Vec<Sector>,
	pub subsectors: Vec<Subsector>,
	pub sky: AssetHandle<Wall>,
	pub switches: FnvHashMap<AssetHandle<Wall>, AssetHandle<Wall>>,
}

#[derive(Clone, Debug)]
pub struct MapDynamic {
	pub anim_states_flat: FnvHashMap<AssetHandle<Flat>, AnimState>,
	pub anim_states_wall: FnvHashMap<AssetHandle<Wall>, AnimState>,
	pub map: AssetHandle<Map>,
	pub linedefs: Vec<LinedefDynamic>,
	pub sectors: Vec<SectorDynamic>,
}

#[derive(Clone, Debug)]
pub struct Anim<T> {
	pub frames: Vec<AssetHandle<T>>,
	pub frame_time: Duration,
}

#[derive(Clone, Copy, Debug)]
pub struct AnimState {
	pub frame: usize,
	pub timer: Timer,
}

pub struct Thing {
	pub position: Vector2<f32>,
	pub angle: Angle,
	pub doomednum: u16,
	pub flags: ThingFlags,
}

bitflags! {
	#[derive(Deserialize)]
	pub struct ThingFlags: u16 {
		const EASY = 0b00000000_00000001;
		const NORMAL = 0b00000000_00000010;
		const HARD = 0b00000000_00000100;
		const DEAF = 0b00000000_00001000;
		const DMONLY = 0b00000000_00010000;
	}
}

#[derive(Clone, Debug)]
pub struct Linedef {
	pub line: Line2,
	pub normal: Vector2<f32>,
	pub planes: Vec<Plane3>,
	pub bbox: AABB2,
	pub flags: LinedefFlags,
	pub solid_mask: SolidMask,
	pub special_type: u16,
	pub sector_tag: u16,
	pub sidedefs: [Option<Sidedef>; 2],
}

#[derive(Clone, Debug)]
pub struct LinedefDynamic {
	pub entity: Entity,
	pub sidedefs: [Option<SidedefDynamic>; 2],
	pub texture_offset: Vector2<f32>,
}

#[derive(Clone, Debug)]
pub struct Sidedef {
	pub texture_offset: Vector2<f32>,
	pub textures: [TextureType<Wall>; 3],
	pub sector_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SidedefSlot {
	Top = 0,
	Bottom = 1,
	Middle = 2,
}

#[derive(Clone, Debug)]
pub struct SidedefDynamic {
	pub textures: [TextureType<Wall>; 3],
}

#[derive(Clone, Debug)]
pub struct LinedefRef {
	pub map_entity: Entity,
	pub index: usize,
}

#[derive(Clone, Debug)]
pub struct Seg {
	pub line: Line2,
	pub normal: Vector2<f32>,
	pub linedef: Option<(usize, Side)>,
}

#[derive(Clone, Debug)]
pub struct Subsector {
	pub segs: Vec<Seg>,
	pub bbox: AABB2,
	pub planes: Vec<Plane3>,
	pub linedefs: Vec<usize>,
	pub sector_index: usize,
}

#[derive(Clone, Debug)]
pub struct Node {
	pub plane: Plane2,
	pub linedefs: Vec<usize>,
	pub child_bboxes: [AABB2; 2],
	pub child_indices: [NodeChild; 2],
}

#[derive(Copy, Clone, Debug)]
pub enum NodeChild {
	Subsector(usize),
	Node(usize),
}

#[derive(Clone, Debug)]
pub struct Sector {
	pub interval: Interval,
	pub textures: [TextureType<Flat>; 2],
	pub light_level: f32,
	pub special_type: u16,
	pub sector_tag: u16,
	pub linedefs: Vec<usize>,
	pub subsectors: Vec<usize>,
	pub neighbours: Vec<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SectorSlot {
	Floor = 0,
	Ceiling = 1,
}

#[derive(Clone, Debug)]
pub struct SectorDynamic {
	pub entity: Entity,
	pub light_level: f32,
	pub interval: Interval,
}

#[derive(Clone, Debug)]
pub struct SectorRef {
	pub map_entity: Entity,
	pub index: usize,
}

impl Map {
	pub fn find_subsector(&self, point: Vector2<f32>) -> &Subsector {
		let mut child = NodeChild::Node(0);

		loop {
			child = match child {
				NodeChild::Subsector(index) => return &self.subsectors[index],
				NodeChild::Node(index) => {
					let node = &self.nodes[index];
					let dot = point.dot(&node.plane.normal) - node.plane.distance;
					node.child_indices[(dot <= 0.0) as usize]
				}
			};
		}
	}

	pub fn traverse_nodes<F: FnMut(NodeChild)>(&self, node: NodeChild, bbox: &AABB2, func: &mut F) {
		func(node);

		if let NodeChild::Node(index) = node {
			let node = &self.nodes[index];
			let sides = [
				Vector2::new(bbox[0].min, bbox[1].min).dot(&node.plane.normal)
					- node.plane.distance,
				Vector2::new(bbox[0].min, bbox[1].max).dot(&node.plane.normal)
					- node.plane.distance,
				Vector2::new(bbox[0].max, bbox[1].min).dot(&node.plane.normal)
					- node.plane.distance,
				Vector2::new(bbox[0].max, bbox[1].max).dot(&node.plane.normal)
					- node.plane.distance,
			];

			if sides.iter().any(|x| *x >= 0.0) {
				self.traverse_nodes(node.child_indices[Side::Right as usize], bbox, func);
			}

			if sides.iter().any(|x| *x <= 0.0) {
				self.traverse_nodes(node.child_indices[Side::Left as usize], bbox, func);
			}
		}
	}

	pub fn lowest_neighbour_floor(&self, map_dynamic: &MapDynamic, sector_index: usize) -> f32 {
		self.sectors[sector_index]
			.neighbours
			.iter()
			.map(|index| map_dynamic.sectors[*index].interval.min)
			.min_by(|x, y| x.partial_cmp(y).unwrap())
			.unwrap_or(self.sectors[sector_index].interval.min)
	}

	pub fn lowest_neighbour_floor_above(
		&self,
		map_dynamic: &MapDynamic,
		sector_index: usize,
		height: f32,
	) -> f32 {
		self.sectors[sector_index]
			.neighbours
			.iter()
			.map(|index| map_dynamic.sectors[*index].interval.min)
			.filter(|h| *h > height)
			.min_by(|x, y| x.partial_cmp(y).unwrap())
			.unwrap_or(self.sectors[sector_index].interval.min)
	}

	pub fn highest_neighbour_floor(&self, map_dynamic: &MapDynamic, sector_index: usize) -> f32 {
		self.sectors[sector_index]
			.neighbours
			.iter()
			.map(|index| map_dynamic.sectors[*index].interval.min)
			.max_by(|x, y| x.partial_cmp(y).unwrap())
			.unwrap_or(-500.0)
	}

	pub fn lowest_neighbour_ceiling(&self, map_dynamic: &MapDynamic, sector_index: usize) -> f32 {
		self.sectors[sector_index]
			.neighbours
			.iter()
			.map(|index| map_dynamic.sectors[*index].interval.max)
			.min_by(|x, y| x.partial_cmp(y).unwrap())
			.unwrap_or(32768.0)
	}

	/*pub fn highest_neighbour_ceiling(&self, map_dynamic: &MapDynamic, sector_index: usize) -> f32 {
		self.sectors[sector_index]
			.neighbours
			.iter()
			.map(|index| map_dynamic.sectors[*index].interval.max)
			.max_by(|x, y| x.partial_cmp(y).unwrap())
			.unwrap_or(0.0)
	}*/
}

pub fn spawn_things(
	things: Vec<Thing>,
	world: &mut World,
	resources: &mut Resources,
	map_handle: &AssetHandle<Map>,
) -> anyhow::Result<()> {
	let (asset_storage, entity_types) =
		<(Read<AssetStorage>, Read<MobjTypes>)>::fetch_mut(resources);

	let mut command_buffer = CommandBuffer::new(world);

	for (i, thing) in things.into_iter().enumerate() {
		if thing.flags.intersects(ThingFlags::DMONLY) {
			continue;
		}

		if !thing.flags.intersects(ThingFlags::EASY) {
			continue;
		}

		// Fetch entity template
		let handle = match entity_types.doomednums.get(&thing.doomednum) {
			Some(some) => some,
			None => {
				log::warn!("Thing {} has invalid thing type {}", i, thing.doomednum);
				continue;
			}
		};
		let template = asset_storage.get(handle).unwrap();

		// Create entity and add components
		let entity = command_buffer.insert((), vec![()])[0];
		template
			.components
			.add_to_entity(entity, &mut command_buffer);

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

	command_buffer.write(world);

	// TODO very ugly way to do it
	for (entity, (spawn_on_ceiling, mut transform)) in
		<(Read<SpawnOnCeiling>, Write<Transform>)>::query().iter_entities_mut(world)
	{
		command_buffer.remove_component::<SpawnOnCeiling>(entity);

		let map = asset_storage.get(&map_handle).unwrap();
		let position = Vector2::new(transform.position[0], transform.position[1]);
		let ssect = map.find_subsector(position);
		let sector = &map.sectors[ssect.sector_index];
		transform.position[2] = sector.interval.max - spawn_on_ceiling.offset;
	}

	command_buffer.write(world);
	Ok(())
}

pub fn spawn_player(world: &mut World, resources: &mut Resources) -> anyhow::Result<Entity> {
	let mut command_buffer = CommandBuffer::new(world);

	// Get spawn point transform
	let transform = <(Read<Transform>, Read<SpawnPoint>)>::query()
		.iter(world)
		.find_map(|(t, s)| if s.player_num == 1 { Some(*t) } else { None })
		.unwrap();

	// Fetch entity template
	let (asset_storage, entity_types) =
		<(Read<AssetStorage>, Read<MobjTypes>)>::fetch_mut(resources);
	let handle = entity_types
		.names
		.get("PLAYER")
		.ok_or(anyhow!("Entity type not found: {}", "PLAYER"))?;
	let template = asset_storage.get(handle).unwrap();

	// Create entity and add components
	let entity = command_buffer.insert((), vec![()])[0];
	template
		.components
		.add_to_entity(entity, &mut command_buffer);
	command_buffer.add_component(entity, transform);

	command_buffer.write(world);

	Ok(entity)
}

pub fn spawn_map_entities(
	world: &mut World,
	resources: &Resources,
	map_handle: &AssetHandle<Map>,
) -> anyhow::Result<()> {
	let mut command_buffer = CommandBuffer::new(world);
	let (asset_storage, linedef_types, sector_types) =
		<(Read<AssetStorage>, Read<LinedefTypes>, Read<SectorTypes>)>::fetch(resources);
	let map = asset_storage.get(&map_handle).unwrap();

	// Create map entity
	let map_entity = command_buffer.insert((), vec![()])[0];
	let anim_states_flat = map
		.anims_flat
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
	let anim_states_wall = map
		.anims_wall
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
		anim_states_flat,
		anim_states_wall,
		map: map_handle.clone(),
		linedefs: Vec::with_capacity(map.linedefs.len()),
		sectors: Vec::with_capacity(map.sectors.len()),
	};

	// Create linedef entities
	for (i, linedef) in map.linedefs.iter().enumerate() {
		// Create entity and set reference
		let entity = command_buffer.insert((), vec![()])[0];
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

		if linedef.special_type == 0 {
			continue;
		}

		// Fetch and add entity template
		let handle = match linedef_types.doomednums.get(&linedef.special_type) {
			Some(some) => some,
			None => {
				log::warn!(
					"Linedef {} has invalid special type {}",
					i,
					linedef.special_type
				);
				continue;
			}
		};

		let template = asset_storage.get(handle).unwrap();
		template
			.components
			.add_to_entity(entity, &mut command_buffer);

		if template.components.len() == 0 {
			log::debug!(
				"Linedef {} has special type {} with empty template",
				i,
				linedef.special_type
			);
		}
	}

	// Create sector entities
	for (i, sector) in map.sectors.iter().enumerate() {
		// Create entity and set reference
		let entity = command_buffer.insert((), vec![()])[0];
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

		if sector.special_type == 0 {
			continue;
		}

		// Fetch and add entity template
		let handle = match sector_types.doomednums.get(&sector.special_type) {
			Some(some) => some,
			None => {
				log::warn!(
					"Sector {} has invalid special type {}",
					i,
					sector.special_type
				);
				continue;
			}
		};

		let template = asset_storage.get(handle).unwrap();
		template
			.components
			.add_to_entity(entity, &mut command_buffer);

		if template.components.len() == 0 {
			log::debug!(
				"Sector {} has special type {} with empty template",
				i,
				sector.special_type
			);
		}
	}

	command_buffer.add_component(map_entity, map_dynamic);
	command_buffer.write(world);
	Ok(())
}
