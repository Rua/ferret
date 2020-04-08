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
			let bbox = AABB3::from_radius_height(box_collider.radius, box_collider.height);

			let (new_position, new_velocity) = movement_xy(
				*delta,
				*&map,
				&map_dynamic,
				&transform_component,
				&box_collider_component,
				&bbox,
				transform_component.get(entity).unwrap().position,
				velocity.velocity,
			);
			let transform = transform_component.get_mut(entity).unwrap();
			transform.position = new_position;
			velocity.velocity = new_velocity;

			let (new_position, new_velocity) = movement_z(
				*delta,
				*&map,
				&map_dynamic,
				&bbox,
				transform.position,
				velocity.velocity,
			);

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

fn movement_xy(
	delta: Duration,
	map: &Map,
	map_dynamic: &MapDynamic,
	transform_component: &WriteStorage<Transform>,
	box_collider_component: &ReadStorage<BoxCollider>,
	entity_bbox: &AABB3,
	mut position: Vector3<f32>,
	mut velocity: Vector3<f32>,
) -> (Vector3<f32>, Vector3<f32>) {
	if velocity[0] == 0.0 && velocity[1] == 0.0 {
		return (position, velocity);
	}

	let time_left = delta;

	{
		let mut move_step = Line3::new(position, velocity * time_left.as_secs_f32());
		move_step.dir[2] = 0.0;

		if let Some(intersect) = trace(
			&move_step,
			&entity_bbox,
			map,
			map_dynamic,
			transform_component,
			box_collider_component,
		) {
			// Push back against the collision
			let change = intersect.normal * velocity.dot(&intersect.normal) * 1.01;
			velocity -= change;

			// Try another move
			let mut move_step = Line3::new(position, velocity * time_left.as_secs_f32());
			move_step.dir[2] = 0.0;

			if let Some(_intersect) = trace(
				&move_step,
				&entity_bbox,
				map,
				map_dynamic,
				transform_component,
				box_collider_component,
			) {
				velocity = nalgebra::zero();
			} else {
				position += move_step.dir;
			}
		} else {
			position += move_step.dir;
		}
	}

	(position, velocity)
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
	let move_step2 = Line2::from(move_step);
	let current_bbox = AABB2::from(entity_bbox).offset(move_step2.point);
	let move_bbox = current_bbox.union(&current_bbox.offset(move_step2.dir));

	let bbox_corners = [
		Vector2::new(current_bbox[0].min, current_bbox[1].min),
		Vector2::new(current_bbox[0].min, current_bbox[1].max),
		Vector2::new(current_bbox[0].max, current_bbox[1].max),
		Vector2::new(current_bbox[0].max, current_bbox[1].min),
	];

	let mut ret: Option<Intersect> = None;

	for linedef in map.linedefs.iter() {
		if !move_bbox.overlaps(&linedef.bbox) {
			continue;
		}

		if let Some(intersect) = intersect_linedef(&move_step2, &bbox_corners, linedef) {
			if intersect.fraction >= ret.as_ref().map_or(1.0, |x| x.fraction) {
				continue;
			}

			if let [Some(front_sidedef), Some(back_sidedef)] = &linedef.sidedefs {
				let front_sector_dynamic = &map_dynamic.sectors[front_sidedef.sector_index];
				let back_sector_dynamic = &map_dynamic.sectors[back_sidedef.sector_index];

				if !(front_sector_dynamic.floor_height <= move_step.point[2] + entity_bbox[2].min
					&& back_sector_dynamic.floor_height <= move_step.point[2] + entity_bbox[2].min
					&& front_sector_dynamic.ceiling_height
						>= move_step.point[2] + entity_bbox[2].max
					&& back_sector_dynamic.ceiling_height
						>= move_step.point[2] + entity_bbox[2].max)
				{
					ret = Some(intersect);
				}
			} else {
				ret = Some(intersect);
			}
		}
	}

	for (transform, box_collider) in (transform_component, box_collider_component).join() {
		let position = Vector2::new(transform.position[0], transform.position[1]);

		// Don't collide against self
		if position == move_step2.point {
			continue;
		}

		let bbox = AABB2::from_radius(box_collider.radius).offset(position);
		let intervals = Vector2::from_iterator((0..2).map(|i| {
			Interval::new(
				(bbox[i].min - current_bbox[i].max) / move_step.dir[i],
				(bbox[i].max - current_bbox[i].min) / move_step.dir[i],
			)
			.normalize()
		}));

		let intersection = intervals[0].intersection(intervals[1]);

		if !intersection.is_empty()
			&& intersection.min >= 0.0
			&& intersection.min < ret.as_ref().map_or(1.0, |x| x.fraction)
		{
			ret = Some(Intersect {
				fraction: intersection.min,
				normal: BBOX_NORMALS[if intersection.min == intervals[0].min {
					if move_step.dir[0] > 0.0 {
						2
					} else {
						0
					}
				} else {
					if move_step.dir[1] > 0.0 {
						3
					} else {
						1
					}
				}],
			});
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

fn intersect_linedef(
	move_step: &Line2,
	bbox_corners: &[Vector2<f32>; 4],
	linedef: &Linedef,
) -> Option<Intersect> {
	let mut ret: Option<Intersect> = None;

	for i in 0..4 {
		// Intersect bbox corner with linedef
		if let Some((fraction, linedef_fraction)) =
			Line2::new(bbox_corners[i], move_step.dir).intersect(&linedef.line)
		{
			if fraction >= 0.0
				&& fraction < ret.as_ref().map_or(1.0, |x| x.fraction)
				&& linedef_fraction >= 0.0
				&& linedef_fraction <= 1.0
			{
				ret = Some(Intersect {
					fraction,
					normal: if move_step.dir.dot(&linedef.normal) > 0.0 {
						// Flip the normal if we're on the left side of the linedef
						Vector3::new(-linedef.normal[0], -linedef.normal[1], 0.0)
					} else {
						Vector3::new(linedef.normal[0], linedef.normal[1], 0.0)
					},
				});
			}
		}

		// Intersect linedef vertices with bbox edge
		let bbox_edge = Line2::new(bbox_corners[i], bbox_corners[(i + 1) % 4] - bbox_corners[i]);
		let linedef_vertices = [linedef.line.point, linedef.line.point + linedef.line.dir];

		for vertex in &linedef_vertices {
			if let Some((fraction, edge_fraction)) =
				Line2::new(*vertex, -move_step.dir).intersect(&bbox_edge)
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

	ret
}

fn movement_z(
	delta: Duration,
	map: &Map,
	map_dynamic: &MapDynamic,
	entity_bbox: &AABB3,
	mut position: Vector3<f32>,
	mut velocity: Vector3<f32>,
) -> (Vector3<f32>, Vector3<f32>) {
	if velocity[2] == 0.0 {
		return (position, velocity);
	}

	let move_step = Line3::new(
		position,
		Vector3::new(0.0, 0.0, velocity[2] * delta.as_secs_f32()),
	);
	let position_bbox = entity_bbox.offset(move_step.point);

	for (i, sector) in map.sectors.iter().enumerate() {
		if let Some(intersect) = trace_sector(
			&move_step,
			&position_bbox,
			sector,
			&map_dynamic.sectors[i],
			&map.subsectors,
		) {
			velocity[2] = 0.0;
			return (position, velocity);
		}
	}

	position += move_step.dir;

	(position, velocity)
}

fn trace_sector(
	move_step: &Line3,
	position_bbox: &AABB3,
	sector: &Sector,
	sector_dynamic: &SectorDynamic,
	subsectors: &[GLSSect],
) -> Option<Intersect> {
	let intersect = if move_step.dir[2] > 0.0 {
		Intersect {
			fraction: (sector_dynamic.ceiling_height - position_bbox[2].max) / move_step.dir[2],
			normal: Vector3::new(0.0, 0.0, -1.0),
		}
	} else {
		Intersect {
			fraction: (sector_dynamic.floor_height - position_bbox[2].min) / move_step.dir[2],
			normal: Vector3::new(0.0, 0.0, 1.0),
		}
	};

	if intersect.fraction < 0.0 || intersect.fraction > 1.0 {
		return None;
	}

	let position_bbox2 = AABB2::from(position_bbox);
	let bbox_corners = [
		Vector2::new(position_bbox2[0].min, position_bbox2[1].min),
		Vector2::new(position_bbox2[0].min, position_bbox2[1].max),
		Vector2::new(position_bbox2[0].max, position_bbox2[1].max),
		Vector2::new(position_bbox2[0].max, position_bbox2[1].min),
	];

	// Separating axis theorem
	for subsector in sector.subsectors.iter().map(|i| &subsectors[*i]) {
		if !position_bbox2.overlaps(&subsector.bbox) {
			continue;
		}

		if subsector.segs.iter().all(|seg| {
			Interval::from_iterator(bbox_corners.iter().map(|c| seg.normal.dot(c)))
				.overlaps(seg.interval)
		}) {
			// All axes had overlap, so the subsector as a whole does overlap
			return Some(intersect);
		}
	}

	// No overlapping subsectors were found
	None
}
