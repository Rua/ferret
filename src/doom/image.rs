use crate::assets::{AssetFormat, DataSource};
use byteorder::{ReadBytesExt, LE};
use nalgebra::Vector2;
use sdl2::{
	pixels::{Color, PixelFormatEnum},
	surface::Surface,
};
use std::{
	error::Error,
	io::{Cursor, Read, Seek, SeekFrom},
};

pub type DoomPalette = [Color; 256];

pub struct DoomPaletteFormat;

impl AssetFormat for DoomPaletteFormat {
	type Asset = DoomPalette;

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

		Ok(palette)
	}
}

pub struct DoomImage {
	pub data: Vec<u8>,
	pub size: Vector2<usize>,
	pub offset: Vector2<f32>,
}

pub struct DoomImageFormat;

impl AssetFormat for DoomImageFormat {
	type Asset = DoomImage;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let palette = DoomPaletteFormat.import("PLAYPAL", source)?;
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
			offset: Vector2::new(offset_x, offset_y),
		})
	}
}
