use crate::{
	assets::{AssetFormat, DataSource},
	doom::image::{ImageFormat, PaletteFormat},
	renderer::texture::TextureBuilder,
};
use nalgebra::Vector2;
use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface};
use serde::Deserialize;
use std::{
	collections::HashMap,
	error::Error,
	io::{Cursor, Read, Seek, SeekFrom},
	str,
};
use vulkano::{format::Format, image::Dimensions};

#[derive(Clone, Copy)]
pub struct FlatFormat;

impl AssetFormat for FlatFormat {
	type Asset = TextureBuilder;

	fn import(&self, name: &str, source: &impl DataSource) -> Result<Self::Asset, Box<dyn Error>> {
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

		// Create the image
		let builder = TextureBuilder::new()
			.with_data(surface.without_lock().unwrap().to_owned())
			.with_dimensions(Dimensions::Dim2d {
				width: surface.width(),
				height: surface.height(),
			})
			.with_format(Format::R8G8B8A8Unorm);

		Ok(builder)
	}
}

#[derive(Clone, Copy)]
pub struct PNamesFormat;

impl AssetFormat for PNamesFormat {
	type Asset = Vec<[u8; 8]>;

	fn import(&self, name: &str, source: &impl DataSource) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(name)?);
		let count: u32 = bincode::deserialize_from(&mut data)?;
		let mut ret = Vec::with_capacity(count as usize);

		for _ in 0..count {
			ret.push(bincode::deserialize_from(&mut data)?);
		}

		Ok(ret)
	}
}

#[derive(Clone, Copy)]
pub struct TextureFormat;

impl AssetFormat for TextureFormat {
	type Asset = TextureBuilder;

	fn import(&self, name: &str, source: &impl DataSource) -> Result<Self::Asset, Box<dyn Error>> {
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

		// Create the image
		let builder = TextureBuilder::new()
			.with_data(surface.without_lock().unwrap().to_owned())
			.with_dimensions(Dimensions::Dim2d {
				width: surface.width(),
				height: surface.height(),
			})
			.with_format(Format::R8G8B8A8Unorm);

		Ok(builder)
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

#[derive(Clone, Copy)]
pub struct TexturesFormat;

impl AssetFormat for TexturesFormat {
	type Asset = HashMap<String, TextureInfo>;

	fn import(&self, name: &str, source: &impl DataSource) -> Result<Self::Asset, Box<dyn Error>> {
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

#[derive(Clone, Copy)]
pub struct RawTexturesFormat;

impl AssetFormat for RawTexturesFormat {
	type Asset = Vec<(RawTextureInfo, Vec<RawPatchInfo>)>;

	fn import(&self, name: &str, source: &impl DataSource) -> Result<Self::Asset, Box<dyn Error>> {
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
