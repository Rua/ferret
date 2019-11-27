use crate::{
	assets::{AssetFormat, AssetHandle, AssetStorage, DataSource},
	doom::{
		image::{ImageFormat, PaletteFormat},
		map::DoomMap,
		wad::WadLoader,
	},
	renderer::{
		texture::{Texture, TextureBuilder},
		video::Video,
	},
};
use nalgebra::{Vector2, Vector3};
use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface};
use serde::Deserialize;
use specs::{ReadExpect, SystemData, World, Write};
use std::{
	collections::{hash_map::Entry, HashMap, HashSet},
	error::Error,
	io::{Cursor, Read, Seek, SeekFrom},
	str,
};
use vulkano::{format::Format, image::Dimensions};

pub fn load_textures(
	map: &DoomMap,
	world: &World,
) -> Result<[HashMap<String, (AssetHandle<Texture>, usize)>; 2], Box<dyn Error>> {
	let (mut loader, mut texture_storage, video) = <(
		Write<WadLoader>,
		Write<AssetStorage<Texture>>,
		ReadExpect<Video>,
	)>::fetch(world);

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
		let surface = TextureFormat.import(name, &mut *loader)?;
		texture_names_indices.insert(name, surfaces.len());
		surfaces.push(surface);
	}

	for name in flat_names {
		let surface = FlatFormat.import(name, &mut *loader)?;
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

pub struct FlatFormat;

impl AssetFormat for FlatFormat {
	type Asset = Surface<'static>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let palette = PaletteFormat.import("PLAYPAL", source)?;
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

pub struct PNamesFormat;

impl AssetFormat for PNamesFormat {
	type Asset = Vec<[u8; 8]>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(name)?);
		let count: u32 = bincode::deserialize_from(&mut data)?;
		let mut ret = Vec::with_capacity(count as usize);

		for _ in 0..count {
			ret.push(bincode::deserialize_from(&mut data)?);
		}

		Ok(ret)
	}
}

pub struct TextureFormat;

impl AssetFormat for TextureFormat {
	type Asset = Surface<'static>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let pnames = PNamesFormat.import("PNAMES", source)?;
		let mut texture_info = TexturesFormat.import("TEXTURE1", source)?;
		texture_info.extend(TexturesFormat.import("TEXTURE2", source)?);

		let name = name.to_ascii_uppercase();
		let texture_info = texture_info
			.get(&name)
			.ok_or(format!("Texture {} does not exist", name))?;

		let mut surface = Surface::new(
			texture_info.size[0] as u32,
			texture_info.size[1] as u32,
			PixelFormatEnum::RGBA32,
		)?;

		texture_info
			.patches
			.iter()
			.try_for_each(|patch_info| -> Result<(), Box<dyn Error>> {
				let name =
					String::from(str::from_utf8(&pnames[patch_info.index])?.trim_end_matches('\0'));
				let mut patch = ImageFormat.import(&name, source)?;
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

				Ok(())
			})?;

		Ok(surface)
	}
}

pub struct PatchInfo {
	pub offset: Vector2<i32>,
	pub index: usize,
}

pub struct TextureInfo {
	pub size: Vector2<u32>,
	pub patches: Vec<PatchInfo>,
}

pub struct TexturesFormat;

impl AssetFormat for TexturesFormat {
	type Asset = HashMap<String, TextureInfo>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		RawTexturesFormat
			.import(name, source)?
			.into_iter()
			.map(|(texture, patches)| {
				let mut name = String::from(str::from_utf8(&texture.name)?.trim_end_matches('\0'));
				name.make_ascii_uppercase();

				let patches = patches
					.into_iter()
					.map(|patch| PatchInfo {
						offset: Vector2::new(patch.offset[0] as i32, patch.offset[1] as i32),
						index: patch.index as usize,
					})
					.collect();

				Ok((
					name,
					TextureInfo {
						size: Vector2::new(texture.size[0] as u32, texture.size[1] as u32),
						patches: patches,
					},
				))
			})
			.collect()
	}
}

#[derive(Deserialize)]
pub struct RawPatchInfo {
	pub offset: [i16; 2],
	pub index: u16,
	_unused: u32,
}

#[derive(Deserialize)]
pub struct RawTextureInfo {
	pub name: [u8; 8],
	_unused1: u32,
	pub size: [u16; 2],
	_unused2: u32,
	pub patch_count: u16,
}

pub struct RawTexturesFormat;

impl AssetFormat for RawTexturesFormat {
	type Asset = Vec<(RawTextureInfo, Vec<RawPatchInfo>)>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(name)?);

		let count: u32 = bincode::deserialize_from(&mut data)?;
		let mut offsets: Vec<u32> = Vec::with_capacity(count as usize);

		for _ in 0..count {
			offsets.push(bincode::deserialize_from(&mut data)?);
		}

		offsets
			.into_iter()
			.map(|offset| {
				data.seek(SeekFrom::Start(offset as u64))?;

				let texture_info: RawTextureInfo = bincode::deserialize_from(&mut data)?;
				let mut patches: Vec<RawPatchInfo> =
					Vec::with_capacity(texture_info.patch_count as usize);

				for _ in 0..texture_info.patch_count {
					patches.push(bincode::deserialize_from(&mut data)?);
				}

				Ok((texture_info, patches))
			})
			.collect()
	}
}
