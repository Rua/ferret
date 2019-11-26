use crate::{
	assets::{AssetFormat, AssetHandle, AssetStorage, DataSource},
	doom::{
		components::TransformComponent,
		entities::{DOOMEDNUMS, ENTITIES},
		image::{DoomImageFormat, DoomPaletteFormat},
		wad::WadLoader,
	},
	geometry::BoundingBox2,
	renderer::{
		mesh::{Mesh, MeshBuilder},
		texture::{Texture, TextureBuilder},
		video::Video,
	},
};
use byteorder::{ReadBytesExt, LE};
use nalgebra::{Matrix, Vector2, Vector3};
use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface};
use specs::{world::Builder, ReadExpect, SystemData, World, WorldExt, Write};
use std::{
	collections::{
		hash_map::{Entry, HashMap},
		HashSet,
	},
	error::Error,
	io::{Cursor, ErrorKind, Read, Seek, SeekFrom},
	str,
};
use vulkano::{format::Format, image::Dimensions};

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
	let [textures, flats] = load_textures(map_data, world)?;

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

fn load_textures(
	map: &DoomMap,
	world: &World,
) -> Result<[HashMap<String, (AssetHandle<Texture>, usize)>; 2], Box<dyn Error>> {
	let (mut loader, mut texture_storage, video) = <(
		Write<WadLoader>,
		Write<AssetStorage<Texture>>,
		ReadExpect<Video>,
	) as SystemData>::fetch(world);

	let mut texture_names: HashSet<&str> = HashSet::new();
	for sidedef in &map.sidedefs {
		if let Some(name) = &sidedef.top_texture_name {
			texture_names.insert(name.as_str());
		}

		if let Some(name) = &sidedef.bottom_texture_name {
			texture_names.insert(name.as_str());
		}

		if let Some(name) = &sidedef.middle_texture_name {
			texture_names.insert(name.as_str());
		}
	}

	let mut flat_names: HashSet<&str> = HashSet::new();
	for sector in &map.sectors {
		flat_names.insert(sector.floor_flat_name.as_str());
		flat_names.insert(sector.ceiling_flat_name.as_str());
	}

	// Load all the surfaces, while storing name-index mapping
	let mut surfaces: Vec<Surface> = Vec::with_capacity(texture_names.len() + flat_names.len());
	let mut texture_names_indices: HashMap<&str, usize> =
		HashMap::with_capacity(texture_names.len());
	let mut flat_names_indices: HashMap<&str, usize> = HashMap::with_capacity(flat_names.len());

	for name in texture_names {
		let surface = DoomTextureFormat.import(name, &mut *loader)?;
		texture_names_indices.insert(name, surfaces.len());
		surfaces.push(surface);
	}

	for name in flat_names {
		let surface = DoomFlatFormat.import(name, &mut *loader)?;
		flat_names_indices.insert(name, surfaces.len());
		surfaces.push(surface);
	}

	// Group surfaces by size in a HashMap, while keeping track of which goes where
	let mut surfaces_by_size: HashMap<[u32; 2], Vec<Surface<'static>>> = HashMap::new();
	let mut sizes_and_layers: Vec<([u32; 2], usize)> = Vec::with_capacity(surfaces.len());

	for surface in surfaces {
		let size = [surface.width(), surface.height()];
		let entry = match surfaces_by_size.entry(size) {
			Entry::Occupied(item) => item.into_mut(),
			Entry::Vacant(item) => item.insert(Vec::new()),
		};

		sizes_and_layers.push((size, entry.len()));
		entry.push(surface);
	}

	// Turn the grouped surfaces into textures
	let textures_by_size = surfaces_by_size
		.into_iter()
		.map(|entry| {
			let surfaces = entry.1;
			let size = Vector3::new(
				surfaces[0].width(),
				surfaces[0].height(),
				surfaces.len() as u32,
			);

			// Find the corresponding Vulkan pixel format
			let format = match surfaces[0].pixel_format_enum() {
				PixelFormatEnum::RGB24 => Format::R8G8B8Unorm,
				PixelFormatEnum::BGR24 => Format::B8G8R8Unorm,
				PixelFormatEnum::RGBA32 => Format::R8G8B8A8Unorm,
				PixelFormatEnum::BGRA32 => Format::B8G8R8A8Unorm,
				_ => unimplemented!(),
			};

			let layer_size = surfaces[0].without_lock().unwrap().len();
			let mut data = vec![0u8; layer_size * surfaces.len()];

			// Copy all the layers into the buffer
			for (chunk, surface) in data.chunks_exact_mut(layer_size).zip(surfaces) {
				chunk.copy_from_slice(surface.without_lock().unwrap());
			}

			// Create the image
			let (texture, future) = TextureBuilder::new()
				.with_data(data, format)
				.with_dimensions(Dimensions::Dim2dArray {
					width: size[0],
					height: size[1],
					array_layers: size[2],
				})
				.build(&video.queues().graphics)
				.unwrap_or_else(|e| panic!("Error building texture: {}", e));

			let handle = texture_storage.insert(texture);
			(entry.0, handle)
		})
		.collect::<HashMap<[u32; 2], AssetHandle<Texture>>>();

	// Now create the final Vec and return
	let grouped_textures: Vec<(AssetHandle<Texture>, usize)> = sizes_and_layers
		.into_iter()
		.map(|entry| (textures_by_size[&entry.0].clone(), entry.1))
		.collect();

	// Recombine names with textures
	Ok([
		texture_names_indices
			.into_iter()
			.map(|entry| (entry.0.to_owned(), grouped_textures[entry.1].clone()))
			.collect(),
		flat_names_indices
			.into_iter()
			.map(|entry| (entry.0.to_owned(), grouped_textures[entry.1].clone()))
			.collect(),
	])
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
		for seg in segs.iter() {
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

					// Add wall
					let start_vertex = match seg.vertex_indices[0] {
						EitherVertex::Normal(index) => &map.vertexes[index],
						EitherVertex::GL(index) => &map.gl_vert[index],
					};

					let end_vertex = match seg.vertex_indices[1] {
						EitherVertex::Normal(index) => &map.vertexes[index],
						EitherVertex::GL(index) => &map.gl_vert[index],
					};

					let top_peg_factor = if linedef.flags & 8 != 0 {
						[0.0, -1.0]
					} else {
						[1.0, 0.0]
					};

					let bottom_peg_factor = if linedef.flags & 16 != 0 {
						[1.0, 0.0]
					} else {
						[0.0, -1.0]
					};

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
								front_sidedef.texture_offset,
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
								front_sidedef.texture_offset,
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
								front_sidedef.texture_offset,
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
								front_sidedef.texture_offset,
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
	linedefs: Vec<Linedef>,
	sidedefs: Vec<Sidedef>,
	vertexes: Vec<Vector2<f32>>,
	sectors: Vec<Sector>,
	gl_vert: Vec<Vector2<f32>>,
	gl_segs: Vec<GLSeg>,
	gl_ssect: Vec<GLSSect>,
	gl_nodes: Vec<GLNode>,
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
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 1))?);
		let mut things = Vec::new();

		loop {
			let position_x = match data.read_i16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break;
					} else {
						return Err(Box::from(err));
					}
				}
			} as f32;
			let position_y = data.read_i16::<LE>()? as f32;
			let angle = data.read_i16::<LE>()? as f32;
			let doomednum = data.read_u16::<LE>()?;
			let flags = data.read_u16::<LE>()?;

			things.push(Thing {
				position: Vector2::new(position_x, position_y),
				angle,
				doomednum,
				flags,
			});
		}

		Ok(things)
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
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 2))?);
		let mut linedefs = Vec::new();

		loop {
			let start_vertex_index = match data.read_u16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break;
					} else {
						return Err(Box::from(err));
					}
				}
			} as usize;
			let end_vertex_index = data.read_u16::<LE>()? as usize;
			let flags = data.read_u16::<LE>()?;
			let special_type = data.read_u16::<LE>()?;
			let sector_tag = data.read_u16::<LE>()?;
			let right_sidedef_index = data.read_u16::<LE>()? as usize;
			let left_sidedef_index = data.read_u16::<LE>()? as usize;

			linedefs.push(Linedef {
				vertex_indices: [start_vertex_index, end_vertex_index],
				flags,
				special_type,
				sector_tag,
				sidedef_indices: [
					if right_sidedef_index == 0xFFFF {
						None
					} else {
						Some(right_sidedef_index)
					},
					if left_sidedef_index == 0xFFFF {
						None
					} else {
						Some(left_sidedef_index)
					},
				],
			});
		}

		Ok(linedefs)
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
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 3))?);
		let mut sidedefs = Vec::new();

		loop {
			let texture_offset_x = match data.read_i16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break;
					} else {
						return Err(Box::from(err));
					}
				}
			} as f32;
			let texture_offset_y = data.read_i16::<LE>()? as f32;
			let top_texture_name = {
				let mut name = [0u8; 8];
				data.read_exact(&mut name)?;
				String::from(str::from_utf8(&name)?.trim_end_matches('\0'))
			};
			let bottom_texture_name = {
				let mut name = [0u8; 8];
				data.read_exact(&mut name)?;
				String::from(str::from_utf8(&name)?.trim_end_matches('\0'))
			};
			let middle_texture_name = {
				let mut name = [0u8; 8];
				data.read_exact(&mut name)?;
				String::from(str::from_utf8(&name)?.trim_end_matches('\0'))
			};
			let sector_index = data.read_u16::<LE>()? as usize;

			sidedefs.push(Sidedef {
				texture_offset: Vector2::new(texture_offset_x, texture_offset_y),
				top_texture_name: if top_texture_name == "-" {
					None
				} else {
					Some(top_texture_name)
				},
				bottom_texture_name: if bottom_texture_name == "-" {
					None
				} else {
					Some(bottom_texture_name)
				},
				middle_texture_name: if middle_texture_name == "-" {
					None
				} else {
					Some(middle_texture_name)
				},
				sector_index,
			});
		}

		Ok(sidedefs)
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
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 4))?);
		let mut vertexes = Vec::new();

		loop {
			let x = match data.read_i16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break;
					} else {
						return Err(Box::from(err));
					}
				}
			} as f32;
			let y = data.read_i16::<LE>()? as f32;

			vertexes.push(Vector2::new(x, y));
		}

		Ok(vertexes)
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
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 8))?);
		let mut sectors = Vec::new();

		loop {
			let floor_height = match data.read_i16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break;
					} else {
						return Err(Box::from(err));
					}
				}
			} as f32;
			let ceiling_height = data.read_i16::<LE>()? as f32;
			let floor_flat_name = {
				let mut name = [0u8; 8];
				data.read_exact(&mut name)?;
				String::from(str::from_utf8(&name)?.trim_end_matches('\0'))
			};
			let ceiling_flat_name = {
				let mut name = [0u8; 8];
				data.read_exact(&mut name)?;
				String::from(str::from_utf8(&name)?.trim_end_matches('\0'))
			};
			let light_level = data.read_u16::<LE>()?;
			let special_type = data.read_u16::<LE>()?;
			let sector_tag = data.read_u16::<LE>()?;

			sectors.push(Sector {
				floor_height,
				ceiling_height,
				floor_flat_name,
				ceiling_flat_name,
				light_level,
				special_type,
				sector_tag,
			});
		}

		Ok(sectors)
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
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 1))?);
		let mut gl_vert = Vec::new();

		let mut signature = [0u8; 4];
		data.read_exact(&mut signature)?;

		if &signature != b"gNd2" {
			return Err(Box::from("No gNd2 signature found"));
		}

		loop {
			let x = match data.read_i32::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break;
					} else {
						return Err(Box::from(err));
					}
				}
			} as f32;
			let y = data.read_i32::<LE>()? as f32;

			gl_vert.push(Vector2::new(x / 65536.0, y / 65536.0));
		}

		Ok(gl_vert)
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
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 2))?);
		let mut gl_segs = Vec::new();

		loop {
			let start_vertex_index = match data.read_u16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break;
					} else {
						return Err(Box::from(err));
					}
				}
			} as usize;
			let end_vertex_index = data.read_u16::<LE>()? as usize;
			let linedef_index = data.read_u16::<LE>()? as usize;
			let side = data.read_u16::<LE>()?;
			let partner_seg_index = data.read_u16::<LE>()? as usize;

			gl_segs.push(GLSeg {
				vertex_indices: [
					if (start_vertex_index & 0x8000) != 0 {
						EitherVertex::GL(start_vertex_index & 0x7FFF)
					} else {
						EitherVertex::Normal(start_vertex_index)
					},
					if (end_vertex_index & 0x8000) != 0 {
						EitherVertex::GL(end_vertex_index & 0x7FFF)
					} else {
						EitherVertex::Normal(end_vertex_index)
					},
				],
				linedef_index: {
					if linedef_index == 0xFFFF {
						None
					} else {
						Some(linedef_index)
					}
				},
				sidedef_index: None,
				side: if side != 0 { Side::Left } else { Side::Right },
				partner_seg_index: {
					if partner_seg_index == 0xFFFF {
						None
					} else {
						Some(partner_seg_index)
					}
				},
			});
		}

		Ok(gl_segs)
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
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 3))?);
		let mut gl_ssect = Vec::new();

		loop {
			let seg_count = match data.read_u16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break;
					} else {
						return Err(Box::from(err));
					}
				}
			} as usize;
			let first_seg_index = data.read_u16::<LE>()? as usize;

			gl_ssect.push(GLSSect {
				seg_count,
				first_seg_index,
				sector_index: 0,
			});
		}

		Ok(gl_ssect)
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
		let mut data = Cursor::new(source.load(&format!("{}/+{}", name, 4))?);
		let mut gl_nodes = Vec::new();

		loop {
			let partition_point_x = match data.read_i16::<LE>() {
				Ok(val) => val,
				Err(err) => {
					if err.kind() == ErrorKind::UnexpectedEof {
						break;
					} else {
						return Err(Box::from(err));
					}
				}
			} as f32;
			let partition_point_y = data.read_i16::<LE>()? as f32;
			let partition_dir_x = data.read_i16::<LE>()? as f32;
			let partition_dir_y = data.read_i16::<LE>()? as f32;
			let right_bbox_top = data.read_i16::<LE>()? as f32;
			let right_bbox_bottom = data.read_i16::<LE>()? as f32;
			let right_bbox_left = data.read_i16::<LE>()? as f32;
			let right_bbox_right = data.read_i16::<LE>()? as f32;
			let left_bbox_top = data.read_i16::<LE>()? as f32;
			let left_bbox_bottom = data.read_i16::<LE>()? as f32;
			let left_bbox_left = data.read_i16::<LE>()? as f32;
			let left_bbox_right = data.read_i16::<LE>()? as f32;
			let right_child_index = data.read_u16::<LE>()? as usize;
			let left_child_index = data.read_u16::<LE>()? as usize;

			gl_nodes.push(GLNode {
				partition_point: Vector2::new(partition_point_x, partition_point_y),
				partition_dir: Vector2::new(partition_dir_x, partition_dir_y),
				bbox: [
					BoundingBox2::from_extents(
						right_bbox_top,
						right_bbox_bottom,
						right_bbox_left,
						right_bbox_right,
					),
					BoundingBox2::from_extents(
						left_bbox_top,
						left_bbox_bottom,
						left_bbox_left,
						left_bbox_right,
					),
				],
				child_indices: [
					if (right_child_index & 0x8000) != 0 {
						ChildNode::Leaf(right_child_index & 0x7FFF)
					} else {
						ChildNode::Branch(right_child_index)
					},
					if (left_child_index & 0x8000) != 0 {
						ChildNode::Leaf(left_child_index & 0x7FFF)
					} else {
						ChildNode::Branch(left_child_index)
					},
				],
			});
		}

		Ok(gl_nodes)
	}
}

pub struct DoomFlatFormat;

impl AssetFormat for DoomFlatFormat {
	type Asset = Surface<'static>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let palette = DoomPaletteFormat.import("PLAYPAL", source)?;
		let mut data = Cursor::new(source.load(name)?);
		let mut surface = Surface::new(64, 64, PixelFormatEnum::RGBA32)?;

		{
			let pixels = surface.without_lock_mut().unwrap();
			let mut flat_pixels = [0u8; 64 * 64];

			data.read_exact(&mut flat_pixels)?;

			for i in 0..flat_pixels.len() {
				let color = palette[flat_pixels[i] as usize];
				pixels[4 * i + 0] = color.r;
				pixels[4 * i + 1] = color.g;
				pixels[4 * i + 2] = color.b;
				pixels[4 * i + 3] = color.a;
			}
		}

		Ok(surface)
	}
}

pub struct DoomPNamesFormat;

impl AssetFormat for DoomPNamesFormat {
	type Asset = Vec<String>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(name)?);
		let count = data.read_u32::<LE>()? as usize;
		let mut pnames = Vec::with_capacity(count);

		for _ in 0..count {
			let mut name = [0u8; 8];
			data.read_exact(&mut name)?;
			let name = String::from(str::from_utf8(&name)?.trim_end_matches('\0'));
			pnames.push(name);
		}

		Ok(pnames)
	}
}

pub struct DoomTextureFormat;

impl AssetFormat for DoomTextureFormat {
	type Asset = Surface<'static>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let pnames = DoomPNamesFormat.import("PNAMES", source)?;
		let mut texture_info = DoomTexturesFormat.import("TEXTURE1", source)?;
		texture_info.extend(DoomTexturesFormat.import("TEXTURE2", source)?);

		let name = name.to_ascii_uppercase();
		let texture_info = texture_info
			.get(&name)
			.ok_or(format!("Texture {} does not exist", name))?;

		let mut surface = Surface::new(
			texture_info.size[0] as u32,
			texture_info.size[1] as u32,
			PixelFormatEnum::RGBA32,
		)?;

		for patch_info in &texture_info.patches {
			let name = &pnames[patch_info.index];

			// Use to_surface because the offsets of patches are ignored anyway
			let mut patch = DoomImageFormat.import(&name, source)?;
			let surface2 = Surface::from_data(
				&mut patch.data,
				patch.size[0] as u32,
				patch.size[1] as u32,
				patch.size[0] as u32 * 4,
				PixelFormatEnum::RGBA32,
			)?;
			surface2.blit(
				None,
				&mut surface,
				Rect::new(
					patch_info.offset[0] as i32,
					patch_info.offset[1] as i32,
					0,
					0,
				),
			)?;
		}

		Ok(surface)
	}
}

pub struct DoomPatchInfo {
	pub offset: Vector2<i32>,
	pub index: usize,
}

pub struct DoomTextureInfo {
	pub size: Vector2<u32>,
	pub patches: Vec<DoomPatchInfo>,
}

pub struct DoomTexturesFormat;

impl AssetFormat for DoomTexturesFormat {
	type Asset = HashMap<String, DoomTextureInfo>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(name)?);
		let mut texture_info = HashMap::new();

		let count = data.read_u32::<LE>()? as usize;
		let mut offsets = vec![0u32; count];
		data.read_u32_into::<LE>(&mut offsets)?;

		for i in 0..count {
			data.seek(SeekFrom::Start(offsets[i] as u64))?;

			let mut name = [0u8; 8];
			data.read_exact(&mut name)?;
			let mut name = String::from(str::from_utf8(&name)?.trim_end_matches('\0'));
			name.make_ascii_uppercase();

			data.read_u32::<LE>()?; // unused bytes

			let size_x = data.read_u16::<LE>()? as u32;
			let size_y = data.read_u16::<LE>()? as u32;

			data.read_u32::<LE>()?; // unused bytes

			let patch_count = data.read_u16::<LE>()? as usize;
			let mut patches = Vec::with_capacity(patch_count);

			for _j in 0..patch_count {
				let offset_x = data.read_i16::<LE>()? as i32;
				let offset_y = data.read_i16::<LE>()? as i32;
				let patch_index = data.read_u16::<LE>()? as usize;

				data.read_u32::<LE>()?; // unused bytes

				patches.push(DoomPatchInfo {
					offset: Vector2::new(offset_x, offset_y),
					index: patch_index,
				});
			}

			texture_info.insert(
				name,
				DoomTextureInfo {
					size: Vector2::new(size_x, size_y),
					patches: patches,
				},
			);
		}

		Ok(texture_info)
	}
}
