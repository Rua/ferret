use crate::{
	common::assets::{AssetHandle, AssetStorage},
	doom::{
		assets::{
			image::Image,
			map::{textures::TextureType, LinedefFlags, Map, SectorSlot, Side, SidedefSlot},
		},
		game::map::MapDynamic,
	},
};
use fnv::FnvHashMap;
use nalgebra::Vector2;
use vulkano::{image::ImageDimensions, impl_vertex};

#[derive(Clone, Debug, Default)]
pub struct Vertex {
	pub in_position: [f32; 3],
	pub in_texture_coord: [f32; 2],
	pub in_light_level: f32,
}
impl_vertex!(Vertex, in_position, in_texture_coord, in_light_level);

#[derive(Clone, Debug, Default)]
pub struct SkyVertex {
	pub in_position: [f32; 3],
}
impl_vertex!(SkyVertex, in_position);

pub fn make_meshes(
	map: &Map,
	map_dynamic: &MapDynamic,
	extra_light: f32,
	asset_storage: &AssetStorage,
) -> anyhow::Result<(
	FnvHashMap<AssetHandle<Image>, (Vec<Vertex>, Vec<u32>)>,
	FnvHashMap<AssetHandle<Image>, (Vec<Vertex>, Vec<u32>)>,
	(Vec<SkyVertex>, Vec<u32>),
)> {
	#[inline]
	fn push_wall(
		vertices: &mut Vec<Vertex>,
		indices: &mut Vec<u32>,
		vert_h: [Vector2<f32>; 2],
		vert_v: [f32; 2],
		tex_v: [f32; 2],
		offset: Vector2<f32>,
		dimensions: ImageDimensions,
		light_level: f32,
	) {
		let width = (vert_h[1] - vert_h[0]).norm();
		indices.push(u32::max_value());

		for (h, v) in [(1, 0), (0, 0), (0, 1), (1, 1)] {
			indices.push(vertices.len() as u32);
			vertices.push(Vertex {
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
		vertices: &mut Vec<SkyVertex>,
		indices: &mut Vec<u32>,
		vert_h: [Vector2<f32>; 2],
		vert_v: [f32; 2],
	) {
		indices.push(u32::max_value());

		for (h, v) in [(1, 0), (0, 0), (0, 1), (1, 1)] {
			indices.push(vertices.len() as u32);
			vertices.push(SkyVertex {
				in_position: [vert_h[h][0], vert_h[h][1], vert_v[v]],
			});
		}
	}

	#[inline]
	fn push_flat<'a>(
		vertices: &mut Vec<Vertex>,
		indices: &mut Vec<u32>,
		iter: impl Iterator<Item = &'a Vector2<f32>>,
		vert_z: f32,
		dimensions: ImageDimensions,
		light_level: f32,
	) {
		indices.push(u32::max_value());

		for vert in iter {
			indices.push(vertices.len() as u32);
			vertices.push(Vertex {
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
		vertices: &mut Vec<SkyVertex>,
		indices: &mut Vec<u32>,
		iter: impl Iterator<Item = &'a Vector2<f32>>,
		vert_z: f32,
	) {
		indices.push(u32::max_value());

		for vert in iter {
			indices.push(vertices.len() as u32);
			vertices.push(SkyVertex {
				in_position: [vert[0], vert[1], vert_z],
			});
		}
	}

	let mut flat_meshes: FnvHashMap<AssetHandle<Image>, (Vec<Vertex>, Vec<u32>)> =
		FnvHashMap::default();
	let mut wall_meshes: FnvHashMap<AssetHandle<Image>, (Vec<Vertex>, Vec<u32>)> =
		FnvHashMap::default();
	let mut sky_mesh: (Vec<SkyVertex>, Vec<u32>) = (Vec::new(), Vec::new());

	// Walls
	for (linedef_index, linedef) in map.linedefs.iter().enumerate() {
		let linedef_dynamic = &map_dynamic.linedefs[linedef_index];

		for side in [Side::Right, Side::Left] {
			let front_sidedef = match &linedef.sidedefs[side as usize] {
				Some(x) => x,
				None => continue,
			};
			let front_sidedef_dynamic = linedef_dynamic.sidedefs[side as usize].as_ref().unwrap();
			let mut texture_offset = front_sidedef.texture_offset;

			// Doom only scrolls the front/right sidedef. Why? Who knows.
			if side == Side::Right {
				texture_offset += linedef_dynamic.texture_offset;
			}

			let front_sector_dynamic = &map_dynamic.sectors[front_sidedef.sector_index];

			// Swap the vertices if we're on the left side of the linedef
			let linedef_vertices = match side {
				Side::Right => [linedef.line.point, linedef.line.end_point()],
				Side::Left => [linedef.line.end_point(), linedef.line.point],
			};

			// Two-sided or one-sided sidedef?
			if let Some(back_sidedef) = &linedef.sidedefs[!side as usize] {
				let back_sector_dynamic = &map_dynamic.sectors[back_sidedef.sector_index];
				let intersection = front_sector_dynamic
					.interval
					.intersection(back_sector_dynamic.interval);
				let spans = [
					front_sector_dynamic.interval.max,
					intersection.max,
					intersection.min,
					front_sector_dynamic.interval.min,
				];

				// Top section
				match &front_sidedef_dynamic.textures[SidedefSlot::Top as usize] {
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
						let dimensions = asset_storage
							.get(handle)
							.unwrap()
							.image_view
							.image()
							.dimensions();
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
							front_sector_dynamic.light_level + extra_light,
						);
					}
				}

				// Bottom section
				match &front_sidedef_dynamic.textures[SidedefSlot::Bottom as usize] {
					TextureType::None => (),
					TextureType::Sky => unimplemented!(),
					TextureType::Normal(handle) => {
						let dimensions = asset_storage
							.get(handle)
							.unwrap()
							.image_view
							.image()
							.dimensions();
						let (ref mut vertices, ref mut indices) = wall_meshes
							.entry(handle.clone())
							.or_insert((vec![], vec![]));

						let tex_v = if linedef.flags.contains(LinedefFlags::DONTPEGBOTTOM) {
							[
								front_sector_dynamic.interval.max - spans[2],
								front_sector_dynamic.interval.max - spans[3],
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
							front_sector_dynamic.light_level + extra_light,
						);
					}
				}

				// Middle section
				match &front_sidedef_dynamic.textures[SidedefSlot::Middle as usize] {
					TextureType::None => (),
					TextureType::Sky => unimplemented!(),
					TextureType::Normal(handle) => {
						let dimensions = asset_storage
							.get(handle)
							.unwrap()
							.image_view
							.image()
							.dimensions();
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
							front_sector_dynamic.light_level + extra_light,
						);
					}
				}
			} else {
				match &front_sidedef_dynamic.textures[SidedefSlot::Middle as usize] {
					TextureType::None => (),
					TextureType::Sky => unimplemented!(),
					TextureType::Normal(handle) => {
						let dimensions = asset_storage
							.get(handle)
							.unwrap()
							.image_view
							.image()
							.dimensions();
						let (ref mut vertices, ref mut indices) = wall_meshes
							.entry(handle.clone())
							.or_insert((vec![], vec![]));

						let tex_v = if linedef.flags.contains(LinedefFlags::DONTPEGBOTTOM) {
							[-front_sector_dynamic.interval.len(), 0.0]
						} else {
							[0.0, front_sector_dynamic.interval.len()]
						};

						push_wall(
							vertices,
							indices,
							linedef_vertices,
							[
								front_sector_dynamic.interval.max,
								front_sector_dynamic.interval.min,
							],
							tex_v,
							texture_offset,
							dimensions,
							front_sector_dynamic.light_level + extra_light,
						);
					}
				}
			}
		}
	}

	// Flats
	for (i, sector) in map.sectors.iter().enumerate() {
		let sector_dynamic = &map_dynamic.sectors[i];

		for segs in sector.subsectors.iter().map(|i| &map.subsectors[*i].segs) {
			// Floor
			let iter = segs.iter().map(|seg| &seg.line.point).rev();

			match &sector.textures[SectorSlot::Floor as usize] {
				TextureType::None => (),
				TextureType::Sky => push_sky_flat(
					&mut sky_mesh.0,
					&mut sky_mesh.1,
					iter,
					sector_dynamic.interval.min,
				),
				TextureType::Normal(handle) => {
					let dimensions = asset_storage
						.get(handle)
						.unwrap()
						.image_view
						.image()
						.dimensions();
					let (ref mut vertices, ref mut indices) = flat_meshes
						.entry(handle.clone())
						.or_insert((vec![], vec![]));

					push_flat(
						vertices,
						indices,
						iter,
						sector_dynamic.interval.min,
						dimensions,
						sector_dynamic.light_level + extra_light,
					);
				}
			}

			// Ceiling
			let iter = segs.iter().map(|seg| &seg.line.point);

			match &sector.textures[SectorSlot::Ceiling as usize] {
				TextureType::None => (),
				TextureType::Sky => push_sky_flat(
					&mut sky_mesh.0,
					&mut sky_mesh.1,
					iter,
					sector_dynamic.interval.max,
				),
				TextureType::Normal(handle) => {
					let dimensions = asset_storage
						.get(handle)
						.unwrap()
						.image_view
						.image()
						.dimensions();
					let (ref mut vertices, ref mut indices) = flat_meshes
						.entry(handle.clone())
						.or_insert((vec![], vec![]));

					push_flat(
						vertices,
						indices,
						iter,
						sector_dynamic.interval.max,
						dimensions,
						sector_dynamic.light_level + extra_light,
					);
				}
			}
		}
	}

	Ok((flat_meshes, wall_meshes, sky_mesh))
}
