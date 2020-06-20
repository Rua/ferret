use crate::{
	assets::{AssetHandle, AssetStorage},
	audio::Sound,
	doom::{
		client::{UseAction, UseEvent},
		components::Transform,
		map::{LinedefRef, Map, MapDynamic, SectorRef},
		physics::{BoxCollider, SectorTracer, TouchAction, TouchEvent},
		switch::{SwitchActive, SwitchParams},
	},
	geometry::Side,
};
use legion::prelude::{
	CommandBuffer, Entity, EntityStore, IntoQuery, Read, Resources, Runnable, SystemBuilder, Write,
};
use shrev::EventChannel;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct DoorActive {
	pub state: DoorState,
	pub end_state: DoorState,
	pub speed: f32,
	pub wait_time: Duration,
	pub time_left: Duration,
	pub can_reverse: bool,

	pub open_sound: AssetHandle<Sound>,
	pub open_height: f32,

	pub close_sound: AssetHandle<Sound>,
	pub close_height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoorState {
	Closed,
	Opening,
	Open,
	Closing,
}

#[derive(Clone, Debug)]
pub struct DoorParams {
	pub start_state: DoorState,
	pub end_state: DoorState,
	pub speed: f32,
	pub wait_time: Duration,
	pub can_reverse: bool,

	pub open_sound: AssetHandle<Sound>,
	pub close_sound: AssetHandle<Sound>,
}

pub fn door_active_system() -> Box<dyn Runnable> {
	SystemBuilder::new("door_active_system")
		.read_resource::<AssetStorage>()
		.read_resource::<Duration>()
		.write_resource::<Vec<(AssetHandle<Sound>, Entity)>>()
		.with_query(<(Read<SectorRef>, Write<DoorActive>)>::query())
		.read_component::<BoxCollider>() // used by SectorTracer
		.read_component::<Transform>() // used by SectorTracer
		.write_component::<MapDynamic>()
		.build_thread_local(move |command_buffer, world, resources, query| {
			let (asset_storage, delta, sound_queue) = resources;
			let (mut map_dynamic_world, mut world) = world.split::<Write<MapDynamic>>();
			let (mut query_world, world) = world.split_for_query(&query);
			let tracer = SectorTracer { world: &world };

			for (entity, (sector_ref, mut door_active)) in query.iter_entities_mut(&mut query_world)
			{
				let mut map_dynamic = map_dynamic_world
					.get_component_mut::<MapDynamic>(sector_ref.map_entity)
					.unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let sector = &map.sectors[sector_ref.index];
				let sector_dynamic = &mut map_dynamic.sectors[sector_ref.index];

				let new_state = match door_active.state {
					DoorState::Closed => {
						if let Some(new_time) = door_active.time_left.checked_sub(**delta) {
							door_active.time_left = new_time;
							None
						} else {
							if sector_dynamic.interval.max == door_active.open_height {
								// Already open
								Some(DoorState::Open)
							} else {
								Some(DoorState::Opening)
							}
						}
					}
					DoorState::Opening => {
						let move_step = door_active.speed * delta.as_secs_f32();
						sector_dynamic.interval.max += move_step;

						if sector_dynamic.interval.max > door_active.open_height {
							sector_dynamic.interval.max = door_active.open_height;
							Some(DoorState::Open)
						} else {
							None
						}
					}
					DoorState::Open => {
						if let Some(new_time) = door_active.time_left.checked_sub(**delta) {
							door_active.time_left = new_time;
							None
						} else {
							if sector_dynamic.interval.max == door_active.close_height {
								// Already closed
								Some(DoorState::Closed)
							} else {
								Some(DoorState::Closing)
							}
						}
					}
					DoorState::Closing => {
						// Check if the door bumped something on the way down
						let move_step = -door_active.speed * delta.as_secs_f32();
						let trace = tracer.trace(
							-sector_dynamic.interval.max,
							-1.0,
							move_step,
							sector.subsectors.iter().map(|i| &map.subsectors[*i]),
						);

						// TODO use fraction
						if trace.collision.is_some() {
							if door_active.can_reverse {
								// Re-open the door
								Some(DoorState::Opening)
							} else {
								// Hang there until the obstruction is gone
								None
							}
						} else {
							sector_dynamic.interval.max += move_step;

							if sector_dynamic.interval.max <= door_active.close_height {
								sector_dynamic.interval.max = door_active.close_height;
								Some(DoorState::Closed)
							} else {
								None
							}
						}
					}
				};

				// State transition
				if let Some(new_state) = new_state {
					if new_state == door_active.end_state {
						command_buffer.remove_component::<DoorActive>(entity);
					} else {
						door_active.state = new_state;

						match new_state {
							DoorState::Opening => {
								sound_queue.push((door_active.open_sound.clone(), entity));
							}
							DoorState::Open => {
								door_active.time_left = door_active.wait_time;
							}
							DoorState::Closing => {
								sound_queue.push((door_active.close_sound.clone(), entity));
							}
							DoorState::Closed => {
								door_active.time_left = door_active.wait_time;
							}
						}
					}
				}
			}
		})
}

#[derive(Clone, Debug)]
pub struct DoorUse {
	pub params: DoorParams,
	pub retrigger: bool,
}

pub fn door_use_system(resources: &mut Resources) -> Box<dyn Runnable> {
	let mut use_event_reader = resources
		.get_mut::<EventChannel<UseEvent>>()
		.unwrap()
		.register_reader();

	SystemBuilder::new("door_use_system")
		.read_resource::<AssetStorage>()
		.read_resource::<EventChannel<UseEvent>>()
		.read_component::<LinedefRef>()
		.read_component::<MapDynamic>()
		.read_component::<UseAction>()
		.write_component::<DoorActive>()
		.build_thread_local(move |command_buffer, world, resources, _| {
			let (asset_storage, use_event_channel) = resources;
			let (mut door_active_world, world) = world.split::<Write<DoorActive>>();

			for use_event in use_event_channel.read(&mut use_event_reader) {
				let linedef_ref = world
					.get_component::<LinedefRef>(use_event.linedef_entity)
					.unwrap();
				let map_dynamic = world
					.get_component::<MapDynamic>(linedef_ref.map_entity)
					.unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let linedef = &map.linedefs[linedef_ref.index];

				if let Some(UseAction::DoorUse(door_use)) = world
					.get_component::<UseAction>(use_event.linedef_entity)
					.as_deref()
				{
					if let Some(back_sidedef) = &linedef.sidedefs[Side::Left as usize] {
						let sector_index = back_sidedef.sector_index;
						let sector_entity = map_dynamic.sectors[sector_index].entity;

						if let Some(mut door_active) =
							door_active_world.get_component_mut::<DoorActive>(sector_entity)
						{
							if door_use.params.can_reverse {
								door_active.time_left = Duration::default();

								match door_active.state {
									DoorState::Closing | DoorState::Closed => {
										// Re-open the door
										door_active.state = DoorState::Closed;
									}
									DoorState::Opening | DoorState::Open => {
										// Close the door early
										door_active.state = DoorState::Open;
									}
								}
							}
						} else {
							activate(
								&door_use.params,
								command_buffer,
								sector_index,
								&map,
								&map_dynamic,
							);

							if !door_use.retrigger {
								command_buffer
									.remove_component::<UseAction>(use_event.linedef_entity);
							}
						}
					} else {
						log::error!("Used door linedef {} has no back sector", linedef_ref.index);
					}
				}
			}
		})
}

#[derive(Clone, Debug)]
pub struct DoorSwitchUse {
	pub params: DoorParams,
	pub switch_params: SwitchParams,
}

pub fn door_switch_system(resources: &mut Resources) -> Box<dyn Runnable> {
	let mut use_event_reader = resources
		.get_mut::<EventChannel<UseEvent>>()
		.unwrap()
		.register_reader();

	SystemBuilder::new("door_switch_system")
		.read_resource::<AssetStorage>()
		.read_resource::<EventChannel<UseEvent>>()
		.write_resource::<Vec<(AssetHandle<Sound>, Entity)>>()
		.read_component::<DoorActive>() // used by activate_with_tag
		.read_component::<LinedefRef>()
		.read_component::<SwitchActive>()
		.read_component::<UseAction>()
		.write_component::<MapDynamic>()
		.build_thread_local(move |command_buffer, world, resources, _| {
			let (asset_storage, use_event_channel, sound_queue) = resources;
			let (mut map_dynamic_world, world) = world.split::<Write<MapDynamic>>();

			for use_event in use_event_channel.read(&mut use_event_reader) {
				let linedef_ref = world
					.get_component::<LinedefRef>(use_event.linedef_entity)
					.unwrap();
				let mut map_dynamic = map_dynamic_world
					.get_component_mut::<MapDynamic>(linedef_ref.map_entity)
					.unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let linedef = &map.linedefs[linedef_ref.index];

				if let Some(UseAction::DoorSwitchUse(door_use)) = world
					.get_component::<UseAction>(use_event.linedef_entity)
					.as_deref()
				{
					// Skip if switch is already in active state
					if world.has_component::<SwitchActive>(use_event.linedef_entity) {
						continue;
					}

					let activated = activate_with_tag(
						&door_use.params,
						command_buffer,
						linedef.sector_tag,
						&world,
						map,
						map_dynamic.as_ref(),
					);

					if activated {
						let activated = crate::doom::switch::activate(
							&door_use.switch_params,
							command_buffer,
							sound_queue.as_mut(),
							linedef_ref.index,
							map,
							map_dynamic.as_mut(),
						);

						if activated && door_use.switch_params.retrigger_time.is_none() {
							command_buffer.remove_component::<UseAction>(use_event.linedef_entity);
						}
					}
				}
			}
		})
}

#[derive(Clone, Debug)]
pub struct DoorTouch {
	pub params: DoorParams,
	pub retrigger: bool,
}

pub fn door_touch_system(resources: &mut Resources) -> Box<dyn Runnable> {
	let mut touch_event_reader = resources
		.get_mut::<EventChannel<TouchEvent>>()
		.unwrap()
		.register_reader();

	SystemBuilder::new("door_touch_system")
		.read_resource::<AssetStorage>()
		.read_resource::<EventChannel<TouchEvent>>()
		.read_component::<DoorActive>() // used by activate_with_tag
		.read_component::<LinedefRef>()
		.read_component::<TouchAction>()
		.write_component::<MapDynamic>()
		.build_thread_local(move |command_buffer, world, resources, _| {
			let (asset_storage, touch_event_channel) = resources;
			let (mut map_dynamic_world, world) = world.split::<Write<MapDynamic>>();

			for touch_event in touch_event_channel.read(&mut touch_event_reader) {
				if touch_event.collision.is_some() {
					continue;
				}

				let linedef_ref = if let Some(linedef_ref) =
					world.get_component::<LinedefRef>(touch_event.touched)
				{
					linedef_ref
				} else {
					continue;
				};
				let map_dynamic = map_dynamic_world
					.get_component_mut::<MapDynamic>(linedef_ref.map_entity)
					.unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let linedef = &map.linedefs[linedef_ref.index];

				match world
					.get_component::<TouchAction>(touch_event.touched)
					.as_deref()
				{
					Some(TouchAction::DoorTouch(door_touch)) => {
						if activate_with_tag(
							&door_touch.params,
							command_buffer,
							linedef.sector_tag,
							&world,
							map,
							map_dynamic.as_ref(),
						) {
							if !door_touch.retrigger {
								command_buffer.remove_component::<TouchAction>(touch_event.touched);
							}
						}
					}
					_ => {}
				}
			}
		})
}

fn activate(
	params: &DoorParams,
	command_buffer: &mut CommandBuffer,
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
		DoorActive {
			state: params.start_state,
			end_state: params.end_state,
			speed: params.speed,
			wait_time: params.wait_time,
			time_left: Duration::default(),
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

		if world.has_component::<DoorActive>(sector_entity) {
			continue;
		}

		activated = true;
		activate(params, command_buffer, sector_index, map, map_dynamic);
	}

	activated
}
