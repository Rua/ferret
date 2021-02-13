use crate::{
	common::assets::{AssetHandle, AssetStorage, ImportData},
	doom::{data::FONTS, image::Image},
};
use anyhow::Context;
use derivative::Derivative;
use fnv::FnvHashMap;
use nalgebra::Vector2;
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct UiTransform {
	pub position: Vector2<f32>,
	pub depth: f32,
	pub alignment: [UiAlignment; 2],
	pub size: Vector2<f32>,
	pub stretch: [bool; 2],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiImage {
	pub image: AssetHandle<Image>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiText {
	pub text: String,
	pub font: AssetHandle<Font>,
}

#[derive(Clone, Copy, Debug, Derivative, Serialize, Deserialize)]
#[derivative(Default)]
pub enum UiAlignment {
	#[derivative(Default)]
	Near = 0,
	Middle = 1,
	Far = 2,
}

#[derive(Clone, Copy, Debug)]
pub struct UiParams {
	dimensions: Vector2<f32>,
	framebuffer_dimensions: Vector2<f32>,
	viewport_dimensions: Vector2<f32>,
	alignment_offsets: [Vector2<f32>; 3],
	stretch_offsets: [Vector2<f32>; 2],
}

impl UiParams {
	pub fn new(dimensions: [u32; 2]) -> UiParams {
		let framebuffer_dimensions = Vector2::new(dimensions[0] as f32, dimensions[1] as f32);
		let ratio = (framebuffer_dimensions[0] / framebuffer_dimensions[1]) / (4.0 / 3.0);

		// If the current aspect ratio is wider than 4:3, stretch horizontally.
		// If narrower, stretch vertically.
		let base_dimensions = Vector2::new(320.0, 200.0);
		let dimensions = if ratio >= 1.0 {
			Vector2::new(base_dimensions[0] * ratio, base_dimensions[1])
		} else {
			Vector2::new(base_dimensions[0], base_dimensions[1] / ratio)
		};
		let alignment_offsets = [
			Vector2::zeros(),
			(dimensions - base_dimensions) * 0.5,
			dimensions - base_dimensions,
		];
		let stretch_offsets = [Vector2::zeros(), dimensions - base_dimensions];

		let viewport_dimensions = Vector2::new(
			framebuffer_dimensions[0],
			(1.0 - 32.0 / dimensions[1]) * framebuffer_dimensions[1],
		);

		UiParams {
			dimensions,
			framebuffer_dimensions,
			viewport_dimensions,
			alignment_offsets,
			stretch_offsets,
		}
	}

	#[inline]
	pub fn dimensions(&self) -> Vector2<f32> {
		self.dimensions
	}

	#[inline]
	pub fn framebuffer_dimensions(&self) -> Vector2<f32> {
		self.framebuffer_dimensions
	}

	#[inline]
	pub fn viewport_dimensions(&self) -> Vector2<f32> {
		self.viewport_dimensions
	}

	#[inline]
	pub fn align(&self, alignment: [UiAlignment; 2]) -> Vector2<f32> {
		Vector2::new(
			self.alignment_offsets[alignment[0] as usize][0],
			self.alignment_offsets[alignment[1] as usize][1],
		)
	}

	#[inline]
	pub fn stretch(&self, stretch: [bool; 2]) -> Vector2<f32> {
		Vector2::new(
			self.stretch_offsets[stretch[0] as usize][0],
			self.stretch_offsets[stretch[1] as usize][1],
		)
	}
}

#[derive(Clone, Debug)]
pub struct Font {
	pub characters: FnvHashMap<char, AssetHandle<Image>>,
	pub spacing: FontSpacing,
}

#[derive(Clone, Copy, Debug)]
pub enum FontSpacing {
	FixedWidth { width: f32 },
	VariableWidth { space_width: f32 },
}

pub fn import_font(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let func = FONTS
		.get(path.as_str())
		.with_context(|| format!("Font \"{}\" not found", path))?;
	let template = func(asset_storage);
	Ok(Box::new(template))
}
