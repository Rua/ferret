use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		frame::FrameState,
		geometry::Side,
		spawn::SpawnMergerHandlerSet,
		time::Timer,
	},
	doom::{
		client::{UseAction, UseEvent},
		map::{LinedefRef, Map, MapDynamic},
		physics::{TouchEvent, Touchable},
		sectormove::{CeilingMove, SectorMove, SectorMoveEvent, SectorMoveEventType},
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
pub struct DoorActive {
	pub state: DoorState,
	pub end_state: DoorState,
	pub speed: f32,
	pub wait_timer: Timer,
	pub can_reverse: bool,

	pub open_sound: Option<AssetHandle<Sound>>,
	pub open_height: f32,

	pub close_sound: Option<AssetHandle<Sound>>,
	pub close_height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DoorState {
	Closed,
	Opening,
	Open,
	Closing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DoorParams {
	pub start_state: DoorState,
	pub end_state: DoorState,
	pub speed: f32,
	pub wait_time: Duration,
	pub can_reverse: bool,

	pub open_sound: Option<AssetHandle<Sound>>,
	pub close_sound: Option<AssetHandle<Sound>>,
}

pub fn door_active_system(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry, mut sector_move_event_channel) = <(
		Write<SpawnMergerHandlerSet>,
		Write<Registry<String>>,
		Write<EventChannel<SectorMoveEvent>>,
	)>::fetch_mut(resources);

	registry.register::<DoorActive>("DoorActive".into());
	handler_set.register_clone::<DoorActive>();

	let mut sector_move_event_reader = sector_move_event_channel.register_reader();

	SystemBuilder::new("door_active_system")
		.read_resource::<FrameState>()
		.read_resource::<EventChannel<SectorMoveEvent>>()
		.with_query(<(Entity, &mut CeilingMove, &mut DoorActive)>::query())
		.build(move |command_buffer, world, resources, query| {
			let (frame_state, sector_move_event_channel) = resources;

			for (&entity, ceiling_move, mut door_active) in query.iter_mut(world) {
				let sector_move = &mut ceiling_move.0;

				if sector_move.velocity != 0.0 {
					continue;
				}

				if door_active.wait_timer.is_elapsed(frame_state.time) {
					let sound = if sector_move.target == door_active.close_height {
						door_active.state = DoorState::Opening;
						sector_move.velocity = door_active.speed;
						sector_move.target = door_active.open_height;
						&door_active.open_sound
					} else {
						door_active.state = DoorState::Closing;
						sector_move.velocity = -door_active.speed;
						sector_move.target = door_active.close_height;
						&door_active.close_sound
					};

					if let Some(sound) = sound {
						command_buffer.push((entity, StartSound(sound.clone())));
					}
				}
			}

			for event in sector_move_event_channel
				.read(&mut sector_move_event_reader)
				.filter(|e| e.normal == -1.0)
			{
				let (ceiling_move, door_active) = match query.get_mut(world, event.entity) {
					Ok((_, ceiling_move, door_active)) => (ceiling_move, door_active),
					_ => continue,
				};

				let sector_move = &mut ceiling_move.0;

				if sector_move.velocity == 0.0 {
					continue;
				}

				match event.event_type {
					SectorMoveEventType::Collided => {
						if door_active.can_reverse {
							sector_move.velocity = -sector_move.velocity;

							let sound = if sector_move.velocity > 0.0 {
								door_active.state = DoorState::Opening;
								sector_move.target = door_active.open_height;
								&door_active.open_sound
							} else {
								door_active.state = DoorState::Closing;
								sector_move.target = door_active.close_height;
								&door_active.close_sound
							};

							if let Some(sound) = sound {
								command_buffer.push((event.entity, StartSound(sound.clone())));
							}
						}
					}
					SectorMoveEventType::TargetReached => {
						sector_move.velocity = 0.0;

						if sector_move.target == door_active.open_height {
							door_active.state = DoorState::Open;
						} else {
							door_active.state = DoorState::Closed;
						}

						if door_active.state == door_active.end_state {
							command_buffer.remove_component::<CeilingMove>(event.entity);
							command_buffer.remove_component::<DoorActive>(event.entity);
						} else {
							door_active.wait_timer.restart(frame_state.time);
						}
					}
				}
			}
		})
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DoorUse {
	pub params: DoorParams,
	pub retrigger: bool,
}

pub fn door_use_system(resources: &mut Resources) -> impl Runnable {
	let mut use_event_reader = resources
		.get_mut::<EventChannel<UseEvent>>()
		.unwrap()
		.register_reader();

	SystemBuilder::new("door_use_system")
		.read_resource::<AssetStorage>()
		.read_resource::<EventChannel<UseEvent>>()
		.read_resource::<FrameState>()
		.with_query(<(&LinedefRef, &UseAction)>::query())
		.with_query(<&MapDynamic>::query())
		.with_query(<(&mut CeilingMove, &mut DoorActive)>::query())
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, use_event_channel, frame_state) = resources;
			let (mut world2, world) = world.split_for_query(&queries.2);

			for use_event in use_event_channel.read(&mut use_event_reader) {
				let (linedef_ref, door_use) = match queries.0.get(&world, use_event.linedef_entity)
				{
					Ok((linedef_ref, UseAction::DoorUse(door_use))) => (linedef_ref, door_use),
					_ => continue,
				};

				let map_dynamic = queries.1.get(&world, linedef_ref.map_entity).unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let linedef = &map.linedefs[linedef_ref.index];

				let back_sidedef = match &linedef.sidedefs[Side::Left as usize] {
					Some(back_sidedef) => back_sidedef,
					_ => {
						log::error!("Used door linedef {} has no back sector", linedef_ref.index);
						continue;
					}
				};

				let sector_index = back_sidedef.sector_index;
				let sector_entity = map_dynamic.sectors[sector_index].entity;

				if let Ok((ceiling_move, door_active)) =
					queries.2.get_mut(&mut world2, sector_entity)
				{
					let sector_move = &mut ceiling_move.0;

					if door_use.params.can_reverse {
						door_active.wait_timer.set_target(frame_state.time);
						sector_move.velocity = 0.0;

						if sector_move.velocity < 0.0
							|| sector_move.velocity == 0.0
								&& sector_move.target == door_active.close_height
						{
							// Re-open the door
							door_active.state = DoorState::Closed;
						} else {
							// Close the door early
							door_active.state = DoorState::Open;
						}
					}
				} else {
					activate(
						&door_use.params,
						command_buffer,
						frame_state,
						sector_index,
						&map,
						&map_dynamic,
					);

					if !door_use.retrigger {
						command_buffer.remove_component::<UseAction>(use_event.linedef_entity);
					}
				}
			}
		})
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DoorSwitchUse {
	pub params: DoorParams,
	pub switch_params: SwitchParams,
}

pub fn door_switch_system(resources: &mut Resources) -> impl Runnable {
	let mut use_event_reader = resources
		.get_mut::<EventChannel<UseEvent>>()
		.unwrap()
		.register_reader();

	SystemBuilder::new("door_switch_system")
		.read_resource::<AssetStorage>()
		.read_resource::<EventChannel<UseEvent>>()
		.read_resource::<FrameState>()
		.with_query(<(&LinedefRef, &UseAction)>::query().filter(!component::<SwitchActive>()))
		.with_query(<&mut MapDynamic>::query())
		.read_component::<DoorActive>() // used by activate_with_tag
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, use_event_channel, frame_state) = resources;
			let (mut world1, world) = world.split_for_query(&queries.1);

			for use_event in use_event_channel.read(&mut use_event_reader) {
				let (linedef_ref, door_switch_use) =
					match queries.0.get(&world, use_event.linedef_entity) {
						Ok((linedef_ref, UseAction::DoorSwitchUse(door_switch_use))) => {
							(linedef_ref, door_switch_use)
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
					&door_switch_use.params,
					command_buffer,
					frame_state,
					linedef.sector_tag,
					&world,
					map,
					map_dynamic,
				);

				if activated {
					crate::doom::switch::activate(
						&door_switch_use.switch_params,
						command_buffer,
						frame_state,
						linedef_ref.index,
						map,
						map_dynamic,
					);

					if door_switch_use.switch_params.retrigger_time.is_none() {
						command_buffer.remove_component::<UseAction>(use_event.linedef_entity);
					}
				}
			}
		})
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DoorLinedefTouch {
	pub params: DoorParams,
	pub retrigger: bool,
}

pub fn door_linedef_touch(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<DoorLinedefTouch>("DoorLinedefTouch".into());
	handler_set.register_clone::<DoorLinedefTouch>();

	SystemBuilder::new("door_linedef_touch")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameState>()
		.with_query(<(Entity, &TouchEvent, &DoorLinedefTouch)>::query())
		.with_query(<&LinedefRef>::query())
		.with_query(<&mut MapDynamic>::query())
		.read_component::<DoorActive>() // used by activate_with_tag
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, frame_state) = resources;

			let (world0, mut world) = world.split_for_query(&queries.0);
			let (mut world1, mut world) = world.split_for_query(&queries.1);
			let (mut world2, world) = world.split_for_query(&queries.2);

			for (&entity, touch_event, door_linedef_touch) in queries.0.iter(&world0) {
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
						&door_linedef_touch.params,
						command_buffer,
						frame_state,
						linedef.sector_tag,
						&world,
						map,
						map_dynamic,
					) {
						if !door_linedef_touch.retrigger {
							command_buffer.remove_component::<Touchable>(touch_event.entity);
						}
					}
				}
			}
		})
}

fn activate(
	params: &DoorParams,
	command_buffer: &mut CommandBuffer,
	frame_state: &FrameState,
	sector_index: usize,
	map: &Map,
	map_dynamic: &MapDynamic,
) {
	let sector_dynamic = &map_dynamic.sectors[sector_index];

	let open_height = if params.start_state == DoorState::Open {
		sector_dynamic.interval.max
	} else {
		map.lowest_neighbour_ceiling(map_dynamic, sector_index) - 4.0
	};

	let close_height = sector_dynamic.interval.min;

	command_buffer.add_component(
		sector_dynamic.entity,
		CeilingMove(SectorMove {
			velocity: 0.0,
			target: sector_dynamic.interval.max,
			sound: None,
			sound_timer: Timer::new_elapsed(frame_state.time, Duration::default()),
		}),
	);

	command_buffer.add_component(
		sector_dynamic.entity,
		DoorActive {
			state: params.start_state,
			end_state: params.end_state,
			speed: params.speed,
			wait_timer: Timer::new_elapsed(frame_state.time, params.wait_time),
			can_reverse: params.can_reverse,

			open_sound: params.open_sound.clone(),
			open_height,

			close_sound: params.close_sound.clone(),
			close_height,
		},
	);
}

fn activate_with_tag<W: EntityStore>(
	params: &DoorParams,
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
			.get_component::<DoorActive>()
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
