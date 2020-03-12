use crate::{
	assets::AssetStorage,
	doom::{
		components::{BoxCollider, MapDynamic, Transform, Velocity},
		map::{Linedef, Map},
	},
	geometry::{BoundingBox2, Line},
};
use lazy_static::lazy_static;
use nalgebra::{Vector2, Vector3};
use specs::{Join, ReadExpect, ReadStorage, RunNow, World, WriteStorage};
use std::time::Duration;

#[derive(Default)]
pub struct PhysicsSystem;

impl<'a> RunNow<'a> for PhysicsSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (
			delta,
			map_storage,
			box_collider_component,
			map_dynamic_component,
			mut transform_component,
			mut velocity_component,
		) = world.system_data::<(
			ReadExpect<Duration>,
			ReadExpect<AssetStorage<Map>>,
			ReadStorage<BoxCollider>,
			ReadStorage<MapDynamic>,
			WriteStorage<Transform>,
			WriteStorage<Velocity>,
		)>();

		let map_dynamic = map_dynamic_component.join().next().unwrap();
		let map = map_storage.get(&map_dynamic.map).unwrap();

		for (box_collider, transform, velocity) in (
			&box_collider_component,
			&mut transform_component,
			&mut velocity_component,
		)
			.join()
		{
			//transform.position += velocity.velocity * delta.as_secs_f32();
			xy_movement(
				*delta,
				*&map,
				*&box_collider,
				&mut transform.position,
				&mut velocity.velocity,
			);
		}
	}
}

fn xy_movement(
	delta: Duration,
	map: &Map,
	collider: &BoxCollider,
	position: &mut Vector3<f32>,
	velocity: &mut Vector3<f32>,
) {
	lazy_static! {
		static ref MAXMOVE: f32 = 30.0 / crate::doom::FRAME_TIME.as_secs_f32();
	}

	if velocity[0] == 0.0 && velocity[1] == 0.0 {
		return;
	}

	let mut new_position = Vector2::new(position[0], position[1]);
	let mut new_velocity = Vector2::new(velocity[0], velocity[1]);
	let time_left = delta;

	if new_velocity[0] > *MAXMOVE {
		new_velocity[0] = *MAXMOVE;
	} else if new_velocity[0] < -*MAXMOVE {
		new_velocity[0] = -*MAXMOVE;
	}

	if new_velocity[1] > *MAXMOVE {
		new_velocity[1] = *MAXMOVE;
	} else if new_velocity[1] < -*MAXMOVE {
		new_velocity[1] = -*MAXMOVE;
	}

	let bbox = BoundingBox2::from_radius(collider.radius);

	{
		let move_step = Line::new(new_position, new_velocity * time_left.as_secs_f32());

		if let Some(intersect) = trace(&move_step, &bbox, map) {
			// Push back against the collision
			let change = intersect.normal * new_velocity.dot(&intersect.normal) * 1.01;
			new_velocity -= change;

			// Try another move
			let move_step = Line::new(new_position, new_velocity * time_left.as_secs_f32());

			if let Some(_intersect) = trace(&move_step, &bbox, map) {
				new_velocity = nalgebra::zero();
			} else {
				new_position += move_step.dir;
			}
		} else {
			new_position += move_step.dir;
		}
	}

	position[0] = new_position[0];
	position[1] = new_position[1];
	velocity[0] = new_velocity[0];
	velocity[1] = new_velocity[1];
}

#[derive(Clone, Copy, Debug)]
struct Intersect {
	fraction: f32,
	normal: Vector2<f32>,
}

fn trace(move_step: &Line, bbox: &BoundingBox2, map: &Map) -> Option<Intersect> {
	let mut ret: Option<Intersect> = None;

	for linedef in map.linedefs.iter() {
		if let Some(intersect) = intersect_linedef(move_step, bbox, linedef) {
			if intersect.fraction < ret.as_ref().map_or(1.0, |x| x.fraction) {
				ret = Some(intersect)
			}
		}
	}

	ret
}

fn intersect_linedef(
	move_step: &Line,
	bbox: &BoundingBox2,
	linedef: &Linedef,
) -> Option<Intersect> {
	let bbox_corners = [
		Vector2::new(bbox.min[0], bbox.min[1]) + move_step.point,
		Vector2::new(bbox.min[0], bbox.max[1]) + move_step.point,
		Vector2::new(bbox.max[0], bbox.max[1]) + move_step.point,
		Vector2::new(bbox.max[0], bbox.min[1]) + move_step.point,
	];

	let bbox_normals = [
		Vector2::new(-1.0, 0.0),
		Vector2::new(0.0, 1.0),
		Vector2::new(1.0, 0.0),
		Vector2::new(0.0, -1.0),
	];

	let mut ret: Option<Intersect> = None;

	for i in 0..4 {
		// Intersect bbox corner with linedef
		if let Some((fraction, linedef_fraction)) =
			Line::new(bbox_corners[i], move_step.dir).intersect(&linedef.line)
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
						-linedef.normal
					} else {
						linedef.normal
					},
				});
			}
		}

		// Intersect linedef vertices with bbox edge
		let bbox_edge = Line::new(bbox_corners[i], bbox_corners[(i + 1) % 4] - bbox_corners[i]);
		let linedef_vertices = [linedef.line.point, linedef.line.point + linedef.line.dir];

		for vertex in &linedef_vertices {
			if let Some((fraction, edge_fraction)) =
				Line::new(*vertex, -move_step.dir).intersect(&bbox_edge)
			{
				if fraction >= 0.0
					&& fraction < ret.as_ref().map_or(1.0, |x| x.fraction)
					&& edge_fraction >= 0.0
					&& edge_fraction <= 1.0
				{
					ret = Some(Intersect {
						fraction,
						normal: -bbox_normals[i],
					});
				}
			}
		}
	}

	ret
}
