use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		frame::FrameState,
		spawn::SpawnMergerHandlerSet,
		time::Timer,
	},
	doom::{
		client::{UseAction, UseEvent},
		components::Transform,
		map::{LinedefRef, Map, MapDynamic},
		physics::{BoxCollider, TouchAction, TouchEvent},
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
pub struct PlatActive {
	pub speed: f32,
	pub wait_timer: Timer,
	pub can_reverse: bool,

	pub start_sound: Option<AssetHandle<Sound>>,
	pub finish_sound: Option<AssetHandle<Sound>>,

	pub low_height: f32,
	pub high_height: f32,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub enum PlatTargetHeight {
	Current,
	LowestNeighbourFloor,
}

pub fn plat_active_system(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut sector_move_event_channel) = <(
		Write<SpawnMergerHandlerSet>,
		Write<EventChannel<SectorMoveEvent>>,
	)>::fetch_mut(resources);

	handler_set.register_clone::<PlatActive>();
	let mut sector_move_event_reader = sector_move_event_channel.register_reader();

	SystemBuilder::new("plat_active_system")
		.read_resource::<FrameState>()
		.read_resource::<EventChannel<SectorMoveEvent>>()
		.with_query(<(Entity, &mut FloorMove, &mut PlatActive)>::query())
		.read_component::<BoxCollider>() // used by SectorTracer
		.read_component::<Transform>() // used by SectorTracer
		.build(move |command_buffer, world, resources, query| {
			let (frame_state, sector_move_event_channel) = resources;

			for (&entity, floor_move, plat_active) in query.iter_mut(world) {
				let sector_move = &mut floor_move.0;

				if sector_move.velocity != 0.0 {
					continue;
				}

				if plat_active.wait_timer.is_elapsed(frame_state.time) {
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
							plat_active.wait_timer.restart(frame_state.time);
						}
					}
				}
			}
		})
}

#[derive(Clone, Debug)]
pub struct PlatSwitchUse {
	pub params: PlatParams,
	pub switch_params: SwitchParams,
}

pub fn plat_switch_system(resources: &mut Resources) -> impl Runnable {
	let mut use_event_reader = resources
		.get_mut::<EventChannel<UseEvent>>()
		.unwrap()
		.register_reader();

	SystemBuilder::new("plat_switch_system")
		.read_resource::<AssetStorage>()
		.read_resource::<EventChannel<UseEvent>>()
		.read_resource::<FrameState>()
		.with_query(<(&LinedefRef, &UseAction)>::query().filter(!component::<SwitchActive>()))
		.with_query(<&mut MapDynamic>::query())
		.read_component::<PlatActive>() // used by activate_with_tag
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, use_event_channel, frame_state) = resources;
			let (mut world1, world) = world.split_for_query(&queries.1);

			for use_event in use_event_channel.read(&mut use_event_reader) {
				let (linedef_ref, plat_switch_use) =
					match queries.0.get(&world, use_event.linedef_entity) {
						Ok((linedef_ref, UseAction::PlatSwitchUse(plat_switch_use))) => {
							(linedef_ref, plat_switch_use)
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
					&plat_switch_use.params,
					command_buffer,
					frame_state,
					linedef.sector_tag,
					&world,
					map,
					map_dynamic,
				);

				if activated {
					crate::doom::switch::activate(
						&plat_switch_use.switch_params,
						command_buffer,
						frame_state,
						linedef_ref.index,
						map,
						map_dynamic,
					);

					if plat_switch_use.switch_params.retrigger_time.is_none() {
						command_buffer.remove_component::<UseAction>(use_event.linedef_entity);
					}
				}
			}
		})
}

#[derive(Clone, Debug)]
pub struct PlatTouch {
	pub params: PlatParams,
	pub retrigger: bool,
}

pub fn plat_touch_system(resources: &mut Resources) -> impl Runnable {
	let mut touch_event_reader = resources
		.get_mut::<EventChannel<TouchEvent>>()
		.unwrap()
		.register_reader();

	SystemBuilder::new("plat_touch_system")
		.read_resource::<AssetStorage>()
		.read_resource::<EventChannel<TouchEvent>>()
		.read_resource::<FrameState>()
		.with_query(<(&LinedefRef, &TouchAction)>::query())
		.with_query(<&mut MapDynamic>::query())
		.read_component::<PlatActive>() // used by activate_with_tag
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, touch_event_channel, frame_state) = resources;

			let (mut world0, mut world) = world.split_for_query(&queries.0);
			let (mut world1, world) = world.split_for_query(&queries.1);

			for touch_event in touch_event_channel.read(&mut touch_event_reader) {
				if touch_event.collision.is_some() {
					continue;
				}

				let (linedef_ref, plat_touch) =
					match queries.0.get_mut(&mut world0, touch_event.touched) {
						Ok((linedef_ref, TouchAction::PlatTouch(plat_touch))) => {
							(linedef_ref, plat_touch)
						}
						_ => continue,
					};

				let map_dynamic = queries
					.1
					.get_mut(&mut world1, linedef_ref.map_entity)
					.unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let linedef = &map.linedefs[linedef_ref.index];

				if activate_with_tag(
					&plat_touch.params,
					command_buffer,
					frame_state,
					linedef.sector_tag,
					&world,
					map,
					map_dynamic,
				) {
					if !plat_touch.retrigger {
						command_buffer.remove_component::<TouchAction>(touch_event.touched);
					}
				}
			}
		})
}

fn activate(
	params: &PlatParams,
	command_buffer: &mut CommandBuffer,
	frame_state: &FrameState,
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
			sound_timer: Timer::new(frame_state.time, params.move_sound_time),
		}),
	);

	command_buffer.add_component(
		sector_dynamic.entity,
		PlatActive {
			speed: params.speed,
			wait_timer: Timer::new_elapsed(frame_state.time, params.wait_time),
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
			.get_component::<PlatActive>()
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
