use crate::{
	assets::AssetStorage,
	doom::{
		data::FRAME_TIME,
		map::{MapDynamic, SectorRef},
	},
};
use legion::prelude::{IntoQuery, Read, ResourceSet, Resources, World, Write};
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use std::time::Duration;

pub fn light_system() -> Box<dyn FnMut(&mut World, &mut Resources)> {
	Box::new(|world, resources| {
		let (asset_storage, delta, mut rng) =
			<(Read<AssetStorage>, Read<Duration>, Write<Pcg64Mcg>)>::fetch_mut(resources);

		for (sector_ref, mut light_flash) in
			unsafe { <(Read<SectorRef>, Write<LightFlash>)>::query().iter_unchecked(world) }
		{
			let mut map_dynamic = unsafe {
				world
					.get_component_mut_unchecked::<MapDynamic>(sector_ref.map_entity)
					.unwrap()
			};
			let map_dynamic = map_dynamic.as_mut();
			let sector_dynamic = &mut map_dynamic.sectors[sector_ref.index];

			if let Some(new_time) = light_flash.time_left.checked_sub(*delta) {
				light_flash.time_left = new_time;
			} else {
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

				match light_flash.flash_type {
					LightFlashType::Broken => {
						if light_flash.state {
							light_flash.time_left =
								light_flash.on_time * (rng.gen::<bool>() as u32) + FRAME_TIME;
							sector_dynamic.light_level = max_light;
						} else {
							light_flash.time_left =
								light_flash.off_time.mul_f64(rng.gen::<f64>()) + FRAME_TIME;
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
						light_flash.time_left = time.mul_f64(rng.gen::<f64>()) + FRAME_TIME;
						light_flash.flash_type = LightFlashType::Strobe;
					}
				}
			}
		}

		for (sector_ref, mut light_glow) in
			unsafe { <(Read<SectorRef>, Write<LightGlow>)>::query().iter_unchecked(world) }
		{
			let mut map_dynamic = unsafe {
				world
					.get_component_mut_unchecked::<MapDynamic>(sector_ref.map_entity)
					.unwrap()
			};
			let map_dynamic = map_dynamic.as_mut();
			let sector_dynamic = &mut map_dynamic.sectors[sector_ref.index];

			let map = asset_storage.get(&map_dynamic.map).unwrap();
			let sector = &map.sectors[sector_ref.index];
			let speed = light_glow.speed * delta.as_secs_f32();

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
pub struct LightFlash {
	pub on_time: Duration,
	pub off_time: Duration,
	pub time_left: Duration,
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

#[derive(Clone, Copy, Debug, Default)]
pub struct LightGlow {
	pub speed: f32,
	pub state: bool,
}
