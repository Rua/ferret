use crate::{
	assets::{Asset, AssetStorage, DataSource},
	doom::{
		map::{
			textures::{Flat, TextureType, WallTexture},
			GLNode, GLSSect, GLSeg, Linedef, Map, NodeChild, Sector, Sidedef,
		},
		physics::SolidMask,
		wad::WadLoader,
	},
	geometry::{Angle, Interval, Line2, Plane, Side, AABB2},
};
use anyhow::{bail, ensure};
use bitflags::bitflags;
use byteorder::{ReadBytesExt, LE};
use nalgebra::{Vector2, Vector3};
use serde::Deserialize;
use std::io::Read;

pub struct MapData {
	pub linedefs: Vec<u8>,
	pub sidedefs: Vec<u8>,
	pub vertexes: Vec<u8>,
	pub sectors: Vec<u8>,
	pub gl_vert: Vec<u8>,
	pub gl_segs: Vec<u8>,
	pub gl_ssect: Vec<u8>,
	pub gl_nodes: Vec<u8>,
}

impl Asset for Map {
	type Data = Self;
	type Intermediate = MapData;
	const NAME: &'static str = "Map";

	fn import(name: &str, source: &impl DataSource) -> anyhow::Result<Self::Intermediate> {
		let gl_name = format!("GL_{}", name);

		Ok(MapData {
			linedefs: source.load(&format!("{}/+{}", name, 2))?,
			sidedefs: source.load(&format!("{}/+{}", name, 3))?,
			vertexes: source.load(&format!("{}/+{}", name, 4))?,
			sectors: source.load(&format!("{}/+{}", name, 8))?,
			gl_vert: source.load(&format!("{}/+{}", gl_name, 1))?,
			gl_segs: source.load(&format!("{}/+{}", gl_name, 2))?,
			gl_ssect: source.load(&format!("{}/+{}", gl_name, 3))?,
			gl_nodes: source.load(&format!("{}/+{}", gl_name, 4))?,
		})
	}
}

pub fn build_map(
	map_data: MapData,
	sky_name: &str,
	loader: &mut WadLoader,
	flat_storage: &mut AssetStorage<Flat>,
	wall_texture_storage: &mut AssetStorage<WallTexture>,
) -> anyhow::Result<Map> {
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

	let vertexes = build_vertexes(&vertexes_data)?;
	let mut sectors = build_sectors(&sectors_data, loader, flat_storage)?;
	let mut sidedefs = build_sidedefs(&sidedefs_data, &sectors, loader, wall_texture_storage)?;
	let linedefs = build_linedefs(&linedefs_data, &vertexes, &mut sectors, &mut sidedefs)?;

	let gl_vert = build_gl_vert(&gl_vert_data)?;
	let mut gl_segs = build_gl_segs(&gl_segs_data, &vertexes, &gl_vert, &linedefs)?;
	let gl_ssect = build_gl_ssect(&gl_ssect_data, &mut gl_segs, &mut sectors, &linedefs)?;
	let gl_nodes = build_gl_nodes(&gl_nodes_data, &gl_ssect)?;

	Ok(Map {
		linedefs,
		sectors,
		subsectors: gl_ssect,
		nodes: gl_nodes,
		sky,
	})
}

fn build_vertexes(data: &[u8]) -> anyhow::Result<Vec<Vector2<f32>>> {
	let chunks = data.chunks(4);
	let mut ret = Vec::with_capacity(chunks.len());

	for mut chunk in chunks {
		ret.push(Vector2::new(
			chunk.read_i16::<LE>()? as f32,
			chunk.read_i16::<LE>()? as f32,
		));
	}

	Ok(ret)
}

fn build_gl_vert(mut data: &[u8]) -> anyhow::Result<Vec<Vector2<f32>>> {
	let mut buf = [0u8; 4];
	data.read_exact(&mut buf)?;

	ensure!(&buf == b"gNd2", "No gNd2 signature found in GL_VERT lump");

	let chunks = data.chunks(8);
	let mut ret = Vec::with_capacity(chunks.len());

	for mut chunk in chunks {
		ret.push(Vector2::new(
			chunk.read_i32::<LE>()? as f32 / 65536.0,
			chunk.read_i32::<LE>()? as f32 / 65536.0,
		));
	}

	Ok(ret)
}

fn build_sectors(
	data: &[u8],
	loader: &mut WadLoader,
	flat_storage: &mut AssetStorage<Flat>,
) -> anyhow::Result<Vec<Sector>> {
	let chunks = data.chunks(26);
	let mut ret = Vec::with_capacity(chunks.len());

	for mut chunk in chunks {
		let mut buf = [0u8; 8];

		ret.push(Sector {
			interval: Interval::new(
				chunk.read_i16::<LE>()? as f32,
				chunk.read_i16::<LE>()? as f32,
			),
			floor_texture: {
				chunk.read_exact(&mut buf)?;

				if &buf == b"-\0\0\0\0\0\0\0" {
					TextureType::None
				} else {
					let name = std::str::from_utf8(&buf)?.trim_end_matches('\0').to_owned();

					if name == "F_SKY1" {
						TextureType::Sky
					} else {
						TextureType::Normal(flat_storage.load(&name, &mut *loader))
					}
				}
			},
			ceiling_texture: {
				chunk.read_exact(&mut buf)?;

				if &buf == b"-\0\0\0\0\0\0\0" {
					TextureType::None
				} else {
					let name = std::str::from_utf8(&buf)?.trim_end_matches('\0').to_owned();

					if name == "F_SKY1" {
						TextureType::Sky
					} else {
						TextureType::Normal(flat_storage.load(&name, &mut *loader))
					}
				}
			},
			light_level: chunk.read_u16::<LE>()? as f32 / 255.0,
			special_type: chunk.read_u16::<LE>()?,
			sector_tag: chunk.read_u16::<LE>()?,
			neighbours: Vec::new(),
			subsectors: Vec::new(),
		});
	}

	Ok(ret)
}

fn build_sidedefs(
	data: &[u8],
	sectors: &Vec<Sector>,
	loader: &mut WadLoader,
	wall_texture_storage: &mut AssetStorage<WallTexture>,
) -> anyhow::Result<Vec<Option<Sidedef>>> {
	let chunks = data.chunks(30);
	let mut ret = Vec::with_capacity(chunks.len());

	for (i, mut chunk) in chunks.enumerate() {
		let mut buf = [0u8; 8];

		ret.push(Some(Sidedef {
			texture_offset: Vector2::new(
				chunk.read_i16::<LE>()? as f32,
				chunk.read_i16::<LE>()? as f32,
			),
			top_texture: {
				chunk.read_exact(&mut buf)?;

				if &buf == b"-\0\0\0\0\0\0\0" {
					TextureType::None
				} else {
					let name = std::str::from_utf8(&buf)?.trim_end_matches('\0').to_owned();

					if name == "F_SKY1" {
						TextureType::Sky
					} else {
						TextureType::Normal(wall_texture_storage.load(&name, &mut *loader))
					}
				}
			},
			bottom_texture: {
				chunk.read_exact(&mut buf)?;

				if &buf == b"-\0\0\0\0\0\0\0" {
					TextureType::None
				} else {
					let name = std::str::from_utf8(&buf)?.trim_end_matches('\0').to_owned();

					if name == "F_SKY1" {
						TextureType::Sky
					} else {
						TextureType::Normal(wall_texture_storage.load(&name, &mut *loader))
					}
				}
			},
			middle_texture: {
				chunk.read_exact(&mut buf)?;

				if &buf == b"-\0\0\0\0\0\0\0" {
					TextureType::None
				} else {
					let name = std::str::from_utf8(&buf)?.trim_end_matches('\0').to_owned();

					if name == "F_SKY1" {
						TextureType::Sky
					} else {
						TextureType::Normal(wall_texture_storage.load(&name, &mut *loader))
					}
				}
			},
			sector_index: {
				let sector_index = chunk.read_u16::<LE>()? as usize;

				ensure!(
					sector_index < sectors.len(),
					"Sidedef {} has invalid sector index {}",
					i,
					sector_index
				);

				sector_index
			},
		}));
	}

	Ok(ret)
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

fn build_linedefs(
	data: &[u8],
	vertexes: &Vec<Vector2<f32>>,
	sectors: &mut Vec<Sector>,
	sidedefs: &mut Vec<Option<Sidedef>>,
) -> anyhow::Result<Vec<Linedef>> {
	let chunks = data.chunks(14);
	let mut ret = Vec::with_capacity(chunks.len());

	for (i, mut chunk) in chunks.enumerate() {
		// Read data
		let vertex_indices = [
			chunk.read_u16::<LE>()? as usize,
			chunk.read_u16::<LE>()? as usize,
		];

		let flags = LinedefFlags::from_bits_truncate(chunk.read_u16::<LE>()?);
		let special_type = chunk.read_u16::<LE>()?;
		let sector_tag = chunk.read_u16::<LE>()?;

		let sidedef_indices = [
			match chunk.read_u16::<LE>()? as usize {
				0xFFFF => None,
				x => Some(x),
			},
			match chunk.read_u16::<LE>()? as usize {
				0xFFFF => None,
				x => Some(x),
			},
		];

		for index in vertex_indices.iter() {
			ensure!(
				*index < vertexes.len(),
				"Linedef {} has invalid vertex index {}",
				i,
				index
			);
		}

		for index in sidedef_indices.iter().flatten() {
			ensure!(
				*index < sidedefs.len(),
				"Linedef {} has invalid sidedef index {}",
				i,
				index
			);
		}

		// Put it all together
		let mut sidedefs = [
			sidedef_indices[0].map(|x| sidedefs[x].take().unwrap()),
			sidedef_indices[1].map(|x| sidedefs[x].take().unwrap()),
		];

		if let [Some(ref mut front_sidedef), Some(ref mut back_sidedef)] = &mut sidedefs {
			// Set sector neighbours
			if front_sidedef.sector_index != back_sidedef.sector_index {
				let front_sector_neighbours = &mut sectors[front_sidedef.sector_index].neighbours;
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

		let dir = vertexes[vertex_indices[1]] - vertexes[vertex_indices[0]];
		let line = Line2::new(vertexes[vertex_indices[0]], dir);
		let normal = Vector2::new(dir[1], -dir[0]).normalize();
		let bbox = {
			let mut bbox = AABB2::empty();
			bbox.add_point(vertexes[vertex_indices[0]]);
			bbox.add_point(vertexes[vertex_indices[1]]);
			bbox
		};

		let mut planes = Vec::from(&bbox.planes()[..]);

		if normal[0] != 0.0 && normal[1] != 0.0 {
			planes.push(Plane {
				distance: line.point.dot(&normal),
				normal: Vector3::new(normal[0], normal[1], 0.0),
			});
			planes.push(Plane {
				distance: -line.point.dot(&normal),
				normal: Vector3::new(-normal[0], -normal[1], 0.0),
			});
		}

		ret.push(Linedef {
			line,
			normal,
			planes,
			bbox,
			flags,
			solid_mask: if flags.intersects(LinedefFlags::BLOCKING) {
				SolidMask::all()
			} else if flags.intersects(LinedefFlags::BLOCKMONSTERS) {
				SolidMask::MONSTER
			} else {
				SolidMask::empty()
			},
			special_type,
			sector_tag,
			sidedefs,
		});
	}

	Ok(ret)
}

fn build_gl_segs(
	data: &[u8],
	vertexes: &Vec<Vector2<f32>>,
	gl_vert: &Vec<Vector2<f32>>,
	linedefs: &Vec<Linedef>,
) -> anyhow::Result<Vec<GLSeg>> {
	let chunks = data.chunks(10);
	let mut ret = Vec::with_capacity(chunks.len());

	for (i, mut chunk) in chunks.enumerate() {
		let vertices = [
			match chunk.read_u16::<LE>()? as usize {
				x if x & 0x8000 != 0 => {
					let index = x & 0x7FFF;
					ensure!(
						index < gl_vert.len(),
						"GLSeg {} has invalid vertex index {}",
						i,
						index
					);
					gl_vert[index]
				}
				index => {
					ensure!(
						index < vertexes.len(),
						"GLSeg {} has invalid vertex index {}",
						i,
						index
					);
					vertexes[index]
				}
			},
			match chunk.read_u16::<LE>()? as usize {
				x if x & 0x8000 != 0 => {
					let index = x & 0x7FFF;
					ensure!(
						index < gl_vert.len(),
						"GLSeg {} has invalid vertex index {}",
						i,
						index
					);
					gl_vert[index]
				}
				index => {
					ensure!(
						index < vertexes.len(),
						"GLSeg {} has invalid vertex index {}",
						i,
						index
					);
					vertexes[index]
				}
			},
		];

		let dir = vertices[1] - vertices[0];

		ret.push(GLSeg {
			line: Line2::new(vertices[0], dir),
			normal: Vector2::new(dir[1], -dir[0]).normalize(),
			linedef_index: match chunk.read_u16::<LE>()? as usize {
				0xFFFF => None,
				index => {
					ensure!(
						index < linedefs.len(),
						"Seg {} has invalid linedef index {}",
						i,
						index
					);
					Some(index)
				}
			},
			linedef_side: match chunk.read_u16::<LE>()? as usize {
				0 => Side::Right,
				_ => Side::Left,
			},
			//partner_seg_index: data.partner_seg_index,
		});

		let _partner_seg_index = match chunk.read_u16::<LE>()? as usize {
			0xFFFF => None,
			x => Some(x),
		};
	}

	Ok(ret)
}

fn build_gl_ssect(
	data: &[u8],
	gl_segs: &mut Vec<GLSeg>,
	sectors: &mut Vec<Sector>,
	linedefs: &Vec<Linedef>,
) -> anyhow::Result<Vec<GLSSect>> {
	let chunks = data.chunks(4);
	let mut ret = Vec::with_capacity(chunks.len());

	for (i, mut chunk) in chunks.enumerate() {
		let seg_count = chunk.read_u16::<LE>()? as usize;
		let first_seg_index = chunk.read_u16::<LE>()? as usize;

		ensure!(
			first_seg_index < gl_segs.len(),
			"Subsector {} has invalid first seg index {}",
			i,
			first_seg_index
		);
		ensure!(
			first_seg_index + seg_count <= gl_segs.len(),
			"Subsector {} has overflowing seg count {}",
			i,
			seg_count
		);

		let segs = &mut gl_segs[first_seg_index..first_seg_index + seg_count];

		let sector_index = {
			if let Some(sidedef) = segs.iter().find_map(|seg| match seg.linedef_index {
				None => None,
				Some(index) => linedefs[index].sidedefs[seg.linedef_side as usize].as_ref(),
			}) {
				sidedef.sector_index
			} else {
				bail!("No sector could be found for subsector {}", i);
			}
		};

		let bbox = {
			let mut bbox = AABB2::empty();
			for seg in segs.iter() {
				bbox.add_point(seg.line.point);
			}
			bbox
		};

		let mut planes = Vec::from(&bbox.planes()[..]);

		planes.extend(segs.iter().filter_map(|seg| {
			if seg.normal[0] != 0.0 && seg.normal[1] != 0.0 {
				Some(Plane {
					distance: seg.line.point.dot(&-seg.normal),
					normal: Vector3::new(-seg.normal[0], -seg.normal[1], 0.0),
				})
			} else {
				None
			}
		}));

		sectors[sector_index].subsectors.push(i);

		ret.push(GLSSect {
			segs: segs.to_owned(),
			planes,
			sector_index,
			bbox,
		});
	}

	Ok(ret)
}

fn build_gl_nodes(data: &[u8], gl_ssect: &Vec<GLSSect>) -> anyhow::Result<Vec<GLNode>> {
	let chunks = data.chunks(28);
	let mut ret = Vec::with_capacity(chunks.len());
	let len = chunks.len();

	for (i, mut chunk) in chunks.enumerate() {
		let partition_point = Vector2::new(
			chunk.read_i16::<LE>()? as f32,
			chunk.read_i16::<LE>()? as f32,
		);

		let partition_dir = Vector2::new(
			chunk.read_i16::<LE>()? as f32,
			chunk.read_i16::<LE>()? as f32,
		);

		ret.push(GLNode {
			partition_line: Line2::new(partition_point, partition_dir),
			normal: Vector2::new(partition_dir[1], -partition_dir[0]).normalize(),
			child_bboxes: [
				AABB2::from_extents(
					chunk.read_i16::<LE>()? as f32,
					chunk.read_i16::<LE>()? as f32,
					chunk.read_i16::<LE>()? as f32,
					chunk.read_i16::<LE>()? as f32,
				),
				AABB2::from_extents(
					chunk.read_i16::<LE>()? as f32,
					chunk.read_i16::<LE>()? as f32,
					chunk.read_i16::<LE>()? as f32,
					chunk.read_i16::<LE>()? as f32,
				),
			],
			child_indices: [
				match chunk.read_u16::<LE>()? as usize {
					x if x & 0x8000 != 0 => {
						let index = x & 0x7FFF;
						ensure!(
							(index as usize) < gl_ssect.len(),
							"Node {} has invalid subsector index {}",
							i,
							index
						);
						NodeChild::Subsector(index)
					}
					index => {
						ensure!(
							index < len,
							"Node {} has invalid child node index {}",
							i,
							index
						);
						NodeChild::Node(len - index - 1)
					}
				},
				match chunk.read_u16::<LE>()? as usize {
					x if x & 0x8000 != 0 => {
						let index = x & 0x7FFF;
						ensure!(
							(index as usize) < gl_ssect.len(),
							"Node {} has invalid subsector index {}",
							i,
							index
						);
						NodeChild::Subsector(index)
					}
					index => {
						ensure!(
							index < len,
							"Node {} has invalid child node index {}",
							i,
							index
						);
						NodeChild::Node(len - index - 1)
					}
				},
			],
		});
	}

	Ok(ret.into_iter().rev().collect())
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

pub fn build_things(data: &[u8]) -> anyhow::Result<Vec<Thing>> {
	let chunks = data.chunks(10);
	let mut ret = Vec::with_capacity(chunks.len());

	for mut chunk in chunks {
		ret.push(Thing {
			position: Vector2::new(
				chunk.read_i16::<LE>()? as f32,
				chunk.read_i16::<LE>()? as f32,
			),
			angle: Angle::from_degrees(chunk.read_u16::<LE>()? as f64),
			doomednum: chunk.read_u16::<LE>()?,
			flags: ThingFlags::from_bits_truncate(chunk.read_u16::<LE>()?),
		});
	}

	Ok(ret)
}
