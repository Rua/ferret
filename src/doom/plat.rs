use crate::{
	assets::{AssetHandle, AssetStorage},
	audio::Sound,
	doom::{
		client::{UseAction, UseEvent},
		components::Transform,
		map::{LinedefRef, Map, MapDynamic, SectorRef},
		physics::{BoxCollider, SectorPushTracer, TouchAction, TouchEvent},
		switch::{SwitchActive, SwitchParams},
	},
	quadtree::Quadtree,
};
use legion::prelude::{
	CommandBuffer, Entity, EntityStore, IntoQuery, Read, Resources, Runnable, SystemBuilder, Write,
};
use shrev::EventChannel;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct PlatActive {
	pub state: PlatState,
	pub speed: f32,
	pub wait_time: Duration,
	pub time_left: Duration,
	pub can_reverse: bool,

	pub start_sound: Option<AssetHandle<Sound>>,
	pub move_sound: Option<AssetHandle<Sound>>,
	pub move_sound_time: Duration,
	pub move_sound_time_left: Duration,
	pub finish_sound: Option<AssetHandle<Sound>>,

	pub low_height: f32,
	pub high_height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlatState {
	GoingDown,
	GoingUp,
	Waiting,
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

pub fn plat_active_system() -> Box<dyn Runnable> {
	SystemBuilder::new("plat_active_system")
		.read_resource::<AssetStorage>()
		.read_resource::<Duration>()
		.read_resource::<Quadtree>()
		.write_resource::<Vec<(AssetHandle<Sound>, Entity)>>()
		.with_query(<(Read<SectorRef>, Write<PlatActive>)>::query())
		.read_component::<BoxCollider>() // used by SectorTracer
		.write_component::<MapDynamic>()
		.write_component::<Transform>()
		.build_thread_local(move |command_buffer, world, resources, query| {
			let (asset_storage, delta, quadtree, sound_queue) = resources;
			let (mut map_dynamic_world, mut world) = world.split::<Write<MapDynamic>>();
			let (mut query_world, mut world) = world.split_for_query(&query);

			for (entity, (sector_ref, mut plat_active)) in query.iter_entities_mut(&mut query_world)
			{
				let mut map_dynamic = map_dynamic_world
					.get_component_mut::<MapDynamic>(sector_ref.map_entity)
					.unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let sector = &map.sectors[sector_ref.index];

				let new_state = match plat_active.state {
					PlatState::GoingDown => {
						if let Some(new_time) =
							plat_active.move_sound_time_left.checked_sub(**delta)
						{
							plat_active.move_sound_time_left = new_time;
						} else {
							plat_active.move_sound_time_left = plat_active.move_sound_time;

							if let Some(sound) = &plat_active.move_sound {
								sound_queue.push((sound.clone(), entity));
							}
						}

						let move_step = -plat_active.speed * delta.as_secs_f32();
						let sector_dynamic = &mut map_dynamic.sectors[sector_ref.index];
						sector_dynamic.interval.min += move_step;

						if sector_dynamic.interval.min < plat_active.low_height {
							sector_dynamic.interval.min = plat_active.low_height;
							Some(PlatState::Waiting)
						} else {
							None
						}
					}
					PlatState::GoingUp => {
						if let Some(new_time) =
							plat_active.move_sound_time_left.checked_sub(**delta)
						{
							plat_active.move_sound_time_left = new_time;
						} else {
							plat_active.move_sound_time_left = plat_active.move_sound_time;

							if let Some(sound) = &plat_active.move_sound {
								sound_queue.push((sound.clone(), entity));
							}
						}

						// Check if we bumped something on the way
						let current_height = map_dynamic.sectors[sector_ref.index].interval.min;
						let distance_left = plat_active.high_height - current_height;
						let move_step = plat_active.speed * delta.as_secs_f32().min(distance_left);

						let tracer = SectorPushTracer {
							map,
							map_dynamic: &map_dynamic,
							quadtree,
							world: &world,
						};

						let trace = tracer.trace(
							current_height,
							1.0,
							move_step,
							sector.subsectors.iter().map(|i| &map.subsectors[*i]),
						);

						// Push the entities out of the way
						for pushed_entity in trace.pushed_entities.iter() {
							let mut transform = world
								.get_component_mut::<Transform>(pushed_entity.entity)
								.unwrap();
							transform.position += pushed_entity.move_step;
						}

						// Move the plat into place
						let sector_dynamic = &mut map_dynamic.sectors[sector_ref.index];
						sector_dynamic.interval.min += trace.move_step;

						if trace.fraction < 1.0 {
							// We got obstructed on the way up
							if plat_active.can_reverse {
								Some(PlatState::GoingDown)
							} else {
								// Hang there until the obstruction is gone
								None
							}
						} else {
							if trace.move_step == distance_left {
								// Reached target height
								sector_dynamic.interval.min = plat_active.high_height;
								Some(PlatState::Waiting)
							} else {
								None
							}
						}
					}
					PlatState::Waiting => {
						if let Some(new_time) = plat_active.time_left.checked_sub(**delta) {
							plat_active.time_left = new_time;
							None
						} else {
							let sector_dynamic = &mut map_dynamic.sectors[sector_ref.index];

							if sector_dynamic.interval.min == plat_active.low_height {
								Some(PlatState::GoingUp)
							} else {
								Some(PlatState::GoingDown)
							}
						}
					}
				};

				// State transition
				if let Some(new_state) = new_state {
					plat_active.state = new_state;

					match new_state {
						PlatState::GoingDown | PlatState::GoingUp => {
							if let Some(sound) = &plat_active.start_sound {
								sound_queue.push((sound.clone(), entity));
							}
						}
						PlatState::Waiting => {
							if let Some(sound) = &plat_active.finish_sound {
								sound_queue.push((sound.clone(), entity));
							}

							let sector_dynamic = &mut map_dynamic.sectors[sector_ref.index];
							if sector_dynamic.interval.min == plat_active.high_height {
								command_buffer.remove_component::<PlatActive>(entity);
							} else {
								plat_active.time_left = plat_active.wait_time;
							}
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

pub fn plat_switch_system(resources: &mut Resources) -> Box<dyn Runnable> {
	let mut use_event_reader = resources
		.get_mut::<EventChannel<UseEvent>>()
		.unwrap()
		.register_reader();

	SystemBuilder::new("plat_switch_system")
		.read_resource::<AssetStorage>()
		.read_resource::<EventChannel<UseEvent>>()
		.write_resource::<Vec<(AssetHandle<Sound>, Entity)>>()
		.read_component::<PlatActive>() // used by activate_with_tag
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

				if let Some(UseAction::PlatSwitchUse(plat_use)) = world
					.get_component::<UseAction>(use_event.linedef_entity)
					.as_deref()
				{
					// Skip if switch is already in active state
					if world.has_component::<SwitchActive>(use_event.linedef_entity) {
						continue;
					}

					let activated = activate_with_tag(
						&plat_use.params,
						command_buffer,
						linedef.sector_tag,
						&world,
						map,
						map_dynamic.as_ref(),
					);

					if activated {
						let activated = crate::doom::switch::activate(
							&plat_use.switch_params,
							command_buffer,
							sound_queue.as_mut(),
							linedef_ref.index,
							map,
							map_dynamic.as_mut(),
						);

						if activated && plat_use.switch_params.retrigger_time.is_none() {
							command_buffer.remove_component::<UseAction>(use_event.linedef_entity);
						}
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

pub fn plat_touch_system(resources: &mut Resources) -> Box<dyn Runnable> {
	let mut touch_event_reader = resources
		.get_mut::<EventChannel<TouchEvent>>()
		.unwrap()
		.register_reader();

	SystemBuilder::new("plat_touch_system")
		.read_resource::<AssetStorage>()
		.read_resource::<EventChannel<TouchEvent>>()
		.read_component::<PlatActive>() // used by activate_with_tag
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
					Some(TouchAction::PlatTouch(plat_touch)) => {
						if activate_with_tag(
							&plat_touch.params,
							command_buffer,
							linedef.sector_tag,
							&world,
							map,
							map_dynamic.as_ref(),
						) {
							if !plat_touch.retrigger {
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
	params: &PlatParams,
	command_buffer: &mut CommandBuffer,
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
		PlatActive {
			state: PlatState::Waiting,
			speed: params.speed,
			wait_time: params.wait_time,
			time_left: Duration::default(),
			can_reverse: params.can_reverse,

			start_sound: params.start_sound.clone(),
			move_sound: params.move_sound.clone(),
			move_sound_time: params.move_sound_time,
			move_sound_time_left: Duration::default(),
			finish_sound: params.finish_sound.clone(),

			high_height,
			low_height,
		},
	);
}

fn activate_with_tag<W: EntityStore>(
	params: &PlatParams,
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

		if world.has_component::<PlatActive>(sector_entity) {
			continue;
		}

		activated = true;
		activate(params, command_buffer, sector_index, map, map_dynamic);
	}

	activated
}
