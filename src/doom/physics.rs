use crate::{
	assets::AssetStorage,
	doom::{
		components::{Transform, Velocity},
		map::{GLSSect, Linedef, Map, MapDynamic, Sector, SectorDynamic},
	},
	geometry::{Interval, Line2, Line3, AABB2, AABB3},
};
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
			//transform.position += velocity.velocity * delta.as_secs_f32();
			let entity_bbox = AABB3::from_radius_height(box_collider.radius, box_collider.height);
			let mut new_position = transform_component.get(entity).unwrap().position;
			let mut new_velocity = velocity.velocity;

			if new_velocity == nalgebra::zero::<Vector3<f32>>() {
				continue;
			}

			let time_left = *delta;

			{
				let move_step = Line3::new(new_position, new_velocity * time_left.as_secs_f32());

				if let Some(intersect) = trace(
					&move_step,
					&entity_bbox,
					map,
					map_dynamic,
					&transform_component,
					&box_collider_component,
				) {
					// Push back against the collision
					let change = intersect.normal * new_velocity.dot(&intersect.normal) * 1.01;
					new_velocity -= change;

					// Try another move
					let move_step =
						Line3::new(new_position, new_velocity * time_left.as_secs_f32());

					if let Some(_intersect) = trace(
						&move_step,
						&entity_bbox,
						map,
						map_dynamic,
						&transform_component,
						&box_collider_component,
					) {
						new_velocity = nalgebra::zero();
					} else {
						new_position += move_step.dir;
					}
				} else {
					new_position += move_step.dir;
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
}

#[derive(Clone, Copy, Debug)]
struct Intersect {
	fraction: f32,
	normal: Vector3<f32>,
}

fn trace(
	move_step: &Line3,
	entity_bbox: &AABB3,
	map: &Map,
	map_dynamic: &MapDynamic,
	transform_component: &WriteStorage<Transform>,
	box_collider_component: &ReadStorage<BoxCollider>,
) -> Option<Intersect> {
	let mut ret: Option<Intersect> = None;

	for linedef in map.linedefs.iter() {
		if let Some(intersect) = trace_linedef(&move_step, &entity_bbox, linedef, &map_dynamic) {
			if intersect.fraction < ret.as_ref().map_or(1.0, |x| x.fraction) {
				ret = Some(intersect);
			}
		}
	}

	for (i, sector) in map.sectors.iter().enumerate() {
		if let Some(intersect) = trace_sector(
			&move_step,
			&entity_bbox,
			sector,
			&map_dynamic.sectors[i],
			&map.subsectors,
		) {
			if intersect.fraction < ret.as_ref().map_or(1.0, |x| x.fraction) {
				ret = Some(intersect);
			}
		}
	}

	for (transform, box_collider) in (transform_component, box_collider_component).join() {
		if let Some(intersect) = trace_aabb(
			&move_step,
			&entity_bbox,
			transform.position,
			&AABB3::from_radius_height(box_collider.radius, box_collider.height),
		) {
			if intersect.fraction < ret.as_ref().map_or(1.0, |x| x.fraction) {
				ret = Some(intersect);
			}
		}
	}

	ret
}

lazy_static! {
	static ref BBOX_NORMALS: [Vector3<f32>; 4] = [
		Vector3::new(1.0, 0.0, 0.0),   // right
		Vector3::new(0.0, 1.0, 0.0),   // up
		Vector3::new(-1.0, 0.0, 0.0),  // left
		Vector3::new(0.0, -1.0, 0.0),  // down
	];
}

fn trace_linedef(
	move_step: &Line3,
	entity_bbox: &AABB3,
	linedef: &Linedef,
	map_dynamic: &MapDynamic,
) -> Option<Intersect> {
	let move_step2 = Line2::from(move_step);
	let entity_start_bbox2 = AABB2::from(entity_bbox).offset(move_step2.point);
	let move_bbox2 = entity_start_bbox2.union(&entity_start_bbox2.offset(move_step2.dir));

	if !move_bbox2.overlaps(&linedef.bbox) {
		return None;
	}

	let entity_start_bbox_corners = [
		Vector2::new(entity_start_bbox2[0].min, entity_start_bbox2[1].min),
		Vector2::new(entity_start_bbox2[0].min, entity_start_bbox2[1].max),
		Vector2::new(entity_start_bbox2[0].max, entity_start_bbox2[1].max),
		Vector2::new(entity_start_bbox2[0].max, entity_start_bbox2[1].min),
	];

	let mut ret: Option<Intersect> = None;

	for i in 0..4 {
		// Intersect bbox corner with linedef
		if let Some((fraction, linedef_fraction)) =
			Line2::new(entity_start_bbox_corners[i], move_step2.dir).intersect(&linedef.line)
		{
			if fraction >= 0.0
				&& fraction < ret.as_ref().map_or(1.0, |x| x.fraction)
				&& linedef_fraction >= 0.0
				&& linedef_fraction <= 1.0
			{
				ret = Some(Intersect {
					fraction,
					normal: if move_step2.dir.dot(&linedef.normal) > 0.0 {
						// Flip the normal if we're on the left side of the linedef
						Vector3::new(-linedef.normal[0], -linedef.normal[1], 0.0)
					} else {
						Vector3::new(linedef.normal[0], linedef.normal[1], 0.0)
					},
				});
			}
		}

		// Intersect linedef vertices with bbox edge
		let entity_start_bbox_edge = Line2::new(
			entity_start_bbox_corners[i],
			entity_start_bbox_corners[(i + 1) % 4] - entity_start_bbox_corners[i],
		);
		let linedef_vertices = [linedef.line.point, linedef.line.point + linedef.line.dir];

		for vertex in &linedef_vertices {
			if let Some((fraction, edge_fraction)) =
				Line2::new(*vertex, -move_step2.dir).intersect(&entity_start_bbox_edge)
			{
				if fraction >= 0.0
					&& fraction < ret.as_ref().map_or(1.0, |x| x.fraction)
					&& edge_fraction >= 0.0
					&& edge_fraction <= 1.0
				{
					ret = Some(Intersect {
						fraction,
						normal: -BBOX_NORMALS[i],
					});
				}
			}
		}
	}

	if let Some(intersect) = ret {
		if let [Some(front_sidedef), Some(back_sidedef)] = &linedef.sidedefs {
			let front_sector_dynamic = &map_dynamic.sectors[front_sidedef.sector_index];
			let back_sector_dynamic = &map_dynamic.sectors[back_sidedef.sector_index];
			let end_position = move_step.point + move_step.dir * intersect.fraction;
			let end_bbox = entity_bbox.offset(end_position);
			let interval = front_sector_dynamic
				.interval
				.intersection(back_sector_dynamic.interval);

			if end_bbox[2].is_inside(interval) {
				return None;
			}
		}
	}

	ret
}

fn trace_sector(
	move_step: &Line3,
	entity_bbox: &AABB3,
	sector: &Sector,
	sector_dynamic: &SectorDynamic,
	subsectors: &[GLSSect],
) -> Option<Intersect> {
	let entity_start_bbox = entity_bbox.offset(move_step.point);
	let intersect = if move_step.dir[2] > 0.0 {
		Intersect {
			fraction: (sector_dynamic.interval.max - entity_start_bbox[2].max) / move_step.dir[2],
			normal: Vector3::new(0.0, 0.0, -1.0),
		}
	} else {
		Intersect {
			fraction: (sector_dynamic.interval.min - entity_start_bbox[2].min) / move_step.dir[2],
			normal: Vector3::new(0.0, 0.0, 1.0),
		}
	};

	if intersect.fraction < 0.0 || intersect.fraction > 1.0 {
		return None;
	}

	let entity_end_bbox2 =
		AABB2::from(&(entity_start_bbox.offset(move_step.dir * intersect.fraction)));
	let entity_end_bbox_corners = [
		Vector2::new(entity_end_bbox2[0].min, entity_end_bbox2[1].min),
		Vector2::new(entity_end_bbox2[0].min, entity_end_bbox2[1].max),
		Vector2::new(entity_end_bbox2[0].max, entity_end_bbox2[1].max),
		Vector2::new(entity_end_bbox2[0].max, entity_end_bbox2[1].min),
	];

	// Separating axis theorem
	for subsector in sector.subsectors.iter().map(|i| &subsectors[*i]) {
		if !entity_end_bbox2.overlaps(&subsector.bbox) {
			continue;
		}

		if subsector.segs.iter().all(|seg| {
			Interval::from_iterator(entity_end_bbox_corners.iter().map(|c| seg.normal.dot(c)))
				.overlaps(seg.interval)
		}) {
			// All axes had overlap, so the subsector as a whole does overlap
			return Some(intersect);
		}
	}

	// No overlapping subsectors were found
	None
}

fn trace_aabb(
	move_step: &Line3,
	entity_bbox: &AABB3,
	other_position: Vector3<f32>,
	other_bbox: &AABB3,
) -> Option<Intersect> {
	let entity_start_bbox = entity_bbox.offset(move_step.point);

	// Don't collide against self
	if other_position == move_step.point {
		return None;
	}

	let other_start_bbox = other_bbox.offset(other_position);
	let intervals = Vector3::from_iterator((0..3).map(|i| {
		Interval::new(
			// TODO: handle case where move_step.dir[i] == 0.0
			(other_start_bbox[i].min - entity_start_bbox[i].max) / move_step.dir[i],
			(other_start_bbox[i].max - entity_start_bbox[i].min) / move_step.dir[i],
		)
		.normalize()
	}));

	let intersection = intervals[0]
		.intersection(intervals[1])
		.intersection(intervals[2]);

	if intersection.is_empty() || intersection.min < 0.0 || intersection.min > 1.0 {
		return None;
	}

	Some(Intersect {
		fraction: intersection.min,
		// TODO: make less ugly/more generic
		normal: if intersection.min == intervals[0].min {
			if move_step.dir[0] > 0.0 {
				Vector3::new(-1.0, 0.0, 0.0)
			} else {
				Vector3::new(1.0, 0.0, 0.0)
			}
		} else if intersection.min == intervals[1].min {
			if move_step.dir[1] > 0.0 {
				Vector3::new(0.0, -1.0, 0.0)
			} else {
				Vector3::new(0.0, 1.0, 0.0)
			}
		} else {
			if move_step.dir[2] > 0.0 {
				Vector3::new(0.0, 0.0, -1.0)
			} else {
				Vector3::new(0.0, 0.0, 1.0)
			}
		},
	})
}
