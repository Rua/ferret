use crate::{
	assets::AssetStorage,
	doom::{
		components::{Transform, Velocity},
		map::{GLSSect, Map, MapDynamic},
	},
	geometry::{Interval, AABB2, AABB3},
};
use arrayvec::ArrayVec;
use bitflags::bitflags;
use lazy_static::lazy_static;
use nalgebra::{Vector2, Vector3};
use specs::{
	Component, DenseVecStorage, Entities, Join, ReadExpect, ReadStorage, RunNow, World,
	WriteStorage,
};
use specs_derive::Component;
use std::time::Duration;

#[derive(Default)]
pub struct PhysicsSystem;

impl<'a> RunNow<'a> for PhysicsSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (
			entities,
			delta,
			map_storage,
			box_collider_component,
			map_dynamic_component,
			mut transform_component,
			mut velocity_component,
		) = world.system_data::<(
			Entities,
			ReadExpect<Duration>,
			ReadExpect<AssetStorage<Map>>,
			ReadStorage<BoxCollider>,
			ReadStorage<MapDynamic>,
			WriteStorage<Transform>,
			WriteStorage<Velocity>,
		)>();

		let map_dynamic = map_dynamic_component.join().next().unwrap();
		let map = map_storage.get(&map_dynamic.map).unwrap();

		// Clone the mask so that transform_component is free to be borrowed during the loop
		let transform_mask = transform_component.mask().clone();

		for (entity, box_collider, _, velocity) in (
			&entities,
			&box_collider_component,
			transform_mask,
			&mut velocity_component,
		)
			.join()
		{
			let tracer = EntityTracer {
				map,
				map_dynamic,
				transform_component: &transform_component,
				box_collider_component: &box_collider_component,
			};

			let entity_bbox = AABB3::from_radius_height(box_collider.radius, box_collider.height);
			let mut new_position = transform_component.get(entity).unwrap().position;
			let mut new_velocity = velocity.velocity;

			if new_velocity == Vector3::zeros() {
				continue;
			}

			let time_left = *delta;

			for _ in 0..4 {
				let move_step = new_velocity * time_left.as_secs_f32();

				// TODO: variable solid mask
				if let Some(intersect) = tracer.trace(
					&entity_bbox.offset(new_position),
					move_step,
					SolidMask::NON_MONSTER,
				) {
					new_position += move_step * intersect.fraction;

					// Push back against the collision
					let change = intersect.normal * new_velocity.dot(&intersect.normal) * 1.01;
					new_velocity -= change;

					// Avoid bouncing too much
					if new_velocity.dot(&velocity.velocity) <= 0.0 {
						new_velocity = Vector3::zeros();
						break;
					}
				} else {
					new_position += move_step;
					break;
				}
			}

			if let None = tracer.trace(
				&entity_bbox.offset(new_position),
				Vector3::new(0.0, 0.0, -0.25),
				SolidMask::NON_MONSTER, // TODO solid mask
			) {
				// Entity isn't on ground, apply gravity
				const GRAVITY: f32 = 1.0 * crate::doom::FRAME_RATE * crate::doom::FRAME_RATE;
				new_velocity[2] -= GRAVITY * delta.as_secs_f32();
			}

			let transform = transform_component.get_mut(entity).unwrap();
			transform.position = new_position;
			velocity.velocity = new_velocity;
		}
	}
}

#[derive(Clone, Component, Copy, Debug)]
pub struct BoxCollider {
	pub height: f32,
	pub radius: f32,
	pub solid_mask: SolidMask,
}

bitflags! {
	pub struct SolidMask: u16 {
		const NON_MONSTER = 0b01;
		const MONSTER = 0b10;
	}
}

#[derive(Clone, Debug)]
pub struct Intersect {
	fraction: f32,
	normal: Vector3<f32>,
}

pub struct EntityTracer<'a> {
	pub map: &'a Map,
	pub map_dynamic: &'a MapDynamic,
	pub transform_component: &'a WriteStorage<'a, Transform>,
	pub box_collider_component: &'a ReadStorage<'a, BoxCollider>,
}

const DISTANCE_EPSILON: f32 = 0.03125;

impl<'a> EntityTracer<'a> {
	pub fn trace(
		&self,
		entity_bbox: &AABB3,
		move_step: Vector3<f32>,
		entity_solid_mask: SolidMask,
	) -> Option<Intersect> {
		let mut ret: Option<Intersect> = None;
		let move_bbox = entity_bbox.union(&entity_bbox.offset(move_step));
		let move_bbox2 = AABB2::from(&move_bbox);

		for linedef in self
			.map
			.linedefs
			.iter()
			.filter(|l| move_bbox2.overlaps(&l.bbox))
		{
			let along = Vector2::new(-linedef.normal[1], linedef.normal[0]);
			let planes = ArrayVec::from([
				Plane {
					distance: linedef.line.point.dot(&linedef.normal),
					normal: Vector3::new(linedef.normal[0], linedef.normal[1], 0.0),
					collides: true,
				},
				Plane {
					distance: -linedef.line.point.dot(&linedef.normal),
					normal: Vector3::new(-linedef.normal[0], -linedef.normal[1], 0.0),
					collides: true,
				},
				Plane {
					distance: -linedef.line.point.dot(&along),
					normal: Vector3::new(-along[0], -along[1], 0.0),
					collides: false,
				},
				Plane {
					distance: (linedef.line.point + linedef.line.dir).dot(&along),
					normal: Vector3::new(along[0], along[1], 0.0),
					collides: false,
				},
			]);

			if let [Some(front_sidedef), Some(back_sidedef)] = &linedef.sidedefs {
				let front_interval = &self.map_dynamic.sectors[front_sidedef.sector_index].interval;
				let back_interval = &self.map_dynamic.sectors[back_sidedef.sector_index].interval;

				let intersection = front_interval.intersection(*back_interval);
				let union = front_interval.union(*back_interval);
				let intervals = ArrayVec::from([
					(Interval::new(union.min, intersection.min), SolidMask::all()),
					(
						Interval::new(intersection.min, intersection.max),
						linedef.solid_mask,
					),
					(Interval::new(intersection.max, union.max), SolidMask::all()),
				]);

				for (interval, solid_mask) in intervals.into_iter() {
					if interval.is_empty() {
						continue;
					}

					let z_planes = ArrayVec::from([
						Plane {
							distance: -interval.min,
							normal: Vector3::new(0.0, 0.0, -1.0),
							collides: false,
						},
						Plane {
							distance: interval.max,
							normal: Vector3::new(0.0, 0.0, 1.0),
							collides: false,
						},
					]);

					let iter = planes.iter().cloned().chain(z_planes.into_iter());

					if let Some((fraction, normal)) = trace_planes(&entity_bbox, move_step, iter) {
						if fraction < ret.as_ref().map_or(1.0, |x| x.fraction)
							&& entity_solid_mask.intersects(solid_mask)
						{
							ret = Some(Intersect { fraction, normal });
						}
					}
				}
			} else if let [Some(front_sidedef), None] = &linedef.sidedefs {
				let front_interval = &self.map_dynamic.sectors[front_sidedef.sector_index].interval;
				let z_planes = ArrayVec::from([
					Plane {
						distance: -front_interval.min,
						normal: Vector3::new(0.0, 0.0, -1.0),
						collides: false,
					},
					Plane {
						distance: front_interval.max,
						normal: Vector3::new(0.0, 0.0, 1.0),
						collides: false,
					},
				]);

				let iter = planes.into_iter().chain(z_planes.into_iter());

				if let Some((fraction, normal)) = trace_planes(&entity_bbox, move_step, iter) {
					if fraction < ret.as_ref().map_or(1.0, |x| x.fraction) {
						ret = Some(Intersect { fraction, normal });
					}
				}
			}
		}

		for (sector_index, sector) in self.map.sectors.iter().enumerate() {
			let sector_dynamic = &self.map_dynamic.sectors[sector_index];

			for (distance, normal) in ArrayVec::from([
				(-sector_dynamic.interval.max, Vector3::new(0.0, 0.0, -1.0)),
				(sector_dynamic.interval.min, Vector3::new(0.0, 0.0, 1.0)),
			])
			.into_iter()
			{
				for subsector in sector
					.subsectors
					.iter()
					.map(|i| &self.map.subsectors[*i])
					.filter(|s| move_bbox2.overlaps(&s.bbox))
				{
					let iter = subsector
						.segs
						.iter()
						.map(|seg| Plane {
							distance: seg.line.point.dot(&seg.normal),
							normal: Vector3::new(seg.normal[0], seg.normal[1], 0.0),
							collides: false,
						})
						.chain(Some(Plane {
							distance,
							normal,
							collides: true,
						}));

					if let Some((fraction, normal)) = trace_planes(&entity_bbox, move_step, iter) {
						if fraction < ret.as_ref().map_or(1.0, |x| x.fraction) {
							ret = Some(Intersect { fraction, normal });
						}
					}
				}
			}
		}

		for (transform, box_collider) in
			(self.transform_component, self.box_collider_component).join()
		{
			let other_bbox = AABB3::from_radius_height(box_collider.radius, box_collider.height)
				.offset(transform.position);

			// Don't collide against self
			if entity_bbox == &other_bbox {
				continue;
			}

			if !move_bbox.overlaps(&other_bbox) {
				continue;
			}

			let dirs = [(-other_bbox.min(), -1.0), (other_bbox.max(), 1.0)];
			let planes = dirs.iter().flat_map(|&(distance, n)| {
				(0..3).map(move |i| {
					let mut normal = Vector3::zeros();
					normal[i] = n;
					Plane {
						distance: distance[i],
						normal,
						collides: true,
					}
				})
			});

			if let Some((fraction, normal)) = trace_planes(&entity_bbox, move_step, planes) {
				if fraction < ret.as_ref().map_or(1.0, |x| x.fraction)
					&& entity_solid_mask.intersects(box_collider.solid_mask)
				{
					ret = Some(Intersect { fraction, normal });
				}
			}
		}

		ret
	}
}

pub struct SectorTracer<'a> {
	pub transform_component: &'a ReadStorage<'a, Transform>,
	pub box_collider_component: &'a ReadStorage<'a, BoxCollider>,
}

impl<'a> SectorTracer<'a> {
	pub fn trace<'b>(
		&self,
		distance: f32,
		normal: f32,
		move_step: f32,
		subsectors: impl Iterator<Item = &'b GLSSect> + Clone,
	) -> Option<f32> {
		let normal = Vector3::new(0.0, 0.0, normal);
		let move_step = Vector3::new(0.0, 0.0, -move_step);
		let mut ret: Option<f32> = None;

		for (transform, box_collider) in
			(self.transform_component, self.box_collider_component).join()
		{
			let entity_bbox = AABB3::from_radius_height(box_collider.radius, box_collider.height)
				.offset(transform.position);
			let entity_bbox2 = AABB2::from(&entity_bbox);

			for subsector in subsectors
				.clone()
				.filter(|s| entity_bbox2.overlaps(&s.bbox))
			{
				let iter = subsector
					.segs
					.iter()
					.map(|seg| Plane {
						distance: seg.line.point.dot(&seg.normal),
						normal: Vector3::new(seg.normal[0], seg.normal[1], 0.0),
						collides: false,
					})
					.chain(Some(Plane {
						distance,
						normal,
						collides: true,
					}));

				if let Some((fraction, _)) = trace_planes(&entity_bbox, move_step, iter) {
					if fraction < ret.unwrap_or(1.0) {
						ret = Some(fraction);
					}
				}
			}
		}

		ret
	}
}

#[derive(Clone, Debug)]
struct Plane {
	distance: f32,
	normal: Vector3<f32>,
	collides: bool,
}

fn trace_planes(
	entity_bbox: &AABB3,
	move_step: Vector3<f32>,
	planes: impl IntoIterator<Item = Plane>,
) -> Option<(f32, Vector3<f32>)> {
	let mut interval = Interval::new(-1.0, 1.0);
	let mut ret = None;

	for plane in planes.into_iter() {
		let closest_point =
			entity_bbox
				.vector()
				.zip_map(&plane.normal, |b, n| if n < 0.0 { b.max } else { b.min });
		let start_dist = closest_point.dot(&plane.normal) - plane.distance;
		let move_dist = move_step.dot(&plane.normal);

		if start_dist < 0.0 && start_dist + move_dist < 0.0 {
			continue;
		}

		if move_dist < 0.0 {
			let fraction = (start_dist - DISTANCE_EPSILON) / -move_dist;

			if fraction > interval.min {
				interval.min = fraction;

				if plane.collides {
					ret = Some((f32::max(0.0, interval.min), plane.normal));
				}
			}
		} else {
			if start_dist > 0.0 {
				return None;
			}

			let fraction = (start_dist + DISTANCE_EPSILON) / -move_dist;

			if fraction < interval.max {
				interval.max = fraction;
			}
		}
	}

	if !interval.is_empty() {
		ret
	} else {
		None
	}
}

lazy_static! {
	static ref BBOX_NORMALS: [Vector3<f32>; 4] = [
		Vector3::new(1.0, 0.0, 0.0),   // right
		Vector3::new(0.0, 1.0, 0.0),   // up
		Vector3::new(-1.0, 0.0, 0.0),  // left
		Vector3::new(0.0, -1.0, 0.0),  // down
	];
}
