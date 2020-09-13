use crate::common::assets::{Asset, AssetStorage, ImportData};
use byteorder::{ReadBytesExt, LE};
use std::{
	io::{Cursor, Read, Seek, SeekFrom},
	ops::Deref,
	sync::Arc,
};
use vulkano::image::ImageViewAccess;

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct RGBAColor {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct IAColor {
	pub i: u8,
	pub a: u8,
}

pub struct Palette([RGBAColor; 256]);

impl Deref for Palette {
	type Target = [RGBAColor; 256];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Asset for Palette {
	type Data = Self;
	const NAME: &'static str = "Palette";
	const NEEDS_PROCESSING: bool = false;

	fn import(name: &str, asset_storage: &mut AssetStorage) -> anyhow::Result<Box<dyn ImportData>> {
		let mut reader = Cursor::new(asset_storage.source().load(name)?);
		let mut palette = [RGBAColor {
			r: 0,
			g: 0,
			b: 0,
			a: 0,
		}; 256];

		for color in palette.iter_mut() {
			let r = reader.read_u8()?;
			let g = reader.read_u8()?;
			let b = reader.read_u8()?;
			*color = RGBAColor { r, g, b, a: 0xFF };
		}

		Ok(Box::new(Palette(palette)))
	}
}

#[derive(Clone, Debug)]
pub struct ImageData {
	pub data: Vec<IAColor>,
	pub size: [usize; 2],
	pub offset: [isize; 2],
}

impl ImageData {
	fn load(name: &str, asset_storage: &mut AssetStorage) -> anyhow::Result<ImageData> {
		let mut reader = Cursor::new(asset_storage.source().load(name)?);

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

		Ok(ImageData { data, size, offset })
	}
}

impl Asset for ImageData {
	type Data = Self;
	const NAME: &'static str = "ImageData";
	const NEEDS_PROCESSING: bool = false;

	fn import(name: &str, asset_storage: &mut AssetStorage) -> anyhow::Result<Box<dyn ImportData>> {
		Ok(Box::new(ImageData::load(name, asset_storage)?))
	}
}

pub struct Image {
	pub image: Arc<dyn ImageViewAccess + Send + Sync>,
	pub offset: [isize; 2],
}

impl Asset for Image {
	type Data = Self;
	const NAME: &'static str = "Image";
	const NEEDS_PROCESSING: bool = true;

	fn import(name: &str, asset_storage: &mut AssetStorage) -> anyhow::Result<Box<dyn ImportData>> {
		Ok(Box::new(ImageData::load(name, asset_storage)?))
	}
}
