pub mod lumps;
pub mod meshes;
pub mod textures;

use crate::{
	assets::{Asset, AssetFormat, AssetHandle, AssetStorage, DataSource},
	component::EntityTemplate,
	doom::{
		components::{
			LinedefDynamic, MapDynamic, SectorDynamic, SpawnOnCeiling, SpawnPoint, Transform,
		},
		entities::{LinedefTypes, MobjTypes, SectorTypes},
		map::{
			lumps::{
				ChildNode, EitherVertex, GLNodesFormat, GLSSectFormat, GLSegsFormat, GLVertFormat,
				LinedefFlags, LinedefsFormat, MapData, SectorsFormat, SidedefsFormat, Thing,
				VertexesFormat,
			},
			textures::{Flat, WallTexture},
		},
		wad::WadLoader,
	},
	geometry::{BoundingBox2, Line, Side},
};
use derivative::Derivative;
use nalgebra::{Vector2, Vector3};
use specs::{
	storage::StorageEntry, Entity, Join, ReadExpect, ReadStorage, World, WorldExt, WriteStorage,
};
use std::{
	collections::{hash_map::Entry, HashMap},
	error::Error,
	fmt::Debug,
};

pub fn spawn_things(
	things: Vec<Thing>,
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

#[derive(Clone, Debug)]
pub struct Map {
	pub linedefs: Vec<Linedef>,
	pub sectors: Vec<Sector>,
	pub gl_nodes: Vec<GLNode>,
	pub sky: AssetHandle<WallTexture>,
}

impl Asset for Map {
	type Data = Self;
	type Intermediate = MapData;
	const NAME: &'static str = "Map";

	fn import(
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Intermediate, Box<dyn Error + Send + Sync>> {
		let gl_name = format!("GL_{}", name);

		let linedefs = LinedefsFormat.import(name, source)?;
		let sidedefs = SidedefsFormat.import(name, source)?;
		let vertexes = VertexesFormat.import(name, source)?;
		let sectors = SectorsFormat.import(name, source)?;

		let gl_vert = GLVertFormat.import(&gl_name, source)?;
		let gl_segs = GLSegsFormat.import(&gl_name, source)?;
		let gl_ssect = GLSSectFormat.import(&gl_name, source)?;
		let gl_nodes = GLNodesFormat.import(&gl_name, source)?;

		// Verify all the cross-references

		for (i, sidedef) in sidedefs.iter().enumerate() {
			let index = sidedef.sector_index;
			if index >= sectors.len() {
				return Err(Box::from(format!(
					"Sidedef {} has invalid sector index {}",
					i, index
				)));
			}
		}

		for (i, linedef) in linedefs.iter().enumerate() {
			for index in linedef.sidedef_indices.iter().flatten() {
				if *index >= sidedefs.len() {
					return Err(Box::from(format!(
						"Linedef {} has invalid sidedef index {}",
						i, index
					)));
				}
			}
		}

		for (i, seg) in gl_segs.iter().enumerate() {
			if let Some(index) = seg.linedef_index {
				if index >= linedefs.len() {
					return Err(Box::from(format!(
						"Seg {} has invalid linedef index {}",
						i, index
					)));
				}
			}

			for index in seg.vertex_indices.iter() {
				match *index {
					EitherVertex::GL(index) => {
						if index >= gl_vert.len() {
							return Err(Box::from(format!(
								"Seg {} has invalid vertex index {}",
								i, index
							)));
						}
					}
					EitherVertex::Normal(index) => {
						if index >= vertexes.len() {
							return Err(Box::from(format!(
								"Seg {} has invalid vertex index {}",
								i, index
							)));
						}
					}
				}
			}

			if let Some(index) = seg.partner_seg_index {
				if index >= gl_segs.len() {
					return Err(Box::from(format!(
						"Seg {} has invalid partner seg index {}",
						i, index
					)));
				}
			}
		}

		for (i, ssect) in gl_ssect.iter().enumerate() {
			let index = ssect.first_seg_index;
			if index >= gl_segs.len() {
				return Err(Box::from(format!(
					"Subsector {} has invalid first seg index {}",
					i, ssect.first_seg_index
				)));
			}

			if ssect.first_seg_index + ssect.seg_count > gl_segs.len() {
				return Err(Box::from(format!(
					"Subsector {} has overflowing seg count {}",
					i, ssect.seg_count,
				)));
			}
		}

		for (i, node) in gl_nodes.iter().enumerate() {
			for child in node.child_indices.iter().copied() {
				match child {
					ChildNode::Leaf(index) => {
						if index as usize >= gl_ssect.len() {
							return Err(Box::from(format!(
								"Node {} has invalid subsector index {}",
								i, index
							)));
						}
					}
					ChildNode::Branch(index) => {
						if index as usize >= gl_nodes.len() {
							return Err(Box::from(format!(
								"Node {} has invalid child node index {}",
								i, index
							)));
						}
					}
				}
			}
		}

		Ok(MapData {
			linedefs,
			sidedefs,
			vertexes,
			sectors,
			gl_vert,
			gl_segs,
			gl_ssect,
			gl_nodes,
		})
	}
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

pub fn build_map(
	map_data: MapData,
	sky_name: &str,
	loader: &mut WadLoader,
	flat_storage: &mut AssetStorage<Flat>,
	wall_texture_storage: &mut AssetStorage<WallTexture>,
) -> Result<Map, Box<dyn Error + Send + Sync>> {
	let mut textures = HashMap::new();
	let mut flats = HashMap::new();
	let sky = wall_texture_storage.load(sky_name, loader);

	let MapData {
		linedefs: linedefs_data,
		sidedefs: sidedefs_data,
		vertexes: vertexes_data,
		sectors: sectors_data,
		gl_vert: gl_vert_data,
		gl_segs: gl_segs_data,
		gl_ssect: gl_ssect_data,
		gl_nodes: gl_nodes_data,
	} = map_data;

	let mut sectors = sectors_data
		.into_iter()
		.map(|data| {
			Ok(Sector {
				floor_height: data.floor_height,
				ceiling_height: data.ceiling_height,
				floor_texture: match data.floor_flat_name {
					None => TextureType::None,
					Some(name) if name == "F_SKY1" => TextureType::Sky,
					Some(name) => {
						let handle = match flats.entry(name) {
							Entry::Vacant(entry) => {
								let handle = flat_storage.load(entry.key(), &mut *loader);
								entry.insert(handle)
							}
							Entry::Occupied(entry) => entry.into_mut(),
						};
						TextureType::Normal(handle.clone())
					}
				},
				ceiling_texture: match data.ceiling_flat_name {
					None => TextureType::None,
					Some(name) if name == "F_SKY1" => TextureType::Sky,
					Some(name) => {
						let handle = match flats.entry(name) {
							Entry::Vacant(entry) => {
								let handle = flat_storage.load(entry.key(), &mut *loader);
								entry.insert(handle)
							}
							Entry::Occupied(entry) => entry.into_mut(),
						};
						TextureType::Normal(handle.clone())
					}
				},
				light_level: data.light_level,
				special_type: data.special_type,
				sector_tag: data.special_type,
				subsectors: Vec::new(),
				neighbours: Vec::new(),
			})
		})
		.collect::<Result<Vec<Sector>, Box<dyn Error + Send + Sync>>>()?;

	let mut sidedefs = sidedefs_data
		.into_iter()
		.map(|data| {
			Ok(Some(Sidedef {
				texture_offset: data.texture_offset,
				top_texture: match data.top_texture_name {
					None => TextureType::None,
					Some(name) if name == "F_SKY1" => TextureType::Sky,
					Some(name) => {
						let handle = match textures.entry(name) {
							Entry::Vacant(entry) => {
								let handle = wall_texture_storage.load(entry.key(), &mut *loader);
								entry.insert(handle)
							}
							Entry::Occupied(entry) => entry.into_mut(),
						};
						TextureType::Normal(handle.clone())
					}
				},
				bottom_texture: match data.bottom_texture_name {
					None => TextureType::None,
					Some(name) => {
						let handle = match textures.entry(name.clone()) {
							Entry::Vacant(entry) => {
								let handle = wall_texture_storage.load(entry.key(), &mut *loader);
								entry.insert(handle)
							}
							Entry::Occupied(entry) => entry.into_mut(),
						};
						TextureType::Normal(handle.clone())
					}
				},
				middle_texture: match data.middle_texture_name {
					None => TextureType::None,
					Some(name) => {
						let handle = match textures.entry(name) {
							Entry::Vacant(entry) => {
								let handle = wall_texture_storage.load(entry.key(), &mut *loader);
								entry.insert(handle)
							}
							Entry::Occupied(entry) => entry.into_mut(),
						};
						TextureType::Normal(handle.clone())
					}
				},
				sector_index: data.sector_index,
			}))
		})
		.collect::<Result<Vec<Option<Sidedef>>, Box<dyn Error + Send + Sync>>>()?;

	let linedefs = linedefs_data
		.into_iter()
		.map(|data| {
			let mut sidedefs = [
				data.sidedef_indices[0].map(|x| sidedefs[x].take().unwrap()),
				data.sidedef_indices[1].map(|x| sidedefs[x].take().unwrap()),
			];

			if let [Some(ref mut front_sidedef), Some(ref mut back_sidedef)] = &mut sidedefs {
				// Set sector neighbours
				if front_sidedef.sector_index != back_sidedef.sector_index {
					let front_sector_neighbours =
						&mut sectors[front_sidedef.sector_index].neighbours;
					if !front_sector_neighbours.contains(&back_sidedef.sector_index) {
						front_sector_neighbours.push(back_sidedef.sector_index);
					}

					let back_sector_neighbours = &mut sectors[back_sidedef.sector_index].neighbours;
					if !back_sector_neighbours.contains(&front_sidedef.sector_index) {
						back_sector_neighbours.push(front_sidedef.sector_index);
					}
				}

				// If an upper texture is neighboured by two sky flats, make it sky too
				if sectors[front_sidedef.sector_index].ceiling_texture.is_sky()
					&& sectors[back_sidedef.sector_index].ceiling_texture.is_sky()
				{
					front_sidedef.top_texture = TextureType::Sky;
					back_sidedef.top_texture = TextureType::Sky;
				}
			}

			let dir = vertexes_data[data.vertex_indices[1]] - vertexes_data[data.vertex_indices[0]];

			Ok(Linedef {
				line: Line::new(vertexes_data[data.vertex_indices[0]], dir),
				normal: Vector2::new(dir[1], -dir[0]).normalize(),
				bbox: {
					let mut bbox = BoundingBox2::zero();
					bbox.add_point(vertexes_data[data.vertex_indices[0]]);
					bbox.add_point(vertexes_data[data.vertex_indices[1]]);
					bbox
				},
				flags: data.flags,
				special_type: data.special_type,
				sector_tag: data.sector_tag,
				sidedefs,
			})
		})
		.collect::<Result<Vec<Linedef>, Box<dyn Error + Send + Sync>>>()?;

	let gl_nodes_len = gl_nodes_data.len();
	let mut gl_nodes = gl_nodes_data
		.into_iter()
		.rev()
		.map(|data| {
			Ok(GLNode::Branch(BranchNode {
				partition_line: Line::new(data.partition_point.clone(), data.partition_dir.clone()),
				child_bboxes: data.child_bboxes.clone(),
				child_indices: [
					match data.child_indices[0] {
						ChildNode::Leaf(index) => index + gl_nodes_len,
						ChildNode::Branch(index) => gl_nodes_len - index - 1,
					},
					match data.child_indices[1] {
						ChildNode::Leaf(index) => index + gl_nodes_len,
						ChildNode::Branch(index) => gl_nodes_len - index - 1,
					},
				],
			}))
		})
		.collect::<Result<Vec<GLNode>, Box<dyn Error + Send + Sync>>>()?;

	gl_nodes.reserve(gl_ssect_data.len());

	let gl_segs = gl_segs_data
		.into_iter()
		.map(|data| {
			Ok(GLSeg {
				vertices: [
					match data.vertex_indices[0] {
						EitherVertex::GL(index) => gl_vert_data[index],
						EitherVertex::Normal(index) => vertexes_data[index],
					},
					match data.vertex_indices[1] {
						EitherVertex::GL(index) => gl_vert_data[index],
						EitherVertex::Normal(index) => vertexes_data[index],
					},
				],
				linedef_index: data.linedef_index,
				linedef_side: data.linedef_side,
				partner_seg_index: data.partner_seg_index,
			})
		})
		.collect::<Result<Vec<GLSeg>, Box<dyn Error + Send + Sync>>>()?;

	for (i, ssect) in gl_ssect_data.into_iter().enumerate() {
		let segs = &gl_segs[ssect.first_seg_index as usize
			..ssect.first_seg_index as usize + ssect.seg_count as usize];
		let subsector: Vec<Vector2<f32>> = segs.iter().map(|seg| seg.vertices[0]).collect();
		let sector_index = {
			if let Some(sidedef) = segs.iter().find_map(|seg| match seg.linedef_index {
				None => None,
				Some(index) => linedefs[index].sidedefs[seg.linedef_side as usize].as_ref(),
			}) {
				sidedef.sector_index
			} else {
				return Err(Box::from(format!(
					"No sector could be found for subsector {}",
					i
				)));
			}
		};

		sectors[sector_index].subsectors.push(subsector);

		gl_nodes.push(GLNode::Leaf(LeafNode {
			segs: segs.to_owned(),
			sector_index,
		}))
	}

	Ok(Map {
		linedefs,
		sectors,
		gl_nodes,
		sky,
	})
}

#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub enum TextureType<T> {
	Normal(AssetHandle<T>),
	Sky,
	None,
}

impl<T> TextureType<T> {
	pub fn is_sky(&self) -> bool {
		if let TextureType::Sky = *self {
			true
		} else {
			false
		}
	}
}

#[derive(Clone, Debug)]
pub struct Linedef {
	pub line: Line,
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

	/*pub fn intersects_bbox(&self, bbox: &BoundingBox2) -> bool {
		if bbox.max[0] <= self.bbox.min[0]
			|| bbox.min[0] >= self.bbox.max[0]
			|| bbox.max[1] <= self.bbox.min[1]
			|| bbox.min[1] >= self.bbox.max[1]
		{
			return false;
		}

		let sides = [
			self.line.point_side(Vector2::new(bbox.min[0], bbox.min[1])),
			self.line.point_side(Vector2::new(bbox.min[0], bbox.max[1])),
			self.line.point_side(Vector2::new(bbox.max[0], bbox.min[1])),
			self.line.point_side(Vector2::new(bbox.max[0], bbox.max[1])),
		];

		if sides[0] < 0.0 && sides[1] < 0.0 && sides[2] < 0.0 && sides[3] < 0.0
			|| sides[0] > 0.0 && sides[1] > 0.0 && sides[2] > 0.0 && sides[3] > 0.0
		{
			false
		} else {
			true
		}
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
	pub partition_line: Line,
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
