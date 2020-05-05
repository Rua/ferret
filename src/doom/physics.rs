use crate::{
	assets::AssetStorage,
	doom::{
		components::{Transform, Velocity},
		data::{FRICTION, GRAVITY},
		map::{Map, MapDynamic, Subsector},
	},
	geometry::{Interval, Plane3, AABB2, AABB3},
};
use arrayvec::ArrayVec;
use bitflags::bitflags;
use lazy_static::lazy_static;
use nalgebra::Vector3;
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

			step_slide_move(
				&tracer,
				&mut new_position,
				&mut new_velocity,
				&entity_bbox,
				SolidMask::NON_MONSTER, // TODO solid mask
				*delta,
			);

			let trace = tracer.trace(
				&entity_bbox.offset(new_position),
				Vector3::new(0.0, 0.0, -0.25),
				SolidMask::NON_MONSTER, // TODO solid mask
			);

			if trace.collision.is_some() {
				// Entity is on ground, apply friction
				let factor = FRICTION.powf(delta.as_secs_f32());
				new_velocity[0] *= factor;
				new_velocity[1] *= factor;
			} else {
				// Entity isn't on ground, apply gravity
				new_velocity[2] -= GRAVITY * delta.as_secs_f32();
			}

			let transform = transform_component.get_mut(entity).unwrap();
			transform.position = new_position;
			velocity.velocity = new_velocity;
		}
	}
}

fn step_slide_move(
	tracer: &EntityTracer,
	position: &mut Vector3<f32>,
	velocity: &mut Vector3<f32>,
	entity_bbox: &AABB3,
	solid_mask: SolidMask,
	mut time_left: Duration,
) {
	let original_velocity = *velocity;

	// Slide-move
	for _ in 0..4 {
		let move_step = *velocity * time_left.as_secs_f32();
		let trace = tracer.trace(&entity_bbox.offset(*position), move_step, solid_mask);

		*position += trace.move_step;
		time_left = time_left
			.checked_sub(time_left.mul_f32(trace.fraction))
			.unwrap_or_default();

		if let Some(collision) = trace.collision {
			if let Some(step_z) = collision.step_z {
				// Try to step up
				let move_step = Vector3::new(0.0, 0.0, step_z - position[2]);

				if move_step[2] > 0.0 && move_step[2] < 24.5 {
					let trace = tracer.trace(&entity_bbox.offset(*position), move_step, solid_mask);

					if trace.collision.is_none() {
						*position += trace.move_step;
						continue;
					}
				}
			}

			// Push back against the collision
			*velocity -= collision.normal * velocity.dot(&collision.normal) * 1.01;

			// Avoid bouncing too much
			if velocity.dot(&original_velocity) <= 0.0 {
				*velocity = Vector3::zeros();
				break;
			}
		}

		if time_left == Duration::default() {
			break;
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
pub struct Trace {
	pub fraction: f32,
	pub move_step: Vector3<f32>,
	pub collision: Option<TraceCollision>,
}

#[derive(Clone, Debug)]
pub struct TraceCollision {
	pub normal: Vector3<f32>,
	pub step_z: Option<f32>,
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
	) -> Trace {
		let mut ret = Trace {
			fraction: 1.0,
			move_step,
			collision: None,
		};
		let move_bbox = entity_bbox.union(&entity_bbox.offset(move_step));
		let move_bbox2 = AABB2::from(&move_bbox);

		for linedef in self
			.map
			.linedefs
			.iter()
			.filter(|l| move_bbox2.overlaps(&l.bbox))
		{
			if let [Some(front_sidedef), Some(back_sidedef)] = &linedef.sidedefs {
				let front_interval = &self.map_dynamic.sectors[front_sidedef.sector_index].interval;
				let back_interval = &self.map_dynamic.sectors[back_sidedef.sector_index].interval;

				let intersection = front_interval.intersection(*back_interval);
				let union = front_interval.union(*back_interval);
				let intervals = ArrayVec::from([
					(
						Interval::new(union.min, intersection.min),
						SolidMask::all(),
						true,
					),
					(
						Interval::new(intersection.min, intersection.max),
						linedef.solid_mask,
						false,
					),
					(
						Interval::new(intersection.max, union.max),
						SolidMask::all(),
						false,
					),
				]);

				for (interval, solid_mask, step) in intervals.into_iter() {
					if interval.is_empty() {
						continue;
					}

					let z_planes = [
						Plane3::new(-interval.min, Vector3::new(0.0, 0.0, -1.0)),
						Plane3::new(interval.max, Vector3::new(0.0, 0.0, 1.0)),
					];
					let iter = linedef.planes.iter().chain(z_planes.iter());

					if let Some((fraction, normal)) = trace_planes(&entity_bbox, move_step, iter) {
						if fraction < ret.fraction && entity_solid_mask.intersects(solid_mask) {
							ret = Trace {
								fraction,
								move_step: move_step * fraction,
								collision: Some(TraceCollision {
									normal,
									step_z: if step
										&& !entity_solid_mask.intersects(linedef.solid_mask)
									{
										Some(interval.max + DISTANCE_EPSILON)
									} else {
										None
									},
								}),
							};
						}
					}
				}
			} else if let [Some(front_sidedef), None] = &linedef.sidedefs {
				let front_interval = &self.map_dynamic.sectors[front_sidedef.sector_index].interval;
				let z_planes = [
					Plane3::new(-front_interval.min, Vector3::new(0.0, 0.0, -1.0)),
					Plane3::new(front_interval.max, Vector3::new(0.0, 0.0, 1.0)),
				];
				let iter = linedef.planes.iter().chain(z_planes.iter());

				if let Some((fraction, normal)) = trace_planes(&entity_bbox, move_step, iter) {
					if fraction < ret.fraction {
						ret = Trace {
							fraction,
							move_step: move_step * fraction,
							collision: Some(TraceCollision {
								normal,
								step_z: None,
							}),
						};
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
				let z_planes = [
					Plane3::new(distance, normal),
					Plane3::new(-distance, -normal),
				];

				for subsector in sector
					.subsectors
					.iter()
					.map(|i| &self.map.subsectors[*i])
					.filter(|s| move_bbox2.overlaps(&s.bbox))
				{
					let iter = subsector.planes.iter().chain(z_planes.iter());

					if let Some((fraction, normal)) = trace_planes(&entity_bbox, move_step, iter) {
						if fraction < ret.fraction {
							ret = Trace {
								fraction,
								move_step: move_step * fraction,
								collision: Some(TraceCollision {
									normal,
									step_z: None,
								}),
							};
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

			if let Some((fraction, normal)) =
				trace_planes(&entity_bbox, move_step, &other_bbox.planes())
			{
				if fraction < ret.fraction && entity_solid_mask.intersects(box_collider.solid_mask)
				{
					ret = Trace {
						fraction,
						move_step: move_step * fraction,
						collision: Some(TraceCollision {
							normal,
							step_z: Some(other_bbox[2].max + DISTANCE_EPSILON),
						}),
					};
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
		subsectors: impl Iterator<Item = &'b Subsector> + Clone,
	) -> Trace {
		let normal = Vector3::new(0.0, 0.0, normal);
		let move_step = Vector3::new(0.0, 0.0, move_step);

		let z_planes = [
			Plane3::new(distance, normal),
			Plane3::new(-distance, -normal),
		];

		let mut ret = Trace {
			fraction: 1.0,
			move_step,
			collision: None,
		};

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
				let iter = subsector.planes.iter().chain(z_planes.iter());

				if let Some((fraction, _)) = trace_planes(&entity_bbox, -move_step, iter) {
					if fraction < ret.fraction {
						ret = Trace {
							fraction,
							move_step: move_step * fraction,
							collision: Some(TraceCollision {
								normal: -normal,
								step_z: None,
							}),
						};
					}
				}
			}
		}

		ret
	}
}

fn trace_planes<'a>(
	entity_bbox: &AABB3,
	move_step: Vector3<f32>,
	planes: impl IntoIterator<Item = &'a Plane3>,
) -> Option<(f32, Vector3<f32>)> {
	let mut interval = Interval::new(f32::NEG_INFINITY, 1.0);
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
				ret = Some((f32::max(0.0, interval.min), plane.normal));
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
