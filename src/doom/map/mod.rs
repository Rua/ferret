pub mod load;
pub mod meshes;
pub mod textures;

use anyhow::anyhow;
use crate::{
	assets::{AssetHandle, AssetStorage},
	component::EntityTemplate,
	doom::{
		components::{SpawnOnCeiling, SpawnPoint, Transform},
		entities::{LinedefTypes, MobjTypes, SectorTypes},
		map::{
			load::{LinedefFlags, ThingData},
			textures::{Flat, TextureType, WallTexture},
		},
		physics::SolidMask,
	},
	geometry::{Interval, Line2, Side, AABB2},
};
use nalgebra::{Vector2, Vector3};
use specs::{
	storage::StorageEntry, Component, DenseVecStorage, Entity, Join, ReadExpect, ReadStorage,
	World, WorldExt, WriteStorage,
};
use specs_derive::Component;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Map {
	pub linedefs: Vec<Linedef>,
	pub sectors: Vec<Sector>,
	pub subsectors: Vec<GLSSect>,
	pub nodes: Vec<GLNode>,
	pub sky: AssetHandle<WallTexture>,
}

impl Map {
	pub fn find_subsector(&self, point: Vector2<f32>) -> &GLSSect {
		let mut child = NodeChild::Node(0);

		loop {
			child = match child {
				NodeChild::Subsector(index) => return &self.subsectors[index],
				NodeChild::Node(index) => {
					let node = &self.nodes[index];
					node.child_indices[node.point_side(point) as usize]
				}
			};
		}
	}
}

#[derive(Clone, Component, Debug)]
pub struct MapDynamic {
	pub map: AssetHandle<Map>,
	pub linedefs: Vec<LinedefDynamic>,
	pub sectors: Vec<SectorDynamic>,
}

#[derive(Clone, Debug)]
pub struct Linedef {
	pub line: Line2,
	pub normal: Vector2<f32>,
	pub bbox: AABB2,
	pub flags: LinedefFlags,
	pub solid_mask: SolidMask,
	pub special_type: u16,
	pub sector_tag: u16,
	pub sidedefs: [Option<Sidedef>; 2],
}

impl Linedef {
	pub fn point_side(&self, point: Vector2<f32>) -> Side {
		if self.line.point_side(point) < 0.0 {
			Side::Right
		} else {
			Side::Left
		}
	}

	/*pub fn touches_bbox(&self, bbox: &AABB2) -> bool {
		if !self.bbox.overlaps(bbox) {
			return false;
		}

		let sides = [
			self.line.point_side(Vector2::new(bbox[0].min, bbox[1].min)),
			self.line.point_side(Vector2::new(bbox[0].min, bbox[1].max)),
			self.line.point_side(Vector2::new(bbox[0].max, bbox[1].min)),
			self.line.point_side(Vector2::new(bbox[0].max, bbox[1].max)),
		];

		!(sides.iter().all(|x| *x < 0.0) || sides.iter().all(|x| *x > 0.0))
	}*/
}

#[derive(Clone, Debug)]
pub struct Sidedef {
	pub texture_offset: Vector2<f32>,
	pub top_texture: TextureType<WallTexture>,
	pub bottom_texture: TextureType<WallTexture>,
	pub middle_texture: TextureType<WallTexture>,
	pub sector_index: usize,
}

#[derive(Clone, Debug)]
pub struct LinedefDynamic {
	pub entity: Entity,
	pub texture_offset: Vector2<f32>,
}

#[derive(Clone, Component, Debug)]
pub struct LinedefRef {
	pub map_entity: Entity,
	pub index: usize,
}

#[derive(Clone, Debug)]
pub struct Sector {
	pub interval: Interval,
	pub floor_texture: TextureType<Flat>,
	pub ceiling_texture: TextureType<Flat>,
	pub light_level: f32,
	pub special_type: u16,
	pub sector_tag: u16,
	pub subsectors: Vec<usize>,
	pub neighbours: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct SectorDynamic {
	pub entity: Entity,
	pub light_level: f32,
	pub interval: Interval,
}

#[derive(Clone, Component, Debug)]
pub struct SectorRef {
	pub map_entity: Entity,
	pub index: usize,
}

#[derive(Clone, Debug)]
pub struct GLSSect {
	pub segs: Vec<GLSeg>,
	pub sector_index: usize,
	pub bbox: AABB2,
}

#[derive(Clone, Debug)]
pub struct GLSeg {
	pub line: Line2,
	pub normal: Vector2<f32>,
	pub interval: Interval,
	pub linedef_index: Option<usize>,
	pub linedef_side: Side,
	//pub partner_seg_index: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct GLNode {
	pub partition_line: Line2,
	pub child_bboxes: [AABB2; 2],
	pub child_indices: [NodeChild; 2],
}

impl GLNode {
	pub fn point_side(&self, point: Vector2<f32>) -> Side {
		if self.partition_line.point_side(point) < 0.0 {
			Side::Right
		} else {
			Side::Left
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub enum NodeChild {
	Subsector(usize),
	Node(usize),
}

pub fn spawn_things(
	things: Vec<ThingData>,
	world: &World,
	map_handle: &AssetHandle<Map>,
) -> anyhow::Result<()> {
	for thing in things {
		// Fetch entity template
		let (entity_types, template_storage) = world.system_data::<(
			ReadExpect<MobjTypes>,
			ReadExpect<AssetStorage<EntityTemplate>>,
		)>();
		let handle = entity_types
			.doomednums
			.get(&thing.doomednum)
			.ok_or(anyhow!("Doomednum not found: {}", thing.doomednum))?;
		let template = template_storage.get(handle).unwrap();

		// Create entity and add components
		let entity = world.entities().create();
		template.add_to_entity(entity, world)?;

		// Set entity transform
		let z = {
			let (map_storage, mut spawn_on_ceiling_storage) = world
				.system_data::<(ReadExpect<AssetStorage<Map>>, WriteStorage<SpawnOnCeiling>)>();
			let map = map_storage.get(&map_handle).unwrap();
			let ssect = map.find_subsector(thing.position);
			let sector = &map.sectors[ssect.sector_index];

			if let StorageEntry::Occupied(entry) = spawn_on_ceiling_storage.entry(entity)? {
				sector.interval.max - entry.remove().offset
			} else {
				sector.interval.min
			}
		};

		let mut transform_storage = world.system_data::<WriteStorage<Transform>>();
		transform_storage.insert(
			entity,
			Transform {
				position: Vector3::new(thing.position[0], thing.position[1], z),
				rotation: Vector3::new(0.into(), 0.into(), thing.angle),
			},
		)?;
	}

	Ok(())
}

pub fn spawn_player(world: &World) -> anyhow::Result<Entity> {
	// Get spawn point transform
	let transform = {
		let (transform, spawn_point) =
			world.system_data::<(ReadStorage<Transform>, ReadStorage<SpawnPoint>)>();

		(&transform, &spawn_point)
			.join()
			.find_map(|(t, s)| if s.player_num == 1 { Some(*t) } else { None })
			.unwrap()
	};

	// Fetch entity template
	let (entity_types, template_storage) = world.system_data::<(
		ReadExpect<MobjTypes>,
		ReadExpect<AssetStorage<EntityTemplate>>,
	)>();
	let handle = entity_types
		.names
		.get("PLAYER")
		.ok_or(anyhow!("Entity type not found: {}", "PLAYER"))?;
	let template = template_storage.get(handle).unwrap();

	// Create entity and add components
	let entity = world.entities().create();
	template.add_to_entity(entity, world)?;

	// Set entity transform
	let mut transform_storage = world.system_data::<WriteStorage<Transform>>();
	transform_storage.insert(entity, transform)?;

	Ok(entity)
}

pub fn spawn_map_entities(
	world: &World,
	map_handle: &AssetHandle<Map>,
) -> anyhow::Result<()> {
	let (
		map_storage,
		mut map_dynamic_component,
		template_storage,
		linedef_types,
		mut linedef_ref_component,
		sector_types,
		mut sector_ref_component,
		mut transform_component,
	) = world.system_data::<(
		ReadExpect<AssetStorage<Map>>,
		WriteStorage<MapDynamic>,
		ReadExpect<AssetStorage<EntityTemplate>>,
		ReadExpect<LinedefTypes>,
		WriteStorage<LinedefRef>,
		ReadExpect<SectorTypes>,
		WriteStorage<SectorRef>,
		WriteStorage<Transform>,
	)>();
	let map = map_storage.get(&map_handle).unwrap();

	// Create map entity
	let map_entity = world.entities().create();
	let mut map_dynamic = MapDynamic {
		map: map_handle.clone(),
		linedefs: Vec::with_capacity(map.linedefs.len()),
		sectors: Vec::with_capacity(map.sectors.len()),
	};

	// Create linedef entities
	for (i, linedef) in map.linedefs.iter().enumerate() {
		// Create entity and set reference
		let entity = world.entities().create();
		map_dynamic.linedefs.push(LinedefDynamic {
			entity,
			texture_offset: Vector2::new(0.0, 0.0),
		});
		linedef_ref_component.insert(
			entity,
			LinedefRef {
				map_entity,
				index: i,
			},
		)?;

		if linedef.special_type == 0 {
			continue;
		}

		// Fetch and add entity template
		let handle =
			linedef_types
				.doomednums
				.get(&linedef.special_type)
				.ok_or(anyhow!("Linedef special type not found: {}", linedef.special_type))?;
		let template = template_storage.get(handle).unwrap();
		template.add_to_entity(entity, world)?;
	}

	// Create sector entities
	for (i, sector) in map.sectors.iter().enumerate() {
		// Create entity and set reference
		let entity = world.entities().create();
		map_dynamic.sectors.push(SectorDynamic {
			entity,
			light_level: sector.light_level,
			interval: sector.interval,
		});
		sector_ref_component.insert(
			entity,
			SectorRef {
				map_entity,
				index: i,
			},
		)?;

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

		transform_component.insert(
			entity,
			Transform {
				position: Vector3::new(midpoint[0], midpoint[1], 0.0),
				rotation: Vector3::new(0.into(), 0.into(), 0.into()),
			},
		)?;

		if sector.special_type == 0 {
			continue;
		}

		// Fetch and add entity template
		let handle = sector_types
			.doomednums
			.get(&sector.special_type)
			.ok_or(anyhow!("Sector special type not found: {}", sector.special_type))?;
		let template = template_storage.get(handle).unwrap();
		template.add_to_entity(entity, world)?;
	}

	map_dynamic_component.insert(map_entity, map_dynamic)?;

	Ok(())
}
