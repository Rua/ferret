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
			XYMovement(
				*delta,
				*&map,
				*&box_collider,
				&mut transform.position,
				&mut velocity.velocity,
			);
		}
	}
}

fn XYMovement(
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

	if velocity[0] > *MAXMOVE {
		velocity[0] = *MAXMOVE;
	} else if velocity[0] < -*MAXMOVE {
		velocity[0] = -*MAXMOVE;
	}

	if velocity[1] > *MAXMOVE {
		velocity[1] = *MAXMOVE;
	} else if velocity[1] < -*MAXMOVE {
		velocity[1] = -*MAXMOVE;
	}

	let move_step = Vector2::new(velocity[0], velocity[1]) * delta.as_secs_f32();
	let bbox = BoundingBox2::from_extents(
		position[0] + collider.radius,
		position[0] - collider.radius,
		position[1] - collider.radius,
		position[1] + collider.radius,
	);

	if is_position_valid(map, &bbox, move_step) {
		position[0] += move_step[0];
		position[1] += move_step[1];
	} else {
		velocity[0] = 0.0;
		velocity[1] = 0.0;
	}
}

fn is_position_valid(map: &Map, bbox: &BoundingBox2, move_step: Vector2<f32>) -> bool {
	let bbox_corners = [
		Vector2::new(bbox.min[0], bbox.min[1]),
		Vector2::new(bbox.min[0], bbox.max[1]),
		Vector2::new(bbox.max[0], bbox.max[1]),
		Vector2::new(bbox.max[0], bbox.min[1]),
	];

	for linedef in map.linedefs.iter() {
		let linedef_vertices = [linedef.line.point, linedef.line.point + linedef.line.dir];

		for i in 0..4 {
			// Intersect bbox corner with linedef
			if let Some((trace_p, linedef_p)) =
				Line::new(bbox_corners[i], move_step).intersect(&linedef.line)
			{
				if trace_p >= 0.0 && trace_p <= 1.0 && linedef_p >= 0.0 && linedef_p <= 1.0 {
					return false;
				}
			}

			// Intersect linedef vertices with bbox edge
			let bbox_edge = Line::new(bbox_corners[i], bbox_corners[(i + 1) % 4] - bbox_corners[i]);

			for vertex in &linedef_vertices {
				if let Some((trace_p, edge_p)) =
					Line::new(*vertex, -move_step).intersect(&bbox_edge)
				{
					if trace_p >= 0.0 && trace_p <= 1.0 && edge_p >= 0.0 && edge_p <= 1.0 {
						return false;
					}
				}
			}
		}
	}

	true
}
