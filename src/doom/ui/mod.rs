use crate::{
	common::assets::AssetHandle,
	doom::assets::{
		font::{Font, HexFont},
		image::Image,
	},
};
use derivative::Derivative;
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

pub mod hud;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct UiTransform {
	pub position: Vector2<f32>,
	pub depth: f32,
	pub alignment: [UiAlignment; 2],
	pub size: Vector2<f32>,
	pub stretch: [bool; 2],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiGameView;

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
	factors: Vector2<f32>,
	dimensions: Vector2<f32>,
	framebuffer_dimensions: Vector2<f32>,
	alignment_offsets: [Vector2<f32>; 3],
	stretch_offsets: [Vector2<f32>; 2],
}

impl UiParams {
	pub fn new(framebuffer_dimensions: [u32; 2]) -> UiParams {
		let framebuffer_dimensions = Vector2::new(
			framebuffer_dimensions[0] as f32,
			framebuffer_dimensions[1] as f32,
		);

		// If the current aspect ratio is wider than 4:3, stretch horizontally.
		// If narrower, stretch vertically.
		let ratio = (framebuffer_dimensions[0] / framebuffer_dimensions[1]) / (4.0 / 3.0);
		let factors = if ratio >= 1.0 {
			Vector2::new(ratio, 1.0)
		} else {
			Vector2::new(1.0, 1.0 / ratio)
		};

		let base_dimensions = Vector2::new(320.0, 200.0);
		let dimensions = base_dimensions.component_mul(&factors);
		let alignment_offsets = [
			Vector2::zeros(),
			(dimensions - base_dimensions) * 0.5,
			dimensions - base_dimensions,
		];
		let stretch_offsets = [Vector2::zeros(), dimensions - base_dimensions];

		UiParams {
			factors,
			dimensions,
			framebuffer_dimensions,
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

// TODO ewwww
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiHexFontText {
	pub lines: Vec<String>,
	pub font: AssetHandle<HexFont>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Hidden;
