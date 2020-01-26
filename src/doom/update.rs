use crate::{
	assets::AssetStorage,
	doom::{
		components::{LightFlash, LightGlow, MapDynamic, SectorRef, Transform},
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
}

impl<'a> RunNow<'a> for UpdateSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		self.light_update.run_now(world);
		self.player_move.run_now(world);
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
			mut light_flash_storage,
			mut light_glow_storage,
			mut map_dynamic_storage,
			sector_ref_storage,
			mut rng,
		) = world.system_data::<(
			ReadExpect<AssetStorage<Map>>,
			ReadExpect<Duration>,
			WriteStorage<LightFlash>,
			WriteStorage<LightGlow>,
			WriteStorage<MapDynamic>,
			ReadStorage<SectorRef>,
			WriteExpect<Pcg64Mcg>,
		)>();

		for (sector_ref, light_flash) in (&sector_ref_storage, &mut light_flash_storage).join() {
			if let Some(new_time) = light_flash.time_left.checked_sub(*delta) {
				light_flash.time_left = new_time;
			} else {
				light_flash.state = !light_flash.state;
				let map_dynamic = map_dynamic_storage
					.get_mut(sector_ref.map_entity)
					.expect("map_entity does not have MapDynamic component");
				let map = map_storage.get(&map_dynamic.map).unwrap();

				if light_flash.state {
					light_flash.time_left =
						light_flash.on_time * (rng.gen::<bool>() as u32) + crate::doom::FRAME_TIME;
					map_dynamic.sectors[sector_ref.index].light_level =
						map.sectors[sector_ref.index].light_level;
				} else {
					light_flash.time_left =
						light_flash.off_time.mul_f64(rng.gen::<f64>()) + crate::doom::FRAME_TIME;
					map_dynamic.sectors[sector_ref.index].light_level = map.sectors
						[sector_ref.index]
						.neighbours
						.iter()
						.map(|index| map.sectors[*index].light_level)
						.min_by(|x, y| x.partial_cmp(y).unwrap())
						.unwrap_or(0.0);
				}
			}
		}

		for (sector_ref, light_glow) in (&sector_ref_storage, &mut light_glow_storage).join() {
			let map_dynamic = map_dynamic_storage
				.get_mut(sector_ref.map_entity)
				.expect("map_entity does not have MapDynamic component");
			let map = map_storage.get(&map_dynamic.map).unwrap();
			let speed = light_glow.speed * delta.as_secs_f32();

			if light_glow.state {
				map_dynamic.sectors[sector_ref.index].light_level += speed;
				let max_light = map.sectors[sector_ref.index].light_level;

				if map_dynamic.sectors[sector_ref.index].light_level > max_light {
					map_dynamic.sectors[sector_ref.index].light_level =
						2.0 * max_light - map_dynamic.sectors[sector_ref.index].light_level;
					light_glow.state = !light_glow.state;
				}
			} else {
				map_dynamic.sectors[sector_ref.index].light_level -= speed;
				let min_light = map.sectors[sector_ref.index]
					.neighbours
					.iter()
					.map(|index| map.sectors[*index].light_level)
					.min_by(|x, y| x.partial_cmp(y).unwrap())
					.unwrap_or(0.0);

				if map_dynamic.sectors[sector_ref.index].light_level < min_light {
					map_dynamic.sectors[sector_ref.index].light_level =
						2.0 * min_light - map_dynamic.sectors[sector_ref.index].light_level;
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
