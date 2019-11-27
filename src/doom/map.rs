use crate::{
	assets::{AssetFormat, AssetHandle, AssetStorage, DataSource},
	doom::{
		components::TransformComponent,
		entities::{DOOMEDNUMS, ENTITIES},
	},
	geometry::BoundingBox2,
	renderer::{
		mesh::{Mesh, MeshBuilder},
		texture::Texture,
		video::Video,
	},
};
use nalgebra::{Matrix, Vector2, Vector3};
use serde::Deserialize;
use specs::{world::Builder, ReadExpect, SystemData, World, WorldExt};
use std::{collections::HashMap, error::Error, io::Cursor, str};
use vulkano::image::Dimensions;

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
				rotation: Vector3::new(0.0, 0.0, thing.angle),
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

pub struct MapModel {
	meshes: Vec<(AssetHandle<Texture>, Mesh)>,
}

impl MapModel {
	pub fn new(meshes: Vec<(AssetHandle<Texture>, Mesh)>) -> MapModel {
		MapModel { meshes }
	}

	pub fn meshes(&self) -> &Vec<(AssetHandle<Texture>, Mesh)> {
		&self.meshes
	}
}

#[derive(Debug, Default, Clone)]
pub struct VertexData {
	pub in_position: [f32; 3],
	pub in_texture_coord: [f32; 3],
	pub in_lightlevel: f32,
}
impl_vertex!(VertexData, in_position, in_texture_coord, in_lightlevel);

pub fn make_model(map_data: &DoomMap, world: &World) -> Result<MapModel, Box<dyn Error>> {
	// Load textures and flats
	let [textures, flats] = super::map_textures::load_textures(map_data, world)?;

	// Create meshes
	let meshes = make_meshes(map_data, &textures, &flats, world)?;
	let mut ret = Vec::new();

	let video = world.fetch::<Video>();

	for (tex, (vertices, indices)) in meshes {
		let (mesh, future) = MeshBuilder::new()
			.with_vertices(vertices)
			.with_indices(indices)
			.build(&video.queues().graphics)?;

		ret.push((tex, mesh));
	}

	Ok(MapModel::new(ret))
}

fn make_meshes(
	map: &DoomMap,
	textures: &HashMap<String, (AssetHandle<Texture>, usize)>,
	flats: &HashMap<String, (AssetHandle<Texture>, usize)>,
	world: &World,
) -> Result<HashMap<AssetHandle<Texture>, (Vec<VertexData>, Vec<u32>)>, Box<dyn Error>> {
	fn push_wall(
		vertices: &mut Vec<VertexData>,
		indices: &mut Vec<u32>,
		vert_h: [&Vector2<f32>; 2],
		vert_v: [f32; 2],
		offset: Vector2<f32>,
		peg_factor: [f32; 2],
		dimensions: Dimensions,
		texture_layer: f32,
		light_level: f32,
	) {
		let diff = vert_h[1] - vert_h[0];
		let width = Matrix::norm(&diff);
		let height = vert_v[1] - vert_v[0];
		indices.push(u32::max_value());

		for (h, v) in [(0, 0), (1, 0), (1, 1), (0, 1)].iter().copied() {
			indices.push(vertices.len() as u32);
			vertices.push(VertexData {
				in_position: [vert_h[h][0], vert_h[h][1], vert_v[v]],
				in_texture_coord: [
					(offset[0] + width * h as f32) / dimensions.width() as f32,
					(offset[1] + height * peg_factor[v]) / dimensions.height() as f32,
					texture_layer,
				],
				in_lightlevel: light_level,
			});
		}
	}

	fn push_flat<'a>(
		vertices: &mut Vec<VertexData>,
		indices: &mut Vec<u32>,
		iter: impl Iterator<Item = &'a Vector2<f32>>,
		vert_z: f32,
		dimensions: Dimensions,
		texture_layer: f32,
		light_level: f32,
	) {
		indices.push(u32::max_value());

		for vert in iter {
			indices.push(vertices.len() as u32);
			vertices.push(VertexData {
				in_position: [vert[0], vert[1], vert_z],
				in_texture_coord: [
					vert[0] / dimensions.width() as f32,
					vert[1] / dimensions.height() as f32,
					texture_layer,
				],
				in_lightlevel: light_level,
			});
		}
	}

	let mut meshes: HashMap<AssetHandle<Texture>, (Vec<VertexData>, Vec<u32>)> = HashMap::new();
	let texture_storage = <ReadExpect<AssetStorage<Texture>>>::fetch(world);

	for ssect in &map.gl_ssect {
		let segs = &map.gl_segs[ssect.first_seg_index..ssect.first_seg_index + ssect.seg_count];
		let mut sector = None;

		// Walls
		for (seg_index, seg) in segs.iter().enumerate() {
			if let Some(linedef_index) = seg.linedef_index {
				let linedef = &map.linedefs[linedef_index];

				if let Some(front_sidedef_index) = linedef.sidedef_indices[seg.side as usize] {
					let front_sidedef = &map.sidedefs[front_sidedef_index];

					// Assign sector
					if let Some(s) = sector {
						if s as *const _ != &map.sectors[front_sidedef.sector_index] as *const _ {
							return Err(Box::from("Not all the segs belong to the same sector!"));
						}
					} else {
						sector = Some(&map.sectors[front_sidedef.sector_index]);
					}

					let front_sector = sector.unwrap();

					// Get vertices
					let start_vertex = match seg.vertex_indices[0] {
						EitherVertex::Normal(index) => &map.vertexes[index],
						EitherVertex::GL(index) => &map.gl_vert[index],
					};

					let end_vertex = match seg.vertex_indices[1] {
						EitherVertex::Normal(index) => &map.vertexes[index],
						EitherVertex::GL(index) => &map.gl_vert[index],
					};

					// Set pegging parameters
					let top_peg_factor = if linedef.flags & 8 != 0 {
						[1.0, 0.0] // Align to top
					} else {
						[0.0, -1.0] // Align to bottom
					};

					let bottom_peg_factor = if linedef.flags & 16 != 0 {
						[0.0, -1.0] // Align to bottom
					} else {
						[1.0, 0.0] // Align to top
					};

					// Calculate texture offset
					let distance =
						Matrix::norm(&(start_vertex - &map.vertexes[linedef.vertex_indices[0]]));
					let texture_offset = front_sidedef.texture_offset + Vector2::new(distance, 0.0);

					// Two-sided or one-sided sidedef?
					if let Some(back_sidedef_index) = linedef.sidedef_indices[!seg.side as usize] {
						let back_sidedef = &map.sidedefs[back_sidedef_index];
						let back_sector = &map.sectors[back_sidedef.sector_index];
						let spans = [
							front_sector.floor_height,
							f32::max(back_sector.floor_height, front_sector.floor_height),
							f32::min(front_sector.ceiling_height, back_sector.ceiling_height),
							front_sector.ceiling_height,
						];

						// Top section
						if let Some(texture_name) = &front_sidedef.top_texture_name {
							let texture = &textures[texture_name];
							let dimensions = texture_storage.get(&texture.0).unwrap().dimensions();
							let (ref mut vertices, ref mut indices) =
								meshes.entry(texture.0.clone()).or_insert((vec![], vec![]));

							push_wall(
								vertices,
								indices,
								[start_vertex, end_vertex],
								[spans[2], spans[3]],
								texture_offset,
								top_peg_factor,
								dimensions,
								texture.1 as f32,
								(front_sector.light_level as f32) / 255.0,
							);
						}

						// Bottom section
						if let Some(texture_name) = &front_sidedef.bottom_texture_name {
							let texture = &textures[texture_name];
							let dimensions = texture_storage.get(&texture.0).unwrap().dimensions();
							let (ref mut vertices, ref mut indices) =
								meshes.entry(texture.0.clone()).or_insert((vec![], vec![]));

							push_wall(
								vertices,
								indices,
								[start_vertex, end_vertex],
								[spans[0], spans[1]],
								texture_offset,
								bottom_peg_factor,
								dimensions,
								texture.1 as f32,
								(front_sector.light_level as f32) / 255.0,
							);
						}

						// Middle section
						if let Some(texture_name) = &front_sidedef.middle_texture_name {
							let texture = &textures[texture_name];
							let dimensions = texture_storage.get(&texture.0).unwrap().dimensions();
							let (ref mut vertices, ref mut indices) =
								meshes.entry(texture.0.clone()).or_insert((vec![], vec![]));

							push_wall(
								vertices,
								indices,
								[start_vertex, end_vertex],
								[spans[1], spans[2]],
								texture_offset,
								bottom_peg_factor,
								dimensions,
								texture.1 as f32,
								(front_sector.light_level as f32) / 255.0,
							);
						}
					} else {
						if let Some(texture_name) = &front_sidedef.middle_texture_name {
							let texture = &textures[texture_name];
							let dimensions = texture_storage.get(&texture.0).unwrap().dimensions();
							let (ref mut vertices, ref mut indices) =
								meshes.entry(texture.0.clone()).or_insert((vec![], vec![]));

							push_wall(
								vertices,
								indices,
								[start_vertex, end_vertex],
								[front_sector.floor_height, front_sector.ceiling_height],
								texture_offset,
								bottom_peg_factor,
								dimensions,
								texture.1 as f32,
								(front_sector.light_level as f32) / 255.0,
							);
						}
					}
				}
			}
		}

		let sector = &sector.unwrap();

		// Floor
		{
			let flat = &flats[&sector.floor_flat_name];
			let dimensions = texture_storage.get(&flat.0).unwrap().dimensions();
			let (ref mut vertices, ref mut indices) =
				meshes.entry(flat.0.clone()).or_insert((vec![], vec![]));

			push_flat(
				vertices,
				indices,
				segs.iter().rev().map(|seg| match seg.vertex_indices[0] {
					EitherVertex::Normal(index) => &map.vertexes[index],
					EitherVertex::GL(index) => &map.gl_vert[index],
				}),
				sector.floor_height,
				dimensions,
				flat.1 as f32,
				(sector.light_level as f32) / 255.0,
			);
		}

		// Ceiling
		{
			let flat = &flats[&sector.ceiling_flat_name];
			let dimensions = texture_storage.get(&flat.0).unwrap().dimensions();
			let (ref mut vertices, ref mut indices) =
				meshes.entry(flat.0.clone()).or_insert((vec![], vec![]));

			push_flat(
				vertices,
				indices,
				segs.iter().map(|seg| match seg.vertex_indices[0] {
					EitherVertex::Normal(index) => &map.vertexes[index],
					EitherVertex::GL(index) => &map.gl_vert[index],
				}),
				sector.ceiling_height,
				dimensions,
				flat.1 as f32,
				(sector.light_level as f32) / 255.0,
			);
		}
	}

	Ok(meshes)
}

#[derive(Clone, Debug)]
pub struct DoomMap {
	pub linedefs: Vec<Linedef>,
	pub sidedefs: Vec<Sidedef>,
	pub vertexes: Vec<Vector2<f32>>,
	pub sectors: Vec<Sector>,
	pub gl_vert: Vec<Vector2<f32>>,
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
		let gl_name = format!("GL_{}", name);

		let vertexes = VertexesFormat.import(name, source)?;
		let gl_vert = GLVertFormat.import(&gl_name, source)?;
		let sectors = SectorsFormat.import(name, source)?;

		let sidedefs = SidedefsFormat.import(name, source)?;
		for (i, sidedef) in sidedefs.iter().enumerate() {
			if sidedef.sector_index >= sectors.len() {
				return Err(Box::from(format!(
					"Sidedef {} has invalid sector index {}",
					i, sidedef.sector_index
				)));
			}
		}

		let linedefs = LinedefsFormat.import(name, source)?;
		for (i, linedef) in linedefs.iter().enumerate() {
			for index in linedef.vertex_indices.iter() {
				if *index >= vertexes.len() {
					return Err(Box::from(format!(
						"Linedef {} has invalid vertex index {}",
						i, index
					)));
				}
			}

			for index in linedef.sidedef_indices.iter().filter_map(|x| *x) {
				if index >= sidedefs.len() {
					return Err(Box::from(format!(
						"Linedef {} has invalid sidedef index {}",
						i, index
					)));
				}
			}
		}

		let mut gl_segs = GLSegsFormat.import(&gl_name, source)?;
		for (i, seg) in gl_segs.iter().enumerate() {
			if let Some(index) = seg.linedef_index {
				if index >= linedefs.len() {
					return Err(Box::from(format!(
						"Seg {} has invalid linedef index {}",
						i, index
					)));
				}
			}

			for vertex_index in &seg.vertex_indices {
				let (list, index) = match vertex_index {
					EitherVertex::Normal(index) => (&vertexes, index),
					EitherVertex::GL(index) => (&gl_vert, index),
				};

				if *index >= list.len() {
					return Err(Box::from(format!(
						"Seg {} has invalid vertex index {}",
						i, index
					)));
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

		let mut gl_ssect = GLSSectFormat.import(&gl_name, source)?;
		for (i, ssect) in gl_ssect.iter().enumerate() {
			if ssect.first_seg_index >= gl_segs.len() {
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

		let gl_nodes = GLNodesFormat.import(&gl_name, source)?;
		for (i, node) in gl_nodes.iter().enumerate() {
			for child in node.child_indices.iter() {
				match child {
					ChildNode::Branch(index) => {
						if *index >= gl_nodes.len() {
							return Err(Box::from(format!(
								"Node {} has invalid child node index {}",
								i, index
							)));
						}
					}
					ChildNode::Leaf(index) => {
						if *index >= gl_ssect.len() {
							return Err(Box::from(format!(
								"Node {} has invalid subsector index {}",
								i, index
							)));
						}
					}
				}
			}
		}

		// Provide some extra links between items
		for seg in gl_segs.iter_mut() {
			if let Some(index) = seg.linedef_index {
				seg.sidedef_index = linedefs[index].sidedef_indices[seg.side as usize];
			}
		}

		for (i, ssect) in gl_ssect.iter_mut().enumerate() {
			if let Some(sidedef_index) = &gl_segs
				[ssect.first_seg_index..ssect.first_seg_index + ssect.seg_count]
				.iter()
				.find_map(|seg| seg.sidedef_index)
			{
				ssect.sector_index = sidedefs[*sidedef_index].sector_index;
			} else {
				return Err(Box::from(format!(
					"No sector could be found for subsector {}",
					i
				)));
			}
		}

		Ok(DoomMap {
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

#[derive(Clone, Debug)]
pub struct Linedef {
	pub vertex_indices: [usize; 2],
	pub flags: u16,
	pub special_type: u16,
	pub sector_tag: u16,
	pub sidedef_indices: [Option<usize>; 2],
}

pub struct LinedefsFormat;

impl AssetFormat for LinedefsFormat {
	type Asset = Vec<Linedef>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		RawLinedefsFormat
			.import(name, source)?
			.into_iter()
			.map(|raw| {
				Ok(Linedef {
					vertex_indices: [
						raw.vertex_indices[0] as usize,
						raw.vertex_indices[1] as usize,
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
			.collect()
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

#[derive(Clone, Debug)]
pub struct Sidedef {
	pub texture_offset: Vector2<f32>,
	pub top_texture_name: Option<String>,
	pub bottom_texture_name: Option<String>,
	pub middle_texture_name: Option<String>,
	pub sector_index: usize,
}

pub struct SidedefsFormat;

impl AssetFormat for SidedefsFormat {
	type Asset = Vec<Sidedef>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		RawSidedefsFormat
			.import(name, source)?
			.into_iter()
			.map(|raw| {
				Ok(Sidedef {
					texture_offset: Vector2::new(
						raw.texture_offset[0] as f32,
						raw.texture_offset[1] as f32,
					),
					top_texture_name: if raw.top_texture_name == *b"-\0\0\0\0\0\0\0" {
						None
					} else {
						Some(String::from(
							str::from_utf8(&raw.top_texture_name)?.trim_end_matches('\0'),
						))
					},
					bottom_texture_name: if raw.bottom_texture_name == *b"-\0\0\0\0\0\0\0" {
						None
					} else {
						Some(String::from(
							str::from_utf8(&raw.bottom_texture_name)?.trim_end_matches('\0'),
						))
					},
					middle_texture_name: if raw.middle_texture_name == *b"-\0\0\0\0\0\0\0" {
						None
					} else {
						Some(String::from(
							str::from_utf8(&raw.middle_texture_name)?.trim_end_matches('\0'),
						))
					},
					sector_index: raw.sector_index as usize,
				})
			})
			.collect()
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

pub struct VertexesFormat;

impl AssetFormat for VertexesFormat {
	type Asset = Vec<Vector2<f32>>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		RawVertexesFormat
			.import(name, source)?
			.into_iter()
			.map(|raw| Ok(Vector2::new(raw[0] as f32, raw[1] as f32)))
			.collect()
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

pub struct SectorsFormat;

impl AssetFormat for SectorsFormat {
	type Asset = Vec<Sector>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		RawSectorsFormat
			.import(name, source)?
			.into_iter()
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
			.collect()
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

pub struct GLVertFormat;

impl AssetFormat for GLVertFormat {
	type Asset = Vec<Vector2<f32>>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		RawGLVertFormat
			.import(name, source)?
			.into_iter()
			.map(|raw| {
				Ok(Vector2::new(
					raw[0] as f32 / 65536.0,
					raw[1] as f32 / 65536.0,
				))
			})
			.collect()
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

#[derive(Clone, Debug)]
pub struct GLSeg {
	pub vertex_indices: [EitherVertex; 2],
	pub linedef_index: Option<usize>,
	pub sidedef_index: Option<usize>,
	pub side: Side,
	pub partner_seg_index: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
pub enum EitherVertex {
	Normal(usize),
	GL(usize),
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

pub struct GLSegsFormat;

impl AssetFormat for GLSegsFormat {
	type Asset = Vec<GLSeg>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		RawGLSegsFormat
			.import(name, source)?
			.into_iter()
			.map(|raw| {
				Ok(GLSeg {
					vertex_indices: [
						if (raw.vertex_indices[0] & 0x8000) != 0 {
							EitherVertex::GL(raw.vertex_indices[0] as usize & 0x7FFF)
						} else {
							EitherVertex::Normal(raw.vertex_indices[0] as usize)
						},
						if (raw.vertex_indices[1] & 0x8000) != 0 {
							EitherVertex::GL(raw.vertex_indices[1] as usize & 0x7FFF)
						} else {
							EitherVertex::Normal(raw.vertex_indices[1] as usize)
						},
					],
					linedef_index: {
						if raw.linedef_index == 0xFFFF {
							None
						} else {
							Some(raw.linedef_index as usize)
						}
					},
					sidedef_index: None,
					side: if raw.side != 0 {
						Side::Left
					} else {
						Side::Right
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
			.collect()
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

#[derive(Clone, Debug)]
pub struct GLSSect {
	pub seg_count: usize,
	pub first_seg_index: usize,
	pub sector_index: usize,
}

pub struct GLSSectFormat;

impl AssetFormat for GLSSectFormat {
	type Asset = Vec<GLSSect>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		RawGLSSectFormat
			.import(name, source)?
			.into_iter()
			.map(|raw| {
				Ok(GLSSect {
					seg_count: raw.seg_count as usize,
					first_seg_index: raw.first_seg_index as usize,
					sector_index: 0,
				})
			})
			.collect()
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

pub struct GLNodesFormat;

impl AssetFormat for GLNodesFormat {
	type Asset = Vec<GLNode>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		RawGLNodesFormat
			.import(name, source)?
			.into_iter()
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
			.collect()
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
