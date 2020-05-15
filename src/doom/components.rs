use crate::geometry::Angle;
use derivative::Derivative;
use nalgebra::Vector3;
use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Clone, Component, Copy, Debug)]
pub struct SpawnOnCeiling {
	pub offset: f32,
}

#[derive(Clone, Component, Copy, Debug)]
pub struct SpawnPoint {
	pub player_num: usize,
}

#[derive(Clone, Component, Copy, Debug, Derivative)]
#[derivative(Default)]
pub struct Transform {
	#[derivative(Default(value = "Vector3::zeros()"))]
	pub position: Vector3<f32>,
	#[derivative(Default(value = "Vector3::zeros()"))]
	pub rotation: Vector3<Angle>,
}

#[derive(Clone, Component, Copy, Debug, Derivative)]
#[derivative(Default)]
pub struct Camera {
	#[derivative(Default(value = "Vector3::zeros()"))]
	pub base: Vector3<f32>,
	#[derivative(Default(value = "Vector3::zeros()"))]
	pub offset: Vector3<f32>,
}

#[derive(Clone, Component, Copy, Debug, Derivative)]
#[derivative(Default)]
pub struct Velocity {
	#[derivative(Default(value = "Vector3::zeros()"))]
	pub velocity: Vector3<f32>,
}
