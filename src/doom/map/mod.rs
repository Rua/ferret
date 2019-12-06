pub mod lumps;
pub mod meshes;
pub mod textures;

use crate::{
	assets::AssetHandle,
	doom::{
		components::{SpawnPointComponent, TransformComponent},
		entities::{DOOMEDNUMS, ENTITIES},
		map::lumps::{ChildNode, EitherVertex, LinedefFlags, MapData, Thing},
	},
	geometry::{BoundingBox2, Side},
	renderer::texture::Texture,
};
use nalgebra::{Vector2, Vector3};
use specs::{world::Builder, Entity, Join, ReadStorage, SystemData, World, WorldExt};
use std::error::Error;

pub fn spawn_map_entities(
	things: Vec<Thing>,
	world: &mut World,
	map_data: &DoomMap,
) -> Result<(), Box<dyn Error>> {
	for thing in things {
		let ssect = map_data.find_subsector(thing.position);
		let sector = &map_data.sectors[ssect.sector_index];
		let z = sector.floor_height;

		let entity = world
			.create_entity()
			.with(TransformComponent {
				position: Vector3::new(thing.position[0], thing.position[1], z),
				rotation: Vector3::new(0.into(), 0.into(), thing.angle),
			})
			.build();

		let name = DOOMEDNUMS
			.get(&thing.doomednum)
			.ok_or(
				Box::from(format!("Doomednum not found: {}", thing.doomednum)) as Box<dyn Error>,
			)?;
		let spawn_function = ENTITIES
			.get(name)
			.ok_or(Box::from(format!("Entity not found: {}", name)) as Box<dyn Error>)?;

		spawn_function(entity, world);
	}

	Ok(())
}

pub fn spawn_player(world: &mut World) -> Result<Entity, Box<dyn Error>> {
	let (position, rotation) = {
		let (transform, spawn_point) = <(
			ReadStorage<TransformComponent>,
			ReadStorage<SpawnPointComponent>,
		)>::fetch(world);

		(&transform, &spawn_point)
			.join()
			.find_map(|(t, s)| {
				if s.player_num == 1 {
					Some((t.position, t.rotation))
				} else {
					None
				}
			})
			.unwrap()
	};

	let entity = world
		.create_entity()
		.with(TransformComponent { position, rotation })
		.build();

	let spawn_function = ENTITIES
		.get("PLAYER")
		.ok_or(Box::from(format!("Entity not found: {}", "PLAYER")) as Box<dyn Error>)?;

	spawn_function(entity, world);

	Ok(entity)
}

#[derive(Clone, Debug)]
pub struct DoomMap {
	pub linedefs: Vec<Linedef>,
	pub sectors: Vec<Sector>,
	pub gl_nodes: Vec<GLNode>,
}

impl DoomMap {
	fn find_subsector(&self, point: Vector2<f32>) -> &LeafNode {
		let mut node = 0;

		loop {
			node = match &self.gl_nodes[node] {
				GLNode::Leaf(leaf) => return &leaf,
				GLNode::Branch(branch) => branch.child_indices[branch.point_side(point) as usize],
			};
		}
	}
}

pub fn build_map(map_data: MapData, world: &World) -> Result<DoomMap, Box<dyn Error>> {
	let [textures, flats] = textures::load_textures(&map_data, world)?;

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
				floor_texture: match data.floor_flat_name.as_ref().map(String::as_str) {
					None => TextureType::None,
					Some("F_SKY1") => TextureType::Sky,
					Some(name) => {
						let flat = flats[name].clone();
						TextureType::Normal(flat.0, flat.1)
					}
				},
				ceiling_texture: match data.ceiling_flat_name.as_ref().map(String::as_str) {
					None => TextureType::None,
					Some("F_SKY1") => TextureType::Sky,
					Some(name) => {
						let flat = flats[name].clone();
						TextureType::Normal(flat.0, flat.1)
					}
				},
				light_level: data.light_level,
				special_type: data.special_type,
				sector_tag: data.special_type,
				subsectors: Vec::new(),
			})
		})
		.collect::<Result<Vec<Sector>, Box<dyn Error>>>()?;

	let mut sidedefs = sidedefs_data
		.into_iter()
		.map(|data| {
			Ok(Some(Sidedef {
				texture_offset: data.texture_offset,
				top_texture: match data.top_texture_name.as_ref().map(String::as_str) {
					None => TextureType::None,
					Some("F_SKY1") => TextureType::Sky,
					Some(name) => {
						let flat = textures[name].clone();
						TextureType::Normal(flat.0, flat.1)
					}
				},
				bottom_texture: data.bottom_texture_name.map(|name| textures[&name].clone()),
				middle_texture: data.middle_texture_name.map(|name| textures[&name].clone()),
				sector_index: data.sector_index,
			}))
		})
		.collect::<Result<Vec<Option<Sidedef>>, Box<dyn Error>>>()?;

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
		.collect::<Result<Vec<Linedef>, Box<dyn Error>>>()?;

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
		.collect::<Result<Vec<GLNode>, Box<dyn Error>>>()?;

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
		.collect::<Result<Vec<GLSeg>, Box<dyn Error>>>()?;

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

	Ok(DoomMap {
		linedefs,
		sectors,
		gl_nodes,
	})
}

#[derive(Clone, Debug)]
pub enum TextureType {
	Normal(AssetHandle<Texture>, usize),
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
	pub bottom_texture: Option<(AssetHandle<Texture>, usize)>,
	pub middle_texture: Option<(AssetHandle<Texture>, usize)>,
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
