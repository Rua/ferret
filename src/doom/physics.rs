use crate::{
	assets::AssetStorage,
	doom::{
		components::{BoxCollider, MapDynamic, SectorDynamic, Transform, Velocity},
		map::{Linedef, Map},
	},
	geometry::{BoundingBox3, Line2, Line3},
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
			sector_dynamic_component,
			mut transform_component,
			mut velocity_component,
		) = world.system_data::<(
			ReadExpect<Duration>,
			ReadExpect<AssetStorage<Map>>,
			ReadStorage<BoxCollider>,
			ReadStorage<MapDynamic>,
			ReadStorage<SectorDynamic>,
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
				&map_dynamic,
				&sector_dynamic_component,
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
	map_dynamic: &MapDynamic,
	sector_dynamic_component: &ReadStorage<SectorDynamic>,
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

	let mut new_position = *position;
	let mut new_velocity = *velocity;
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

	let bbox = BoundingBox3::from_radius_height(collider.radius, collider.height);

	{
		let move_step = Line3::new(new_position, new_velocity * time_left.as_secs_f32());

		if let Some(intersect) = trace(
			&move_step,
			&bbox,
			map,
			map_dynamic,
			sector_dynamic_component,
		) {
			// Push back against the collision
			let change = intersect.normal * new_velocity.dot(&intersect.normal) * 1.01;
			new_velocity -= change;

			// Try another move
			let move_step = Line3::new(new_position, new_velocity * time_left.as_secs_f32());

			if let Some(_intersect) = trace(
				&move_step,
				&bbox,
				map,
				map_dynamic,
				sector_dynamic_component,
			) {
				new_velocity = nalgebra::zero();
			} else {
				new_position += move_step.dir;
			}
		} else {
			new_position += move_step.dir;
		}
	}

	*position = new_position;
	*velocity = new_velocity;
}

#[derive(Clone, Copy, Debug)]
struct Intersect {
	fraction: f32,
	normal: Vector3<f32>,
}

fn trace(
	move_step: &Line3,
	bbox: &BoundingBox3,
	map: &Map,
	map_dynamic: &MapDynamic,
	sector_dynamic_component: &ReadStorage<SectorDynamic>,
) -> Option<Intersect> {
	let mut ret: Option<Intersect> = None;

	for linedef in map.linedefs.iter() {
		if let Some(intersect) = intersect_linedef(move_step, bbox, linedef) {
			if intersect.fraction < ret.as_ref().map_or(1.0, |x| x.fraction) {
				if let [Some(front_sidedef), Some(back_sidedef)] = &linedef.sidedefs {
					let front_sector = sector_dynamic_component
						.get(map_dynamic.sectors[front_sidedef.sector_index])
						.unwrap();
					let back_sector = sector_dynamic_component
						.get(map_dynamic.sectors[back_sidedef.sector_index])
						.unwrap();

					if !(front_sector.floor_height <= move_step.point[2] + bbox.min[2]
						&& back_sector.floor_height <= move_step.point[2] + bbox.min[2]
						&& front_sector.ceiling_height >= move_step.point[2] + bbox.max[2]
						&& back_sector.ceiling_height >= move_step.point[2] + bbox.max[2])
					{
						ret = Some(intersect);
					}
				} else {
					ret = Some(intersect);
				}
			}
		}
	}

	ret
}

fn intersect_linedef(
	move_step: &Line3,
	bbox: &BoundingBox3,
	linedef: &Linedef,
) -> Option<Intersect> {
	let move_step = Line2::new(
		Vector2::new(move_step.point[0], move_step.point[1]),
		Vector2::new(move_step.dir[0], move_step.dir[1]),
	);

	let bbox_corners = [
		Vector2::new(bbox.min[0], bbox.min[1]) + move_step.point,
		Vector2::new(bbox.min[0], bbox.max[1]) + move_step.point,
		Vector2::new(bbox.max[0], bbox.max[1]) + move_step.point,
		Vector2::new(bbox.max[0], bbox.min[1]) + move_step.point,
	];

	let bbox_normals = [
		Vector3::new(-1.0, 0.0, 0.0),
		Vector3::new(0.0, 1.0, 0.0),
		Vector3::new(1.0, 0.0, 0.0),
		Vector3::new(0.0, -1.0, 0.0),
	];

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
						normal: -bbox_normals[i],
					});
				}
			}
		}
	}

	ret
}
