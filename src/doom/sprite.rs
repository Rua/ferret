use crate::{
	common::assets::{Asset, AssetHandle, AssetStorage, ImportData},
	doom::image::Image,
};
use anyhow::{bail, Context};
use lazy_static::lazy_static;
use regex::Regex;
use relative_path::RelativePath;
use vulkano::impl_vertex;

pub struct Sprite {
	frames: Vec<Vec<SpriteImageInfo>>,
}

#[derive(Clone)]
pub struct SpriteImageInfo {
	pub flip: f32,
	pub handle: AssetHandle<Image>,
}

impl Sprite {
	pub fn frames(&self) -> &Vec<Vec<SpriteImageInfo>> {
		&self.frames
	}
}

pub struct SpriteBuilder {
	frames: Vec<Vec<SpriteImageInfoIntermediate>>,
	image_names: Vec<String>,
}

#[derive(Clone, Copy)]
pub struct SpriteImageInfoIntermediate {
	pub flip: f32,
	pub image_index: usize,
}

impl SpriteBuilder {
	pub fn new() -> SpriteBuilder {
		SpriteBuilder {
			frames: Vec::new(),
			image_names: Vec::new(),
		}
	}

	pub fn with_frames(mut self, frames: Vec<Vec<SpriteImageInfoIntermediate>>) -> Self {
		self.frames = frames;
		self
	}

	pub fn with_image_names(mut self, image_names: Vec<String>) -> Self {
		self.image_names = image_names;
		self
	}

	pub fn build(self, storage: &mut AssetStorage) -> anyhow::Result<Sprite> {
		let handles: Vec<_> = self
			.image_names
			.into_iter()
			.map(|name| storage.load(&name))
			.collect();

		let frames = self
			.frames
			.into_iter()
			.map(|rotations| {
				rotations
					.into_iter()
					.map(|info| SpriteImageInfo {
						flip: info.flip,
						handle: handles[info.image_index].clone(),
					})
					.collect()
			})
			.collect();

		Ok(Sprite { frames })
	}
}

impl Asset for Sprite {
	const NAME: &'static str = "Sprite";
	const NEEDS_PROCESSING: bool = false;

	fn import(
		path: &RelativePath,
		asset_storage: &mut AssetStorage,
	) -> anyhow::Result<Box<dyn ImportData>> {
		lazy_static! {
			static ref SPRITENAME: Regex =
				Regex::new(r#"^....[A-Z][0-9](?:[A-Z][0-9])?$"#).unwrap();
		}

		let stem = path.file_stem().context("Empty file name")?;

		let mut image_names = Vec::new();
		let mut info = Vec::new();
		let mut max_frame = 0;

		for lump_name in asset_storage
			.source()
			.names()
			.filter(|n| n.starts_with(stem) && SPRITENAME.is_match(n))
		{
			// Regular frame
			let frame = lump_name.chars().nth(4).unwrap() as isize - 'A' as isize;
			assert!(frame >= 0 && frame < 29);
			let rotation = lump_name.chars().nth(5).unwrap() as isize - '1' as isize;
			assert!(rotation >= -1 && rotation < 8);
			info.push((
				frame as usize,
				rotation,
				SpriteImageInfoIntermediate {
					flip: 1.0,
					image_index: image_names.len(),
				},
			));
			max_frame = usize::max(max_frame, frame as usize);

			// Horizontally flipped frame, if any
			if lump_name.len() == 8 {
				let frame = lump_name.chars().nth(6).unwrap() as isize - 'A' as isize;
				assert!(frame >= 0 && frame < 29);
				let rotation = lump_name.chars().nth(7).unwrap() as isize - '1' as isize;
				assert!(rotation >= -1 && rotation < 8);
				info.push((
					frame as usize,
					rotation,
					SpriteImageInfoIntermediate {
						flip: -1.0,
						image_index: image_names.len(),
					},
				));
				max_frame = usize::max(max_frame, frame as usize);
			}

			// Add the texture
			image_names.push(format!("{}.patch", lump_name));
		}

		info.sort_unstable_by(|a, b| Ord::cmp(&a.0, &b.0).then(Ord::cmp(&a.1, &b.1)));
		let mut slice = info.as_slice();
		let mut frames: Vec<Vec<SpriteImageInfoIntermediate>> = vec![Vec::new(); max_frame + 1];

		while !slice.is_empty() {
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
				bail!("Frame {} has an invalid number of rotations", frame);
			}
		}

		Ok(Box::new(
			SpriteBuilder::new()
				.with_frames(frames)
				.with_image_names(image_names)
				.build(asset_storage)?,
		))
	}
}

#[derive(Clone, Debug, Default)]
pub struct VertexData {
	pub in_position: [f32; 2],
	pub in_texture_coord: [f32; 2],
}
impl_vertex!(VertexData, in_position, in_texture_coord);
