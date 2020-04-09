use crate::{
	assets::AssetStorage,
	doom::{
		components::{Transform, Velocity},
		door::DoorUse,
		input::{Action, Axis, UserCommand},
		map::{Map, MapDynamic},
	},
	geometry::{Line2, Side},
	input::{Bindings, InputState},
};
use nalgebra::Vector2;
use shrev::EventChannel;
use specs::{
	Component, DenseVecStorage, Entity, Join, ReadExpect, ReadStorage, RunNow, World, WriteExpect,
	WriteStorage,
};
use specs_derive::Component;

#[derive(Default)]
pub struct Client {
	pub entity: Option<Entity>,
	pub command: UserCommand,
	pub previous_command: UserCommand,
}

#[derive(Default)]
pub struct PlayerCommandSystem;

impl<'a> RunNow<'a> for PlayerCommandSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (bindings, mut client, input_state) = world.system_data::<(
			ReadExpect<Bindings<Action, Axis>>,
			WriteExpect<Client>,
			ReadExpect<InputState>,
		)>();

		let command = UserCommand {
			action_attack: bindings.action_is_down(&Action::Attack, &input_state),
			action_use: bindings.action_is_down(&Action::Use, &input_state),
			axis_forward: bindings.axis_value(&Axis::Forward, &input_state) as f32,
			axis_pitch: bindings.axis_value(&Axis::Pitch, &input_state) as f32,
			axis_strafe: bindings.axis_value(&Axis::Strafe, &input_state) as f32,
			axis_yaw: bindings.axis_value(&Axis::Yaw, &input_state) as f32,
		};

		client.previous_command = client.command;
		client.command = command;
	}
}

#[derive(Default)]
pub struct PlayerMoveSystem;

impl<'a> RunNow<'a> for PlayerMoveSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (client, mut transform_component, mut velocity_component) = world.system_data::<(
			WriteExpect<Client>,
			WriteStorage<Transform>,
			WriteStorage<Velocity>,
		)>();

		if let Some(entity) = client.entity {
			let transform = transform_component.get_mut(entity).unwrap();
			let velocity = velocity_component.get_mut(entity).unwrap();

			transform.rotation[1] += (client.command.axis_pitch * 1e6) as i32;
			transform.rotation[1].0 =
				num_traits::clamp(transform.rotation[1].0, -0x4000_0000, 0x4000_0000);

			transform.rotation[2] -= (client.command.axis_yaw * 1e6) as i32;

			let mut move_dir =
				Vector2::new(client.command.axis_forward, client.command.axis_strafe);
			let len = move_dir.norm();

			if len > 1.0 {
				move_dir /= len;
			}

			move_dir *= 20.0 / crate::doom::FRAME_TIME.as_secs_f32();

			let angles = transform.rotation; //Vector3::new(0.into(), 0.into(), transform.rotation[2]);
			let axes = crate::geometry::angles_to_axes(angles);
			velocity.velocity = axes[0] * move_dir[0] + axes[1] * move_dir[1];
		}
	}
}

#[derive(Default)]
pub struct PlayerUseSystem;

impl<'a> RunNow<'a> for PlayerUseSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (
			client,
			map_asset_storage,
			mut use_event_channel,
			map_dynamic_component,
			use_action_component,
			mut transform_component,
		) = world.system_data::<(
			ReadExpect<Client>,
			ReadExpect<AssetStorage<Map>>,
			WriteExpect<EventChannel<UseEvent>>,
			ReadStorage<MapDynamic>,
			ReadStorage<UseAction>,
			WriteStorage<Transform>,
		)>();

		if let Some(entity) = client.entity {
			if client.command.action_use && !client.previous_command.action_use {
				let transform = transform_component.get_mut(entity).unwrap();
				let map_dynamic = map_dynamic_component.join().next().unwrap();
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
							if use_action_component
								.get(map_dynamic.linedefs[i].entity)
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
					if linedef.point_side(use_line.point) != Side::Right {
						return;
					}

					let linedef_entity = map_dynamic.linedefs[linedef_index].entity;

					if use_action_component.get(linedef_entity).is_some() {
						use_event_channel.single_write(UseEvent { linedef_entity });
					}
				}
			}
		}
	}
}

#[derive(Clone, Component, Debug)]
pub enum UseAction {
	DoorUse(DoorUse),
}

#[derive(Clone, Debug)]
pub struct UseEvent {
	pub linedef_entity: Entity,
}
