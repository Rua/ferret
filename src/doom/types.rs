use crate::{
	assets::{AssetFormat, DataSource},
/*	doom::{
		sprite::{Sprite, SpriteBuilder},
		wad::WadLoader,
	},*/
	renderer::{palette::Palette, /*texture::TextureBuilder*/},
};
use byteorder::{ReadBytesExt, LE};
use nalgebra::Vector2;
use sdl2::{
	pixels::{Color, PixelFormatEnum},
	rect::Rect,
	surface::Surface,
};
use std::{
	//cmp::max,
	collections::hash_map::HashMap,
	error::Error,
	io::{Cursor, Read, Seek, SeekFrom},
	str,
	vec::Vec,
};
/*use vulkano::{format::Format, image::Dimensions};*/

pub struct DoomPalette;

impl AssetFormat for DoomPalette {
	type Asset = Palette;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(name)?);
		let mut palette = [Color {
			r: 0,
			g: 0,
			b: 0,
			a: 0,
		}; 256];

		for i in 0..256 {
			let r = data.read_u8()?;
			let g = data.read_u8()?;
			let b = data.read_u8()?;

			palette[i] = Color::RGB(r, g, b);
		}

		Ok(Palette(palette))
	}
}

pub struct DoomFlat;

impl AssetFormat for DoomFlat {
	type Asset = Surface<'static>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let palette = DoomPalette.import("PLAYPAL", source)?;
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

pub struct DoomImage {
	data: Vec<u8>,
	size: Vector2<usize>,
	_offset: Vector2<f32>,
}

pub struct DoomImageLump;

impl AssetFormat for DoomImageLump {
	type Asset = DoomImage;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let palette = DoomPalette.import("PLAYPAL", source)?;
		let mut data = Cursor::new(source.load(name)?);

		let size_x = data.read_u16::<LE>()? as usize;
		let size_y = data.read_u16::<LE>()? as usize;
		let offset_x = data.read_i16::<LE>()? as f32;
		let offset_y = data.read_i16::<LE>()? as f32;

		let mut column_offsets = vec![0; size_x];
		data.read_u32_into::<LE>(&mut column_offsets)?;

		let mut surface = Surface::new(size_x as u32, size_y as u32, PixelFormatEnum::RGBA32)?;
		let pitch = surface.pitch() as usize;
		assert_eq!(pitch, size_x * 4);

		let pixels = surface.without_lock_mut().unwrap();

		for col in 0..size_x as usize {
			data.seek(SeekFrom::Start(column_offsets[col] as u64))?;
			let mut start_row = data.read_u8()? as usize;

			while start_row != 255 {
				// Read pixels in one vertical "post"
				let post_height = data.read_u8()?;
				let mut post_pixels = vec![0u8; post_height as usize];
				data.read_u8()?; // Padding byte
				data.read_exact(&mut post_pixels)?;
				data.read_u8()?; // Padding byte

				// Paint the pixels onto the main image
				for i in 0..post_pixels.len() {
					assert!(start_row + i < size_y as usize);
					let color = palette[post_pixels[i] as usize];
					pixels[pitch * (start_row + i) + 4 * col + 0] = color.r;
					pixels[pitch * (start_row + i) + 4 * col + 1] = color.g;
					pixels[pitch * (start_row + i) + 4 * col + 2] = color.b;
					pixels[pitch * (start_row + i) + 4 * col + 3] = color.a;
				}

				start_row = data.read_u8()? as usize;
			}
		}

		Ok(DoomImage {
			data: pixels.to_owned(),
			size: Vector2::new(size_x, size_y),
			_offset: Vector2::new(offset_x, offset_y),
		})
	}
}

pub struct DoomPNames;

impl AssetFormat for DoomPNames {
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

/*pub mod sound {
	use super::*;

	pub struct DoomSound {
		sampling_rate: u16,
		samples: Vec<u8>,
	}

	pub fn from_data<T: Read>(data: &mut T) -> Result<DoomSound, Box<dyn Error>> {
		let signature = data.read_u16::<LE>()?;

		if signature != 3 {
			panic!("No Doom sound file signature found.");
		}

		let sampling_rate = data.read_u16::<LE>()?;
		let num_samples = data.read_u32::<LE>()? as usize;
		let mut samples = vec![0u8; num_samples as usize];

		data.read_exact(&mut samples)?;

		// Remove padding bytes at start and end
		if samples.ends_with(&[samples[num_samples - 17]; 16]) {
			samples.drain(num_samples - 17..);
		}

		if samples.starts_with(&[samples[16]; 16]) {
			samples.drain(..16);
		}

		Ok(DoomSound {
			sampling_rate: sampling_rate,
			samples: samples,
		})
	}

	pub fn from_wad(name: &str, loader: &mut WadLoader) -> Result<DoomSound, Box<dyn Error>> {
		let mut data = Cursor::new(loader.load(name)?);
		from_data(&mut data)
	}
}*/

//pub struct DoomSprite;

/*impl AssetFormat for DoomSprite {
	type Asset = SpriteBuilder;

	fn import(&self, name: &str, source: &mut impl DataSource) -> Result<Self::Asset, Box<dyn Error>> {
		let mut lumpnames = source.names()
			.filter(|n| n.starts_with(name))
			.map(str::to_owned)
			.collect::<Vec<_>>();
		lumpnames.sort_unstable();

		Ok(Sprite::new(
			images,
			frames,
			SpriteOrientation::ViewPlaneParallelUpright,
			max_size,
		))
	}
}*/

pub struct DoomTexture;

impl AssetFormat for DoomTexture {
	type Asset = Surface<'static>;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let pnames = DoomPNames.import("PNAMES", source)?;
		let mut texture_info = TextureInfoLump.import("TEXTURE1", source)?;
		texture_info.extend(TextureInfoLump.import("TEXTURE2", source)?);
		let texture_info = &texture_info[name];

		let mut surface = Surface::new(
			texture_info.size[0] as u32,
			texture_info.size[1] as u32,
			PixelFormatEnum::RGBA32,
		)?;

		for patch_info in &texture_info.patches {
			let name = &pnames[patch_info.index];

			// Use to_surface because the offsets of patches are ignored anyway
			let mut patch = DoomImageLump.import(&name, source)?;
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

pub struct TextureInfoLump;

impl AssetFormat for TextureInfoLump {
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
			let name = String::from(str::from_utf8(&name)?.trim_end_matches('\0'));

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
