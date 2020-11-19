use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		frame::FrameState,
		spawn::SpawnMergerHandlerSet,
		time::Timer,
	},
	doom::{
		client::{UseAction, UseEvent},
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
	Entity, EntityStore, IntoQuery, Resources, SystemBuilder, Write,
};
use shrev::EventChannel;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct FloorActive {
	pub finish_sound: Option<AssetHandle<Sound>>,
}

#[derive(Clone, Debug)]
pub struct FloorParams {
	pub speed: f32,
	pub target_height_base: FloorTargetHeight,
	pub target_height_offset: f32,
	pub move_sound: Option<AssetHandle<Sound>>,
	pub move_sound_time: Duration,
	pub finish_sound: Option<AssetHandle<Sound>>,
}

#[derive(Clone, Copy, Debug)]
pub enum FloorTargetHeight {
	Current,
	LowestNeighbourFloor,
	LowestNeighbourFloorAbove,
	LowestNeighbourCeiling,
	HighestNeighbourFloor,
}

pub fn floor_active_system(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut sector_move_event_channel) = <(
		Write<SpawnMergerHandlerSet>,
		Write<EventChannel<SectorMoveEvent>>,
	)>::fetch_mut(resources);

	handler_set.register_clone::<FloorActive>();
	let mut sector_move_event_reader = sector_move_event_channel.register_reader();

	SystemBuilder::new("floor_active_system")
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

#[derive(Clone, Debug)]
pub struct FloorSwitchUse {
	pub params: FloorParams,
	pub switch_params: SwitchParams,
}

pub fn floor_switch_system(resources: &mut Resources) -> impl Runnable {
	let mut use_event_reader = resources
		.get_mut::<EventChannel<UseEvent>>()
		.unwrap()
		.register_reader();

	SystemBuilder::new("floor_switch_system")
		.read_resource::<AssetStorage>()
		.read_resource::<EventChannel<UseEvent>>()
		.read_resource::<FrameState>()
		.with_query(<(&LinedefRef, &UseAction)>::query().filter(!component::<SwitchActive>()))
		.with_query(<&mut MapDynamic>::query())
		.read_component::<FloorActive>() // used by activate_with_tag
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, use_event_channel, frame_state) = resources;
			let (mut world1, world) = world.split_for_query(&queries.1);

			for use_event in use_event_channel.read(&mut use_event_reader) {
				let (linedef_ref, floor_switch_use) =
					match queries.0.get(&world, use_event.linedef_entity) {
						Ok((linedef_ref, UseAction::FloorSwitchUse(floor_switch_use))) => {
							(linedef_ref, floor_switch_use)
						}
						_ => continue,
					};

				let map_dynamic = queries
					.1
					.get_mut(&mut world1, linedef_ref.map_entity)
					.unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let linedef = &map.linedefs[linedef_ref.index];

				let activated = activate_with_tag(
					&floor_switch_use.params,
					command_buffer,
					frame_state,
					linedef.sector_tag,
					&world,
					map,
					map_dynamic,
				);

				if activated {
					crate::doom::switch::activate(
						&floor_switch_use.switch_params,
						command_buffer,
						frame_state,
						linedef_ref.index,
						map,
						map_dynamic,
					);

					if floor_switch_use.switch_params.retrigger_time.is_none() {
						command_buffer.remove_component::<UseAction>(use_event.linedef_entity);
					}
				}
			}
		})
}

#[derive(Clone, Debug)]
pub struct FloorLinedefTouch {
	pub params: FloorParams,
	pub retrigger: bool,
}

pub fn floor_linedef_touch(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<FloorLinedefTouch>();

	SystemBuilder::new("floor_linedef_touch")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameState>()
		.with_query(<(Entity, &TouchEvent, &FloorLinedefTouch)>::query())
		.with_query(<&LinedefRef>::query())
		.with_query(<&mut MapDynamic>::query())
		.read_component::<FloorActive>() // used by activate_with_tag
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, frame_state) = resources;

			let (world0, mut world) = world.split_for_query(&queries.0);
			let (mut world1, mut world) = world.split_for_query(&queries.1);
			let (mut world2, world) = world.split_for_query(&queries.2);

			for (&entity, touch_event, floor_linedef_touch) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if touch_event.collision.is_some() {
					continue;
				}

				if let Ok(linedef_ref) = queries.1.get_mut(&mut world1, touch_event.entity) {
					let map_dynamic = queries
						.2
						.get_mut(&mut world2, linedef_ref.map_entity)
						.unwrap();
					let map = asset_storage.get(&map_dynamic.map).unwrap();
					let linedef = &map.linedefs[linedef_ref.index];

					if activate_with_tag(
						&floor_linedef_touch.params,
						command_buffer,
						frame_state,
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
	frame_state: &FrameState,
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
			sound_timer: Timer::new_elapsed(frame_state.time, params.move_sound_time),
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
	frame_state: &FrameState,
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
			frame_state,
			sector_index,
			map,
			map_dynamic,
		);
	}

	activated
}
