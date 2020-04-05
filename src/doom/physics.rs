use crate::{
	assets::AssetStorage,
	doom::{
		components::{Transform, Velocity},
		map::{Linedef, Map, MapDynamic},
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
	bbox: &AABB3,
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
			&bbox,
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
				&bbox,
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
		Vector2::new(current_bbox.min[0], current_bbox.min[1]),
		Vector2::new(current_bbox.min[0], current_bbox.max[1]),
		Vector2::new(current_bbox.max[0], current_bbox.max[1]),
		Vector2::new(current_bbox.max[0], current_bbox.min[1]),
	];

	let mut ret: Option<Intersect> = None;

	for linedef in map.linedefs.iter() {
		if let Some(intersect) = intersect_linedef(&move_step2, &move_bbox, &bbox_corners, linedef)
		{
			if intersect.fraction < ret.as_ref().map_or(1.0, |x| x.fraction) {
				if let [Some(front_sidedef), Some(back_sidedef)] = &linedef.sidedefs {
					let front_sector_dynamic = &map_dynamic.sectors[front_sidedef.sector_index];
					let back_sector_dynamic = &map_dynamic.sectors[back_sidedef.sector_index];

					if !(front_sector_dynamic.floor_height
						<= move_step.point[2] + entity_bbox.min[2]
						&& back_sector_dynamic.floor_height
							<= move_step.point[2] + entity_bbox.min[2]
						&& front_sector_dynamic.ceiling_height
							>= move_step.point[2] + entity_bbox.max[2]
						&& back_sector_dynamic.ceiling_height
							>= move_step.point[2] + entity_bbox.max[2])
					{
						ret = Some(intersect);
					}
				} else {
					ret = Some(intersect);
				}
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
				(bbox.min[i] - current_bbox.max[i]) / move_step.dir[i],
				(bbox.max[i] - current_bbox.min[i]) / move_step.dir[i],
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
	move_bbox: &AABB2,
	bbox_corners: &[Vector2<f32>; 4],
	linedef: &Linedef,
) -> Option<Intersect> {
	if !move_bbox.overlaps(&linedef.bbox) {
		return None;
	}

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
	bbox: &AABB3,
	mut position: Vector3<f32>,
	mut velocity: Vector3<f32>,
) -> (Vector3<f32>, Vector3<f32>) {
	if velocity[2] == 0.0 {
		return (position, velocity);
	}

	let ssect = map.find_subsector(Vector2::new(position[0], position[1]));
	let sector_dynamic = &map_dynamic.sectors[ssect.sector_index];

	let mut min = sector_dynamic.floor_height;
	let mut max = sector_dynamic.ceiling_height;
	let bbox2 = (&bbox.offset(position)).into();

	for linedef in map.linedefs.iter() {
		if linedef.touches_bbox(&bbox2) {
			if let [Some(front_sidedef), Some(back_sidedef)] = &linedef.sidedefs {
				let front_sector_dynamic = &map_dynamic.sectors[front_sidedef.sector_index];
				let back_sector_dynamic = &map_dynamic.sectors[back_sidedef.sector_index];

				min = f32::max(min, front_sector_dynamic.floor_height);
				min = f32::max(min, back_sector_dynamic.floor_height);
				max = f32::min(max, front_sector_dynamic.ceiling_height);
				max = f32::min(max, back_sector_dynamic.ceiling_height);
			}
		}
	}

	position[2] += velocity[2] * delta.as_secs_f32();

	if position[2] <= min - bbox.min[2] {
		position[2] = min - bbox.min[2];

		if velocity[2] < 0.0 {
			velocity[2] = 0.0;
		}
	} else if position[2] >= max - bbox.max[2] {
		position[2] = max - bbox.max[2];

		if velocity[2] > 0.0 {
			velocity[2] = 0.0;
		}
	}

	(position, velocity)
}
