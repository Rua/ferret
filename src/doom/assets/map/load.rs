use crate::{
	common::{
		assets::{AssetHandle, AssetStorage, ImportData},
		geometry::{Angle, Interval, Line2, Plane2, Plane3, Side, AABB2},
	},
	doom::{
		assets::{
			image::Image,
			map::{
				textures::TextureType, Anim, Linedef, Map, Node, NodeChild, Sector, SectorSlot,
				Seg, Sidedef, SidedefSlot, Subsector, Thing, ThingFlags,
			},
			wad::read_string,
		},
		data::anims::{AnimData, ANIMS, SWITCHES},
		game::{physics::SolidBits, trace::CollisionPlane},
		iwad::IWADInfo,
	},
};
use anyhow::{bail, ensure};
use bitflags::bitflags;
use byteorder::{ReadBytesExt, LE};
use fnv::FnvHashMap;
use nalgebra::{Vector2, Vector3};
use relative_path::RelativePath;
use serde::Deserialize;
use std::{cmp::Ordering, io::Read};

pub struct MapData {
	pub linedefs: Vec<Linedef>,
	pub subsectors: Vec<Subsector>,
	pub nodes: Vec<Node>,
	pub sectors: Vec<Sector>,
}

pub fn import_map(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let source = asset_storage.source();

	// Load GL nodes if available
	let gl_path = path.with_file_name(format!("gl_{}", path));
	let gl_data = (|| -> Option<[Vec<u8>; 4]> {
		let gl_vert = source.load(&gl_path.with_extension("gl_vert")).ok()?;
		let gl_segs = source.load(&gl_path.with_extension("gl_segs")).ok()?;
		let gl_ssect = source.load(&gl_path.with_extension("gl_ssect")).ok()?;
		let gl_nodes = source.load(&gl_path.with_extension("gl_nodes")).ok()?;

		Some([gl_vert, gl_segs, gl_ssect, gl_nodes])
	})();

	let linedefs = source.load(&path.with_extension("linedefs"))?;
	let sidedefs = source.load(&path.with_extension("sidedefs"))?;
	let vertexes = source.load(&path.with_extension("vertexes"))?;
	let segs = source.load(&path.with_extension("segs"))?;
	let ssectors = source.load(&path.with_extension("ssectors"))?;
	let nodes = source.load(&path.with_extension("nodes"))?;
	let sectors = source.load(&path.with_extension("sectors"))?;

	let vertexes = build_vertexes(vertexes)?;
	let mut sectors = build_sectors(sectors, asset_storage)?;
	let mut sidedefs = build_sidedefs(sidedefs, &sectors, asset_storage)?;
	let linedefs = build_linedefs(linedefs, &vertexes, &mut sectors, &mut sidedefs)?;

	let (mut subsectors, mut nodes) = if let Some([gl_vert, gl_segs, gl_ssect, gl_nodes]) = gl_data
	{
		let gl_vert = build_gl_vert(gl_vert)?;
		let gl_segs = build_gl_segs(gl_segs, &vertexes, &gl_vert, &linedefs)?;
		let gl_ssect = build_gl_ssect(gl_ssect, &gl_segs, &linedefs)?;
		let gl_nodes = build_gl_nodes(gl_nodes, &gl_ssect)?;

		(gl_ssect, gl_nodes)
	} else {
		// GL nodes are not available, so use the regular nodes
		log::warn!("GL nodes are not available for map, falling back to standard nodes");

		let segs = build_segs(segs, &vertexes, &linedefs)?;
		let mut ssectors = build_ssectors(ssectors, &segs, &linedefs)?;
		let nodes = build_nodes(nodes, &ssectors)?;

		// Add floating point precision to segs,
		// and create extra segs to make full convex polygons
		fixup_nodes(
			NodeChild::Node(0),
			&nodes,
			&linedefs,
			&mut ssectors,
			&mut Vec::new(),
		)?;

		(ssectors, nodes)
	};

	// Add subsectors to sectors
	for (i, subsector) in subsectors.iter().enumerate() {
		sectors[subsector.sector_index].subsectors.push(i);
	}

	// Add linedefs to nodes
	add_node_linedefs(&mut nodes, &mut subsectors, &linedefs);

	let map_data = MapData {
		linedefs,
		subsectors,
		nodes,
		sectors,
	};

	Ok(Box::new(map_data))
}

fn build_vertexes(data: Vec<u8>) -> anyhow::Result<Vec<Vector2<f32>>> {
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

fn build_sectors(data: Vec<u8>, asset_storage: &mut AssetStorage) -> anyhow::Result<Vec<Sector>> {
	let chunks = data.chunks(26);
	let mut ret = Vec::with_capacity(chunks.len());

	for mut reader in chunks {
		ret.push(Sector {
			interval: Interval::new(
				reader.read_i16::<LE>()? as f32,
				reader.read_i16::<LE>()? as f32,
			),
			textures: [
				{
					let name = read_string(&mut reader)?;

					if &name == "-" {
						TextureType::None
					} else {
						if &name == "f_sky1" {
							TextureType::Sky
						} else {
							TextureType::Normal(asset_storage.load(&format!("{}.flat", name)))
						}
					}
				},
				{
					let name = read_string(&mut reader)?;

					if &name == "-" {
						TextureType::None
					} else {
						if &name == "f_sky1" {
							TextureType::Sky
						} else {
							TextureType::Normal(asset_storage.load(&format!("{}.flat", name)))
						}
					}
				},
			],
			light_level: reader.read_u16::<LE>()? as f32 / 255.0,
			special_type: reader.read_u16::<LE>()?,
			sector_tag: reader.read_u16::<LE>()?,
			linedefs: Vec::new(),
			neighbours: Vec::new(),
			subsectors: Vec::new(),
		});
	}

	Ok(ret)
}

fn build_sidedefs(
	data: Vec<u8>,
	sectors: &[Sector],
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Vec<Option<Sidedef>>> {
	let chunks = data.chunks(30);
	let mut ret = Vec::with_capacity(chunks.len());

	for (i, mut reader) in chunks.enumerate() {
		ret.push(Some(Sidedef {
			texture_offset: Vector2::new(
				reader.read_i16::<LE>()? as f32,
				reader.read_i16::<LE>()? as f32,
			),
			textures: [
				{
					let name = read_string(&mut reader)?;

					if &name == "-" {
						TextureType::None
					} else {
						if &name == "f_sky1" {
							TextureType::Sky
						} else {
							TextureType::Normal(asset_storage.load(&format!("{}.texture", name)))
						}
					}
				},
				{
					let name = read_string(&mut reader)?;

					if &name == "-" {
						TextureType::None
					} else {
						if &name == "f_sky1" {
							TextureType::Sky
						} else {
							TextureType::Normal(asset_storage.load(&format!("{}.texture", name)))
						}
					}
				},
				{
					let name = read_string(&mut reader)?;

					if &name == "-" {
						TextureType::None
					} else {
						if &name == "f_sky1" {
							TextureType::Sky
						} else {
							TextureType::Normal(asset_storage.load(&format!("{}.texture", name)))
						}
					}
				},
			],
			sector_index: {
				let sector_index = reader.read_u16::<LE>()? as usize;

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
	data: Vec<u8>,
	vertexes: &[Vector2<f32>],
	sectors: &mut [Sector],
	sidedefs: &mut [Option<Sidedef>],
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
			// Set sector linedefs
			sectors[front_sidedef.sector_index].linedefs.push(i);
			sectors[back_sidedef.sector_index].linedefs.push(i);

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
			if sectors[front_sidedef.sector_index].textures[SectorSlot::Ceiling as usize].is_sky()
				&& sectors[back_sidedef.sector_index].textures[SectorSlot::Ceiling as usize]
					.is_sky()
			{
				front_sidedef.textures[SidedefSlot::Top as usize] = TextureType::Sky;
				back_sidedef.textures[SidedefSlot::Top as usize] = TextureType::Sky;
			}
		} else if let [Some(ref mut front_sidedef), None] = &mut sidedefs {
			// Set sector linedefs
			sectors[front_sidedef.sector_index].linedefs.push(i);
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

		let mut collision_planes = bbox
			.planes()
			.iter()
			.map(|p| CollisionPlane(*p, true))
			.collect::<Vec<_>>();

		if normal[0] != 0.0 && normal[1] != 0.0 {
			collision_planes.push(CollisionPlane(
				Plane3::new(
					Vector3::new(normal[0], normal[1], 0.0),
					line.point.dot(&normal),
				),
				true,
			));
			collision_planes.push(CollisionPlane(
				Plane3::new(
					Vector3::new(-normal[0], -normal[1], 0.0),
					-line.point.dot(&normal),
				),
				true,
			));
		}

		ret.push(Linedef {
			line,
			normal,
			collision_planes,
			bbox,
			flags,
			blocks_types: if flags.intersects(LinedefFlags::BLOCKING) {
				SolidBits::PLAYER | SolidBits::MONSTER
			} else if flags.intersects(LinedefFlags::BLOCKMONSTERS) {
				SolidBits::MONSTER
			} else {
				SolidBits::empty()
			},
			special_type,
			sector_tag,
			sidedefs,
		});
	}

	Ok(ret)
}

fn build_segs(
	data: Vec<u8>,
	vertexes: &[Vector2<f32>],
	linedefs: &[Linedef],
) -> anyhow::Result<Vec<Seg>> {
	let chunks = data.chunks(12);
	let mut ret = Vec::with_capacity(chunks.len());

	for (i, mut chunk) in chunks.enumerate() {
		let vertices = [
			{
				let index = chunk.read_u16::<LE>()? as usize;
				ensure!(
					index < vertexes.len(),
					"Seg {} has invalid vertex index {}",
					i,
					index
				);
				vertexes[index]
			},
			{
				let index = chunk.read_u16::<LE>()? as usize;
				ensure!(
					index < vertexes.len(),
					"Seg {} has invalid vertex index {}",
					i,
					index
				);
				vertexes[index]
			},
		];

		let _angle = chunk.read_i16::<LE>()?;
		let dir = vertices[1] - vertices[0];

		ret.push(Seg {
			line: Line2::new(vertices[0], dir),
			normal: Vector2::new(dir[1], -dir[0]).normalize(),
			linedef: {
				let index = chunk.read_u16::<LE>()? as usize;
				let side = match chunk.read_u16::<LE>()? as usize {
					0 => Side::Right,
					_ => Side::Left,
				};

				ensure!(
					index < linedefs.len(),
					"Seg {} has invalid linedef index {}",
					i,
					index
				);
				Some((index, side))
			},
		});
	}

	Ok(ret)
}

fn build_ssectors(
	data: Vec<u8>,
	segs: &[Seg],
	linedefs: &[Linedef],
) -> anyhow::Result<Vec<Subsector>> {
	let chunks = data.chunks(4);
	let mut ret = Vec::with_capacity(chunks.len());

	for (i, mut chunk) in chunks.enumerate() {
		let seg_count = chunk.read_u16::<LE>()? as usize;
		let first_seg_index = chunk.read_u16::<LE>()? as usize;

		ensure!(
			first_seg_index < segs.len(),
			"SSECTOR {} has invalid first seg index {}",
			i,
			first_seg_index
		);
		ensure!(seg_count > 0, "SSECTOR {} has zero seg count", i,);
		ensure!(
			first_seg_index + seg_count <= segs.len(),
			"SSECTOR {} has overflowing seg count {}",
			i,
			seg_count
		);

		let segs = &segs[first_seg_index..first_seg_index + seg_count];

		let sector_index = {
			if let Some(sidedef) = segs.iter().find_map(|seg| match seg.linedef {
				None => None,
				Some((index, side)) => linedefs[index].sidedefs[side as usize].as_ref(),
			}) {
				sidedef.sector_index
			} else {
				bail!("No sector could be found for subsector {}", i);
			}
		};

		ret.push(Subsector {
			segs: segs.to_owned(),
			bbox: AABB2::empty(),
			collision_planes: Vec::new(),
			linedefs: segs
				.iter()
				.filter_map(|seg| seg.linedef.map(|(i, _)| i))
				.collect(),
			sector_index,
		});
	}

	Ok(ret)
}

fn build_nodes(data: Vec<u8>, ssectors: &[Subsector]) -> anyhow::Result<Vec<Node>> {
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

		let normal = Vector2::new(partition_dir[1], -partition_dir[0]).normalize();
		let distance = partition_point.dot(&normal);

		ret.push(Node {
			plane: Plane2::new(normal, distance),
			linedefs: Vec::new(),
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
							(index as usize) < ssectors.len(),
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
							(index as usize) < ssectors.len(),
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

fn build_gl_vert(data: Vec<u8>) -> anyhow::Result<Vec<Vector2<f32>>> {
	let mut data = data.as_slice();
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

fn build_gl_segs(
	data: Vec<u8>,
	vertexes: &[Vector2<f32>],
	gl_vert: &[Vector2<f32>],
	linedefs: &[Linedef],
) -> anyhow::Result<Vec<Seg>> {
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

		ret.push(Seg {
			line: Line2::new(vertices[0], dir),
			normal: Vector2::new(dir[1], -dir[0]).normalize(),
			linedef: {
				let index = chunk.read_u16::<LE>()? as usize;
				let side = match chunk.read_u16::<LE>()? as usize {
					0 => Side::Right,
					_ => Side::Left,
				};

				match index {
					0xFFFF => None,
					index => {
						ensure!(
							index < linedefs.len(),
							"GLSeg {} has invalid linedef index {}",
							i,
							index
						);
						Some((index, side))
					}
				}
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
	data: Vec<u8>,
	gl_segs: &[Seg],
	linedefs: &[Linedef],
) -> anyhow::Result<Vec<Subsector>> {
	let chunks = data.chunks(4);
	let mut ret = Vec::with_capacity(chunks.len());

	for (i, mut chunk) in chunks.enumerate() {
		let seg_count = chunk.read_u16::<LE>()? as usize;
		let first_seg_index = chunk.read_u16::<LE>()? as usize;

		ensure!(
			first_seg_index < gl_segs.len(),
			"GLSSect {} has invalid first seg index {}",
			i,
			first_seg_index
		);
		ensure!(seg_count > 0, "GLSSect {} has zero seg count", i,);
		ensure!(
			first_seg_index + seg_count <= gl_segs.len(),
			"GLSSect {} has overflowing seg count {}",
			i,
			seg_count
		);

		let segs = &gl_segs[first_seg_index..first_seg_index + seg_count];

		let sector_index = {
			if let Some(sidedef) = segs.iter().find_map(|seg| match seg.linedef {
				None => None,
				Some((index, side)) => linedefs[index].sidedefs[side as usize].as_ref(),
			}) {
				sidedef.sector_index
			} else {
				bail!("No sector could be found for GLSSect {}", i);
			}
		};

		let (bbox, collision_planes) = generate_subsector_planes(&segs);

		ret.push(Subsector {
			segs: segs.to_owned(),
			collision_planes,
			linedefs: segs
				.iter()
				.filter_map(|seg| seg.linedef.map(|(i, _)| i))
				.collect(),
			sector_index,
			bbox,
		});
	}

	Ok(ret)
}

fn build_gl_nodes(data: Vec<u8>, gl_ssect: &[Subsector]) -> anyhow::Result<Vec<Node>> {
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

		let normal = Vector2::new(partition_dir[1], -partition_dir[0]).normalize();
		let distance = partition_point.dot(&normal);

		ret.push(Node {
			plane: Plane2::new(normal, distance),
			linedefs: Vec::new(),
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
							"GLNode {} has invalid subsector index {}",
							i,
							index
						);
						NodeChild::Subsector(index)
					}
					index => {
						ensure!(
							index < len,
							"GLNode {} has invalid child node index {}",
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
							"GLNode {} has invalid subsector index {}",
							i,
							index
						);
						NodeChild::Subsector(index)
					}
					index => {
						ensure!(
							index < len,
							"GLNode {} has invalid child node index {}",
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
			r#type: chunk.read_u16::<LE>()?,
			flags: ThingFlags::from_bits_truncate(chunk.read_u16::<LE>()?),
		});
	}

	Ok(ret)
}

fn generate_subsector_planes(segs: &[Seg]) -> (AABB2, Vec<CollisionPlane>) {
	let bbox = {
		let mut bbox = AABB2::empty();
		for seg in segs.iter() {
			bbox.add_point(seg.line.point);
		}
		bbox
	};

	let mut collision_planes = bbox
		.planes()
		.iter()
		.map(|p| CollisionPlane(*p, true))
		.collect::<Vec<_>>();

	collision_planes.extend(segs.iter().filter_map(|seg| {
		if seg.normal[0] != 0.0 && seg.normal[1] != 0.0 {
			Some(CollisionPlane(
				Plane3::new(
					Vector3::new(-seg.normal[0], -seg.normal[1], 0.0),
					seg.line.point.dot(&-seg.normal),
				),
				false,
			))
		} else {
			None
		}
	}));

	(bbox, collision_planes)
}

fn fixup_nodes(
	child: NodeChild,
	nodes: &[Node],
	linedefs: &[Linedef],
	subsectors: &mut [Subsector],
	planes: &mut Vec<Plane2>,
) -> anyhow::Result<()> {
	match child {
		NodeChild::Node(index) => {
			let node = &nodes[index];

			planes.push(node.plane);
			fixup_nodes(
				node.child_indices[Side::Right as usize],
				nodes,
				linedefs,
				subsectors,
				planes,
			)?;
			planes.pop();

			planes.push(node.plane.inverse());
			fixup_nodes(
				node.child_indices[Side::Left as usize],
				nodes,
				linedefs,
				subsectors,
				planes,
			)?;
			planes.pop();
		}
		NodeChild::Subsector(index) => {
			let subsector = &mut subsectors[index];
			fixup_segs(index, &mut subsector.segs, linedefs, planes)?;
			rebuild_segs(&mut subsector.segs, &planes)?;

			let (bbox, collision_planes) = generate_subsector_planes(&subsector.segs);
			subsector.bbox = bbox;
			subsector.collision_planes = collision_planes;
		}
	}

	Ok(())
}

fn fixup_segs(
	subsector_index: usize,
	segs: &mut [Seg],
	linedefs: &[Linedef],
	planes: &[Plane2],
) -> anyhow::Result<()> {
	for seg in segs.iter_mut() {
		if let Some((linedef_index, linedef_side)) = seg.linedef {
			let linedef = &linedefs[linedef_index];
			let line = match linedef_side {
				Side::Left => linedef.line.inverse(),
				Side::Right => linedef.line,
			};
			let mut interval = Interval::new(0.0, 1.0);

			for plane in planes.iter() {
				if let Some(t) = intersect_line_plane(&line, plane) {
					if line.dir.dot(&plane.normal) > 0.0 {
						if t > interval.min && t < 1.0 {
							interval.min = t;
						}
					} else {
						if t < interval.max && t > 0.0 {
							interval.max = t;
						}
					}
				}
			}

			if !interval.is_proper() {
				log::warn!(
					"Subsector {} linedef {} has been reduced to zero length by BSP plane intersections",
					subsector_index,
					linedef_index,
				);
			} else {
				let line = Line2::new(
					line.point + line.dir * interval.min,
					line.dir * (interval.max - interval.min),
				);
				seg.line = line;
				seg.normal = Vector2::new(line.dir[1], -line.dir[0]).normalize();
			};
		}
	}

	Ok(())
}

fn rebuild_segs(segs: &mut Vec<Seg>, planes: &[Plane2]) -> anyhow::Result<()> {
	let mut points: Vec<(Vector2<f32>, Option<Seg>)> = segs
		.iter()
		.map(|seg| (seg.line.point, Some(seg.clone())))
		.collect();

	// Add seg end points
	for seg in segs.iter() {
		let point = seg.line.end_point();

		// Point must not be on an existing point
		let points_check = |(other, _): &(Vector2<f32>, _)| (other - point).norm() >= 0.01;

		if points.iter().all(&points_check) {
			points.push((point, None));
		}
	}

	// Find implicit points by intersecting planes
	for i_plane in 0..(planes.len() - 1) {
		for j_plane in (i_plane + 1)..planes.len() {
			let p1 = &planes[i_plane];
			let p2 = &planes[j_plane];

			let point = if let Some(point) = intersect_planes(&p1, p2) {
				point
			} else {
				continue;
			};

			// Point must be in front of plane
			let plane_check = |p: &Plane2| point.dot(&p.normal) - p.distance >= -0.1;

			// Point must be in front of seg
			let seg_check =
				|seg: &Seg| point.dot(&seg.normal) - seg.line.point.dot(&seg.normal) >= -0.1;

			// Point must not be on an existing point
			let points_check = |(other, _): &(Vector2<f32>, _)| (other - point).norm() >= 0.01;

			if planes.iter().all(&plane_check)
				&& segs.iter().all(&seg_check)
				&& points.iter().all(&points_check)
			{
				points.push((point, None));
			}
		}
	}

	// Sort points in anticlockwise order around their center
	let center = points.iter().map(|(p, _)| p).sum::<Vector2<f32>>() / points.len() as f32;
	points.sort_unstable_by(|(a, _), (b, _)| {
		let ac = a - center;
		let bc = b - center;

		if ac[0] >= 0.0 && bc[0] < 0.0 {
			Ordering::Greater
		} else if ac[0] < 0.0 && bc[0] >= 0.0 {
			Ordering::Less
		} else if ac[0] == 0.0 && bc[0] == 0.0 {
			if ac[1] >= 0.0 || bc[1] >= 0.0 {
				if a[1] > b[1] {
					Ordering::Greater
				} else {
					Ordering::Less
				}
			} else if b[1] > a[1] {
				Ordering::Greater
			} else {
				Ordering::Less
			}
		} else if ac.perp(&bc) < 0.0 {
			Ordering::Greater
		} else {
			Ordering::Less
		}
	});

	// Add segs in reverse order
	segs.clear();
	let first_point = points.last().unwrap().0;
	while let Some((point, seg)) = points.pop() {
		let next = points.last().map(|(p, _)| p).unwrap_or(&first_point);

		segs.push({
			if let Some(seg) = seg {
				//assert_eq!(seg.line.end_point(), *next);
				seg
			} else {
				let line = Line2::new(point, next - point);

				Seg {
					line,
					normal: Vector2::new(line.dir[1], -line.dir[0]).normalize(),
					linedef: None,
				}
			}
		});
	}

	Ok(())
}

fn intersect_line_plane(line: &Line2, plane: &Plane2) -> Option<f32> {
	let denom = line.dir.dot(&plane.normal);

	if denom.abs() < 0.01 {
		return None;
	}

	Some((plane.distance - line.point.dot(&plane.normal)) / denom)
}

fn intersect_planes(plane1: &Plane2, plane2: &Plane2) -> Option<nalgebra::Vector2<f32>> {
	let denom = plane1.normal.perp(&plane2.normal);

	if denom.abs() < 0.01 {
		return None;
	}

	let t = (plane2.distance - plane1.distance * plane1.normal.dot(&plane2.normal)) / denom;
	let matrix = nalgebra::Matrix2::new(plane1.distance, -t, t, plane1.distance);

	Some(matrix * plane1.normal)
}

pub fn get_anims(
	data: &[AnimData],
	asset_storage: &mut AssetStorage,
) -> FnvHashMap<AssetHandle<Image>, Anim> {
	let mut ret = FnvHashMap::default();

	for anim_data in data {
		assert!(!anim_data.frames.is_empty());
		let name = anim_data.frames.last().unwrap();
		if let Some(handle) = asset_storage.handle_for(name) {
			ret.insert(
				handle,
				Anim {
					frames: anim_data
						.frames
						.iter()
						.map(|name| asset_storage.load(name))
						.collect(),
					frame_time: anim_data.frame_time,
				},
			);
		}
	}

	ret
}

pub fn get_switches(
	asset_storage: &mut AssetStorage,
) -> FnvHashMap<AssetHandle<Image>, AssetHandle<Image>> {
	let mut ret = FnvHashMap::default();

	for [name1, name2] in SWITCHES.iter() {
		let handle1 = asset_storage.handle_for(name1);
		let handle2 = asset_storage.handle_for(name2);

		if handle1.is_none() && handle2.is_none() {
			continue;
		}

		let handle1 = handle1.unwrap_or_else(|| asset_storage.load(name1));
		let handle2 = handle2.unwrap_or_else(|| asset_storage.load(name2));

		ret.insert(handle1.clone(), handle2.clone());
		ret.insert(handle2, handle1);
	}

	ret
}

fn add_node_linedefs<'a>(
	nodes: &'a mut [Node],
	subsectors: &'a mut [Subsector],
	linedefs: &[Linedef],
) {
	fn traverse_nodes(
		index: usize,
		path: &mut Vec<usize>,
		subsector_paths: &mut Vec<Vec<usize>>,
		nodes: &[Node],
	) {
		path.push(index);

		for child in nodes[index].child_indices.into_iter() {
			match child {
				NodeChild::Subsector(index) => subsector_paths[index] = path.clone(),
				NodeChild::Node(index) => traverse_nodes(index, path, subsector_paths, nodes),
			}
		}

		path.pop();
	}

	// Find the BSP traversal path for each subsector
	let mut subsector_paths: Vec<Vec<usize>> = vec![Vec::new(); subsectors.len()];
	traverse_nodes(0, &mut Vec::new(), &mut subsector_paths, nodes);

	// Find and then iterate over the subsectors each linedef appears in
	let mut linedef_subsectors: Vec<Vec<usize>> = vec![Vec::new(); linedefs.len()];

	for (i, subsector) in subsectors.iter().enumerate() {
		for index in subsector.linedefs.iter().copied() {
			linedef_subsectors[index].push(i);
		}
	}

	for (linedef_index, subs) in linedef_subsectors.into_iter().enumerate() {
		// A linedef appearing in only one subsector has itself as its common parent
		if subs.len() <= 1 {
			continue;
		}

		// Find the max common path depth for the subsectors
		let mut depth = 0;

		loop {
			let mut steps = subs.iter().map(|i| subsector_paths[*i].get(depth));

			if steps.clone().all(|s| s.is_none()) {
				break;
			}

			let first = steps.next().unwrap();

			if steps.all(|s| s == first) {
				depth += 1;
			} else {
				break;
			}
		}

		// Add linedef to node
		let index = subsector_paths[subs[0]][depth - 1];
		nodes[index].linedefs.push(linedef_index);

		// Remove linedef from subsectors
		for subsector_index in subs {
			subsectors[subsector_index]
				.linedefs
				.retain(|x| *x != linedef_index);
		}
	}
}

pub fn process_map(asset_storage: &mut AssetStorage, iwadinfo: &IWADInfo) {
	asset_storage.process::<Map, _>(|name, data, asset_storage| {
		let map_data: MapData = *data.downcast().ok().expect("Not a MapData");
		let map_info = iwadinfo.maps.get(name).unwrap(); // TODO handle missing info

		let MapData {
			linedefs,
			subsectors,
			nodes,
			sectors,
		} = map_data;

		// Create map-wide bounding box
		let mut bbox = AABB2::empty();

		for linedef in &linedefs {
			bbox.add_point(linedef.line.point);
			bbox.add_point(linedef.line.end_point());
		}

		Ok(Map {
			name: map_info.name.to_owned(),
			anims: get_anims(&ANIMS, asset_storage),
			bbox,
			sky: asset_storage.load(map_info.sky),
			switches: get_switches(asset_storage),
			exit: map_info.exit.map(|s| s.to_owned()),
			secret_exit: map_info.secret_exit.map(|s| s.to_owned()),

			linedefs,
			nodes,
			sectors,
			subsectors,
		})
	});
}
