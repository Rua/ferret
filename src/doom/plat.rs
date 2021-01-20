use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		spawn::SpawnMergerHandlerSet,
		time::{GameTime, Timer},
	},
	doom::{
		client::{Usable, UseEvent},
		components::Transform,
		map::{LinedefRef, Map, MapDynamic},
		physics::{BoxCollider, TouchEvent, Touchable},
		sectormove::{FloorMove, SectorMove, SectorMoveEvent, SectorMoveEventType},
		sound::{Sound, StartSound},
		state::weapon::Owner,
		switch::{SwitchActive, SwitchParams},
	},
};
use legion::{
	component,
	systems::{CommandBuffer, ResourceSet, Runnable},
	Entity, EntityStore, IntoQuery, Registry, Resources, SystemBuilder, Write,
};
use serde::{Deserialize, Serialize};
use shrev::EventChannel;
use std::time::Duration;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlatActive {
	pub speed: f32,
	pub wait_timer: Timer,
	pub can_reverse: bool,

	pub start_sound: Option<AssetHandle<Sound>>,
	pub finish_sound: Option<AssetHandle<Sound>>,

	pub low_height: f32,
	pub high_height: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlatParams {
	pub speed: f32,
	pub wait_time: Duration,
	pub can_reverse: bool,

	pub start_sound: Option<AssetHandle<Sound>>,
	pub move_sound: Option<AssetHandle<Sound>>,
	pub move_sound_time: Duration,
	pub finish_sound: Option<AssetHandle<Sound>>,

	pub low_height_base: PlatTargetHeight,
	pub low_height_offset: f32,
	pub high_height_base: PlatTargetHeight,
	pub high_height_offset: f32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PlatTargetHeight {
	Current,
	LowestNeighbourFloor,
}

pub fn plat_active(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry, mut sector_move_event_channel) = <(
		Write<SpawnMergerHandlerSet>,
		Write<Registry<String>>,
		Write<EventChannel<SectorMoveEvent>>,
	)>::fetch_mut(resources);

	registry.register::<PlatActive>("PlatActive".into());
	handler_set.register_clone::<PlatActive>();

	let mut sector_move_event_reader = sector_move_event_channel.register_reader();

	SystemBuilder::new("plat_active")
		.read_resource::<GameTime>()
		.read_resource::<EventChannel<SectorMoveEvent>>()
		.with_query(<(Entity, &mut FloorMove, &mut PlatActive)>::query())
		.read_component::<BoxCollider>() // used by SectorTracer
		.read_component::<Owner>() // used by SectorTracer
		.read_component::<Transform>() // used by SectorTracer
		.build(move |command_buffer, world, resources, query| {
			let (game_time, sector_move_event_channel) = resources;

			for (&entity, floor_move, plat_active) in query.iter_mut(world) {
				let sector_move = &mut floor_move.0;

				if sector_move.velocity != 0.0 {
					continue;
				}

				if plat_active.wait_timer.is_elapsed(**game_time) {
					if let Some(sound) = &plat_active.start_sound {
						command_buffer.push((entity, StartSound(sound.clone())));
					}

					if sector_move.target == plat_active.low_height {
						sector_move.velocity = plat_active.speed;
						sector_move.target = plat_active.high_height;
					} else {
						sector_move.velocity = -plat_active.speed;
						sector_move.target = plat_active.low_height;
					}
				}
			}

			for event in sector_move_event_channel
				.read(&mut sector_move_event_reader)
				.filter(|e| e.normal == 1.0)
			{
				let (floor_move, plat_active) = match query.get_mut(world, event.entity) {
					Ok((_, floor_move, plat_active)) => (floor_move, plat_active),
					_ => continue,
				};

				let sector_move = &mut floor_move.0;

				if sector_move.velocity == 0.0 {
					continue;
				}

				match event.event_type {
					SectorMoveEventType::Collided => {
						if plat_active.can_reverse {
							sector_move.velocity = -sector_move.velocity;

							if sector_move.velocity < 0.0 {
								sector_move.target = plat_active.low_height;
							} else {
								sector_move.target = plat_active.high_height;
							}

							if let Some(sound) = &plat_active.start_sound {
								command_buffer.push((event.entity, StartSound(sound.clone())));
							}
						}
					}
					SectorMoveEventType::TargetReached => {
						sector_move.velocity = 0.0;

						if let Some(sound) = &plat_active.finish_sound {
							command_buffer.push((event.entity, StartSound(sound.clone())));
						}

						if sector_move.target == plat_active.high_height {
							command_buffer.remove_component::<FloorMove>(event.entity);
							command_buffer.remove_component::<PlatActive>(event.entity);
						} else {
							plat_active.wait_timer.restart(**game_time);
						}
					}
				}
			}
		})
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlatSwitchUse {
	pub params: PlatParams,
	pub switch_params: SwitchParams,
}

pub fn plat_switch_use(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<PlatSwitchUse>("PlatSwitchUse".into());
	handler_set.register_clone::<PlatSwitchUse>();

	SystemBuilder::new("plat_switch_use")
		.read_resource::<AssetStorage>()
		.read_resource::<GameTime>()
		.with_query(<(Entity, &UseEvent, &PlatSwitchUse)>::query())
		.with_query(<&LinedefRef>::query().filter(!component::<SwitchActive>()))
		.with_query(<&mut MapDynamic>::query())
		.read_component::<PlatActive>() // used by activate_with_tag
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, game_time) = resources;
			let (mut world2, world) = world.split_for_query(&queries.2);

			for (&entity, use_event, plat_switch_use) in queries.0.iter(&world) {
				command_buffer.remove(entity);

				if let Ok(linedef_ref) = queries.1.get(&world, use_event.entity) {
					let map_dynamic = queries
						.2
						.get_mut(&mut world2, linedef_ref.map_entity)
						.unwrap();
					let map = asset_storage.get(&map_dynamic.map).unwrap();
					let linedef = &map.linedefs[linedef_ref.index];

					let activated = activate_with_tag(
						&plat_switch_use.params,
						command_buffer,
						**game_time,
						linedef.sector_tag,
						&world,
						map,
						map_dynamic,
					);

					if activated {
						crate::doom::switch::activate(
							&plat_switch_use.switch_params,
							command_buffer,
							**game_time,
							linedef_ref.index,
							map,
							map_dynamic,
						);

						if plat_switch_use.switch_params.retrigger_time.is_none() {
							command_buffer.remove_component::<Usable>(use_event.entity);
						}
					}
				}
			}
		})
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlatLinedefTouch {
	pub params: PlatParams,
	pub retrigger: bool,
}

pub fn plat_linedef_touch(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<PlatLinedefTouch>("PlatLinedefTouch".into());
	handler_set.register_clone::<PlatLinedefTouch>();

	SystemBuilder::new("plat_linedef_touch")
		.read_resource::<AssetStorage>()
		.read_resource::<GameTime>()
		.with_query(<(Entity, &TouchEvent, &PlatLinedefTouch)>::query())
		.with_query(<&LinedefRef>::query())
		.with_query(<&mut MapDynamic>::query())
		.read_component::<PlatActive>() // used by activate_with_tag
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, game_time) = resources;
			let (mut world2, world) = world.split_for_query(&queries.2);

			for (&entity, touch_event, plat_linedef_touch) in queries.0.iter(&world) {
				command_buffer.remove(entity);

				if touch_event.collision.is_some() {
					continue;
				}

				if let Ok(linedef_ref) = queries.1.get(&world, touch_event.entity) {
					let map_dynamic = queries
						.2
						.get_mut(&mut world2, linedef_ref.map_entity)
						.unwrap();
					let map = asset_storage.get(&map_dynamic.map).unwrap();
					let linedef = &map.linedefs[linedef_ref.index];

					if activate_with_tag(
						&plat_linedef_touch.params,
						command_buffer,
						**game_time,
						linedef.sector_tag,
						&world,
						map,
						map_dynamic,
					) {
						if !plat_linedef_touch.retrigger {
							command_buffer.remove_component::<Touchable>(touch_event.entity);
						}
					}
				}
			}
		})
}

fn activate(
	params: &PlatParams,
	command_buffer: &mut CommandBuffer,
	game_time: GameTime,
	sector_index: usize,
	map: &Map,
	map_dynamic: &MapDynamic,
) {
	let sector_dynamic = &map_dynamic.sectors[sector_index];

	let low_height = match params.low_height_base {
		PlatTargetHeight::Current => sector_dynamic.interval.min + params.low_height_offset,
		PlatTargetHeight::LowestNeighbourFloor => {
			map.lowest_neighbour_floor(map_dynamic, sector_index) + params.low_height_offset
		}
	};

	let high_height = match params.high_height_base {
		PlatTargetHeight::Current => sector_dynamic.interval.min + params.high_height_offset,
		PlatTargetHeight::LowestNeighbourFloor => {
			map.lowest_neighbour_floor(map_dynamic, sector_index) + params.high_height_offset
		}
	};

	command_buffer.add_component(
		sector_dynamic.entity,
		FloorMove(SectorMove {
			velocity: 0.0,
			target: sector_dynamic.interval.min,
			sound: params.move_sound.clone(),
			sound_timer: Timer::new(game_time, params.move_sound_time),
		}),
	);

	command_buffer.add_component(
		sector_dynamic.entity,
		PlatActive {
			speed: params.speed,
			wait_timer: Timer::new_elapsed(game_time, params.wait_time),
			can_reverse: params.can_reverse,

			start_sound: params.start_sound.clone(),
			finish_sound: params.finish_sound.clone(),

			high_height,
			low_height,
		},
	);
}

fn activate_with_tag<W: EntityStore>(
	params: &PlatParams,
	command_buffer: &mut CommandBuffer,
	game_time: GameTime,
	sector_tag: u16,
	world: &W,
	map: &Map,
	map_dynamic: &MapDynamic,
) -> bool {
	let mut activated = false;

	// Activate all the doors with the same tag
	for (sector_index, _) in map
		.sectors
		.iter()
		.enumerate()
		.filter(|(_, s)| s.sector_tag == sector_tag)
	{
		let sector_entity = map_dynamic.sectors[sector_index].entity;

		if world
			.entry_ref(sector_entity)
			.unwrap()
			.get_component::<PlatActive>()
			.is_ok()
		{
			continue;
		}

		activated = true;
		activate(
			params,
			command_buffer,
			game_time,
			sector_index,
			map,
			map_dynamic,
		);
	}

	activated
}
