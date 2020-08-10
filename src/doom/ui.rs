use crate::{assets::AssetHandle, doom::image::Image};
use derivative::Derivative;
use nalgebra::{Vector2, Vector3};

#[derive(Clone, Copy, Debug, Derivative)]
#[derivative(Default)]
pub struct UiTransform {
	#[derivative(Default(value = "Vector3::zeros()"))]
	pub position: Vector3<f32>,
	pub alignment: [UiAlignment; 2],
	#[derivative(Default(value = "Vector2::zeros()"))]
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
