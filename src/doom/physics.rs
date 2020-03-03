use crate::{
	assets::AssetStorage,
	doom::{
		components::{BoxCollider, MapDynamic, Transform, Velocity},
		map::{Linedef, Map},
	},
	geometry::BoundingBox2,
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

	let position2 = Vector2::new(position[0], position[1]);
	let mov = Vector2::new(velocity[0], velocity[1]);
	let ptry = position2 + mov * delta.as_secs_f32();

	if is_position_valid(map, collider.radius, ptry) {
		position[0] = ptry[0];
		position[1] = ptry[1];
	} else {
		velocity[0] = 0.0;
		velocity[1] = 0.0;
	}
}

fn is_position_valid(map: &Map, radius: f32, target_pos: Vector2<f32>) -> bool {
	let bbox = BoundingBox2::from_extents(
		target_pos[0] + radius,
		target_pos[0] - radius,
		target_pos[1] - radius,
		target_pos[1] + radius,
	);

	for _linedef in map.linedefs.iter().filter(|l| l.intersects_bbox(&bbox)) {
		return false;
	}

	true
}
