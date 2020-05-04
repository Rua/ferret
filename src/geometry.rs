use nalgebra::{
	allocator::Allocator, storage::Owned, DefaultAllocator, DimName, Vector2, Vector3, VectorN, U2,
	U3,
};

#[derive(Debug, Clone, Copy)]
pub struct Line<D>
where
	D: DimName,
	DefaultAllocator: Allocator<f32, D>,
	Owned<f32, D>: Copy,
{
	pub point: VectorN<f32, D>,
	pub dir: VectorN<f32, D>,
}

pub type Line2 = Line<U2>;
pub type Line3 = Line<U3>;

impl<D> Line<D>
where
	D: DimName,
	DefaultAllocator: Allocator<f32, D>,
	Owned<f32, D>: Copy,
{
	#[inline]
	pub fn new(point: VectorN<f32, D>, dir: VectorN<f32, D>) -> Line<D> {
		assert_ne!(dir, VectorN::zeros());
		Line { point, dir }
	}

	#[inline]
	pub fn inverse(&self) -> Line<D> {
		Line {
			point: self.point + self.dir,
			dir: -self.dir,
		}
	}
}

impl Line2 {
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

#[derive(Clone, Copy, Debug)]
pub struct Plane<D>
where
	D: DimName,
	DefaultAllocator: Allocator<f32, D>,
	Owned<f32, D>: Copy,
{
	pub distance: f32,
	pub normal: VectorN<f32, D>,
}

pub type Plane2 = Plane<U2>;
pub type Plane3 = Plane<U3>;

impl<D> Plane<D>
where
	D: DimName,
	DefaultAllocator: Allocator<f32, D>,
	Owned<f32, D>: Copy,
{
	#[inline]
	pub fn new(distance: f32, normal: VectorN<f32, D>) -> Plane<D> {
		assert_ne!(normal, VectorN::zeros());
		Plane { distance, normal }
	}

	#[inline]
	pub fn inverse(&self) -> Plane<D> {
		Plane {
			distance: -self.distance,
			normal: -self.normal,
		}
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
	pub min: f32,
	pub max: f32,
}

impl Interval {
	#[inline]
	pub fn new(min: f32, max: f32) -> Interval {
		Interval { min, max }
	}

	#[inline]
	pub fn empty() -> Interval {
		Interval {
			min: std::f32::INFINITY,
			max: std::f32::NEG_INFINITY,
		}
	}

	/*#[inline]
	pub fn full() -> Interval {
		Interval {
			min: std::f32::NEG_INFINITY,
			max: std::f32::INFINITY,
		}
	}*/

	/*#[inline]
	pub fn from_iterator(iter: impl IntoIterator<Item = f32>) -> Interval {
		let mut ret = Interval::empty();

		for value in iter.into_iter() {
			ret = ret.add(value);
		}

		ret
	}*/

	#[inline]
	pub fn len(self) -> f32 {
		self.max - self.min
	}

	#[inline]
	pub fn is_empty(self) -> bool {
		self.min > self.max
	}

	#[inline]
	pub fn is_empty_or_point(self) -> bool {
		self.min >= self.max
	}

	/*#[inline]
	pub fn is_inside(self, other: Interval) -> bool {
		self.min >= other.min && self.max <= other.max
	}*/

	/*#[inline]
	pub fn normalize(self) -> Interval {
		if self.is_empty() {
			Interval {
				min: self.max,
				max: self.min,
			}
		} else {
			self
		}
	}*/

	#[inline]
	pub fn add(self, value: f32) -> Interval {
		Interval {
			min: f32::min(self.min, value),
			max: f32::max(self.max, value),
		}
	}

	/*#[inline]
	pub fn contains(&self, value: f32) -> bool {
		self.min <= value && self.max >= value
	}*/

	#[inline]
	pub fn intersection(self, other: Interval) -> Interval {
		Interval {
			min: f32::max(self.min, other.min),
			max: f32::min(self.max, other.max),
		}
	}

	#[inline]
	pub fn offset(self, value: f32) -> Interval {
		Interval {
			min: self.min + value,
			max: self.max + value,
		}
	}

	#[inline]
	pub fn overlaps(self, other: Interval) -> bool {
		self.min <= other.max && self.max >= other.min
	}

	#[inline]
	pub fn union(self, other: Interval) -> Interval {
		Interval {
			min: f32::min(self.min, other.min),
			max: f32::max(self.max, other.max),
		}
	}
}

impl std::fmt::Display for Interval {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}, {}]", self.min, self.max)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct AABB<D>(VectorN<Interval, D>)
where
	D: DimName,
	DefaultAllocator: Allocator<Interval, D>,
	Owned<Interval, D>: Copy;

pub type AABB2 = AABB<U2>;
pub type AABB3 = AABB<U3>;

impl<D> AABB<D>
where
	D: DimName,
	DefaultAllocator: Allocator<Interval, D>,
	Owned<Interval, D>: Copy,
{
	#[inline]
	/*pub fn new(min: VectorN<f32, D>, max: VectorN<f32, D>) -> AABB<D> {
		assert!((0..D::dim()).all(|i| min[i] <= max[i]));
		AABB { min, max }
	}*/
	#[inline]
	pub fn empty() -> AABB<D> {
		AABB(VectorN::repeat(Interval::empty()))
	}

	#[inline]
	pub fn add_point(&mut self, point: VectorN<f32, D>)
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		self.0.zip_apply(&point, |s, p| s.add(p));
	}

	#[inline]
	pub fn max(&self) -> VectorN<f32, D>
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		self.0.map(|i| i.max)
	}

	#[inline]
	pub fn min(&self) -> VectorN<f32, D>
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		self.0.map(|i| i.min)
	}

	#[inline]
	pub fn vector(&self) -> &VectorN<Interval, D>
	where
		DefaultAllocator: Allocator<Interval, D>,
		Owned<Interval, D>: Copy,
	{
		&self.0
	}

	#[inline]
	pub fn offset(&self, offset: VectorN<f32, D>) -> AABB<D>
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		AABB(self.0.zip_map(&offset, |s, o| s.offset(o)))
	}

	#[inline]
	pub fn overlaps(&self, other: &AABB<D>) -> bool {
		(0..D::dim()).all(|i| self.0[i].overlaps(other.0[i]))
	}

	#[inline]
	pub fn union(&self, other: &AABB<D>) -> AABB<D> {
		AABB(self.0.zip_map(&other.0, |s, o| s.union(o)))
	}
}

impl<D> std::ops::Index<usize> for AABB<D>
where
	D: DimName,
	DefaultAllocator: Allocator<Interval, D>,
	Owned<Interval, D>: Copy,
{
	type Output = Interval;

	#[inline]
	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

impl AABB2 {
	/*pub fn from_radius(radius: f32) -> AABB2 {
		AABB(Vector2::new(
			Interval::new(-radius, radius),
			Interval::new(-radius, radius),
		))
	}*/

	#[inline]
	pub fn from_extents(top: f32, bottom: f32, left: f32, right: f32) -> AABB2 {
		AABB(Vector2::new(
			Interval::new(left, right),
			Interval::new(bottom, top),
		))
	}

	#[inline]
	pub fn planes(&self) -> [Plane3; 4] {
		[
			Plane3::new(-self[0].min, Vector3::new(-1.0, 0.0, 0.0)),
			Plane3::new(self[0].max, Vector3::new(1.0, 0.0, 0.0)),
			Plane3::new(-self[1].min, Vector3::new(0.0, -1.0, 0.0)),
			Plane3::new(self[1].max, Vector3::new(0.0, 1.0, 0.0)),
		]
	}
}

impl From<&AABB3> for AABB2 {
	#[inline]
	fn from(bbox: &AABB3) -> AABB2 {
		AABB(Vector2::new(bbox.0[0], bbox.0[1]))
	}
}

impl AABB3 {
	#[inline]
	pub fn from_radius_height(radius: f32, height: f32) -> AABB3 {
		AABB(Vector3::new(
			Interval::new(-radius, radius),
			Interval::new(-radius, radius),
			Interval::new(0.0, height),
		))
	}

	#[inline]
	pub fn planes(&self) -> [Plane3; 6] {
		[
			Plane3::new(-self[0].min, Vector3::new(-1.0, 0.0, 0.0)),
			Plane3::new(self[0].max, Vector3::new(1.0, 0.0, 0.0)),
			Plane3::new(-self[1].min, Vector3::new(0.0, -1.0, 0.0)),
			Plane3::new(self[1].max, Vector3::new(0.0, 1.0, 0.0)),
			Plane3::new(-self[2].min, Vector3::new(0.0, 0.0, -1.0)),
			Plane3::new(self[2].max, Vector3::new(0.0, 0.0, 1.0)),
		]
	}
}

/*impl From<&AABB2> for AABB3 {
	fn from(bounding_box: &AABB2) -> AABB3 {
		AABB3::new(
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
	pub fn to_units(self) -> f64 {
		self.0 as f64 / MAX_AS_F64
	}

	#[inline]
	pub fn to_degrees(self) -> f64 {
		self.to_units() * 360.0
	}

	#[inline]
	pub fn to_radians(self) -> f64 {
		self.to_units() * 2.0 * std::f64::consts::PI
	}

	#[inline]
	pub fn to_units_unsigned(self) -> f64 {
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
