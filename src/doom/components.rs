use crate::geometry::Angle;
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

#[derive(Clone, Component, Copy, Debug)]
pub struct Transform {
	pub position: Vector3<f32>,
	pub rotation: Vector3<Angle>,
}

impl Default for Transform {
	fn default() -> Transform {
		Transform {
			position: Vector3::new(0.0, 0.0, 0.0),
			rotation: Vector3::new(0.into(), 0.into(), 0.into()),
		}
	}
}

#[derive(Clone, Component, Copy, Debug)]
pub struct Velocity {
	pub velocity: Vector3<f32>,
}

impl Default for Velocity {
	fn default() -> Velocity {
		Velocity {
			velocity: Vector3::new(0.0, 0.0, 0.0),
		}
	}
}
