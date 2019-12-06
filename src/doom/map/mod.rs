pub mod lumps;
pub mod meshes;
pub mod textures;

use crate::{
	assets::{AssetFormat, DataSource},
	doom::{
		components::{SpawnPointComponent, TransformComponent},
		entities::{DOOMEDNUMS, ENTITIES},
		map::lumps::{ChildNode, EitherVertex, LinedefFlags, MapDataFormat, Thing},
	},
	geometry::{BoundingBox2, Side},
};
use nalgebra::{Vector2, Vector3};
use specs::{world::Builder, Entity, Join, ReadStorage, SystemData, World, WorldExt};
use std::{error::Error, str};

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
	pub sidedefs: Vec<Sidedef>,
	pub sectors: Vec<Sector>,
	pub gl_segs: Vec<GLSeg>,
	pub gl_ssect: Vec<GLSSect>,
	pub gl_nodes: Vec<GLNode>,
}

impl DoomMap {
	fn find_subsector(&self, point: Vector2<f32>) -> &GLSSect {
		let mut node = &self.gl_nodes[self.gl_nodes.len() - 1];

		loop {
			node = match node.child_indices[node.point_side(point) as usize] {
				ChildNode::Branch(node_id) => &self.gl_nodes[node_id],
				ChildNode::Leaf(ssect_id) => return &self.gl_ssect[ssect_id],
			}
		}
	}
}

pub struct DoomMapFormat;

impl AssetFormat for DoomMapFormat {
	type Asset = DoomMap;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let map_data = MapDataFormat.import(name, source)?;

		let vertexes = map_data.vertexes;

		let sectors = map_data
			.sectors
			.into_iter()
			.map(|raw| {
				Ok(Sector {
					floor_height: raw.floor_height,
					ceiling_height: raw.ceiling_height,
					floor_flat_name: raw.floor_flat_name,
					ceiling_flat_name: raw.ceiling_flat_name,
					light_level: raw.light_level,
					special_type: raw.special_type,
					sector_tag: raw.special_type,
				})
			})
			.collect::<Result<Vec<Sector>, Box<dyn Error>>>()?;

		let sidedefs = map_data
			.sidedefs
			.into_iter()
			.map(|raw| {
				Ok(Sidedef {
					texture_offset: raw.texture_offset,
					top_texture_name: raw.top_texture_name,
					bottom_texture_name: raw.bottom_texture_name,
					middle_texture_name: raw.middle_texture_name,
					sector_index: raw.sector_index,
				})
			})
			.collect::<Result<Vec<Sidedef>, Box<dyn Error>>>()?;

		let linedefs = map_data
			.linedefs
			.iter()
			.map(|raw| {
				Ok(Linedef {
					vertices: [
						vertexes[raw.vertex_indices[0]],
						vertexes[raw.vertex_indices[1]],
					],
					flags: raw.flags,
					special_type: raw.special_type,
					sector_tag: raw.sector_tag,
					sidedef_indices: raw.sidedef_indices,
				})
			})
			.collect::<Result<Vec<Linedef>, Box<dyn Error>>>()?;

		let gl_vert = map_data.gl_vert;

		let gl_segs = map_data
			.gl_segs
			.iter()
			.map(|raw| {
				Ok(GLSeg {
					vertices: [
						match raw.vertex_indices[0] {
							EitherVertex::GL(index) => gl_vert[index],
							EitherVertex::Normal(index) => vertexes[index],
						},
						match raw.vertex_indices[1] {
							EitherVertex::GL(index) => gl_vert[index],
							EitherVertex::Normal(index) => vertexes[index],
						},
					],
					linedef_index: raw.linedef_index,
					sidedef_index: match raw.linedef_index {
						None => None,
						Some(index) => linedefs[index].sidedef_indices[raw.linedef_side as usize],
					},
					linedef_side: raw.linedef_side,
					partner_seg_index: raw.partner_seg_index,
				})
			})
			.collect::<Result<Vec<GLSeg>, Box<dyn Error>>>()?;

		let gl_ssect = map_data
			.gl_ssect
			.iter()
			.enumerate()
			.map(|(i, raw)| {
				Ok(GLSSect {
					seg_count: raw.seg_count as usize,
					first_seg_index: raw.first_seg_index as usize,
					sector_index: {
						let segs = &gl_segs[raw.first_seg_index as usize
							..raw.first_seg_index as usize + raw.seg_count as usize];
						if let Some(sidedef_index) = segs.iter().find_map(|seg| seg.sidedef_index) {
							sidedefs[sidedef_index].sector_index
						} else {
							return Err(Box::from(format!(
								"No sector could be found for subsector {}",
								i
							)));
						}
					},
				})
			})
			.collect::<Result<Vec<GLSSect>, Box<dyn Error>>>()?;

		let gl_nodes = map_data
			.gl_nodes
			.into_iter()
			.map(|raw| {
				Ok(GLNode {
					partition_point: raw.partition_point,
					partition_dir: raw.partition_dir,
					child_bboxes: raw.child_bboxes,
					child_indices: raw.child_indices,
				})
			})
			.collect::<Result<Vec<GLNode>, Box<dyn Error>>>()?;

		Ok(DoomMap {
			linedefs,
			sidedefs,
			sectors,
			gl_segs,
			gl_ssect,
			gl_nodes,
		})
	}
}

#[derive(Clone, Debug)]
pub struct Linedef {
	pub vertices: [Vector2<f32>; 2],
	pub flags: LinedefFlags,
	pub special_type: u16,
	pub sector_tag: u16,
	pub sidedef_indices: [Option<usize>; 2],
}

#[derive(Clone, Debug)]
pub struct Sidedef {
	pub texture_offset: Vector2<f32>,
	pub top_texture_name: Option<String>,
	pub bottom_texture_name: Option<String>,
	pub middle_texture_name: Option<String>,
	pub sector_index: usize,
}

#[derive(Clone, Debug)]
pub struct Sector {
	pub floor_height: f32,
	pub ceiling_height: f32,
	pub floor_flat_name: String,
	pub ceiling_flat_name: String,
	pub light_level: f32,
	pub special_type: u16,
	pub sector_tag: u16,
}

#[derive(Clone, Debug)]
pub struct GLSeg {
	pub vertices: [Vector2<f32>; 2],
	pub linedef_index: Option<usize>,
	pub sidedef_index: Option<usize>,
	pub linedef_side: Side,
	pub partner_seg_index: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct GLSSect {
	pub seg_count: usize,
	pub first_seg_index: usize,
	pub sector_index: usize,
}

#[derive(Clone, Debug)]
pub struct GLNode {
	pub partition_point: Vector2<f32>,
	pub partition_dir: Vector2<f32>,
	pub child_bboxes: [BoundingBox2; 2],
	pub child_indices: [ChildNode; 2],
}

impl GLNode {
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
