use crate::{
	common::{
		assets::AssetStorage,
		time::{FrameTime, Timer},
	},
	doom::{
		data::FRAME_TIME,
		map::{MapDynamic, SectorRef},
	},
};
use legion::{systems::Runnable, IntoQuery, SystemBuilder};
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use std::time::Duration;

pub fn light_flash_system() -> impl Runnable {
	SystemBuilder::new("light_flash_system")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameTime>()
		.write_resource::<Pcg64Mcg>()
		.with_query(<(&SectorRef, &mut LightFlash)>::query())
		.with_query(<&mut MapDynamic>::query())
		.build(move |_, world, resources, queries| {
			let (asset_storage, frame_time, rng) = resources;
			let (mut world0, mut world) = world.split_for_query(&queries.0);

			for (sector_ref, mut light_flash) in queries.0.iter_mut(&mut world0) {
				let map_dynamic = queries
					.1
					.get_mut(&mut world, sector_ref.map_entity)
					.unwrap();
				let sector_dynamic = &mut map_dynamic.sectors[sector_ref.index];

				light_flash.timer.tick(frame_time.delta);

				if light_flash.timer.is_zero() {
					light_flash.state = !light_flash.state;
					let map = asset_storage.get(&map_dynamic.map).unwrap();
					let sector = &map.sectors[sector_ref.index];

					let max_light = sector.light_level;
					let min_light = sector
						.neighbours
						.iter()
						.map(|index| map.sectors[*index].light_level)
						.min_by(|x, y| x.partial_cmp(y).unwrap())
						.unwrap_or(0.0);

					let new_time = match light_flash.flash_type {
						LightFlashType::Broken => {
							if light_flash.state {
								sector_dynamic.light_level = max_light;
								light_flash.on_time * (rng.gen::<bool>() as u32) + FRAME_TIME
							} else {
								sector_dynamic.light_level = min_light;
								light_flash.off_time.mul_f64(rng.gen::<f64>()) + FRAME_TIME
							}
						}
						LightFlashType::Strobe => {
							if light_flash.state {
								sector_dynamic.light_level = max_light;
								light_flash.on_time
							} else {
								sector_dynamic.light_level = if min_light == max_light {
									0.0
								} else {
									min_light
								};

								light_flash.off_time
							}
						}
						LightFlashType::StrobeUnSync(time) => {
							light_flash.flash_type = LightFlashType::Strobe;
							time.mul_f64(rng.gen::<f64>()) + FRAME_TIME
						}
					};

					light_flash.timer.set(new_time);
				}
			}
		})
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LightFlash {
	pub on_time: Duration,
	pub off_time: Duration,
	pub timer: Timer,
	pub state: bool,
	pub flash_type: LightFlashType,
}

#[derive(Clone, Copy, Debug)]
pub enum LightFlashType {
	Broken,
	Strobe,
	StrobeUnSync(Duration),
}

impl Default for LightFlashType {
	fn default() -> LightFlashType {
		LightFlashType::Broken
	}
}

pub fn light_glow_system() -> impl Runnable {
	SystemBuilder::new("light_glow_system")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameTime>()
		.with_query(<(&SectorRef, &mut LightGlow)>::query())
		.with_query(<&mut MapDynamic>::query())
		.build(move |_, world, resources, queries| {
			let (asset_storage, frame_time) = resources;
			let (mut world0, mut world) = world.split_for_query(&queries.0);

			for (sector_ref, mut light_glow) in queries.0.iter_mut(&mut world0) {
				let map_dynamic = queries
					.1
					.get_mut(&mut world, sector_ref.map_entity)
					.unwrap();
				let sector_dynamic = &mut map_dynamic.sectors[sector_ref.index];

				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let sector = &map.sectors[sector_ref.index];
				let speed = light_glow.speed * frame_time.delta.as_secs_f32();

				if light_glow.state {
					sector_dynamic.light_level += speed;
					let max_light = sector.light_level;

					if sector_dynamic.light_level > max_light {
						sector_dynamic.light_level -= speed;
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

					if sector_dynamic.light_level <= min_light {
						sector_dynamic.light_level += speed;
						light_glow.state = !light_glow.state;
					}
				}
			}
		})
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LightGlow {
	pub speed: f32,
	pub state: bool,
}
