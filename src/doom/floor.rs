use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		spawn::SpawnMergerHandlerSet,
		time::{GameTime, Timer},
	},
	doom::{
		client::{Usable, UseEvent},
		map::{LinedefRef, Map, MapDynamic},
		physics::{TouchEvent, Touchable},
		sectormove::{FloorMove, SectorMove, SectorMoveEvent, SectorMoveEventType},
		sound::{Sound, StartSound},
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
pub struct FloorActive {
	pub finish_sound: Option<AssetHandle<Sound>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FloorParams {
	pub speed: f32,
	pub target_height_base: FloorTargetHeight,
	pub target_height_offset: f32,
	pub move_sound: Option<AssetHandle<Sound>>,
	pub move_sound_time: Duration,
	pub finish_sound: Option<AssetHandle<Sound>>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum FloorTargetHeight {
	Current,
	LowestNeighbourFloor,
	LowestNeighbourFloorAbove,
	LowestNeighbourCeiling,
	HighestNeighbourFloor,
}

pub fn floor_active(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry, mut sector_move_event_channel) = <(
		Write<SpawnMergerHandlerSet>,
		Write<Registry<String>>,
		Write<EventChannel<SectorMoveEvent>>,
	)>::fetch_mut(resources);

	registry.register::<FloorActive>("FloorActive".into());
	handler_set.register_clone::<FloorActive>();

	let mut sector_move_event_reader = sector_move_event_channel.register_reader();

	SystemBuilder::new("floor_active")
		.read_resource::<EventChannel<SectorMoveEvent>>()
		.with_query(<(&mut FloorMove, &mut FloorActive)>::query())
		.build(move |command_buffer, world, resources, query| {
			let sector_move_event_channel = resources;

			for event in sector_move_event_channel
				.read(&mut sector_move_event_reader)
				.filter(|e| e.normal == 1.0)
			{
				let (floor_move, floor_active) = match query.get_mut(world, event.entity) {
					Ok(x) => x,
					_ => continue,
				};

				let sector_move = &floor_move.0;

				if sector_move.velocity == 0.0 {
					continue;
				}

				match event.event_type {
					SectorMoveEventType::Collided => {
						// Hang there until the obstruction is gone
					}
					SectorMoveEventType::TargetReached => {
						if let Some(sound) = &floor_active.finish_sound {
							command_buffer.push((event.entity, StartSound(sound.clone())));
						}

						command_buffer.remove_component::<FloorMove>(event.entity);
						command_buffer.remove_component::<FloorActive>(event.entity);
					}
				}
			}
		})
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FloorSwitchUse {
	pub params: FloorParams,
	pub switch_params: SwitchParams,
}

pub fn floor_switch_use(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<FloorSwitchUse>("FloorSwitchUse".into());
	handler_set.register_clone::<FloorSwitchUse>();

	SystemBuilder::new("floor_switch_use")
		.read_resource::<AssetStorage>()
		.read_resource::<GameTime>()
		.with_query(<(Entity, &UseEvent, &FloorSwitchUse)>::query())
		.with_query(<&LinedefRef>::query().filter(!component::<SwitchActive>()))
		.with_query(<&mut MapDynamic>::query())
		.read_component::<FloorActive>() // used by activate_with_tag
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, game_time) = resources;
			let (mut world2, world) = world.split_for_query(&queries.2);

			for (&entity, use_event, floor_switch_use) in queries.0.iter(&world) {
				command_buffer.remove(entity);

				if let Ok(linedef_ref) = queries.1.get(&world, use_event.entity) {
					let map_dynamic = queries
						.2
						.get_mut(&mut world2, linedef_ref.map_entity)
						.unwrap();
					let map = asset_storage.get(&map_dynamic.map).unwrap();
					let linedef = &map.linedefs[linedef_ref.index];

					let activated = activate_with_tag(
						&floor_switch_use.params,
						command_buffer,
						**game_time,
						linedef.sector_tag,
						&world,
						map,
						map_dynamic,
					);

					if activated {
						crate::doom::switch::activate(
							&floor_switch_use.switch_params,
							command_buffer,
							**game_time,
							linedef_ref.index,
							map,
							map_dynamic,
						);

						if floor_switch_use.switch_params.retrigger_time.is_none() {
							command_buffer.remove_component::<Usable>(use_event.entity);
						}
					}
				}
			}
		})
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FloorLinedefTouch {
	pub params: FloorParams,
	pub retrigger: bool,
}

pub fn floor_linedef_touch(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<FloorLinedefTouch>("FloorLinedefTouch".into());
	handler_set.register_clone::<FloorLinedefTouch>();

	SystemBuilder::new("floor_linedef_touch")
		.read_resource::<AssetStorage>()
		.read_resource::<GameTime>()
		.with_query(<(Entity, &TouchEvent, &FloorLinedefTouch)>::query())
		.with_query(<&LinedefRef>::query())
		.with_query(<&mut MapDynamic>::query())
		.read_component::<FloorActive>() // used by activate_with_tag
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, game_time) = resources;
			let (mut world2, world) = world.split_for_query(&queries.2);

			for (&entity, touch_event, floor_linedef_touch) in queries.0.iter(&world) {
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
						&floor_linedef_touch.params,
						command_buffer,
						**game_time,
						linedef.sector_tag,
						&world,
						map,
						map_dynamic,
					) {
						if !floor_linedef_touch.retrigger {
							command_buffer.remove_component::<Touchable>(touch_event.entity);
						}
					}
				}
			}
		})
}

fn activate(
	params: &FloorParams,
	command_buffer: &mut CommandBuffer,
	game_time: GameTime,
	sector_index: usize,
	map: &Map,
	map_dynamic: &MapDynamic,
) {
	let sector_dynamic = &map_dynamic.sectors[sector_index];

	let target = match params.target_height_base {
		FloorTargetHeight::Current => sector_dynamic.interval.min + params.target_height_offset,
		FloorTargetHeight::LowestNeighbourFloor => {
			map.lowest_neighbour_floor(map_dynamic, sector_index) + params.target_height_offset
		}
		FloorTargetHeight::LowestNeighbourFloorAbove => {
			map.lowest_neighbour_floor_above(map_dynamic, sector_index, sector_dynamic.interval.min)
				+ params.target_height_offset
		}
		FloorTargetHeight::LowestNeighbourCeiling => {
			let mut target_height = map.lowest_neighbour_ceiling(map_dynamic, sector_index);

			if target_height > sector_dynamic.interval.min {
				target_height = sector_dynamic.interval.min;
			}

			target_height + params.target_height_offset
		}
		FloorTargetHeight::HighestNeighbourFloor => {
			let target_height = map.highest_neighbour_floor(map_dynamic, sector_index);

			if target_height != sector_dynamic.interval.min {
				target_height + params.target_height_offset
			} else {
				target_height
			}
		}
	};

	let direction = if target < sector_dynamic.interval.min {
		-1.0
	} else {
		1.0
	};

	command_buffer.add_component(
		sector_dynamic.entity,
		FloorMove(SectorMove {
			velocity: direction * params.speed,
			target,
			sound: params.move_sound.clone(),
			sound_timer: Timer::new_elapsed(game_time, params.move_sound_time),
		}),
	);

	command_buffer.add_component(
		sector_dynamic.entity,
		FloorActive {
			finish_sound: params.finish_sound.clone(),
		},
	);
}

fn activate_with_tag<W: EntityStore>(
	params: &FloorParams,
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
			.get_component::<FloorActive>()
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
