use crate::{
	assets::{Asset, AssetFormat, DataSource},
	doom::image::ImageFormat,
	renderer::{
		mesh::{Mesh, MeshBuilder},
		texture::{Texture, TextureBuilder},
	},
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, sync::Arc};
use vulkano::{device::Queue, format::Format, image::Dimensions, impl_vertex, sync::GpuFuture};

pub struct Sprite {
	frames: Vec<Vec<SpriteInfo>>,
	meshes: Vec<Mesh>,
	textures: Vec<Texture>,
}

impl Asset for Sprite {
	type Data = SpriteBuilder;
}

impl Sprite {
	pub fn frames(&self) -> &Vec<Vec<SpriteInfo>> {
		&self.frames
	}

	pub fn meshes(&self) -> &Vec<Mesh> {
		&self.meshes
	}

	pub fn textures(&self) -> &Vec<Texture> {
		&self.textures
	}
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct SpriteInfo {
	pub mesh_index: usize,
	pub texture_index: usize,
}

pub struct SpriteBuilder {
	frames: Vec<Vec<SpriteInfo>>,
	meshes: Vec<MeshBuilder>,
	textures: Vec<TextureBuilder>,
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
		fn make_mesh(size: [usize; 2], offset: [isize; 2], flip: bool) -> Vec<VertexData> {
			let mut mesh = Vec::new();
			for (h, v) in [(1, 0), (0, 0), (0, 1), (1, 1)].iter().copied() {
				mesh.push(VertexData {
					in_position: [
						// Opposite signs because y increases downwards in image space, but
						// upwards in world space
						(-offset[0] + h * size[0] as isize) as f32,
						(offset[1] + v * -(size[1] as isize)) as f32,
					],
					in_texture_coord: [if flip { -h } else { h } as f32, v as f32],
				});
			}
			mesh
		}

		lazy_static! {
			static ref SPRITENAME: Regex =
				Regex::new(r#"^....[A-Z][0-9](?:[A-Z][0-9])?$"#).unwrap();
		}

		let mut textures = Vec::new();
		let mut meshes = Vec::new();
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
				SpriteInfo {
					mesh_index: meshes.len(),
					texture_index: textures.len(),
				},
			));
			max_frame = usize::max(max_frame, frame as usize);

			let mesh = make_mesh(image.size, image.offset, false);
			meshes.push(MeshBuilder::new().with_vertices(mesh));

			// Horizontally flipped frame, if any
			if lumpname.len() == 8 {
				let frame = lumpname.chars().nth(6).unwrap() as isize - 'A' as isize;
				assert!(frame >= 0 && frame < 29);
				let rotation = lumpname.chars().nth(7).unwrap() as isize - '1' as isize;
				assert!(rotation >= -1 && rotation < 8);
				info.push((
					frame as usize,
					rotation,
					SpriteInfo {
						mesh_index: meshes.len(),
						texture_index: textures.len(),
					},
				));
				max_frame = usize::max(max_frame, frame as usize);

				let mesh = make_mesh(image.size, image.offset, true);
				meshes.push(MeshBuilder::new().with_vertices(mesh));
			}

			// Add the texture
			let builder = TextureBuilder::new()
				.with_data(image.data)
				.with_dimensions(Dimensions::Dim2d {
					width: image.size[0] as u32,
					height: image.size[1] as u32,
				})
				.with_format(Format::R8G8B8A8Unorm);
			textures.push(builder);
		}

		info.sort_unstable();
		let mut slice = info.as_slice();
		let mut frames: Vec<Vec<SpriteInfo>> = vec![Vec::new(); max_frame + 1];

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
			.with_meshes(meshes)
			.with_textures(textures))
	}
}

impl SpriteBuilder {
	pub fn new() -> SpriteBuilder {
		SpriteBuilder {
			frames: Vec::new(),
			meshes: Vec::new(),
			textures: Vec::new(),
		}
	}

	pub fn with_frames(mut self, frames: Vec<Vec<SpriteInfo>>) -> Self {
		self.frames = frames;
		self
	}

	pub fn with_meshes(mut self, meshes: Vec<MeshBuilder>) -> Self {
		self.meshes = meshes;
		self
	}

	pub fn with_textures(mut self, textures: Vec<TextureBuilder>) -> Self {
		self.textures = textures;
		self
	}

	pub fn build(
		self,
		queue: Arc<Queue>,
	) -> Result<(Sprite, Box<dyn GpuFuture>), Box<dyn Error + Send + Sync>> {
		let ret_future: Box<dyn GpuFuture> = Box::from(vulkano::sync::now(queue.device().clone()));

		let meshes = self
			.meshes
			.into_iter()
			.map(|builder| {
				let (mesh, future) = builder.build(queue.clone())?;
				Ok(mesh)
			})
			.collect::<Result<_, Box<dyn Error + Send + Sync>>>()?;

		let textures = self
			.textures
			.into_iter()
			.map(|builder| {
				let (texture, future) = builder.build(queue.clone())?;
				Ok(texture)
			})
			.collect::<Result<_, Box<dyn Error + Send + Sync>>>()?;

		Ok((
			Sprite {
				frames: self.frames,
				meshes,
				textures,
			},
			ret_future,
		))
	}
}
