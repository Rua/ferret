use crate::{
	assets::{AssetFormat, DataSource},
	geometry::{Angle, BoundingBox2, Side},
};
use bitflags::bitflags;
use byteorder::{ReadBytesExt, LE};
use nalgebra::Vector2;
use serde::Deserialize;
use std::{
	error::Error,
	io::{Cursor, Read},
};

pub struct MapData {
	pub linedefs: Vec<Linedef>,
	pub sidedefs: Vec<Sidedef>,
	pub vertexes: Vec<Vector2<f32>>,
	pub sectors: Vec<Sector>,
	pub gl_vert: Vec<Vector2<f32>>,
	pub gl_segs: Vec<GLSeg>,
	pub gl_ssect: Vec<GLSSect>,
	pub gl_nodes: Vec<GLNode>,
}

#[derive(Clone, Copy)]
pub struct MapDataFormat;

impl AssetFormat for MapDataFormat {
	type Asset = MapData;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
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
		const MPONLY = 0b00000000_00001000;
	}
}

#[derive(Clone, Copy)]
pub struct ThingsFormat;

impl AssetFormat for ThingsFormat {
	type Asset = Vec<Thing>;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(&format!("{}/+{}", name, 1))?);
		let mut ret = Vec::new();

		while (reader.position() as usize) < reader.get_ref().len() {
			ret.push(Thing {
				position: Vector2::new(
					reader.read_i16::<LE>()? as f32,
					reader.read_i16::<LE>()? as f32,
				),
				angle: Angle::from_degrees(reader.read_u16::<LE>()? as f64),
				doomednum: reader.read_u16::<LE>()?,
				flags: ThingFlags::from_bits_truncate(reader.read_u16::<LE>()?),
			});
		}

		Ok(ret)
	}
}

pub struct Linedef {
	pub vertex_indices: [usize; 2],
	pub flags: LinedefFlags,
	pub special_type: u16,
	pub sector_tag: u16,
	pub sidedef_indices: [Option<usize>; 2],
}

bitflags! {
	#[derive(Deserialize)]
	pub struct LinedefFlags: u16 {
		const BLOCKING = 0b00000000_00000001;
		const BLOCKMONSTERS = 0b00000000_00000010;
		const TWOSIDED = 0b00000000_00000100;
		const DONTPEGTOP = 0b00000000_00001000;
		const DONTPEGBOTTOM = 0b00000000_00010000;
		const SECRET = 0b00000000_00100000;
		const BLOCKSOUND = 0b00000000_01000000;
		const NOAUTOMAP = 0b00000000_10000000;
	}
}

#[derive(Clone, Copy)]
pub struct LinedefsFormat;

impl AssetFormat for LinedefsFormat {
	type Asset = Vec<Linedef>;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(&format!("{}/+{}", name, 2))?);
		let mut ret = Vec::new();

		while (reader.position() as usize) < reader.get_ref().len() {
			ret.push(Linedef {
				vertex_indices: [
					reader.read_u16::<LE>()? as usize,
					reader.read_u16::<LE>()? as usize,
				],
				flags: LinedefFlags::from_bits_truncate(reader.read_u16::<LE>()?),
				special_type: reader.read_u16::<LE>()?,
				sector_tag: reader.read_u16::<LE>()?,
				sidedef_indices: [
					match reader.read_u16::<LE>()? as usize {
						0xFFFF => None,
						x => Some(x),
					},
					match reader.read_u16::<LE>()? as usize {
						0xFFFF => None,
						x => Some(x),
					},
				],
			});
		}

		Ok(ret)
	}
}

pub struct Sidedef {
	pub texture_offset: Vector2<f32>,
	pub top_texture_name: Option<String>,
	pub bottom_texture_name: Option<String>,
	pub middle_texture_name: Option<String>,
	pub sector_index: usize,
}

#[derive(Clone, Copy)]
pub struct SidedefsFormat;

impl AssetFormat for SidedefsFormat {
	type Asset = Vec<Sidedef>;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(&format!("{}/+{}", name, 3))?);
		let mut ret = Vec::new();

		while (reader.position() as usize) < reader.get_ref().len() {
			let mut buf = [0u8; 8];

			ret.push(Sidedef {
				texture_offset: Vector2::new(
					reader.read_i16::<LE>()? as f32,
					reader.read_i16::<LE>()? as f32,
				),
				top_texture_name: match {
					reader.read_exact(&mut buf)?;
					&buf
				} {
					b"-\0\0\0\0\0\0\0" => None,
					x => Some(std::str::from_utf8(x)?.trim_end_matches('\0').to_owned()),
				},
				bottom_texture_name: match {
					reader.read_exact(&mut buf)?;
					&buf
				} {
					b"-\0\0\0\0\0\0\0" => None,
					x => Some(std::str::from_utf8(x)?.trim_end_matches('\0').to_owned()),
				},
				middle_texture_name: match {
					reader.read_exact(&mut buf)?;
					&buf
				} {
					b"-\0\0\0\0\0\0\0" => None,
					x => Some(std::str::from_utf8(x)?.trim_end_matches('\0').to_owned()),
				},
				sector_index: reader.read_u16::<LE>()? as usize,
			});
		}

		Ok(ret)
	}
}

#[derive(Clone, Copy)]
pub struct VertexesFormat;

impl AssetFormat for VertexesFormat {
	type Asset = Vec<Vector2<f32>>;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(&format!("{}/+{}", name, 4))?);
		let mut ret = Vec::new();

		while (reader.position() as usize) < reader.get_ref().len() {
			ret.push(Vector2::new(
				reader.read_i16::<LE>()? as f32,
				reader.read_i16::<LE>()? as f32,
			));
		}

		Ok(ret)
	}
}

pub struct Sector {
	pub floor_height: f32,
	pub ceiling_height: f32,
	pub floor_flat_name: Option<String>,
	pub ceiling_flat_name: Option<String>,
	pub light_level: f32,
	pub special_type: u16,
	pub sector_tag: u16,
}

#[derive(Clone, Copy)]
pub struct SectorsFormat;

impl AssetFormat for SectorsFormat {
	type Asset = Vec<Sector>;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(&format!("{}/+{}", name, 8))?);
		let mut ret = Vec::new();

		while (reader.position() as usize) < reader.get_ref().len() {
			let mut buf = [0u8; 8];

			ret.push(Sector {
				floor_height: reader.read_i16::<LE>()? as f32,
				ceiling_height: reader.read_i16::<LE>()? as f32,
				floor_flat_name: match {
					reader.read_exact(&mut buf)?;
					&buf
				} {
					b"-\0\0\0\0\0\0\0" => None,
					x => Some(std::str::from_utf8(x)?.trim_end_matches('\0').to_owned()),
				},
				ceiling_flat_name: match {
					reader.read_exact(&mut buf)?;
					&buf
				} {
					b"-\0\0\0\0\0\0\0" => None,
					x => Some(std::str::from_utf8(x)?.trim_end_matches('\0').to_owned()),
				},
				light_level: reader.read_u16::<LE>()? as f32 / 255.0,
				special_type: reader.read_u16::<LE>()?,
				sector_tag: reader.read_u16::<LE>()?,
			});
		}

		Ok(ret)
	}
}

#[derive(Clone, Copy)]
pub struct GLVertFormat;

impl AssetFormat for GLVertFormat {
	type Asset = Vec<Vector2<f32>>;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(&format!("{}/+{}", name, 1))?);

		let mut buf = [0u8; 4];
		reader.read_exact(&mut buf)?;

		if &buf != b"gNd2" {
			return Err(Box::from("No gNd2 signature found"));
		}

		let mut ret = Vec::new();

		while (reader.position() as usize) < reader.get_ref().len() {
			ret.push(Vector2::new(
				reader.read_i32::<LE>()? as f32 / 65536.0,
				reader.read_i32::<LE>()? as f32 / 65536.0,
			));
		}

		Ok(ret)
	}
}

pub struct GLSeg {
	pub vertex_indices: [EitherVertex; 2],
	pub linedef_index: Option<usize>,
	pub linedef_side: Side,
	pub partner_seg_index: Option<usize>,
}

pub enum EitherVertex {
	Normal(usize),
	GL(usize),
}

#[derive(Clone, Copy)]
pub struct GLSegsFormat;

impl AssetFormat for GLSegsFormat {
	type Asset = Vec<GLSeg>;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(&format!("{}/+{}", name, 2))?);
		let mut ret = Vec::new();

		while (reader.position() as usize) < reader.get_ref().len() {
			ret.push(GLSeg {
				vertex_indices: [
					match reader.read_u16::<LE>()? as usize {
						x if x & 0x8000 != 0 => EitherVertex::GL(x & 0x7FFF),
						x => EitherVertex::Normal(x),
					},
					match reader.read_u16::<LE>()? as usize {
						x if x & 0x8000 != 0 => EitherVertex::GL(x & 0x7FFF),
						x => EitherVertex::Normal(x),
					},
				],
				linedef_index: match reader.read_u16::<LE>()? as usize {
					0xFFFF => None,
					x => Some(x),
				},
				linedef_side: match reader.read_u16::<LE>()? as usize {
					0 => Side::Right,
					_ => Side::Left,
				},
				partner_seg_index: match reader.read_u16::<LE>()? as usize {
					0xFFFF => None,
					x => Some(x),
				},
			});
		}

		Ok(ret)
	}
}

pub struct GLSSect {
	pub seg_count: usize,
	pub first_seg_index: usize,
}

#[derive(Clone, Copy)]
pub struct GLSSectFormat;

impl AssetFormat for GLSSectFormat {
	type Asset = Vec<GLSSect>;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(&format!("{}/+{}", name, 3))?);
		let mut ret = Vec::new();

		while (reader.position() as usize) < reader.get_ref().len() {
			ret.push(GLSSect {
				seg_count: reader.read_u16::<LE>()? as usize,
				first_seg_index: reader.read_u16::<LE>()? as usize,
			});
		}

		Ok(ret)
	}
}

pub struct GLNode {
	pub partition_point: Vector2<f32>,
	pub partition_dir: Vector2<f32>,
	pub child_bboxes: [BoundingBox2; 2],
	pub child_indices: [ChildNode; 2],
}

#[derive(Copy, Clone, Debug)]
pub enum ChildNode {
	Leaf(usize),
	Branch(usize),
}

#[derive(Clone, Copy)]
pub struct GLNodesFormat;

impl AssetFormat for GLNodesFormat {
	type Asset = Vec<GLNode>;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(&format!("{}/+{}", name, 4))?);
		let mut ret = Vec::new();

		while (reader.position() as usize) < reader.get_ref().len() {
			ret.push(GLNode {
				partition_point: Vector2::new(
					reader.read_i16::<LE>()? as f32,
					reader.read_i16::<LE>()? as f32,
				),
				partition_dir: Vector2::new(
					reader.read_i16::<LE>()? as f32,
					reader.read_i16::<LE>()? as f32,
				),
				child_bboxes: [
					BoundingBox2::from_extents(
						reader.read_i16::<LE>()? as f32,
						reader.read_i16::<LE>()? as f32,
						reader.read_i16::<LE>()? as f32,
						reader.read_i16::<LE>()? as f32,
					),
					BoundingBox2::from_extents(
						reader.read_i16::<LE>()? as f32,
						reader.read_i16::<LE>()? as f32,
						reader.read_i16::<LE>()? as f32,
						reader.read_i16::<LE>()? as f32,
					),
				],
				child_indices: [
					match reader.read_u16::<LE>()? as usize {
						x if x & 0x8000 != 0 => ChildNode::Leaf(x & 0x7FFF),
						x => ChildNode::Branch(x),
					},
					match reader.read_u16::<LE>()? as usize {
						x if x & 0x8000 != 0 => ChildNode::Leaf(x & 0x7FFF),
						x => ChildNode::Branch(x),
					},
				],
			});
		}

		Ok(ret)
	}
}
