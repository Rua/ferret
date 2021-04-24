use crate::{
	common::{
		assets::AssetStorage,
		spawn::{ComponentAccessor, SpawnFrom, SpawnMergerHandlerSet},
		time::{DeltaTime, GameTime, Timer},
	},
	doom::{
		data::FRAME_TIME,
		map::{MapDynamic, SectorRef},
	},
};
use legion::{
	systems::{ResourceSet, Runnable},
	IntoQuery, Read, Registry, Resources, SystemBuilder, Write,
};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LightFlash {
	pub flash_type: LightFlashType,
	pub on_time: Duration,
	pub off_time: Duration,
	pub timer: Timer,
	pub state: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct LightFlashDef {
	pub flash_type: LightFlashType,
	pub on_time: Duration,
	pub off_time: Duration,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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

impl SpawnFrom<LightFlashDef> for LightFlash {
	fn spawn(
		component: &LightFlashDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> LightFlash {
		let game_time = <Read<GameTime>>::fetch(resources);

		let LightFlashDef {
			mut flash_type,
			on_time,
			off_time,
		} = component.clone();

		let time = match flash_type {
			LightFlashType::Broken => on_time * (thread_rng().gen::<bool>() as u32) + FRAME_TIME,
			LightFlashType::Strobe => on_time,
			LightFlashType::StrobeUnSync(time) => {
				time.mul_f64(thread_rng().gen::<f64>()) + FRAME_TIME
			}
		};

		if let LightFlashType::StrobeUnSync(_) = flash_type {
			flash_type = LightFlashType::Strobe;
		}

		LightFlash {
			flash_type,
			on_time,
			off_time,
			timer: Timer::new(*game_time, time),
			state: true,
		}
	}
}

pub fn light_flash(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<LightFlash>("LightFlash".into());
	handler_set.register_spawn::<LightFlashDef, LightFlash>();

	SystemBuilder::new("light_flash")
		.read_resource::<AssetStorage>()
		.read_resource::<GameTime>()
		.with_query(<(&SectorRef, &mut LightFlash)>::query())
		.with_query(<&mut MapDynamic>::query())
		.build(move |_command_buffer, world, resources, queries| {
			let (asset_storage, game_time) = resources;
			let (mut world0, mut world) = world.split_for_query(&queries.0);

			for (sector_ref, mut light_flash) in queries.0.iter_mut(&mut world0) {
				let map_dynamic = queries
					.1
					.get_mut(&mut world, sector_ref.map_entity)
					.unwrap();
				let sector_dynamic = &mut map_dynamic.sectors[sector_ref.index];

				if light_flash.timer.is_elapsed(**game_time) {
					light_flash.state = !light_flash.state;
					let map = asset_storage.get(&map_dynamic.map).unwrap();
					let sector = &map.sectors[sector_ref.index];

					// TODO: calculate these once at spawn
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
								light_flash.on_time * (thread_rng().gen::<bool>() as u32)
									+ FRAME_TIME
							} else {
								sector_dynamic.light_level = min_light;
								light_flash.off_time.mul_f64(thread_rng().gen::<f64>()) + FRAME_TIME
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
						LightFlashType::StrobeUnSync(_) => unreachable!(),
					};

					light_flash.timer.restart_with(**game_time, new_time);
				}
			}
		})
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct LightGlow {
	pub speed: f32,
	pub state: bool,
}

pub fn light_glow(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<LightGlow>("LightGlow".into());
	handler_set.register_clone::<LightGlow>();

	SystemBuilder::new("light_glow")
		.read_resource::<AssetStorage>()
		.read_resource::<DeltaTime>()
		.with_query(<(&SectorRef, &mut LightGlow)>::query())
		.with_query(<&mut MapDynamic>::query())
		.build(move |_command_buffer, world, resources, queries| {
			let (asset_storage, delta_time) = resources;
			let (mut world0, mut world) = world.split_for_query(&queries.0);

			for (sector_ref, mut light_glow) in queries.0.iter_mut(&mut world0) {
				let map_dynamic = queries
					.1
					.get_mut(&mut world, sector_ref.map_entity)
					.unwrap();
				let sector_dynamic = &mut map_dynamic.sectors[sector_ref.index];

				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let sector = &map.sectors[sector_ref.index];
				let speed = light_glow.speed * delta_time.0.as_secs_f32();

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
