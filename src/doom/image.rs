use crate::assets::{Asset, AssetFormat, DataSource};
use byteorder::{ReadBytesExt, LE};
use std::{
	error::Error,
	io::{Cursor, Read, Seek, SeekFrom},
};

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct RGBAColor {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct IAColor {
	pub i: u8,
	pub a: u8,
}

pub type Palette = [RGBAColor; 256];

impl Asset for Palette {
	type Data = Self;
	type Intermediate = Self;
	const NAME: &'static str = "Palette";
}

#[derive(Clone, Copy)]
pub struct PaletteFormat;

impl AssetFormat for PaletteFormat {
	type Asset = Palette;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(name)?);
		let mut palette = [RGBAColor {
			r: 0,
			g: 0,
			b: 0,
			a: 0,
		}; 256];

		for i in 0..256 {
			let r = reader.read_u8()?;
			let g = reader.read_u8()?;
			let b = reader.read_u8()?;
			palette[i] = RGBAColor { r, g, b, a: 0xFF };
		}

		Ok(palette)
	}
}

pub struct Image {
	pub data: Vec<IAColor>,
	pub size: [usize; 2],
	pub offset: [isize; 2],
}

#[derive(Clone, Copy)]
pub struct ImageFormat;

impl AssetFormat for ImageFormat {
	type Asset = Image;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(name)?);

		let size = [
			reader.read_u16::<LE>()? as usize,
			reader.read_u16::<LE>()? as usize,
		];
		let offset = [
			reader.read_i16::<LE>()? as isize,
			reader.read_i16::<LE>()? as isize,
		];
		let mut column_offsets = Vec::new();

		for _ in 0..size[0] {
			column_offsets.push(reader.read_u32::<LE>()? as u64);
		}

		let mut data = vec![IAColor::default(); size[0] * size[1]];

		for col in 0..size[0] {
			reader.seek(SeekFrom::Start(column_offsets[col]))?;
			let mut start_row = reader.read_u8()? as usize;

			while start_row != 255 {
				// Read pixels in one vertical "post"
				let post_height = reader.read_u8()? as usize;
				let mut post_pixels = vec![0u8; post_height];
				reader.read_u8()?; // Padding byte
				reader.read_exact(&mut post_pixels)?;
				reader.read_u8()?; // Padding byte

				// Paint the pixels onto the main image
				for i in 0..post_pixels.len() {
					assert!(start_row + i < size[1]);
					data[size[0] * (start_row as usize + i) + col].i = post_pixels[i];
					data[size[0] * (start_row as usize + i) + col].a = 0xFF;
				}

				start_row = reader.read_u8()? as usize;
			}
		}

		Ok(Image { data, size, offset })
	}
}
