use crate::common::{
	assets::{AssetHandle, AssetStorage, ImportData},
	video::{AsBytes, RenderContext},
};
use byteorder::{ReadBytesExt, LE};
use nalgebra::Vector2;
use relative_path::RelativePath;
use std::{
	io::{Cursor, Read, Seek, SeekFrom},
	ops::Deref,
	sync::Arc,
};
use vulkano::{
	format::Format,
	image::{
		view::{ImageView, ImageViewAbstract},
		ImageDimensions, ImmutableImage, MipmapsCount,
	},
};

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

pub fn import_palette(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let mut reader = Cursor::new(asset_storage.source().load(path)?);
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

#[derive(Clone, Debug)]
pub struct ImageData {
	pub data: Vec<IAColor>,
	pub size: Vector2<usize>,
	pub offset: Vector2<isize>,
}

pub struct Image {
	pub image_view: Arc<dyn ImageViewAbstract + Send + Sync>,
	pub offset: Vector2<f32>,
}

impl Image {
	pub fn size(&self) -> Vector2<f32> {
		let [width, height] = self.image_view.image().dimensions().width_height();
		Vector2::new(width as f32, height as f32)
	}
}

pub fn import_patch(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let mut reader = Cursor::new(asset_storage.source().load(path)?);

	let size = Vector2::new(
		reader.read_u16::<LE>()? as usize,
		reader.read_u16::<LE>()? as usize,
	);
	let offset = Vector2::new(
		reader.read_i16::<LE>()? as isize,
		reader.read_i16::<LE>()? as isize,
	);
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

	Ok(Box::new(ImageData { data, size, offset }))
}

pub fn process_images(render_context: &RenderContext, asset_storage: &mut AssetStorage) {
	let palette_handle: AssetHandle<Palette> = asset_storage.load("playpal.palette");

	asset_storage.process::<Image, _>(|data, asset_storage| {
		let image_data: ImageData = *data.downcast().ok().expect("Not an ImageData");
		let palette = asset_storage.get(&palette_handle).unwrap();
		let data: Vec<_> = image_data
			.data
			.into_iter()
			.map(|pixel| {
				if pixel.a == 0xFF {
					palette[pixel.i as usize]
				} else {
					crate::doom::image::RGBAColor::default()
				}
			})
			.collect();

		// Create the image
		let (image, _future) = ImmutableImage::from_iter(
			data.as_bytes().iter().copied(),
			ImageDimensions::Dim2d {
				width: image_data.size[0] as u32,
				height: image_data.size[1] as u32,
				array_layers: 1,
			},
			MipmapsCount::One,
			Format::R8G8B8A8Unorm,
			render_context.queues().graphics.clone(),
		)?;
		let image_view = ImageView::new(image)?;

		Ok(Image {
			image_view,
			offset: Vector2::new(image_data.offset[0] as f32, image_data.offset[1] as f32),
		})
	});
}
