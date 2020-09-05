use crate::common::geometry::Angle;
use derivative::Derivative;
use nalgebra::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct SpawnOnCeiling {
	pub offset: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct SpawnPoint {
	pub player_num: usize,
}

#[derive(Clone, Copy, Debug, Derivative)]
#[derivative(Default)]
pub struct Transform {
	#[derivative(Default(value = "Vector3::zeros()"))]
	pub position: Vector3<f32>,
	#[derivative(Default(value = "Vector3::zeros()"))]
	pub rotation: Vector3<Angle>,
}

#[derive(Clone, Copy, Debug, Derivative)]
#[derivative(Default)]
pub struct Velocity {
	#[derivative(Default(value = "Vector3::zeros()"))]
	pub velocity: Vector3<f32>,
}
