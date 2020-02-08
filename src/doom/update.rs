use crate::{
	assets::AssetStorage,
	audio::{Sound, SoundSource},
	doom::{
		client::Client,
		components::{
			DoorActive, DoorState, DoorUse, LightFlash, LightFlashType, LightGlow, LinedefDynamic,
			MapDynamic, SectorDynamic, TextureScroll, Transform,
		},
		input::{Action, Axis, UserCommand},
		map::Map,
	},
	geometry::Side,
	input::{Bindings, InputState},
};
use nalgebra::Vector2;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use rodio::Source;
use specs::{Entities, Join, ReadExpect, ReadStorage, RunNow, World, WriteExpect, WriteStorage};
use std::time::Duration;

#[derive(Default)]
pub struct UpdateSystem {
	door_update: DoorUpdateSystem,
	light_update: LightUpdateSystem,
	player_command: PlayerCommandSystem,
	player_move: PlayerMoveSystem,
	player_use: PlayerUseSystem,
	texture_scroll: TextureScrollSystem,
}

impl<'a> RunNow<'a> for UpdateSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		self.door_update.run_now(world);
		self.light_update.run_now(world);
		self.player_command.run_now(world);
		self.player_move.run_now(world);
		self.player_use.run_now(world);
		self.texture_scroll.run_now(world);
	}
}

#[derive(Default)]
struct DoorUpdateSystem;

impl<'a> RunNow<'a> for DoorUpdateSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (
			entities,
			sound_device,
			map_storage,
			sound_storage,
			delta,
			mut door_active_component,
			map_dynamic_component,
			mut sector_dynamic_component,
		) = world.system_data::<(
			Entities,
			ReadExpect<rodio::Device>,
			ReadExpect<AssetStorage<Map>>,
			ReadExpect<AssetStorage<Sound>>,
			ReadExpect<Duration>,
			WriteStorage<DoorActive>,
			ReadStorage<MapDynamic>,
			WriteStorage<SectorDynamic>,
		)>();

		let mut done = Vec::new();

		for (entity, sector_dynamic, door_active) in (
			&entities,
			&mut sector_dynamic_component,
			&mut door_active_component,
		)
			.join()
		{
			let map_dynamic = map_dynamic_component
				.get(sector_dynamic.map_entity)
				.expect("map_entity does not have MapDynamic component");
			let map = map_storage.get(&map_dynamic.map).unwrap();

			match door_active.state {
				DoorState::Opening => {
					sector_dynamic.ceiling_height += door_active.speed * delta.as_secs_f32();

					if sector_dynamic.ceiling_height > door_active.target_height {
						sector_dynamic.ceiling_height = door_active.target_height;
						door_active.state = DoorState::Open;
					}
				}
				DoorState::Open => {
					if let Some(new_time) = door_active.time_left.checked_sub(*delta) {
						door_active.time_left = new_time;
					} else {
						door_active.target_height = map.sectors[sector_dynamic.index].floor_height;
						door_active.state = DoorState::Closing;

						// Play sound
						let sound = sound_storage.get(&door_active.close_sound).unwrap();
						let source = SoundSource::new(&sound);
						rodio::play_raw(&sound_device, source.convert_samples());
					}
				}
				DoorState::Closing => {
					sector_dynamic.ceiling_height -= door_active.speed * delta.as_secs_f32();

					if sector_dynamic.ceiling_height < door_active.target_height {
						done.push(entity);
					}
				}
			}
		}

		for entity in done {
			door_active_component.remove(entity);
		}
	}
}

#[derive(Default)]
struct LightUpdateSystem;

impl<'a> RunNow<'a> for LightUpdateSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (
			map_storage,
			delta,
			mut light_flash_component,
			mut light_glow_component,
			map_dynamic_component,
			mut sector_dynamic_component,
			mut rng,
		) = world.system_data::<(
			ReadExpect<AssetStorage<Map>>,
			ReadExpect<Duration>,
			WriteStorage<LightFlash>,
			WriteStorage<LightGlow>,
			ReadStorage<MapDynamic>,
			WriteStorage<SectorDynamic>,
			WriteExpect<Pcg64Mcg>,
		)>();

		for (sector_dynamic, light_flash) in
			(&mut sector_dynamic_component, &mut light_flash_component).join()
		{
			if let Some(new_time) = light_flash.time_left.checked_sub(*delta) {
				light_flash.time_left = new_time;
			} else {
				light_flash.state = !light_flash.state;
				let map_dynamic = map_dynamic_component
					.get(sector_dynamic.map_entity)
					.expect("map_entity does not have MapDynamic component");
				let map = map_storage.get(&map_dynamic.map).unwrap();
				let sector = &map.sectors[sector_dynamic.index];

				let max_light = sector.light_level;
				let min_light = sector
					.neighbours
					.iter()
					.map(|index| map.sectors[*index].light_level)
					.min_by(|x, y| x.partial_cmp(y).unwrap())
					.unwrap_or(0.0);

				match light_flash.flash_type {
					LightFlashType::Broken => {
						if light_flash.state {
							light_flash.time_left = light_flash.on_time
								* (rng.gen::<bool>() as u32)
								+ crate::doom::FRAME_TIME;
							sector_dynamic.light_level = max_light;
						} else {
							light_flash.time_left = light_flash.off_time.mul_f64(rng.gen::<f64>())
								+ crate::doom::FRAME_TIME;
							sector_dynamic.light_level = min_light;
						}
					}
					LightFlashType::Strobe => {
						if light_flash.state {
							light_flash.time_left = light_flash.on_time;
							sector_dynamic.light_level = max_light;
						} else {
							light_flash.time_left = light_flash.off_time;
							sector_dynamic.light_level = if min_light == max_light {
								0.0
							} else {
								min_light
							};
						}
					}
					LightFlashType::StrobeUnSync(time) => {
						light_flash.time_left =
							time.mul_f64(rng.gen::<f64>()) + crate::doom::FRAME_TIME;
						light_flash.flash_type = LightFlashType::Strobe;
					}
				}
			}
		}

		for (sector_dynamic, light_glow) in
			(&mut sector_dynamic_component, &mut light_glow_component).join()
		{
			let map_dynamic = map_dynamic_component
				.get(sector_dynamic.map_entity)
				.expect("map_entity does not have MapDynamic component");
			let map = map_storage.get(&map_dynamic.map).unwrap();
			let sector = &map.sectors[sector_dynamic.index];
			let speed = light_glow.speed * delta.as_secs_f32();

			if light_glow.state {
				sector_dynamic.light_level += speed;
				let max_light = sector.light_level;

				if sector_dynamic.light_level > max_light {
					sector_dynamic.light_level = 2.0 * max_light - sector_dynamic.light_level;
					light_glow.state = !light_glow.state;
				}
			} else {
				sector_dynamic.light_level -= speed;
				let min_light = sector
					.neighbours
					.iter()
					.map(|index| map.sectors[*index].light_level)
					.min_by(|x, y| x.partial_cmp(y).unwrap())
					.unwrap_or(0.0);

				if sector_dynamic.light_level < min_light {
					sector_dynamic.light_level = 2.0 * min_light - sector_dynamic.light_level;
					light_glow.state = !light_glow.state;
				}
			}
		}
	}
}

#[derive(Default)]
struct PlayerCommandSystem;

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
struct PlayerMoveSystem;

impl<'a> RunNow<'a> for PlayerMoveSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (client, mut transform_component) =
			world.system_data::<(WriteExpect<Client>, WriteStorage<Transform>)>();

		if let Some(entity) = client.entity {
			let transform = transform_component.get_mut(entity).unwrap();

			transform.rotation[1] += (client.command.axis_pitch * 1e6) as i32;
			transform.rotation[1].0 =
				num_traits::clamp(transform.rotation[1].0, -0x40000000, 0x40000000);

			transform.rotation[2] -= (client.command.axis_yaw * 1e6) as i32;

			let axes = crate::geometry::angles_to_axes(transform.rotation);
			let mut move_dir =
				Vector2::new(client.command.axis_forward, client.command.axis_strafe);
			let len = move_dir.norm();

			if len > 1.0 {
				move_dir /= len;
			}

			move_dir *= 20.0;

			transform.position += axes[0] * move_dir[0] + axes[1] * move_dir[1];
		}
	}
}

#[derive(Default)]
struct PlayerUseSystem;

impl<'a> RunNow<'a> for PlayerUseSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (
			client,
			sound_device,
			map_storage,
			sound_storage,
			mut door_active_component,
			door_use_component,
			map_dynamic_component,
			sector_dynamic_component,
			mut transform_component,
		) = world.system_data::<(
			ReadExpect<Client>,
			ReadExpect<rodio::Device>,
			ReadExpect<AssetStorage<Map>>,
			ReadExpect<AssetStorage<Sound>>,
			WriteStorage<DoorActive>,
			ReadStorage<DoorUse>,
			ReadStorage<MapDynamic>,
			ReadStorage<SectorDynamic>,
			WriteStorage<Transform>,
		)>();

		if let Some(entity) = client.entity {
			if client.command.action_use && !client.previous_command.action_use {
				let transform = transform_component.get_mut(entity).unwrap();
				let map_dynamic = map_dynamic_component.join().next().unwrap();
				let map = map_storage.get(&map_dynamic.map).unwrap();

				const USERANGE: f32 = 64.0;
				let mut point = transform.position;
				point[2] += 41.0; // TODO: Store view height properly
				let yaw = transform.rotation[2].to_radians() as f32;
				let direction = Vector2::new(yaw.cos(), yaw.sin()) * USERANGE;

				// Find the closest linedef hit
				let mut tmax = 1.0;
				let mut closest_linedef = None;

				for (i, linedef) in map.linedefs.iter().enumerate() {
					let t = crate::geometry::intersect(
						linedef.vertices[0],
						linedef.vertices[1] - linedef.vertices[0],
						Vector2::new(point[0], point[1]),
						direction,
					);

					if t < tmax {
						tmax = t;
						closest_linedef = Some(i);
					}
				}

				// We hit a linedef, use it
				if let Some(linedef_index) = closest_linedef {
					let linedef = &map.linedefs[linedef_index];

					// Used from the back, ignore
					if linedef.point_side(Vector2::new(point[0], point[1])) != Side::Right {
						return;
					}

					let linedef_entity = map_dynamic.linedefs[linedef_index];

					if let Some(door_use) = door_use_component.get(linedef_entity) {
						if let Some(back_sidedef) = &linedef.sidedefs[Side::Left as usize] {
							let sector_index = back_sidedef.sector_index;
							let sector = &map.sectors[sector_index];

							if let Some(target_height) = sector
								.neighbours
								.iter()
								.map(|index| {
									sector_dynamic_component
										.get(map_dynamic.sectors[*index])
										.unwrap()
										.ceiling_height
								})
								.min_by(|x, y| x.partial_cmp(y).unwrap())
							{
								let target_height = target_height - 4.0;
								let sector_entity = map_dynamic.sectors[sector_index];
								let sector_dynamic =
									sector_dynamic_component.get(sector_entity).unwrap();

								if let Some(door_active) =
									door_active_component.get_mut(sector_entity)
								{
									match door_active.state {
										DoorState::Closing => {
											// Re-open the door
											door_active.state = DoorState::Opening;
											door_active.target_height = target_height;

											// Play sound
											let sound =
												sound_storage.get(&door_use.open_sound).unwrap();
											let source = SoundSource::new(&sound);
											rodio::play_raw(
												&sound_device,
												source.convert_samples(),
											);
										}
										DoorState::Opening | DoorState::Open => {
											// Close the door early
											door_active.state = DoorState::Closing;
											door_active.target_height = sector_dynamic.floor_height;

											// Play sound
											let sound =
												sound_storage.get(&door_use.close_sound).unwrap();
											let source = SoundSource::new(&sound);
											rodio::play_raw(
												&sound_device,
												source.convert_samples(),
											);
										}
									}
								} else {
									door_active_component
										.insert(
											sector_entity,
											DoorActive {
												close_sound: door_use.close_sound.clone(),
												state: DoorState::Opening,
												speed: door_use.speed,
												target_height,
												time_left: door_use.wait_time,
											},
										)
										.unwrap();

									// Play sound
									let sound = sound_storage.get(&door_use.open_sound).unwrap();
									let source = SoundSource::new(&sound);
									rodio::play_raw(&sound_device, source.convert_samples());
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

#[derive(Default)]
struct TextureScrollSystem;

impl<'a> RunNow<'a> for TextureScrollSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (delta, mut linedef_dynamic_component, texture_scroll_component) = world
			.system_data::<(
				ReadExpect<Duration>,
				WriteStorage<LinedefDynamic>,
				ReadStorage<TextureScroll>,
			)>();

		for (linedef_dynamic, texture_scroll) in
			(&mut linedef_dynamic_component, &texture_scroll_component).join()
		{
			linedef_dynamic.texture_offset += texture_scroll.speed * delta.as_secs_f32();
		}
	}
}
