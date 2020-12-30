//! Various types and functions used for geometry calculations.

use nalgebra::{
	allocator::Allocator, storage::Owned, DefaultAllocator, DimName, Matrix4, Vector2, Vector3,
	VectorN, U2, U3,
};
use num_traits::identities::Zero;
use serde::{Deserialize, Serialize};

/// A line segment between two points, represented as a start point and a direction vector.
///
/// [`Line2`] and [`Line3`] are type aliases for lines in 2D and 3D space, respectively.
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

/// A [`Line`] in 2D space.
pub type Line2 = Line<U2>;
/// A [`Line`] in 3D space.
pub type Line3 = Line<U3>;

impl<D> Line<D>
where
	D: DimName,
	DefaultAllocator: Allocator<f32, D>,
	Owned<f32, D>: Copy,
{
	/// Constructs a new `Line` from a start point and a direction vector.
	#[inline]
	pub fn new(point: VectorN<f32, D>, dir: VectorN<f32, D>) -> Line<D> {
		Line { point, dir }
	}

	/// Returns a new `Line`, with the direction vector negated and start and end points swapped.
	#[inline]
	pub fn inverse(&self) -> Line<D> {
		Line {
			point: self.point + self.dir,
			dir: -self.dir,
		}
	}

	/// Returns the end point of the line segment, `self.point` + `self.dir`.
	#[inline]
	pub fn end_point(&self) -> VectorN<f32, D> {
		self.point + self.dir
	}
}

impl Line2 {
	/// Calculates the intersection between two 2D `Line`s.
	/// Returns `Some` if there is an intersection, `None` otherwise.
	///
	/// If an intersection is found, a tuple of `f32` from 0.0 to 1.0 inclusive is returned,
	/// giving, for `self` and `other` respectively, the position of the intersection point as the
	/// fraction of the total length from the starting point.
	///
	/// The intersection point is therefore equivalently
	/// * `self.point + tuple.0 * self.dir`
	/// * `other.point + tuple.1 * other.dir`
	#[inline]
	pub fn point_side(&self, point: Vector2<f32>) -> Option<Side> {
		let diff = point - self.point;
		let left = self.dir[1] * diff[0];
		let right = self.dir[0] * diff[1];

		if left > right {
			Some(Side::Right)
		} else if left < right {
			Some(Side::Left)
		} else {
			None
		}
	}

	#[inline]
	pub fn intersects(&self, other: &Line2) -> bool {
		!(self.point_side(other.point) == self.point_side(other.point + other.dir)
			|| other.point_side(self.point) == other.point_side(self.point + self.dir))
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
}

/// Constructs a 2D `Line` from a 3D `Line` by discarding the third coordinate.
impl From<Line3> for Line2 {
	#[inline]
	fn from(line: Line3) -> Line2 {
		Line2::new(line.point.fixed_resize(0.0), line.dir.fixed_resize(0.0))
	}
}

/// An infinite plane, represented as a normal vector and the shortest distance from the origin
/// (along the normal).
///
/// [`Plane2`] and [`Plane3`] are type aliases for planes in 2D and 3D space, respectively.
#[derive(Clone, Copy, Debug)]
pub struct Plane<D>
where
	D: DimName,
	DefaultAllocator: Allocator<f32, D>,
	Owned<f32, D>: Copy,
{
	pub normal: VectorN<f32, D>,
	pub distance: f32,
}

/// A [`Plane`] in 2D space; that is, an infinite line.
pub type Plane2 = Plane<U2>;
/// A [`Plane`] in 3D space.
pub type Plane3 = Plane<U3>;

impl<D> Plane<D>
where
	D: DimName,
	DefaultAllocator: Allocator<f32, D>,
	Owned<f32, D>: Copy,
{
	/// Constructs a new `Plane` from a normal vector and the shortest distance from the origin.
	#[inline]
	pub fn new(normal: VectorN<f32, D>, distance: f32) -> Plane<D> {
		assert!(!normal.is_zero());
		Plane { normal, distance }
	}

	/// Returns a new `Plane` with the normal and distance negated.
	#[inline]
	pub fn inverse(&self) -> Plane<D> {
		Plane {
			normal: -self.normal,
			distance: -self.distance,
		}
	}
}

/// The side of a line or plane that something is on.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Side {
	Right = 0,
	Left = 1,
}

impl From<bool> for Side {
	#[inline]
	fn from(value: bool) -> Side {
		if value {
			Side::Left
		} else {
			Side::Right
		}
	}
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

/// An interval between two points on the real numbers line.
/// * If `min == max`, it represents a single point.
/// * If `min > max`, it represents an empty interval, containing no points.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Interval {
	pub min: f32,
	pub max: f32,
}

impl Interval {
	/// Constructs a new `Interval` from a `min` and `max` value.
	#[inline]
	pub fn new(min: f32, max: f32) -> Interval {
		Interval { min, max }
	}

	/// Constructs a new empty `Interval`,
	/// with `min` and `max` set to positive and negative infinity respectively.
	#[inline]
	pub fn empty() -> Interval {
		Interval {
			min: std::f32::INFINITY,
			max: std::f32::NEG_INFINITY,
		}
	}

	/// Constructs an `Interval` from a single point, with `min` and `max` set to that point.
	#[inline]
	pub fn from_point(point: f32) -> Interval {
		Interval {
			min: point,
			max: point,
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

	/// Returns whether the interval is empty.
	#[inline]
	pub fn is_empty(self) -> bool {
		self.min > self.max
	}

	/// Returns whether the interval is a single point.
	#[inline]
	pub fn is_point(self) -> bool {
		self.min == self.max
	}

	/// Returns whether the interval is proper; that is, contains more than one point.
	#[inline]
	pub fn is_proper(self) -> bool {
		self.min >= self.max
	}

	/// Returns the length of the interval, `max - min`. Negative if the interval is empty.
	#[inline]
	pub fn len(self) -> f32 {
		self.max - self.min
	}

	/// Returns the point halfway between `min` and `max`.
	#[inline]
	pub fn middle(self) -> f32 {
		0.5 * (self.min + self.max)
	}

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

	/// Returns a new `Interval`,
	/// expanded to cover the given point if it is not within the interval already.
	#[inline]
	pub fn add_point(self, point: f32) -> Interval {
		Interval {
			min: f32::min(self.min, point),
			max: f32::max(self.max, point),
		}
	}

	/// Returns a new `Interval` with `value` added to both `min` and `max`.
	#[inline]
	pub fn offset(self, value: f32) -> Interval {
		Interval {
			min: self.min + value,
			max: self.max + value,
		}
	}

	/// Returns a new `Interval`, with `value` added to `min` if it is less than 0,
	/// to `max` if it is greater than 0.
	#[inline]
	pub fn extend(self, value: f32) -> Interval {
		if value < 0.0 {
			Interval {
				min: self.min + value,
				max: self.max,
			}
		} else {
			Interval {
				min: self.min,
				max: self.max + value,
			}
		}
	}

	/*#[inline]
	pub fn contains(&self, value: f32) -> bool {
		self.min <= value && self.max >= value
	}*/

	/// Returns a number representing the direction and distance from the point to the nearest
	/// edge of the interval.
	/// The return value is positive if the interval is above the point,
	/// negative if the interval is below, 0.0 if the point is inside the interval.
	#[inline]
	pub fn direction_from(self, point: f32) -> f32 {
		if point < self.min {
			self.min - point
		} else if point > self.max {
			self.max - point
		} else {
			0.0
		}
	}

	/// Returns the union of `self` with `other`,
	/// representing the points appearing in at least one of the intervals.
	#[inline]
	pub fn union(self, other: Interval) -> Interval {
		Interval {
			min: f32::min(self.min, other.min),
			max: f32::max(self.max, other.max),
		}
	}

	/// Returns the intersection of `self` with `other`,
	/// representing the points both intervals have in common.
	#[inline]
	pub fn intersection(self, other: Interval) -> Interval {
		Interval {
			min: f32::max(self.min, other.min),
			max: f32::min(self.max, other.max),
		}
	}

	/// Returns whether `self` has any overlap with `other`.
	/// Equivalent to `!self.intersection(other).is_empty()`.
	#[inline]
	pub fn overlaps(self, other: Interval) -> bool {
		self.min <= other.max && self.max >= other.min
	}
}

impl std::fmt::Display for Interval {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}, {}]", self.min, self.max)
	}
}

/// An axis-aligned bounding box.
///
/// This is represented internally by a vector of `Interval` for each axis,
/// so many of the methods on `AABB` map straightforwardly to an equivalent on `Interval`.
///
/// [`AABB2`] and [`AABB3`] are type aliases for bounding boxes in 2D and 3D space, respectively.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AABB<D>(VectorN<Interval, D>)
where
	D: DimName,
	DefaultAllocator: Allocator<Interval, D>,
	Owned<Interval, D>: Copy;

/// An [`AABB`] in 2D space.
pub type AABB2 = AABB<U2>;
/// An [`AABB`] in 3D space.
pub type AABB3 = AABB<U3>;

impl<D> AABB<D>
where
	D: DimName,
	DefaultAllocator: Allocator<Interval, D>,
	Owned<Interval, D>: Copy,
{
	/// Constructs a new empty `AABB`, with an empty interval for each axis.
	#[inline]
	pub fn empty() -> AABB<D> {
		AABB(VectorN::repeat(Interval::empty()))
	}

	/// Constructs an `AABB` from a single point,
	/// with `min` and `max` set to that point along each axis.
	#[inline]
	pub fn from_point(point: VectorN<f32, D>) -> AABB<D>
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		AABB(point.map(Interval::from_point))
	}

	/// Constructs an `AABB` directly from a vector of intervals.
	#[inline]
	pub fn from_intervals(intervals: VectorN<Interval, D>) -> AABB<D>
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		AABB(intervals)
	}

	/// Constructs an `AABB` from the minimum and maximum extent of the bounding box.
	#[inline]
	pub fn from_minmax(min: VectorN<f32, D>, max: VectorN<f32, D>) -> AABB<D>
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		AABB(min.zip_map(&max, |min, max| Interval { min, max }))
	}

	/// Returns whether the bounding box has an empty interval along at least one axis.
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.iter().copied().any(Interval::is_empty)
	}

	/// Returns whether the bounding box has a pointlike interval along at least one axis.
	/// The other axes can have any value.
	#[inline]
	pub fn is_point(&self) -> bool {
		self.0.iter().copied().any(Interval::is_point)
	}

	/// Returns a vector containing the `min` value of each axis.
	#[inline]
	pub fn min(&self) -> VectorN<f32, D>
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		self.0.map(|i| i.min)
	}

	/// Returns a vector containing the `max` value of each axis.
	#[inline]
	pub fn max(&self) -> VectorN<f32, D>
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		self.0.map(|i| i.max)
	}

	/// Returns the point containing the middle value of each axis, as given by [`Interval::middle()`].
	#[inline]
	pub fn middle(&self) -> VectorN<f32, D>
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		self.0.map(|i| i.middle())
	}

	/// Returns a reference to the internal vector of `Interval`s.
	#[inline]
	pub fn vector(&self) -> &VectorN<Interval, D>
	where
		DefaultAllocator: Allocator<Interval, D>,
		Owned<Interval, D>: Copy,
	{
		&self.0
	}

	/// Returns a new `AABB`,
	/// expanded to cover the given point if it is not within the box already.
	#[inline]
	pub fn add_point(&mut self, point: VectorN<f32, D>)
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		self.0.zip_apply(&point, |s, p| s.add_point(p));
	}

	/// Returns a new `AABB` with each axis of `offset` added to both `min` and `max` on the
	/// corresponding axis of the bounding box.
	#[inline]
	pub fn offset(&self, offset: VectorN<f32, D>) -> AABB<D>
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		AABB(self.0.zip_map(&offset, |s, o| s.offset(o)))
	}

	/// Returns a new `AABB` with each axis of `offset` added to `min` or `max` of the
	/// corresponding axis of the bounding box, depending on the sign.
	#[inline]
	pub fn extend(&self, offset: VectorN<f32, D>) -> AABB<D>
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		AABB(self.0.zip_map(&offset, |s, o| s.extend(o)))
	}

	/// Returns a vector representing the direction and distance from the point to the nearest
	/// edge of the bounding box.
	/// Each axis of the return value is positive if the interval on that axis is above the point,
	/// negative if the interval is below, 0.0 if the point is inside the interval.
	#[inline]
	pub fn direction_from(&self, point: VectorN<f32, D>) -> VectorN<f32, D>
	where
		DefaultAllocator: Allocator<f32, D>,
		Owned<f32, D>: Copy,
	{
		self.0.zip_map(&point, |i, p| i.direction_from(p))
	}

	/// Returns the union of `self` with `other`,
	/// representing the points appearing in at least one of the bounding boxes.
	#[inline]
	#[allow(dead_code)]
	pub fn union(&self, other: &AABB<D>) -> AABB<D> {
		AABB(self.0.zip_map(&other.0, |s, o| s.union(o)))
	}

	/// Returns the intersection of `self` with `other`,
	/// representing the points both bounding boxes have in common.
	#[inline]
	#[allow(dead_code)]
	pub fn intersection(&self, other: &AABB<D>) -> AABB<D> {
		AABB(self.0.zip_map(&other.0, |s, o| s.intersection(o)))
	}

	/// Returns whether `self` has any overlap with `other`.
	/// Equivalent to `!self.intersection(other).is_empty()`.
	#[inline]
	pub fn overlaps(&self, other: &AABB<D>) -> bool {
		(0..D::dim()).all(|i| self.0[i].overlaps(other.0[i]))
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

impl<D> std::fmt::Display for AABB<D>
where
	D: DimName,
	DefaultAllocator: Allocator<Interval, D>,
	Owned<Interval, D>: Copy,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[")?;
		for dim in self.0.iter() {
			write!(f, "{}", dim)?;
		}
		write!(f, "]")?;

		Ok(())
	}
}

impl AABB2 {
	/// Constructs a new 2D `AABB` containing all points within the given radius.
	pub fn from_radius(radius: f32) -> AABB2 {
		AABB(Vector2::new(
			Interval::new(-radius, radius),
			Interval::new(-radius, radius),
		))
	}

	/// Constructs a new 2D `AABB` from four edge points.
	#[inline]
	pub fn from_extents(top: f32, bottom: f32, left: f32, right: f32) -> AABB2 {
		AABB(Vector2::new(
			Interval::new(left, right),
			Interval::new(bottom, top),
		))
	}

	/// Returns an array of four 3D [`Plane`]s that enclose the bounding box.
	/// The normal of each plane points to the outside.
	#[inline]
	pub fn planes(&self) -> [Plane3; 4] {
		[
			Plane3::new(Vector3::new(-1.0, 0.0, 0.0), -self[0].min),
			Plane3::new(Vector3::new(1.0, 0.0, 0.0), self[0].max),
			Plane3::new(Vector3::new(0.0, -1.0, 0.0), -self[1].min),
			Plane3::new(Vector3::new(0.0, 1.0, 0.0), self[1].max),
		]
	}
}

/// Constructs a 2D `AABB` from a 3D `AABB` by discarding the third axis.
impl From<AABB3> for AABB2 {
	#[inline]
	fn from(bbox: AABB3) -> AABB2 {
		AABB(Vector2::new(bbox.0[0], bbox.0[1]))
	}
}

impl AABB3 {
	/// Constructs a new 2D `AABB` containing all points within the cylinder,
	/// with given radius along the first two axes. The third axis ranges from 0.0 to `height`.
	#[inline]
	pub fn from_radius_height(radius: f32, height: f32) -> AABB3 {
		AABB(Vector3::new(
			Interval::new(-radius, radius),
			Interval::new(-radius, radius),
			Interval::new(0.0, height),
		))
	}

	/// Returns an array of six 3D [`Plane`]s that enclose the bounding box.
	/// The normal of each plane points to the outside.
	#[inline]
	pub fn planes(&self) -> [Plane3; 6] {
		[
			Plane3::new(Vector3::new(-1.0, 0.0, 0.0), -self[0].min),
			Plane3::new(Vector3::new(1.0, 0.0, 0.0), self[0].max),
			Plane3::new(Vector3::new(0.0, -1.0, 0.0), -self[1].min),
			Plane3::new(Vector3::new(0.0, 1.0, 0.0), self[1].max),
			Plane3::new(Vector3::new(0.0, 0.0, -1.0), -self[2].min),
			Plane3::new(Vector3::new(0.0, 0.0, 1.0), self[2].max),
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

/// An angle of orientation, represented internally as an `i32` using
/// [Binary Anglular Measurement](https://en.wikipedia.org/wiki/Binary_scaling#Binary_angles) (BAM).
///
/// The maximum representable range is [-180°, 180°) or [-π, π),
/// addition, subtraction and negation will wrap around if they overflow.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Angle(pub i32);

const MAX_AS_F64: f64 = 0x1_0000_0000u64 as f64;

impl Angle {
	/// Constructs an `Angle` from units, where 1.0 represents a full rotation.
	#[inline]
	pub fn from_units(units: f64) -> Angle {
		Angle((units.rem_euclid(1.0) * MAX_AS_F64) as u32 as i32)
	}

	/// Constructs an `Angle` from degrees.
	#[inline]
	pub fn from_degrees(degrees: f64) -> Angle {
		Angle::from_units(degrees * (1.0 / 360.0))
	}

	/// Constructs an `Angle` from radians.
	#[inline]
	pub fn from_radians(radians: f64) -> Angle {
		Angle::from_units(radians * 0.5 * std::f64::consts::FRAC_1_PI)
	}

	/// Converts to units in the range [-0.5, 0.5).
	#[inline]
	pub fn to_units(self) -> f64 {
		self.0 as f64 / MAX_AS_F64
	}

	/// Converts to units in the range [0.0, 1.0).
	#[inline]
	pub fn to_units_unsigned(self) -> f64 {
		self.0 as u32 as f64 / MAX_AS_F64
	}

	/// Converts to degrees in the range [-180.0, 180.0).
	#[inline]
	pub fn to_degrees(self) -> f64 {
		self.to_units() * 360.0
	}

	/// Converts to radians in the range [-PI, PI).
	#[inline]
	pub fn to_radians(self) -> f64 {
		self.to_units() * 2.0 * std::f64::consts::PI
	}

	/// Computes the sine of the angle.
	#[inline]
	pub fn sin(self) -> f64 {
		self.to_radians().sin()
	}

	/// Computes the cosine of the angle.
	#[inline]
	pub fn cos(self) -> f64 {
		self.to_radians().cos()
	}

	/// Computes the tangent of the angle.
	#[allow(dead_code)]
	#[inline]
	pub fn tan(self) -> f64 {
		self.to_radians().tan()
	}
}

impl Zero for Angle {
	fn zero() -> Self {
		Self::default()
	}

	fn is_zero(&self) -> bool {
		self.0.is_zero()
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

impl std::ops::Mul<f64> for Angle {
	type Output = Self;

	#[inline]
	fn mul(self, other: f64) -> Self {
		Self((self.0 as f64 * other) as i32)
	}
}

impl std::ops::MulAssign<f64> for Angle {
	#[inline]
	fn mul_assign(&mut self, other: f64) {
		*self = *self * other
	}
}

/// Converts a system of rotations to three unit vectors.
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

/// Returns a projection matrix that creates a right-handed world coordinate system where,
/// for input vectors:
/// * Index 0 represents the camera forward axis
/// * Index 1 represents the camera left axis
/// * Index 2 represents the camera up axis
#[rustfmt::skip]
pub fn perspective_matrix(fovx: f32, aspect: f32, depth_range: Interval) -> Matrix4<f32> {
	let fovx = fovx.to_radians();
	let near = depth_range.min;
	let far = depth_range.max;
	let nmf = near - far;
	let f = 1.0 / (fovx * 0.5).tan();

	Matrix4::new(
		0.0       , -f , 0.0        , 0.0               ,
		0.0       , 0.0, -f * aspect, 0.0               ,
		-far / nmf, 0.0, 0.0        , (near * far) / nmf,
		1.0       , 0.0, 0.0        , 0.0               ,
	)
}

/// Constructs an orthographic matrix with bounding planes from the given bounding box.
#[rustfmt::skip]
pub fn ortho_matrix(bbox: AABB3) -> Matrix4<f32> {
	let rml = bbox[0].max - bbox[0].min;
	let tmb = bbox[1].max - bbox[1].min;
	let fmn = bbox[2].max - bbox[2].min;

	Matrix4::new(
		2.0 / rml, 0.0      , 0.0      , -(bbox[0].min + bbox[0].max) / rml,
		0.0      , 2.0 / tmb, 0.0      , -(bbox[1].min + bbox[1].max) / tmb,
		0.0      , 0.0      , 1.0 / fmn, -bbox[2].min / fmn                ,
		0.0      , 0.0      , 0.0      , 1.0                               ,
	)
}
