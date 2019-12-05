use crate::{
	assets::{AssetFormat, DataSource},
	doom::{
		components::{SpawnPointComponent, TransformComponent},
		entities::{DOOMEDNUMS, ENTITIES},
	},
	geometry::{Angle, BoundingBox2},
};
use nalgebra::{Vector2, Vector3};
use serde::Deserialize;
use specs::{world::Builder, Entity, Join, ReadStorage, SystemData, World, WorldExt};
use std::{error::Error, io::Cursor, str};

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
				rotation: Vector3::new(0.into(), 0.into(), Angle::from_degrees(thing.angle as f64)),
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
pub struct Thing {
	pub position: Vector2<f32>,
	pub angle: f32,
	pub doomednum: u16,
	pub flags: u16,
}

pub struct ThingsFormat;

impl AssetFormat for ThingsFormat {
	type Asset = Vec<Thing>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		RawThingsFormat
			.import(name, source)?
			.into_iter()
			.map(|raw| {
				Ok(Thing {
					position: Vector2::new(raw.position[0] as f32, raw.position[1] as f32),
					angle: raw.angle as f32,
					doomednum: raw.doomednum,
					flags: raw.flags,
				})
			})
			.collect()
	}
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
		let raw_map = RawDoomMapFormat.import(name, source)?;

		let vertexes = raw_map
			.vertexes
			.iter()
			.map(|raw| Ok(Vector2::new(raw[0] as f32, raw[1] as f32)))
			.collect::<Result<Vec<Vector2<f32>>, Box<dyn Error>>>()?;

		let sectors = raw_map
			.sectors
			.iter()
			.map(|raw| {
				Ok(Sector {
					floor_height: raw.floor_height as f32,
					ceiling_height: raw.ceiling_height as f32,
					floor_flat_name: String::from(
						str::from_utf8(&raw.floor_flat_name)?.trim_end_matches('\0'),
					),
					ceiling_flat_name: String::from(
						str::from_utf8(&raw.ceiling_flat_name)?.trim_end_matches('\0'),
					),
					light_level: raw.light_level,
					special_type: raw.light_level,
					sector_tag: raw.light_level,
				})
			})
			.collect::<Result<Vec<Sector>, Box<dyn Error>>>()?;

		let sidedefs = raw_map
			.sidedefs
			.iter()
			.map(|raw| {
				Ok(Sidedef {
					texture_offset: Vector2::new(
						raw.texture_offset[0] as f32,
						raw.texture_offset[1] as f32,
					),
					top_texture_name: {
						if raw.top_texture_name == *b"-\0\0\0\0\0\0\0" {
							None
						} else {
							Some(String::from(
								str::from_utf8(&raw.top_texture_name)?.trim_end_matches('\0'),
							))
						}
					},
					bottom_texture_name: {
						if raw.bottom_texture_name == *b"-\0\0\0\0\0\0\0" {
							None
						} else {
							Some(String::from(
								str::from_utf8(&raw.bottom_texture_name)?.trim_end_matches('\0'),
							))
						}
					},
					middle_texture_name: {
						if raw.middle_texture_name == *b"-\0\0\0\0\0\0\0" {
							None
						} else {
							Some(String::from(
								str::from_utf8(&raw.middle_texture_name)?.trim_end_matches('\0'),
							))
						}
					},
					sector_index: raw.sector_index as usize,
				})
			})
			.collect::<Result<Vec<Sidedef>, Box<dyn Error>>>()?;

		let linedefs = raw_map
			.linedefs
			.iter()
			.map(|raw| {
				Ok(Linedef {
					vertices: [
						vertexes[raw.vertex_indices[0] as usize],
						vertexes[raw.vertex_indices[1] as usize],
					],
					flags: raw.flags,
					special_type: raw.special_type,
					sector_tag: raw.sector_tag,
					sidedef_indices: [
						if raw.sidedef_indices[0] == 0xFFFF {
							None
						} else {
							Some(raw.sidedef_indices[0] as usize)
						},
						if raw.sidedef_indices[1] == 0xFFFF {
							None
						} else {
							Some(raw.sidedef_indices[1] as usize)
						},
					],
				})
			})
			.collect::<Result<Vec<Linedef>, Box<dyn Error>>>()?;

		let gl_vert = raw_map
			.gl_vert
			.iter()
			.map(|raw| {
				Ok(Vector2::new(
					raw[0] as f32 / 65536.0,
					raw[1] as f32 / 65536.0,
				))
			})
			.collect::<Result<Vec<Vector2<f32>>, Box<dyn Error>>>()?;

		let gl_segs = raw_map
			.gl_segs
			.iter()
			.map(|raw| {
				Ok(GLSeg {
					vertices: [
						if (raw.vertex_indices[0] & 0x8000) != 0 {
							gl_vert[raw.vertex_indices[0] as usize & 0x7FFF]
						} else {
							vertexes[raw.vertex_indices[0] as usize]
						},
						if (raw.vertex_indices[1] & 0x8000) != 0 {
							gl_vert[raw.vertex_indices[1] as usize & 0x7FFF]
						} else {
							vertexes[raw.vertex_indices[1] as usize]
						},
					],
					linedef_index: {
						if raw.linedef_index == 0xFFFF {
							None
						} else {
							Some(raw.linedef_index as usize)
						}
					},
					sidedef_index: {
						if raw.linedef_index != 0xFFFF {
							linedefs[raw.linedef_index as usize].sidedef_indices[raw.side as usize]
						} else {
							None
						}
					},
					side: {
						if raw.side != 0 {
							Side::Left
						} else {
							Side::Right
						}
					},
					partner_seg_index: {
						if raw.partner_seg_index == 0xFFFF {
							None
						} else {
							Some(raw.partner_seg_index as usize)
						}
					},
				})
			})
			.collect::<Result<Vec<GLSeg>, Box<dyn Error>>>()?;

		let gl_ssect = raw_map
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

		let gl_nodes = raw_map
			.gl_nodes
			.iter()
			.map(|raw| {
				Ok(GLNode {
					partition_point: Vector2::new(
						raw.partition_point[0] as f32,
						raw.partition_point[1] as f32,
					),
					partition_dir: Vector2::new(
						raw.partition_dir[0] as f32,
						raw.partition_dir[1] as f32,
					),
					bbox: [
						BoundingBox2::from_extents(
							raw.child_bboxes[0][0] as f32,
							raw.child_bboxes[0][1] as f32,
							raw.child_bboxes[0][2] as f32,
							raw.child_bboxes[0][3] as f32,
						),
						BoundingBox2::from_extents(
							raw.child_bboxes[1][0] as f32,
							raw.child_bboxes[1][1] as f32,
							raw.child_bboxes[1][2] as f32,
							raw.child_bboxes[1][3] as f32,
						),
					],
					child_indices: [
						if (raw.child_indices[0] & 0x8000) != 0 {
							ChildNode::Leaf(raw.child_indices[0] as usize & 0x7FFF)
						} else {
							ChildNode::Branch(raw.child_indices[0] as usize)
						},
						if (raw.child_indices[1] & 0x8000) != 0 {
							ChildNode::Leaf(raw.child_indices[1] as usize & 0x7FFF)
						} else {
							ChildNode::Branch(raw.child_indices[1] as usize)
						},
					],
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
	pub flags: u16,
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
	pub light_level: u16,
	pub special_type: u16,
	pub sector_tag: u16,
}

#[derive(Clone, Debug)]
pub struct GLSeg {
	pub vertices: [Vector2<f32>; 2],
	pub linedef_index: Option<usize>,
	pub sidedef_index: Option<usize>,
	pub side: Side,
	pub partner_seg_index: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Side {
	Right = 0,
	Left = 1,
}

impl std::ops::Not for Side {
	type Output = Side;

	fn not(self) -> Self::Output {
		match self {
			Side::Right => Side::Left,
			Side::Left => Side::Right,
		}
	}
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
	pub bbox: [BoundingBox2; 2],
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

#[derive(Copy, Clone, Debug)]
pub enum ChildNode {
	Leaf(usize),
	Branch(usize),
}

// Raw Doom lump format

pub struct RawDoomMap {
	pub linedefs: Vec<RawLinedef>,
	pub sidedefs: Vec<RawSidedef>,
	pub vertexes: Vec<[i16; 2]>,
	pub sectors: Vec<RawSector>,
	pub gl_vert: Vec<[i32; 2]>,
	pub gl_segs: Vec<RawGLSeg>,
	pub gl_ssect: Vec<RawGLSSect>,
	pub gl_nodes: Vec<RawGLNode>,
}

struct RawDoomMapFormat;

impl AssetFormat for RawDoomMapFormat {
	type Asset = RawDoomMap;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let gl_name = format!("GL_{}", name);

		let linedefs = RawLinedefsFormat.import(name, source)?;
		let sidedefs = RawSidedefsFormat.import(name, source)?;
		let vertexes = RawVertexesFormat.import(name, source)?;
		let sectors = RawSectorsFormat.import(name, source)?;

		let gl_vert = RawGLVertFormat.import(&gl_name, source)?;
		let gl_segs = RawGLSegsFormat.import(&gl_name, source)?;
		let gl_ssect = RawGLSSectFormat.import(&gl_name, source)?;
		let gl_nodes = RawGLNodesFormat.import(&gl_name, source)?;

		// Verify all the cross-references

		for (i, sidedef) in sidedefs.iter().enumerate() {
			let index = sidedef.sector_index;

			if index as usize >= sectors.len() {
				return Err(Box::from(format!(
					"Sidedef {} has invalid sector index {}",
					i, index
				)));
			}
		}

		for (i, linedef) in linedefs.iter().enumerate() {
			for index in linedef.sidedef_indices.iter().copied() {
				if index != 0xFFFF && index as usize >= sidedefs.len() {
					return Err(Box::from(format!(
						"Linedef {} has invalid sidedef index {}",
						i, index
					)));
				}
			}
		}

		for (i, seg) in gl_segs.iter().enumerate() {
			let index = seg.linedef_index;
			if index != 0xFFFF && index as usize >= linedefs.len() {
				return Err(Box::from(format!(
					"Seg {} has invalid linedef index {}",
					i, seg.linedef_index
				)));
			}

			for index in seg.vertex_indices.iter().copied() {
				if (index & 0x8000) != 0 {
					let index = index & 0x7FFF;

					if index as usize >= gl_vert.len() {
						return Err(Box::from(format!(
							"Seg {} has invalid vertex index {}",
							i, index
						)));
					}
				} else {
					if index as usize >= vertexes.len() {
						return Err(Box::from(format!(
							"Seg {} has invalid vertex index {}",
							i, index
						)));
					}
				};
			}

			let index = seg.partner_seg_index;
			if index != 0xFFFF && index as usize >= gl_segs.len() {
				return Err(Box::from(format!(
					"Seg {} has invalid partner seg index {}",
					i, index
				)));
			}
		}

		for (i, ssect) in gl_ssect.iter().enumerate() {
			let index = ssect.first_seg_index;
			if index as usize >= gl_segs.len() {
				return Err(Box::from(format!(
					"Subsector {} has invalid first seg index {}",
					i, ssect.first_seg_index
				)));
			}

			if ssect.first_seg_index as usize + ssect.seg_count as usize > gl_segs.len() {
				return Err(Box::from(format!(
					"Subsector {} has overflowing seg count {}",
					i, ssect.seg_count,
				)));
			}
		}

		for (i, node) in gl_nodes.iter().enumerate() {
			for child in node.child_indices.iter().copied() {
				if (child & 0x8000) != 0 {
					let index = child & 0x7FFF;
					if index as usize >= gl_ssect.len() {
						return Err(Box::from(format!(
							"Node {} has invalid subsector index {}",
							i, index
						)));
					}
				} else {
					let index = child;
					if index as usize >= gl_nodes.len() {
						return Err(Box::from(format!(
							"Node {} has invalid child node index {}",
							i, index
						)));
					}
				}
			}
		}

		Ok(RawDoomMap {
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

#[derive(Deserialize)]
pub struct RawThing {
	pub position: [i16; 2],
	pub angle: i16,
	pub doomednum: u16,
	pub flags: u16,
}

pub struct RawThingsFormat;

impl AssetFormat for RawThingsFormat {
	type Asset = Vec<RawThing>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 1))?);
		let mut ret = Vec::new();

		while (data.position() as usize) < data.get_ref().len() {
			ret.push(bincode::deserialize_from(&mut data)?);
		}

		Ok(ret)
	}
}

#[derive(Deserialize)]
pub struct RawLinedef {
	pub vertex_indices: [u16; 2],
	pub flags: u16,
	pub special_type: u16,
	pub sector_tag: u16,
	pub sidedef_indices: [u16; 2],
}

pub struct RawLinedefsFormat;

impl AssetFormat for RawLinedefsFormat {
	type Asset = Vec<RawLinedef>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 2))?);
		let mut ret = Vec::new();

		while (data.position() as usize) < data.get_ref().len() {
			ret.push(bincode::deserialize_from(&mut data)?);
		}

		Ok(ret)
	}
}

#[derive(Deserialize)]
pub struct RawSidedef {
	pub texture_offset: [i16; 2],
	pub top_texture_name: [u8; 8],
	pub bottom_texture_name: [u8; 8],
	pub middle_texture_name: [u8; 8],
	pub sector_index: u16,
}

pub struct RawSidedefsFormat;

impl AssetFormat for RawSidedefsFormat {
	type Asset = Vec<RawSidedef>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 3))?);
		let mut ret = Vec::new();

		while (data.position() as usize) < data.get_ref().len() {
			ret.push(bincode::deserialize_from(&mut data)?);
		}

		Ok(ret)
	}
}

pub struct RawVertexesFormat;

impl AssetFormat for RawVertexesFormat {
	type Asset = Vec<[i16; 2]>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 4))?);
		let mut ret = Vec::new();

		while (data.position() as usize) < data.get_ref().len() {
			ret.push(bincode::deserialize_from(&mut data)?);
		}

		Ok(ret)
	}
}

#[derive(Deserialize)]
pub struct RawSector {
	pub floor_height: i16,
	pub ceiling_height: i16,
	pub floor_flat_name: [u8; 8],
	pub ceiling_flat_name: [u8; 8],
	pub light_level: u16,
	pub special_type: u16,
	pub sector_tag: u16,
}

pub struct RawSectorsFormat;

impl AssetFormat for RawSectorsFormat {
	type Asset = Vec<RawSector>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 8))?);
		let mut ret = Vec::new();

		while (data.position() as usize) < data.get_ref().len() {
			ret.push(bincode::deserialize_from(&mut data)?);
		}

		Ok(ret)
	}
}

pub struct RawGLVertFormat;

impl AssetFormat for RawGLVertFormat {
	type Asset = Vec<[i32; 2]>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 1))?);

		if bincode::deserialize_from::<_, [u8; 4]>(&mut data)? != *b"gNd2" {
			return Err(Box::from("No gNd2 signature found"));
		}

		let mut ret = Vec::new();

		while (data.position() as usize) < data.get_ref().len() {
			ret.push(bincode::deserialize_from(&mut data)?);
		}

		Ok(ret)
	}
}

#[derive(Deserialize)]
pub struct RawGLSeg {
	pub vertex_indices: [u16; 2],
	pub linedef_index: u16,
	pub side: u16,
	pub partner_seg_index: u16,
}

pub struct RawGLSegsFormat;

impl AssetFormat for RawGLSegsFormat {
	type Asset = Vec<RawGLSeg>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 2))?);
		let mut ret = Vec::new();

		while (data.position() as usize) < data.get_ref().len() {
			ret.push(bincode::deserialize_from(&mut data)?);
		}

		Ok(ret)
	}
}

#[derive(Deserialize)]
pub struct RawGLSSect {
	pub seg_count: u16,
	pub first_seg_index: u16,
}

pub struct RawGLSSectFormat;

impl AssetFormat for RawGLSSectFormat {
	type Asset = Vec<RawGLSSect>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 3))?);
		let mut ret = Vec::new();

		while (data.position() as usize) < data.get_ref().len() {
			ret.push(bincode::deserialize_from(&mut data)?);
		}

		Ok(ret)
	}
}

#[derive(Deserialize)]
pub struct RawGLNode {
	partition_point: [i16; 2],
	partition_dir: [i16; 2],
	child_bboxes: [[i16; 4]; 2],
	child_indices: [u16; 2],
}

pub struct RawGLNodesFormat;

impl AssetFormat for RawGLNodesFormat {
	type Asset = Vec<RawGLNode>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 4))?);
		let mut ret = Vec::new();

		while (data.position() as usize) < data.get_ref().len() {
			ret.push(bincode::deserialize_from(&mut data)?);
		}

		Ok(ret)
	}
}
