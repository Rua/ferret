pub mod lumps;
pub mod meshes;
pub mod textures;

use crate::{
	assets::{Asset, AssetHandle, AssetStorage},
	component::EntityTemplate,
	doom::{
		components::{SpawnPoint, Transform},
		entities::EntityTypes,
		map::{
			lumps::{ChildNode, EitherVertex, LinedefFlags, MapData, Thing},
			textures::{FlatFormat, TextureFormat},
		},
		wad::WadLoader,
	},
	geometry::{BoundingBox2, Side},
	renderer::texture::Texture,
};
use nalgebra::Vector2;
use specs::{Entity, Join, ReadExpect, ReadStorage, World, WorldExt, WriteStorage};
use std::{
	collections::{hash_map::Entry, HashMap},
	error::Error,
};

pub fn spawn_map_entities(
	things: Vec<Thing>,
	world: &World,
	map: &AssetHandle<Map>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
	for thing in things {
		// Fetch entity template
		let (entity_types, template_storage) = world.system_data::<(
			ReadExpect<EntityTypes>,
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
			let storage = world.system_data::<ReadExpect<AssetStorage<Map>>>();
			let map = storage.get(&map).unwrap();
			let ssect = map.find_subsector(thing.position);
			let sector = &map.sectors[ssect.sector_index];
			sector.floor_height
		};

		let mut transform_storage = world.system_data::<WriteStorage<Transform>>();
		let transform = transform_storage.get_mut(entity).unwrap();
		transform.position[0] = thing.position[0];
		transform.position[1] = thing.position[1];
		transform.position[2] = z;
		transform.rotation[2] = thing.angle;
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
		ReadExpect<EntityTypes>,
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
	*transform_storage.get_mut(entity).unwrap() = transform;

	Ok(entity)
}

#[derive(Clone, Debug)]
pub struct Map {
	pub linedefs: Vec<Linedef>,
	pub sectors: Vec<Sector>,
	pub gl_nodes: Vec<GLNode>,
	pub sky: AssetHandle<Texture>,
}

impl Asset for Map {
	type Data = MapData;
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
	texture_storage: &mut AssetStorage<Texture>,
) -> Result<Map, Box<dyn Error + Send + Sync>> {
	let mut textures = HashMap::new();
	let mut flats = HashMap::new();
	let sky = texture_storage.load(sky_name, TextureFormat, loader);

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
								let handle =
									texture_storage.load(entry.key(), FlatFormat, &mut *loader);
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
								let handle =
									texture_storage.load(entry.key(), FlatFormat, &mut *loader);
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
								let handle =
									texture_storage.load(entry.key(), TextureFormat, &mut *loader);
								entry.insert(handle)
							}
							Entry::Occupied(entry) => entry.into_mut(),
						};
						TextureType::Normal(handle.clone())
					}
				},
				bottom_texture: match data.bottom_texture_name {
					None => None,
					Some(name) => {
						let handle = match textures.entry(name.clone()) {
							Entry::Vacant(entry) => {
								let handle =
									texture_storage.load(entry.key(), TextureFormat, &mut *loader);
								entry.insert(handle)
							}
							Entry::Occupied(entry) => entry.into_mut(),
						};
						Some(handle.clone())
					}
				},
				middle_texture: match data.middle_texture_name {
					None => None,
					Some(name) => {
						let handle = match textures.entry(name) {
							Entry::Vacant(entry) => {
								let handle =
									texture_storage.load(entry.key(), TextureFormat, &mut *loader);
								entry.insert(handle)
							}
							Entry::Occupied(entry) => entry.into_mut(),
						};
						Some(handle.clone())
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

			// If an upper texture is neighboured by two sky flats, make it sky too
			if let [Some(ref mut front_sidedef), Some(ref mut back_sidedef)] = &mut sidedefs {
				if sectors[front_sidedef.sector_index].ceiling_texture.is_sky()
					&& sectors[back_sidedef.sector_index].ceiling_texture.is_sky()
				{
					front_sidedef.top_texture = TextureType::Sky;
					back_sidedef.top_texture = TextureType::Sky;
				}
			}

			Ok(Linedef {
				vertices: [
					vertexes_data[data.vertex_indices[0]],
					vertexes_data[data.vertex_indices[1]],
				],
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
				partition_point: data.partition_point.clone(),
				partition_dir: data.partition_dir.clone(),
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

#[derive(Clone, Debug)]
pub enum TextureType {
	Normal(AssetHandle<Texture>),
	Sky,
	None,
}

impl TextureType {
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
	pub vertices: [Vector2<f32>; 2],
	pub flags: LinedefFlags,
	pub special_type: u16,
	pub sector_tag: u16,
	pub sidedefs: [Option<Sidedef>; 2],
}

#[derive(Clone, Debug)]
pub struct Sidedef {
	pub texture_offset: Vector2<f32>,
	pub top_texture: TextureType,
	pub bottom_texture: Option<AssetHandle<Texture>>,
	pub middle_texture: Option<AssetHandle<Texture>>,
	pub sector_index: usize,
}

#[derive(Clone, Debug)]
pub struct Sector {
	pub floor_height: f32,
	pub ceiling_height: f32,
	pub floor_texture: TextureType,
	pub ceiling_texture: TextureType,
	pub light_level: f32,
	pub special_type: u16,
	pub sector_tag: u16,
	pub subsectors: Vec<Vec<Vector2<f32>>>,
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
	pub partition_point: Vector2<f32>,
	pub partition_dir: Vector2<f32>,
	pub child_bboxes: [BoundingBox2; 2],
	pub child_indices: [usize; 2],
}

impl BranchNode {
	pub fn point_side(&self, point: Vector2<f32>) -> Side {
		let d = point - self.partition_point;
		let left = self.partition_dir[1] * d[0];
		let right = self.partition_dir[0] * d[1];

		if right < left {
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
