use crate::{
	assets::{AssetHandle, AssetStorage},
	doom::{
		components::{LinedefDynamic, MapDynamic, SectorDynamic},
		map::{
			textures::{Flat, WallTexture},
			LinedefFlags, Map, Side, TextureType,
		},
	},
};
use nalgebra::Vector2;
use specs::{ReadExpect, ReadStorage, World};
use std::{collections::HashMap, error::Error};
use vulkano::{image::Dimensions, impl_vertex};

#[derive(Clone, Debug, Default)]
pub struct VertexData {
	pub in_position: [f32; 3],
	pub in_texture_coord: [f32; 2],
	pub in_light_level: f32,
}
impl_vertex!(VertexData, in_position, in_texture_coord, in_light_level);

#[derive(Clone, Debug, Default)]
pub struct SkyVertexData {
	pub in_position: [f32; 3],
}
impl_vertex!(SkyVertexData, in_position);

pub fn make_meshes(
	map: &Map,
	map_dynamic: &MapDynamic,
	world: &World,
) -> Result<
	(
		HashMap<AssetHandle<Flat>, (Vec<VertexData>, Vec<u32>)>,
		(Vec<SkyVertexData>, Vec<u32>),
		HashMap<AssetHandle<WallTexture>, (Vec<VertexData>, Vec<u32>)>,
	),
	Box<dyn Error + Send + Sync>,
> {
	#[inline]
	fn push_wall(
		vertices: &mut Vec<VertexData>,
		indices: &mut Vec<u32>,
		vert_h: [Vector2<f32>; 2],
		vert_v: [f32; 2],
		tex_v: [f32; 2],
		offset: Vector2<f32>,
		dimensions: Dimensions,
		light_level: f32,
	) {
		let width = (vert_h[1] - vert_h[0]).norm();
		indices.push(u32::max_value());

		for (h, v) in [(1, 0), (0, 0), (0, 1), (1, 1)].iter().copied() {
			indices.push(vertices.len() as u32);
			vertices.push(VertexData {
				in_position: [vert_h[h][0], vert_h[h][1], vert_v[v]],
				in_texture_coord: [
					(offset[0] + width * h as f32) / dimensions.width() as f32,
					(offset[1] + tex_v[v]) / dimensions.height() as f32,
				],
				in_light_level: light_level,
			});
		}
	}

	#[inline]
	fn push_sky_wall(
		vertices: &mut Vec<SkyVertexData>,
		indices: &mut Vec<u32>,
		vert_h: [Vector2<f32>; 2],
		vert_v: [f32; 2],
	) {
		indices.push(u32::max_value());

		for (h, v) in [(1, 0), (0, 0), (0, 1), (1, 1)].iter().copied() {
			indices.push(vertices.len() as u32);
			vertices.push(SkyVertexData {
				in_position: [vert_h[h][0], vert_h[h][1], vert_v[v]],
			});
		}
	}

	#[inline]
	fn push_flat<'a>(
		vertices: &mut Vec<VertexData>,
		indices: &mut Vec<u32>,
		iter: impl Iterator<Item = &'a Vector2<f32>>,
		vert_z: f32,
		dimensions: Dimensions,
		light_level: f32,
	) {
		indices.push(u32::max_value());

		for vert in iter {
			indices.push(vertices.len() as u32);
			vertices.push(VertexData {
				in_position: [vert[0], vert[1], vert_z],
				in_texture_coord: [
					vert[0] / dimensions.width() as f32,
					-vert[1] / dimensions.height() as f32,
				],
				in_light_level: light_level,
			});
		}
	}

	#[inline]
	fn push_sky_flat<'a>(
		vertices: &mut Vec<SkyVertexData>,
		indices: &mut Vec<u32>,
		iter: impl Iterator<Item = &'a Vector2<f32>>,
		vert_z: f32,
	) {
		indices.push(u32::max_value());

		for vert in iter {
			indices.push(vertices.len() as u32);
			vertices.push(SkyVertexData {
				in_position: [vert[0], vert[1], vert_z],
			});
		}
	}

	let mut flat_meshes: HashMap<AssetHandle<Flat>, (Vec<VertexData>, Vec<u32>)> = HashMap::new();
	let mut sky_mesh: (Vec<SkyVertexData>, Vec<u32>) = (Vec::new(), Vec::new());
	let mut wall_meshes: HashMap<AssetHandle<WallTexture>, (Vec<VertexData>, Vec<u32>)> =
		HashMap::new();

	let (flat_storage, wall_texture_storage, linedef_dynamic_component, sector_dynamic_component) =
		world.system_data::<(
			ReadExpect<AssetStorage<Flat>>,
			ReadExpect<AssetStorage<WallTexture>>,
			ReadStorage<LinedefDynamic>,
			ReadStorage<SectorDynamic>,
		)>();

	// Walls
	for (linedef_index, linedef) in map.linedefs.iter().enumerate() {
		let linedef_dynamic = linedef_dynamic_component
			.get(map_dynamic.linedefs[linedef_index])
			.unwrap();

		for side in [Side::Right, Side::Left].iter().copied() {
			let front_sidedef = match &linedef.sidedefs[side as usize] {
				Some(x) => x,
				None => continue,
			};
			let mut texture_offset = front_sidedef.texture_offset;

			// Doom only scrolls the front/right sidedef. Why? Who knows.
			if side == Side::Right {
				texture_offset += linedef_dynamic.texture_offset;
			}

			let front_sector_dynamic = sector_dynamic_component
				.get(map_dynamic.sectors[front_sidedef.sector_index])
				.unwrap();

			// Swap the vertices if we're on the left side of the linedef
			let linedef_vertices = match side {
				Side::Right => [linedef.line.point, linedef.line.point + linedef.line.dir],
				Side::Left => [linedef.line.point + linedef.line.dir, linedef.line.point],
			};

			// Two-sided or one-sided sidedef?
			if let Some(back_sidedef) = &linedef.sidedefs[!side as usize] {
				let back_sector_dynamic = sector_dynamic_component
					.get(map_dynamic.sectors[back_sidedef.sector_index])
					.unwrap();
				let spans = [
					front_sector_dynamic.ceiling_height,
					f32::min(
						front_sector_dynamic.ceiling_height,
						back_sector_dynamic.ceiling_height,
					),
					f32::max(
						back_sector_dynamic.floor_height,
						front_sector_dynamic.floor_height,
					),
					front_sector_dynamic.floor_height,
				];

				// Top section
				match &front_sidedef.top_texture {
					TextureType::None => (),
					TextureType::Sky => {
						push_sky_wall(
							&mut sky_mesh.0,
							&mut sky_mesh.1,
							linedef_vertices,
							[spans[0], spans[1]],
						);
					}
					TextureType::Normal(handle) => {
						let dimensions = wall_texture_storage.get(handle).unwrap().dimensions();
						let (ref mut vertices, ref mut indices) = wall_meshes
							.entry(handle.clone())
							.or_insert((vec![], vec![]));

						let tex_v = if linedef.flags.contains(LinedefFlags::DONTPEGTOP) {
							[0.0, spans[0] - spans[1]]
						} else {
							[spans[1] - spans[0], 0.0]
						};

						push_wall(
							vertices,
							indices,
							linedef_vertices,
							[spans[0], spans[1]],
							tex_v,
							texture_offset,
							dimensions,
							front_sector_dynamic.light_level,
						);
					}
				}

				// Bottom section
				match &front_sidedef.bottom_texture {
					TextureType::None => (),
					TextureType::Sky => unimplemented!(),
					TextureType::Normal(handle) => {
						let dimensions = wall_texture_storage.get(handle).unwrap().dimensions();
						let (ref mut vertices, ref mut indices) = wall_meshes
							.entry(handle.clone())
							.or_insert((vec![], vec![]));

						let tex_v = if linedef.flags.contains(LinedefFlags::DONTPEGBOTTOM) {
							[
								front_sector_dynamic.ceiling_height - spans[2],
								front_sector_dynamic.ceiling_height - spans[3],
							]
						} else {
							[0.0, spans[2] - spans[3]]
						};

						push_wall(
							vertices,
							indices,
							linedef_vertices,
							[spans[2], spans[3]],
							tex_v,
							texture_offset,
							dimensions,
							front_sector_dynamic.light_level,
						);
					}
				}

				// Middle section
				match &front_sidedef.middle_texture {
					TextureType::None => (),
					TextureType::Sky => unimplemented!(),
					TextureType::Normal(handle) => {
						let dimensions = wall_texture_storage.get(handle).unwrap().dimensions();
						let (ref mut vertices, ref mut indices) = wall_meshes
							.entry(handle.clone())
							.or_insert((vec![], vec![]));

						let tex_v = if linedef.flags.contains(LinedefFlags::DONTPEGBOTTOM) {
							[spans[2] - spans[1], 0.0]
						} else {
							[0.0, spans[1] - spans[2]]
						};

						push_wall(
							vertices,
							indices,
							linedef_vertices,
							[spans[1], spans[2]],
							tex_v,
							texture_offset,
							dimensions,
							front_sector_dynamic.light_level,
						);
					}
				}
			} else {
				match &front_sidedef.middle_texture {
					TextureType::None => (),
					TextureType::Sky => unimplemented!(),
					TextureType::Normal(handle) => {
						let dimensions = wall_texture_storage.get(handle).unwrap().dimensions();
						let (ref mut vertices, ref mut indices) = wall_meshes
							.entry(handle.clone())
							.or_insert((vec![], vec![]));

						let tex_v = if linedef.flags.contains(LinedefFlags::DONTPEGBOTTOM) {
							[
								front_sector_dynamic.floor_height
									- front_sector_dynamic.ceiling_height,
								0.0,
							]
						} else {
							[
								0.0,
								front_sector_dynamic.ceiling_height
									- front_sector_dynamic.floor_height,
							]
						};

						push_wall(
							vertices,
							indices,
							linedef_vertices,
							[
								front_sector_dynamic.ceiling_height,
								front_sector_dynamic.floor_height,
							],
							tex_v,
							texture_offset,
							dimensions,
							front_sector_dynamic.light_level,
						);
					}
				}
			}
		}
	}

	// Flats
	for (i, sector) in map.sectors.iter().enumerate() {
		let sector_dynamic = sector_dynamic_component
			.get(map_dynamic.sectors[i])
			.unwrap();

		for vertices in &sector.subsectors {
			// Floor
			let iter = vertices.iter().rev();

			match &sector.floor_texture {
				TextureType::None => (),
				TextureType::Sky => push_sky_flat(
					&mut sky_mesh.0,
					&mut sky_mesh.1,
					iter,
					sector_dynamic.floor_height,
				),
				TextureType::Normal(handle) => {
					let dimensions = flat_storage.get(handle).unwrap().dimensions();
					let (ref mut vertices, ref mut indices) = flat_meshes
						.entry(handle.clone())
						.or_insert((vec![], vec![]));

					push_flat(
						vertices,
						indices,
						iter,
						sector_dynamic.floor_height,
						dimensions,
						sector_dynamic.light_level,
					);
				}
			}

			// Ceiling
			let iter = vertices.iter();

			match &sector.ceiling_texture {
				TextureType::None => (),
				TextureType::Sky => push_sky_flat(
					&mut sky_mesh.0,
					&mut sky_mesh.1,
					iter,
					sector_dynamic.ceiling_height,
				),
				TextureType::Normal(handle) => {
					let dimensions = flat_storage.get(handle).unwrap().dimensions();
					let (ref mut vertices, ref mut indices) = flat_meshes
						.entry(handle.clone())
						.or_insert((vec![], vec![]));

					push_flat(
						vertices,
						indices,
						iter,
						sector_dynamic.ceiling_height,
						dimensions,
						sector_dynamic.light_level,
					);
				}
			}
		}
	}

	Ok((flat_meshes, sky_mesh, wall_meshes))
}
