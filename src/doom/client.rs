use crate::{
	assets::AssetStorage,
	doom::{
		components::{Transform, Velocity},
		door::{DoorActive, DoorState, DoorUse},
		input::{Action, Axis, UserCommand},
		map::{Map, MapDynamic},
	},
	geometry::{Line2, Side},
	input::{Bindings, InputState},
};
use nalgebra::Vector2;
use specs::{Entity, Join, ReadExpect, ReadStorage, RunNow, World, WriteExpect, WriteStorage};
use std::time::Duration;

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
				num_traits::clamp(transform.rotation[1].0, -0x40000000, 0x40000000);

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
			map_storage,
			door_use_component,
			map_dynamic_component,
			mut door_active_component,
			mut transform_component,
		) = world.system_data::<(
			ReadExpect<Client>,
			ReadExpect<AssetStorage<Map>>,
			ReadStorage<DoorUse>,
			ReadStorage<MapDynamic>,
			WriteStorage<DoorActive>,
			WriteStorage<Transform>,
		)>();

		if let Some(entity) = client.entity {
			if client.command.action_use && !client.previous_command.action_use {
				let transform = transform_component.get_mut(entity).unwrap();
				let map_dynamic = map_dynamic_component.join().next().unwrap();
				let map = map_storage.get(&map_dynamic.map).unwrap();

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
							pmax = use_p;
							closest_linedef = Some(i);
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

					if let Some(door_use) = door_use_component.get(linedef_entity) {
						if let Some(back_sidedef) = &linedef.sidedefs[Side::Left as usize] {
							let sector_index = back_sidedef.sector_index;
							let sector = &map.sectors[sector_index];

							if let Some(open_height) = sector
								.neighbours
								.iter()
								.map(|index| map_dynamic.sectors[*index].ceiling_height)
								.min_by(|x, y| x.partial_cmp(y).unwrap())
							{
								let open_height = open_height - 4.0;
								let sector_entity = map_dynamic.sectors[sector_index].entity;

								if let Some(door_active) =
									door_active_component.get_mut(sector_entity)
								{
									match door_active.state {
										DoorState::Closing => {
											// Re-open the door
											door_active.state = DoorState::Closed;
											door_active.time_left = door_use.wait_time;
										}
										DoorState::Opening | DoorState::Open => {
											// Close the door early
											door_active.state = DoorState::Open;
											door_active.time_left = Duration::default();
										}
										DoorState::Closed => unreachable!(),
									}
								} else {
									door_active_component
										.insert(
											sector_entity,
											DoorActive {
												open_sound: door_use.open_sound.clone(),
												open_height: open_height,

												close_sound: door_use.close_sound.clone(),
												close_height: map_dynamic.sectors[sector_index]
													.floor_height,

												state: DoorState::Closed,
												speed: door_use.speed,
												time_left: door_use.wait_time,
											},
										)
										.unwrap();
								}
							} else {
								log::error!(
									"Used door linedef {}, sector {}, has no neighbouring sectors",
									linedef_index,
									sector_index
								);
							}
						} else {
							log::error!("Used door linedef {} has no back sector", linedef_index);
						}
					}
				}
			}
		}
	}
}
