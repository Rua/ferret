pub mod load;
pub mod meshes;
pub mod textures;

use crate::{
	assets::{AssetHandle, AssetStorage},
	component::EntityTemplate,
	doom::{
		components::{
			LinedefDynamic, MapDynamic, SectorDynamic, SpawnOnCeiling, SpawnPoint, Transform,
		},
		entities::{LinedefTypes, MobjTypes, SectorTypes},
		map::{
			load::{LinedefFlags, ThingData},
			textures::{Flat, TextureType, WallTexture},
		},
	},
	geometry::{BoundingBox2, Line2, Side},
};
use nalgebra::{Vector2, Vector3};
use specs::{
	storage::StorageEntry, Entity, Join, ReadExpect, ReadStorage, World, WorldExt, WriteStorage,
};
use std::{error::Error, fmt::Debug};

#[derive(Clone, Debug)]
pub struct Map {
	pub linedefs: Vec<Linedef>,
	pub sectors: Vec<Sector>,
	pub gl_nodes: Vec<GLNode>,
	pub sky: AssetHandle<WallTexture>,
}

impl Map {
	pub fn find_subsector(&self, point: Vector2<f32>) -> &LeafNode {
		let mut node = 0;

		loop {
			node = match &self.gl_nodes[node] {
				GLNode::Leaf(leaf) => return &leaf,
				GLNode::Branch(branch) => branch.child_indices[branch.point_side(point) as usize],
			};
		}
	}
}

#[derive(Clone, Debug)]
pub struct Linedef {
	pub line: Line2,
	pub normal: Vector2<f32>,
	pub bbox: BoundingBox2,
	pub flags: LinedefFlags,
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

	pub fn touches_bbox(&self, bbox: &BoundingBox2) -> bool {
		if !self.bbox.overlaps(bbox) {
			return false;
		}

		let sides = [
			self.line.point_side(Vector2::new(bbox.min[0], bbox.min[1])),
			self.line.point_side(Vector2::new(bbox.min[0], bbox.max[1])),
			self.line.point_side(Vector2::new(bbox.max[0], bbox.min[1])),
			self.line.point_side(Vector2::new(bbox.max[0], bbox.max[1])),
		];

		!(sides.iter().all(|x| *x < 0.0) || sides.iter().all(|x| *x > 0.0))
	}
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
pub struct Sector {
	pub floor_height: f32,
	pub ceiling_height: f32,
	pub floor_texture: TextureType<Flat>,
	pub ceiling_texture: TextureType<Flat>,
	pub light_level: f32,
	pub special_type: u16,
	pub sector_tag: u16,
	pub subsectors: Vec<Vec<Vector2<f32>>>,
	pub neighbours: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct GLSeg {
	pub vertices: [Vector2<f32>; 2],
	pub linedef_index: Option<usize>,
	pub linedef_side: Side,
	pub partner_seg_index: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct LeafNode {
	pub segs: Vec<GLSeg>,
	pub sector_index: usize,
}

#[derive(Clone, Debug)]
pub struct BranchNode {
	pub partition_line: Line2,
	pub child_bboxes: [BoundingBox2; 2],
	pub child_indices: [usize; 2],
}

impl BranchNode {
	pub fn point_side(&self, point: Vector2<f32>) -> Side {
		if self.partition_line.point_side(point) < 0.0 {
			Side::Right
		} else {
			Side::Left
		}
	}
}

#[derive(Clone, Debug)]
pub enum GLNode {
	Leaf(LeafNode),
	Branch(BranchNode),
}

pub fn spawn_things(
	things: Vec<ThingData>,
	world: &World,
	map_handle: &AssetHandle<Map>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
	for thing in things {
		// Fetch entity template
		let (entity_types, template_storage) = world.system_data::<(
			ReadExpect<MobjTypes>,
			ReadExpect<AssetStorage<EntityTemplate>>,
		)>();
		let handle = entity_types
			.doomednums
			.get(&thing.doomednum)
			.ok_or(
				Box::from(format!("Doomednum not found: {}", thing.doomednum))
					as Box<dyn Error + Send + Sync>,
			)?;
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
				sector.ceiling_height - entry.remove().offset
			} else {
				sector.floor_height
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

pub fn spawn_player(world: &World) -> Result<Entity, Box<dyn Error + Send + Sync>> {
	// Get spawn point transform
	let transform = {
		let (transform, spawn_point) =
			world.system_data::<(ReadStorage<Transform>, ReadStorage<SpawnPoint>)>();

		(&transform, &spawn_point)
			.join()
			.find_map(|(t, s)| {
				if s.player_num == 1 {
					Some(t.clone())
				} else {
					None
				}
			})
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
		.ok_or(Box::from(format!("Entity type not found: {}", "PLAYER"))
			as Box<dyn Error + Send + Sync>)?;
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
) -> Result<(), Box<dyn Error + Send + Sync>> {
	let (
		map_storage,
		mut map_dynamic_component,
		template_storage,
		linedef_types,
		mut linedef_dynamic_component,
		sector_types,
		mut sector_dynamic_component,
		mut transform_component,
	) = world.system_data::<(
		ReadExpect<AssetStorage<Map>>,
		WriteStorage<MapDynamic>,
		ReadExpect<AssetStorage<EntityTemplate>>,
		ReadExpect<LinedefTypes>,
		WriteStorage<LinedefDynamic>,
		ReadExpect<SectorTypes>,
		WriteStorage<SectorDynamic>,
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
		map_dynamic.linedefs.push(entity);
		linedef_dynamic_component.insert(
			entity,
			LinedefDynamic {
				map_entity,
				index: i,

				texture_offset: Vector2::new(0.0, 0.0),
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
				.ok_or(Box::from(format!(
					"Linedef special type not found: {}",
					linedef.special_type
				)) as Box<dyn Error + Send + Sync>)?;
		let template = template_storage.get(handle).unwrap();
		template.add_to_entity(entity, world)?;
	}

	// Create sector entities
	for (i, sector) in map.sectors.iter().enumerate() {
		// Create entity and set reference
		let entity = world.entities().create();
		map_dynamic.sectors.push(entity);
		sector_dynamic_component.insert(
			entity,
			SectorDynamic {
				map_entity,
				index: i,

				light_level: sector.light_level,
				floor_height: sector.floor_height,
				ceiling_height: sector.ceiling_height,
			},
		)?;

		// Find midpoint of sector for sound purposes
		let mut bbox = BoundingBox2::zero();

		for linedef in map.linedefs.iter() {
			for sidedef in linedef.sidedefs.iter().flatten() {
				if sidedef.sector_index == i {
					bbox.add_point(linedef.line.point);
					bbox.add_point(linedef.line.point + linedef.line.dir);
				}
			}
		}

		let midpoint = (bbox.min + bbox.max) / 2.0;

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
			.ok_or(Box::from(format!(
				"Sector special type not found: {}",
				sector.special_type
			)) as Box<dyn Error + Send + Sync>)?;
		let template = template_storage.get(handle).unwrap();
		template.add_to_entity(entity, world)?;
	}

	map_dynamic_component.insert(map_entity, map_dynamic)?;

	Ok(())
}
