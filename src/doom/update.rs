use crate::{
	assets::AssetStorage,
	doom::{
		components::{LightFlash, LightFlashType, LightGlow, LinedefDynamic, MapDynamic, SectorDynamic, TextureScroll, Transform},
		input::{Action, Axis},
		map::Map,
	},
	input::{Bindings, InputState},
};
use nalgebra::Vector2;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use specs::{Entity, Join, ReadExpect, ReadStorage, RunNow, World, WriteExpect, WriteStorage};
use std::time::Duration;

#[derive(Default)]
pub struct UpdateSystem {
	light_update: LightUpdateSystem,
	player_move: PlayerMoveSystem,
	texture_scroll: TextureScrollSystem,
}

impl<'a> RunNow<'a> for UpdateSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		self.light_update.run_now(world);
		self.player_move.run_now(world);
		self.texture_scroll.run_now(world);
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
struct PlayerMoveSystem;

impl<'a> RunNow<'a> for PlayerMoveSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (entity, mut transform_storage, input_state, bindings) = world.system_data::<(
			ReadExpect<Entity>,
			WriteStorage<Transform>,
			ReadExpect<InputState>,
			ReadExpect<Bindings<Action, Axis>>,
		)>();
		let transform = transform_storage.get_mut(*entity).unwrap();

		transform.rotation[1] += (bindings.axis_value(&Axis::Pitch, &input_state) * 1e6) as i32;
		transform.rotation[1].0 =
			num_traits::clamp(transform.rotation[1].0, -0x40000000, 0x40000000);

		transform.rotation[2] -= (bindings.axis_value(&Axis::Yaw, &input_state) * 1e6) as i32;

		let axes = crate::geometry::angles_to_axes(transform.rotation);
		let mut move_dir = Vector2::new(
			bindings.axis_value(&Axis::Forward, &input_state) as f32,
			bindings.axis_value(&Axis::Strafe, &input_state) as f32,
		);
		let len = move_dir.norm();

		if len > 1.0 {
			move_dir /= len;
		}

		move_dir *= 20.0;

		transform.position += axes[0] * move_dir[0] + axes[1] * move_dir[1];
	}
}

#[derive(Default)]
struct TextureScrollSystem;

impl<'a> RunNow<'a> for TextureScrollSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (
			delta,
			mut linedef_dynamic_component,
			texture_scroll_component,
		) = world.system_data::<(
			ReadExpect<Duration>,
			WriteStorage<LinedefDynamic>,
			ReadStorage<TextureScroll>,
		)>();

		for (linedef_dynamic, texture_scroll) in (&mut linedef_dynamic_component, &texture_scroll_component).join() {
			linedef_dynamic.texture_offset += texture_scroll.speed * delta.as_secs_f32();
		}
	}
}
