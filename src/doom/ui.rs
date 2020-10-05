use crate::{common::assets::AssetHandle, doom::image::Image};
use derivative::Derivative;
use nalgebra::Vector2;

#[derive(Clone, Copy, Debug, Default)]
pub struct UiTransform {
	pub position: Vector2<f32>,
	pub depth: f32,
	pub alignment: [UiAlignment; 2],
	pub size: Vector2<f32>,
	pub stretch: [bool; 2],
}

pub struct UiImage {
	pub image: AssetHandle<Image>,
}

#[derive(Clone, Copy, Debug, Derivative)]
#[derivative(Default)]
pub enum UiAlignment {
	#[derivative(Default)]
	Near = 0,
	Middle = 1,
	Far = 2,
}
