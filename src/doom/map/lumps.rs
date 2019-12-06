use bitflags::bitflags;
use crate::assets::{AssetFormat, DataSource};
use serde::Deserialize;
use std::{error::Error, io::Cursor};

pub struct MapData {
	pub linedefs: Vec<Linedef>,
	pub sidedefs: Vec<Sidedef>,
	pub vertexes: Vec<[i16; 2]>,
	pub sectors: Vec<Sector>,
	pub gl_vert: Vec<[i32; 2]>,
	pub gl_segs: Vec<GLSeg>,
	pub gl_ssect: Vec<GLSSect>,
	pub gl_nodes: Vec<GLNode>,
}

pub struct MapDataFormat;

impl AssetFormat for MapDataFormat {
	type Asset = MapData;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
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

#[derive(Deserialize)]
pub struct Thing {
	pub position: [i16; 2],
	pub angle: i16,
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

pub struct ThingsFormat;

impl AssetFormat for ThingsFormat {
	type Asset = Vec<Thing>;

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
pub struct Linedef {
	pub vertex_indices: [u16; 2],
	pub flags: LinedefFlags,
	pub special_type: u16,
	pub sector_tag: u16,
	pub sidedef_indices: [u16; 2],
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

pub struct LinedefsFormat;

impl AssetFormat for LinedefsFormat {
	type Asset = Vec<Linedef>;

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
pub struct Sidedef {
	pub texture_offset: [i16; 2],
	pub top_texture_name: [u8; 8],
	pub bottom_texture_name: [u8; 8],
	pub middle_texture_name: [u8; 8],
	pub sector_index: u16,
}

pub struct SidedefsFormat;

impl AssetFormat for SidedefsFormat {
	type Asset = Vec<Sidedef>;

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

pub struct VertexesFormat;

impl AssetFormat for VertexesFormat {
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
pub struct Sector {
	pub floor_height: i16,
	pub ceiling_height: i16,
	pub floor_flat_name: [u8; 8],
	pub ceiling_flat_name: [u8; 8],
	pub light_level: u16,
	pub special_type: u16,
	pub sector_tag: u16,
}

pub struct SectorsFormat;

impl AssetFormat for SectorsFormat {
	type Asset = Vec<Sector>;

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

pub struct GLVertFormat;

impl AssetFormat for GLVertFormat {
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
pub struct GLSeg {
	pub vertex_indices: [u16; 2],
	pub linedef_index: u16,
	pub side: u16,
	pub partner_seg_index: u16,
}

pub struct GLSegsFormat;

impl AssetFormat for GLSegsFormat {
	type Asset = Vec<GLSeg>;

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
pub struct GLSSect {
	pub seg_count: u16,
	pub first_seg_index: u16,
}

pub struct GLSSectFormat;

impl AssetFormat for GLSSectFormat {
	type Asset = Vec<GLSSect>;

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
pub struct GLNode {
	pub partition_point: [i16; 2],
	pub partition_dir: [i16; 2],
	pub child_bboxes: [[i16; 4]; 2],
	pub child_indices: [u16; 2],
}

pub struct GLNodesFormat;

impl AssetFormat for GLNodesFormat {
	type Asset = Vec<GLNode>;

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
