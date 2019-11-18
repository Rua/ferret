use nalgebra::{Vector2, Vector3};
use std::f32::{INFINITY, NEG_INFINITY};

#[derive(Debug, Clone)]
pub struct BoundingBox2 {
	pub min: Vector2<f32>,
	pub max: Vector2<f32>,
}

impl BoundingBox2 {
	pub fn new(min: Vector2<f32>, max: Vector2<f32>) -> BoundingBox2 {
		assert!(min[0] <= max[0]);
		assert!(min[1] <= max[1]);

		BoundingBox2 { min, max }
	}

	/*	pub fn zero() -> BoundingBox2 {
		BoundingBox2::new(Vector2::zeros(), Vector2::zeros())
	}*/

	pub fn from_extents(top: f32, bottom: f32, left: f32, right: f32) -> BoundingBox2 {
		BoundingBox2::new(Vector2::new(bottom, left), Vector2::new(top, right))
	}
}

#[derive(Debug, Clone)]
pub struct BoundingBox3 {
	pub min: Vector3<f32>,
	pub max: Vector3<f32>,
}

impl BoundingBox3 {
	pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> BoundingBox3 {
		assert!(min[0] <= max[0]);
		assert!(min[1] <= max[1]);
		assert!(min[2] <= max[2]);

		BoundingBox3 { min, max }
	}

	pub fn zero() -> BoundingBox3 {
		BoundingBox3::new(Vector3::zeros(), Vector3::zeros())
	}
}

impl From<&BoundingBox2> for BoundingBox3 {
	fn from(bounding_box: &BoundingBox2) -> BoundingBox3 {
		BoundingBox3::new(
			Vector3::new(bounding_box.min[0], bounding_box.min[1], NEG_INFINITY),
			Vector3::new(bounding_box.max[0], bounding_box.max[1], INFINITY),
		)
	}
}

#[derive(Debug, Clone)]
pub struct Plane {
	pub normal: Vector3<f32>,
	pub distance: f32,
}
