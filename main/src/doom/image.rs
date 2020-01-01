use crate::assets::{AssetFormat, DataSource};
use sdl2::{
	pixels::{Color, PixelFormatEnum},
	surface::Surface,
};
use serde::Deserialize;
use std::{
	error::Error,
	io::{Cursor, Read, Seek, SeekFrom},
};

pub type Palette = [Color; 256];

#[derive(Clone, Copy)]
pub struct PaletteFormat;

impl AssetFormat for PaletteFormat {
	type Asset = Palette;

	fn import(&self, name: &str, source: &impl DataSource) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(name)?);
		let mut palette = [Color {
			r: 0,
			g: 0,
			b: 0,
			a: 0,
		}; 256];

		for i in 0..256 {
			let rgb: [u8; 3] = bincode::deserialize_from(&mut data)?;
			palette[i] = Color::RGB(rgb[0], rgb[1], rgb[2]);
		}

		Ok(palette)
	}
}

pub struct Image {
	pub data: Vec<u8>,
	pub size: [u16; 2],
	pub offset: [i16; 2],
}

#[derive(Deserialize)]
struct ImageHeader {
	size: [u16; 2],
	offset: [i16; 2],
}

#[derive(Clone, Copy)]
pub struct ImageFormat;

impl AssetFormat for ImageFormat {
	type Asset = Image;

	fn import(&self, name: &str, source: &impl DataSource) -> Result<Self::Asset, Box<dyn Error>> {
		let palette = PaletteFormat.import("PLAYPAL", source)?;
		let mut data = Cursor::new(source.load(name)?);

		let header: ImageHeader = bincode::deserialize_from(&mut data)?;
		let mut column_offsets: Vec<u32> = Vec::new();

		for _ in 0..header.size[0] {
			column_offsets.push(bincode::deserialize_from(&mut data)?);
		}

		let mut surface = Surface::new(
			header.size[0] as u32,
			header.size[1] as u32,
			PixelFormatEnum::RGBA32,
		)?;
		let pitch = surface.pitch() as usize;
		assert_eq!(pitch, header.size[0] as usize * 4);

		let pixels = surface.without_lock_mut().unwrap();

		for col in 0..header.size[0] as usize {
			data.seek(SeekFrom::Start(column_offsets[col] as u64))?;
			let mut start_row: u8 = bincode::deserialize_from(&mut data)?;

			while start_row != 255 {
				// Read pixels in one vertical "post"
				let post_height: u8 = bincode::deserialize_from(&mut data)?;
				let mut post_pixels = vec![0u8; post_height as usize];
				bincode::deserialize_from::<_, u8>(&mut data)?; // Padding byte
				data.read_exact(&mut post_pixels)?;
				bincode::deserialize_from::<_, u8>(&mut data)?; // Padding byte

				// Paint the pixels onto the main image
				for i in 0..post_pixels.len() {
					assert!(start_row as usize + i < header.size[1] as usize);
					let color = palette[post_pixels[i] as usize];
					pixels[pitch * (start_row as usize + i) + 4 * col + 0] = color.r;
					pixels[pitch * (start_row as usize + i) + 4 * col + 1] = color.g;
					pixels[pitch * (start_row as usize + i) + 4 * col + 2] = color.b;
					pixels[pitch * (start_row as usize + i) + 4 * col + 3] = color.a;
				}

				start_row = bincode::deserialize_from(&mut data)?;
			}
		}

		Ok(Image {
			data: pixels.to_owned(),
			size: header.size,
			offset: header.offset,
		})
	}
}
