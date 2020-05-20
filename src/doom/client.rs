use crate::{
	assets::AssetStorage,
	doom::{
		components::{Transform, Velocity},
		data::{FORWARD_ACCEL, STRAFE_ACCEL},
		door::{DoorSwitchUse, DoorUse},
		input::{Action, Axis, UserCommand},
		map::{Map, MapDynamic},
		physics::{BoxCollider, EntityTracer, SolidMask},
	},
	geometry::{Line2, AABB3},
	input::{Bindings, InputState},
	quadtree::Quadtree,
};
use legion::prelude::{Entity, IntoQuery, Read, ResourceSet, Resources, World, Write};
use nalgebra::{Vector2, Vector3};
use shrev::EventChannel;
use std::time::Duration;

#[derive(Default)]
pub struct Client {
	pub entity: Option<Entity>,
	pub command: UserCommand,
	pub previous_command: UserCommand,
}

pub fn player_command_system() -> Box<dyn FnMut(&mut World, &mut Resources)> {
	Box::new(|_world, resources| {
		let (bindings, input_state, mut client) = <(
			Read<Bindings<Action, Axis>>,
			Read<InputState>,
			Write<Client>,
		)>::fetch_mut(resources);
		let mut command = UserCommand {
			action_attack: bindings.action_is_down(&Action::Attack, &input_state),
			action_use: bindings.action_is_down(&Action::Use, &input_state),
			axis_forward: bindings.axis_value(&Axis::Forward, &input_state) as f32,
			axis_pitch: bindings.axis_value(&Axis::Pitch, &input_state) as f32,
			axis_strafe: bindings.axis_value(&Axis::Strafe, &input_state) as f32,
			axis_yaw: bindings.axis_value(&Axis::Yaw, &input_state) as f32,
		};

		if bindings.action_is_down(&Action::Walk, &input_state) {
			command.axis_forward *= 0.5;
			command.axis_strafe *= 0.6;
		}

		client.previous_command = client.command;
		client.command = command;
	})
}

pub fn player_move_system() -> Box<dyn FnMut(&mut World, &mut Resources)> {
	Box::new(|world, resources| {
		let (client, delta, map_storage, quadtree) = <(
			Read<Client>,
			Read<Duration>,
			Read<AssetStorage<Map>>,
			Read<Quadtree>,
		)>::fetch_mut(resources);

		if let Some(entity) = client.entity {
			// Apply rotation
			{
				let mut transform = world.get_component_mut::<Transform>(entity).unwrap();

				transform.rotation[1] += (client.command.axis_pitch * 1e6) as i32;
				transform.rotation[1].0 =
					num_traits::clamp(transform.rotation[1].0, -0x4000_0000, 0x4000_0000);

				transform.rotation[2] -= (client.command.axis_yaw * 1e6) as i32;
			}

			// Apply acceleration
			{
				if client.command.axis_forward == 0.0 && client.command.axis_strafe == 0.0 {
					return;
				}

				let transform = world.get_component::<Transform>(entity).unwrap();
				let map_dynamic = <Read<MapDynamic>>::query().iter(world).next().unwrap();
				let map = map_storage.get(&map_dynamic.map).unwrap();
				let box_collider = world.get_component::<BoxCollider>(entity).unwrap();

				let tracer = EntityTracer {
					map,
					map_dynamic: map_dynamic.as_ref(),
					quadtree: &quadtree,
					world,
				};

				let entity_bbox =
					AABB3::from_radius_height(box_collider.radius, box_collider.height);

				let trace = tracer.trace(
					&entity_bbox.offset(transform.position),
					Vector3::new(0.0, 0.0, -0.25),
					SolidMask::NON_MONSTER, // TODO solid mask
				);

				if trace.collision.is_none() {
					// Player is not on ground
					return;
				}

				let move_dir = Vector2::new(
					client.command.axis_forward.max(-1.0).min(1.0) * FORWARD_ACCEL,
					client.command.axis_strafe.max(-1.0).min(1.0) * STRAFE_ACCEL,
				);

				let angles = Vector3::new(0.into(), 0.into(), transform.rotation[2]);
				let axes = crate::geometry::angles_to_axes(angles);
				let accel = (axes[0] * move_dir[0] + axes[1] * move_dir[1]) * delta.as_secs_f32();

				unsafe {
					world
						.get_component_mut_unchecked::<Velocity>(entity)
						.unwrap()
						.velocity += accel;
				}
			}
		}
	})
}

pub fn player_use_system() -> Box<dyn FnMut(&mut World, &mut Resources)> {
	Box::new(|world, resources| {
		let (client, map_asset_storage, mut use_event_channel) = <(
			Read<Client>,
			Read<AssetStorage<Map>>,
			Write<EventChannel<UseEvent>>,
		)>::fetch_mut(resources);

		if let Some(entity) = client.entity {
			if client.command.action_use && !client.previous_command.action_use {
				let transform = world.get_component::<Transform>(entity).unwrap();
				let map_dynamic = <Read<MapDynamic>>::query().iter(world).next().unwrap();
				let map = map_asset_storage.get(&map_dynamic.map).unwrap();

				const USERANGE: f32 = 64.0;
				let yaw = transform.rotation[2].to_radians() as f32;
				let use_line = Line2::new(
					Vector2::new(transform.position[0], transform.position[1]),
					Vector2::new(yaw.cos(), yaw.sin()) * USERANGE,
				);

				// Find the closest linedef hit
				let mut pmax = 1.0;
				let mut closest_linedef = None;

				for (i, linedef) in map.linedefs.iter().enumerate() {
					if let Some((linedef_p, use_p)) = linedef.line.intersect(&use_line) {
						if linedef_p >= 0.0 && linedef_p <= 1.0 && use_p >= 0.0 && use_p < pmax {
							// Always hit a usable linedef
							if world
								.get_component::<UseAction>(map_dynamic.linedefs[i].entity)
								.is_some()
							{
								pmax = use_p;
								closest_linedef = Some(i);
							} else if let [Some(_front_sidedef), Some(_back_sidedef)] =
								&linedef.sidedefs
							{
								// Skip two-sided linedefs
							} else {
								pmax = use_p;
								closest_linedef = Some(i);
							}
						}
					}
				}

				// We hit a linedef, use it
				if let Some(linedef_index) = closest_linedef {
					let linedef = &map.linedefs[linedef_index];

					// Used from the back, ignore
					if (use_line.point - linedef.line.point).dot(&linedef.normal) <= 0.0 {
						return;
					}

					let linedef_entity = map_dynamic.linedefs[linedef_index].entity;

					if world.get_component::<UseAction>(linedef_entity).is_some() {
						use_event_channel.single_write(UseEvent { linedef_entity });
					}
				}
			}
		}
	})
}

#[derive(Clone, Debug)]
pub enum UseAction {
	DoorUse(DoorUse),
	DoorSwitchUse(DoorSwitchUse),
}

#[derive(Clone, Debug)]
pub struct UseEvent {
	pub linedef_entity: Entity,
}
