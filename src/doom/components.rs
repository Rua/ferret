use crate::common::geometry::Angle;
use nalgebra::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct SpawnOnCeiling {
	pub offset: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct SpawnPoint {
	pub player_num: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Transform {
	pub position: Vector3<f32>,
	pub rotation: Vector3<Angle>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Velocity {
	pub velocity: Vector3<f32>,
}
