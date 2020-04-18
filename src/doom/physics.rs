use crate::{
	assets::AssetStorage,
	doom::{
		components::{Transform, Velocity},
		map::{GLSSect, Map, MapDynamic},
	},
	geometry::{Interval, Line2, AABB2, AABB3},
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
			let move_tracer = MoveTracer {
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
				if let Some(intersect) = move_tracer.trace(
					&entity_bbox.offset(new_position),
					move_step,
					SolidMask::NON_MONSTER,
				) {
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
struct Intersect {
	fraction: f32,
	normal: Vector3<f32>,
	solid_mask: SolidMask,
}

struct MoveTracer<'a> {
	map: &'a Map,
	map_dynamic: &'a MapDynamic,
	transform_component: &'a WriteStorage<'a, Transform>,
	box_collider_component: &'a ReadStorage<'a, BoxCollider>,
}

const DISTANCE_EPSILON: f32 = 0.03125;

impl<'a> MoveTracer<'a> {
	fn trace(
		&self,
		entity_bbox: &AABB3,
		move_step: Vector3<f32>,
		solid_mask: SolidMask,
	) -> Option<Intersect> {
		let mut ret: Option<Intersect> = None;

		if move_step[0] != 0.0 || move_step[1] != 0.0 {
			for linedef_index in 0..self.map.linedefs.len() {
				if let Some(intersect) = self.trace_linedef(&entity_bbox, move_step, linedef_index)
				{
					if intersect.fraction < ret.as_ref().map_or(1.0, |x| x.fraction)
						&& solid_mask.intersects(intersect.solid_mask)
					{
						ret = Some(intersect);
					}
				}
			}
		}

		for sector_index in 0..self.map.sectors.len() {
			let sector = &self.map.sectors[sector_index];
			let sector_dynamic = &self.map_dynamic.sectors[sector_index];

			for subsector in sector.subsectors.iter().map(|i| &self.map.subsectors[*i]) {
				for (distance, normal) in ArrayVec::from([
					(-sector_dynamic.interval.max, Vector3::new(0.0, 0.0, -1.0)),
					(sector_dynamic.interval.min, Vector3::new(0.0, 0.0, 1.0)),
				])
				.into_iter()
				{
					if let Some(intersect) =
						self.trace_subsector(&entity_bbox, move_step, subsector, distance, normal)
					{
						if intersect.fraction < ret.as_ref().map_or(1.0, |x| x.fraction)
							&& solid_mask.intersects(intersect.solid_mask)
						{
							ret = Some(intersect);
						}
					}
				}
			}
		}

		for (transform, box_collider) in
			(self.transform_component, self.box_collider_component).join()
		{
			if let Some(intersect) =
				self.trace_aabb(&entity_bbox, move_step, &box_collider, transform.position)
			{
				if intersect.fraction < ret.as_ref().map_or(1.0, |x| x.fraction)
					&& solid_mask.intersects(intersect.solid_mask)
				{
					ret = Some(intersect);
				}
			}
		}

		ret
	}

	fn trace_linedef(
		&self,
		entity_bbox: &AABB3,
		move_step: Vector3<f32>,
		linedef_index: usize,
	) -> Option<Intersect> {
		let linedef = &self.map.linedefs[linedef_index];

		let move_step2 = Vector2::new(move_step[0], move_step[1]);
		let entity_bbox2 = AABB2::from(entity_bbox);
		let move_bbox2 = entity_bbox2.union(&entity_bbox2.offset(move_step2));

		if !move_bbox2.overlaps(&linedef.bbox) {
			return None;
		}

		let entity_bbox_corners = [
			Vector2::new(entity_bbox2[0].min, entity_bbox2[1].min),
			Vector2::new(entity_bbox2[0].min, entity_bbox2[1].max),
			Vector2::new(entity_bbox2[0].max, entity_bbox2[1].max),
			Vector2::new(entity_bbox2[0].max, entity_bbox2[1].min),
		];

		let mut ret: Option<Intersect> = None;

		for i in 0..4 {
			// Intersect bbox corner with linedef
			if let Some((fraction, linedef_fraction)) =
				Line2::new(entity_bbox_corners[i], move_step2).intersect(&linedef.line)
			{
				if fraction >= 0.0
					&& fraction < ret.as_ref().map_or(1.0, |x| x.fraction)
					&& linedef_fraction >= 0.0
					&& linedef_fraction <= 1.0
				{
					ret = Some(Intersect {
						fraction,
						normal: if move_step2.dot(&linedef.normal) > 0.0 {
							// Flip the normal if we're on the left side of the linedef
							Vector3::new(-linedef.normal[0], -linedef.normal[1], 0.0)
						} else {
							Vector3::new(linedef.normal[0], linedef.normal[1], 0.0)
						},
						solid_mask: SolidMask::all(),
					});
				}
			}

			// Intersect linedef vertices with bbox edge
			let entity_bbox_edge = Line2::new(
				entity_bbox_corners[i],
				entity_bbox_corners[(i + 1) % 4] - entity_bbox_corners[i],
			);
			let linedef_vertices = [linedef.line.point, linedef.line.point + linedef.line.dir];

			for vertex in &linedef_vertices {
				if let Some((fraction, edge_fraction)) =
					Line2::new(*vertex, -move_step2).intersect(&entity_bbox_edge)
				{
					if fraction >= 0.0
						&& fraction < ret.as_ref().map_or(1.0, |x| x.fraction)
						&& edge_fraction >= 0.0 && edge_fraction <= 1.0
					{
						ret = Some(Intersect {
							fraction,
							normal: -BBOX_NORMALS[i],
							solid_mask: SolidMask::all(),
						});
					}
				}
			}
		}

		if let Some(ref mut intersect) = ret {
			if let [Some(front_sidedef), Some(back_sidedef)] = &linedef.sidedefs {
				let front_sector_dynamic = &self.map_dynamic.sectors[front_sidedef.sector_index];
				let back_sector_dynamic = &self.map_dynamic.sectors[back_sidedef.sector_index];
				let end_bbox = entity_bbox.offset(move_step * intersect.fraction);
				let interval = front_sector_dynamic
					.interval
					.intersection(back_sector_dynamic.interval);

				if end_bbox[2].is_inside(interval) {
					intersect.solid_mask = linedef.solid_mask;
				}
			}
		}

		ret
	}

	fn trace_subsector(
		&self,
		entity_bbox: &AABB3,
		move_step: Vector3<f32>,
		subsector: &GLSSect,
		distance: f32,
		normal: Vector3<f32>,
	) -> Option<Intersect> {
		let planes = subsector
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

		trace_planes(&entity_bbox, move_step, planes).map(|(fraction, normal)| Intersect {
			fraction,
			normal,
			solid_mask: SolidMask::all(),
		})
	}

	fn trace_aabb(
		&self,
		entity_bbox: &AABB3,
		move_step: Vector3<f32>,
		box_collider: &BoxCollider,
		other_position: Vector3<f32>,
	) -> Option<Intersect> {
		let other_bbox = AABB3::from_radius_height(box_collider.radius, box_collider.height)
			.offset(other_position);

		// Don't collide against self
		if entity_bbox == &other_bbox {
			return None;
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

		trace_planes(&entity_bbox, move_step, planes).map(|(fraction, normal)| Intersect {
			fraction,
			normal,
			solid_mask: box_collider.solid_mask,
		})
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
