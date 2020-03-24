use nalgebra::{Vector2, Vector3};

#[derive(Debug, Clone)]
pub struct Line2 {
	pub point: Vector2<f32>,
	pub dir: Vector2<f32>,
}

impl Line2 {
	#[inline]
	pub fn new(point: Vector2<f32>, dir: Vector2<f32>) -> Line2 {
		assert!(dir[0] != 0.0 || dir[1] != 0.0);

		Line2 { point, dir }
	}

	#[inline]
	pub fn intersect(&self, other: &Line2) -> Option<(f32, f32)> {
		let normal = Vector2::new(other.dir[1], -other.dir[0]).normalize();
		let denom = self.dir.dot(&normal);

		if denom == 0.0 {
			return None;
		}

		let self_param = (other.point - self.point).dot(&normal) / denom;

		let other_param = (self.point + self.dir * self_param - other.point).dot(&other.dir)
			/ other.dir.dot(&other.dir);

		Some((self_param, other_param))
	}

	// == 0: on line, < 0: right of line, > 0: left of line
	#[inline]
	pub fn point_side(&self, point: Vector2<f32>) -> f32 {
		let d = point - self.point;
		self.dir[0] * d[1] - self.dir[1] * d[0]
	}
}

impl From<&Line3> for Line2 {
	#[inline]
	fn from(line: &Line3) -> Line2 {
		Line2::new(
			Vector2::new(line.point[0], line.point[1]),
			Vector2::new(line.dir[0], line.dir[1]),
		)
	}
}

#[derive(Debug, Clone)]
pub struct Line3 {
	pub point: Vector3<f32>,
	pub dir: Vector3<f32>,
}

impl Line3 {
	#[inline]
	pub fn new(point: Vector3<f32>, dir: Vector3<f32>) -> Line3 {
		assert!(dir[0] != 0.0 || dir[1] != 0.0 || dir[2] != 0.0);

		Line3 { point, dir }
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Side {
	Right = 0,
	Left = 1,
}

impl std::ops::Not for Side {
	type Output = Side;

	#[inline]
	fn not(self) -> Self::Output {
		match self {
			Side::Right => Side::Left,
			Side::Left => Side::Right,
		}
	}
}

/*#[derive(Clone, Debug)]
pub struct Plane {
	pub normal: Vector3<f32>,
	pub distance: f32,
}*/

#[derive(Debug, Clone)]
pub struct BoundingBox2 {
	pub min: Vector2<f32>,
	pub max: Vector2<f32>,
}

impl BoundingBox2 {
	#[inline]
	pub fn new(min: Vector2<f32>, max: Vector2<f32>) -> BoundingBox2 {
		assert!(min[0] <= max[0]);
		assert!(min[1] <= max[1]);

		BoundingBox2 { min, max }
	}

	#[inline]
	pub fn zero() -> BoundingBox2 {
		BoundingBox2 {
			min: Vector2::new(std::f32::INFINITY, std::f32::INFINITY),
			max: Vector2::new(std::f32::NEG_INFINITY, std::f32::NEG_INFINITY),
		}
	}

	/*pub fn from_radius(radius: f32) -> BoundingBox2 {
		BoundingBox2::new(Vector2::new(-radius, -radius), Vector2::new(radius, radius))
	}*/

	#[inline]
	pub fn from_extents(top: f32, bottom: f32, left: f32, right: f32) -> BoundingBox2 {
		BoundingBox2::new(Vector2::new(bottom, left), Vector2::new(top, right))
	}

	#[inline]
	pub fn add_point(&mut self, point: Vector2<f32>) {
		for i in 0..2 {
			self.min[i] = f32::min(self.min[i], point[i]);
			self.max[i] = f32::max(self.max[i], point[i]);
		}
	}

	#[inline]
	pub fn offset(&self, offset: Vector2<f32>) -> BoundingBox2 {
		BoundingBox2 {
			min: self.min + offset,
			max: self.max + offset,
		}
	}

	#[inline]
	pub fn overlaps(&self, other: &BoundingBox2) -> bool {
		self.min[0] <= other.max[0]
			&& self.max[0] >= other.min[0]
			&& self.min[1] <= other.max[1]
			&& self.max[1] >= other.min[1]
	}

	#[inline]
	pub fn union(&self, other: &BoundingBox2) -> BoundingBox2 {
		BoundingBox2 {
			min: Vector2::new(
				f32::min(self.min[0], other.min[0]),
				f32::min(self.min[1], other.min[1]),
			),
			max: Vector2::new(
				f32::max(self.max[0], other.max[0]),
				f32::max(self.max[1], other.max[1]),
			),
		}
	}
}

impl From<&BoundingBox3> for BoundingBox2 {
	#[inline]
	fn from(bbox: &BoundingBox3) -> BoundingBox2 {
		BoundingBox2::new(
			Vector2::new(bbox.min[0], bbox.min[1]),
			Vector2::new(bbox.max[0], bbox.max[1]),
		)
	}
}

#[derive(Debug, Clone)]
pub struct BoundingBox3 {
	pub min: Vector3<f32>,
	pub max: Vector3<f32>,
}

impl BoundingBox3 {
	#[inline]
	pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> BoundingBox3 {
		assert!(min[0] <= max[0]);
		assert!(min[1] <= max[1]);
		assert!(min[2] <= max[2]);

		BoundingBox3 { min, max }
	}

	#[inline]
	pub fn from_radius_height(radius: f32, height: f32) -> BoundingBox3 {
		BoundingBox3::new(
			Vector3::new(-radius, -radius, 0.0),
			Vector3::new(radius, radius, height),
		)
	}

	/*pub fn zero() -> BoundingBox3 {
		BoundingBox3::new(Vector3::zeros(), Vector3::zeros())
	}*/
}

/*impl From<&BoundingBox2> for BoundingBox3 {
	fn from(bounding_box: &BoundingBox2) -> BoundingBox3 {
		BoundingBox3::new(
			Vector3::new(bounding_box.min[0], bounding_box.min[1], NEG_INFINITY),
			Vector3::new(bounding_box.max[0], bounding_box.max[1], INFINITY),
		)
	}
}*/

// Represented internally as BAM
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Angle(pub i32);

const MAX_AS_F64: f64 = 0x1_0000_0000u64 as f64;

impl Angle {
	#[inline]
	pub fn from_units(units: f64) -> Angle {
		Angle((units.rem_euclid(1.0) * MAX_AS_F64) as u32 as i32)
	}

	#[inline]
	pub fn from_degrees(degrees: f64) -> Angle {
		Angle::from_units(degrees * (1.0 / 360.0))
	}

	#[inline]
	pub fn from_radians(radians: f64) -> Angle {
		Angle::from_units(radians * 0.5 * std::f64::consts::FRAC_1_PI)
	}

	#[inline]
	pub fn to_units(&self) -> f64 {
		self.0 as f64 / MAX_AS_F64
	}

	#[inline]
	pub fn to_degrees(&self) -> f64 {
		self.to_units() * 360.0
	}

	#[inline]
	pub fn to_radians(&self) -> f64 {
		self.to_units() * 2.0 * std::f64::consts::PI
	}

	#[inline]
	pub fn to_units_unsigned(&self) -> f64 {
		self.0 as u32 as f64 / MAX_AS_F64
	}

	#[inline]
	pub fn sin(self) -> f64 {
		self.to_radians().sin()
	}

	#[inline]
	pub fn cos(self) -> f64 {
		self.to_radians().cos()
	}

	#[allow(dead_code)]
	#[inline]
	pub fn tan(self) -> f64 {
		self.to_radians().tan()
	}
}

impl std::fmt::Display for Angle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}°", self.to_degrees())
	}
}

impl From<i32> for Angle {
	fn from(val: i32) -> Self {
		Angle(val)
	}
}

impl std::ops::Add for Angle {
	type Output = Self;

	#[inline]
	fn add(self, other: Self) -> Self {
		Self(self.0.wrapping_add(other.0))
	}
}

impl std::ops::AddAssign for Angle {
	#[inline]
	fn add_assign(&mut self, other: Self) {
		*self = *self + other
	}
}

impl std::ops::Add<i32> for Angle {
	type Output = Self;

	#[inline]
	fn add(self, other: i32) -> Self {
		Self(self.0.wrapping_add(other))
	}
}

impl std::ops::AddAssign<i32> for Angle {
	#[inline]
	fn add_assign(&mut self, other: i32) {
		*self = *self + other
	}
}

impl std::ops::Neg for Angle {
	type Output = Self;

	#[inline]
	fn neg(self) -> Self {
		Self(self.0.wrapping_neg())
	}
}

impl std::ops::Sub for Angle {
	type Output = Self;

	#[inline]
	fn sub(self, other: Self) -> Self {
		Self(self.0.wrapping_sub(other.0))
	}
}

impl std::ops::SubAssign for Angle {
	#[inline]
	fn sub_assign(&mut self, other: Self) {
		*self = *self - other
	}
}

impl std::ops::Sub<i32> for Angle {
	type Output = Self;

	#[inline]
	fn sub(self, other: i32) -> Self {
		Self(self.0.wrapping_sub(other))
	}
}

impl std::ops::SubAssign<i32> for Angle {
	#[inline]
	fn sub_assign(&mut self, other: i32) {
		*self = *self - other
	}
}

pub fn angles_to_axes(angles: Vector3<Angle>) -> [Vector3<f32>; 3] {
	let sin = angles.map(Angle::sin);
	let cos = angles.map(Angle::cos);

	[
		Vector3::new(
			(cos[1] * cos[2]) as f32,
			(cos[1] * sin[2]) as f32,
			-sin[1] as f32,
		),
		Vector3::new(
			(sin[0] * sin[1] * cos[2] + cos[0] * -sin[2]) as f32,
			(sin[0] * sin[1] * sin[2] + cos[0] * cos[2]) as f32,
			(sin[0] * cos[1]) as f32,
		),
		Vector3::new(
			(cos[0] * sin[1] * cos[2] - sin[0] * -sin[2]) as f32,
			(cos[0] * sin[1] * sin[2] - sin[0] * cos[2]) as f32,
			(cos[2] * cos[1]) as f32,
		),
	]
}
