use crate::{
	assets::{AssetHandle, AssetStorage},
	audio::Sound,
	doom::{
		client::{PlayerCommandSystem, PlayerMoveSystem, PlayerUseSystem},
		components::{
			DoorActive, DoorState, LightFlash, LightFlashType, LightGlow, LinedefDynamic,
			MapDynamic, SectorDynamic, TextureScroll,
		},
		map::Map,
		physics::PhysicsSystem,
	},
};
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use specs::{
	Entities, Entity, Join, ReadExpect, ReadStorage, RunNow, World, WriteExpect, WriteStorage,
};
use std::time::Duration;

#[derive(Default)]
pub struct UpdateSystem {
	player_command: PlayerCommandSystem,
	player_move: PlayerMoveSystem,
	player_use: PlayerUseSystem,

	physics: PhysicsSystem,

	door_update: DoorUpdateSystem,
	light_update: LightUpdateSystem,
	texture_scroll: TextureScrollSystem,
}

impl<'a> RunNow<'a> for UpdateSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		self.player_command.run_now(world);
		self.player_move.run_now(world);
		self.player_use.run_now(world);

		self.physics.run_now(world);

		self.door_update.run_now(world);
		self.light_update.run_now(world);
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
			delta,
			mut sound_queue,
			mut door_active_component,
			mut sector_dynamic_component,
		) = world.system_data::<(
			Entities,
			ReadExpect<Duration>,
			WriteExpect<Vec<(AssetHandle<Sound>, Entity)>>,
			WriteStorage<DoorActive>,
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
			match door_active.state {
				DoorState::Closed => {
					door_active.state = DoorState::Opening;

					// Play sound
					sound_queue.push((door_active.open_sound.clone(), entity));
				}
				DoorState::Opening => {
					sector_dynamic.ceiling_height += door_active.speed * delta.as_secs_f32();

					if sector_dynamic.ceiling_height > door_active.open_height {
						sector_dynamic.ceiling_height = door_active.open_height;
						door_active.state = DoorState::Open;
					}
				}
				DoorState::Open => {
					if let Some(new_time) = door_active.time_left.checked_sub(*delta) {
						door_active.time_left = new_time;
					} else {
						door_active.state = DoorState::Closing;

						// Play sound
						sound_queue.push((door_active.close_sound.clone(), entity));
					}
				}
				DoorState::Closing => {
					sector_dynamic.ceiling_height -= door_active.speed * delta.as_secs_f32();

					if sector_dynamic.ceiling_height < door_active.close_height {
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
