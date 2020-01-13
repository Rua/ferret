use crate::{
	assets::{Asset, AssetFormat, DataSource},
	doom::image::ImageFormat,
	renderer::{
		texture::{Texture, TextureBuilder},
	},
};
use lazy_static::lazy_static;
use nalgebra::{Matrix4, Vector3};
use regex::Regex;
use std::{error::Error, sync::Arc};
use vulkano::{device::Queue, format::Format, image::Dimensions, impl_vertex, sync::GpuFuture};

pub struct Sprite {
	frames: Vec<Vec<ImageInfo>>,
	textures: Vec<TextureInfo>,
}

impl Asset for Sprite {
	type Data = SpriteBuilder;
}

impl Sprite {
	pub fn frames(&self) -> &Vec<Vec<ImageInfo>> {
		&self.frames
	}

	pub fn textures(&self) -> &Vec<TextureInfo> {
		&self.textures
	}
}

#[derive(Clone, Copy)]
pub struct ImageInfo {
	pub flip: f32,
	pub texture_index: usize,
}

pub struct TextureInfo {
	pub texture: Texture,
	pub matrix: Matrix4<f32>,
}

pub struct SpriteBuilder {
	frames: Vec<Vec<ImageInfo>>,
	textures: Vec<(TextureBuilder, Matrix4<f32>)>,
}

impl SpriteBuilder {
	pub fn new() -> SpriteBuilder {
		SpriteBuilder {
			frames: Vec::new(),
			textures: Vec::new(),
		}
	}

	pub fn with_frames(mut self, frames: Vec<Vec<ImageInfo>>) -> Self {
		self.frames = frames;
		self
	}

	pub fn with_textures(mut self, textures: Vec<(TextureBuilder, Matrix4<f32>)>) -> Self {
		self.textures = textures;
		self
	}

	pub fn build(
		self,
		queue: Arc<Queue>,
	) -> Result<(Sprite, Box<dyn GpuFuture>), Box<dyn Error + Send + Sync>> {
		let ret_future: Box<dyn GpuFuture> = Box::from(vulkano::sync::now(queue.device().clone()));

		let textures = self
			.textures
			.into_iter()
			.map(|(builder, matrix)| {
				let (texture, future) = builder.build(queue.clone())?;
				Ok(TextureInfo {texture, matrix})
			})
			.collect::<Result<_, Box<dyn Error + Send + Sync>>>()?;

		Ok((
			Sprite {
				frames: self.frames,
				textures,
			},
			ret_future,
		))
	}
}

#[derive(Clone, Debug, Default)]
pub struct VertexData {
	pub in_position: [f32; 2],
	pub in_texture_coord: [f32; 2],
}
impl_vertex!(VertexData, in_position, in_texture_coord);

#[derive(Clone, Copy)]
pub struct SpriteFormat;

impl AssetFormat for SpriteFormat {
	type Asset = SpriteBuilder;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		lazy_static! {
			static ref SPRITENAME: Regex =
				Regex::new(r#"^....[A-Z][0-9](?:[A-Z][0-9])?$"#).unwrap();
		}

		let mut textures = Vec::new();
		let mut info = Vec::new();
		let mut max_frame = 0;

		for lumpname in source
			.names()
			.filter(|n| n.starts_with(name) && SPRITENAME.is_match(n))
		{
			// Load the sprite lump
			let image = ImageFormat.import(lumpname, source)?;

			// Regular frame
			let frame = lumpname.chars().nth(4).unwrap() as isize - 'A' as isize;
			assert!(frame >= 0 && frame < 29);
			let rotation = lumpname.chars().nth(5).unwrap() as isize - '1' as isize;
			assert!(rotation >= -1 && rotation < 8);
			info.push((
				frame as usize,
				rotation,
				ImageInfo {
					flip: 1.0,
					texture_index: textures.len(),
				},
			));
			max_frame = usize::max(max_frame, frame as usize);

			// Horizontally flipped frame, if any
			if lumpname.len() == 8 {
				let frame = lumpname.chars().nth(6).unwrap() as isize - 'A' as isize;
				assert!(frame >= 0 && frame < 29);
				let rotation = lumpname.chars().nth(7).unwrap() as isize - '1' as isize;
				assert!(rotation >= -1 && rotation < 8);
				info.push((
					frame as usize,
					rotation,
					ImageInfo {
						flip: -1.0,
						texture_index: textures.len(),
					},
				));
				max_frame = usize::max(max_frame, frame as usize);
			}

			// Add the texture
			let builder = TextureBuilder::new()
				.with_data(image.data)
				.with_dimensions(Dimensions::Dim2d {
					width: image.size[0] as u32,
					height: image.size[1] as u32,
				})
				.with_format(Format::R8G8B8A8Unorm);
			let matrix = Matrix4::new_translation(&Vector3::new(0.0, image.offset[0] as f32, image.offset[1] as f32))
				* Matrix4::new_nonuniform_scaling(&Vector3::new(0.0, image.size[0] as f32, image.size[1] as f32));
			textures.push((builder, matrix));
		}

		info.sort_unstable_by(|a, b| Ord::cmp(&a.0, &b.0).then(Ord::cmp(&a.1, &b.1)));
		let mut slice = info.as_slice();
		let mut frames: Vec<Vec<ImageInfo>> = vec![Vec::new(); max_frame + 1];

		while slice.len() > 0 {
			let frame = slice[0].0;
			let next_pos = slice
				.iter()
				.position(|x| x.0 != frame)
				.unwrap_or(slice.len());
			let current = &slice[..next_pos];
			slice = &slice[next_pos..];

			if current.len() == 1 {
				let rotation = current[0].1;
				assert_eq!(rotation, -1);
				frames[frame] = current.iter().map(|r| r.2).collect();
			} else if current.len() == 8 {
				frames[frame] = current
					.iter()
					.enumerate()
					.map(|(i, r)| {
						assert_eq!(i as isize, r.1);
						r.2
					})
					.collect();
			} else {
				return Err(Box::from(format!(
					"Frame {} has an invalid number of rotations",
					frame
				)));
			}
		}

		Ok(SpriteBuilder::new()
			.with_frames(frames)
			.with_textures(textures))
	}
}
