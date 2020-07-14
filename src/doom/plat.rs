use crate::{
	assets::{AssetHandle, AssetStorage},
	audio::Sound,
	doom::{
		client::{UseAction, UseEvent},
		components::Transform,
		map::{LinedefRef, Map, MapDynamic, SectorRef},
		physics::{BoxCollider, TouchAction, TouchEvent},
		sectormove::{SectorMove, SectorMoveEvent, SectorMoveEventType},
		switch::{SwitchActive, SwitchParams},
	},
	timer::Timer,
};
use legion::prelude::{
	CommandBuffer, Entity, EntityStore, IntoQuery, Read, Resources, Runnable, SystemBuilder, Write,
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

pub fn plat_active_system(resources: &mut Resources) -> Box<dyn Runnable> {
	let mut sector_move_event_reader = resources
		.get_mut::<EventChannel<SectorMoveEvent>>()
		.unwrap()
		.register_reader();

	SystemBuilder::new("plat_active_system")
		.read_resource::<Duration>()
		.read_resource::<EventChannel<SectorMoveEvent>>()
		.write_resource::<Vec<(AssetHandle<Sound>, Entity)>>()
		.with_query(<(Read<SectorRef>, Write<PlatActive>, Write<SectorMove>)>::query())
		.read_component::<BoxCollider>() // used by SectorTracer
		.write_component::<MapDynamic>()
		.write_component::<Transform>()
		.build_thread_local(move |command_buffer, world, resources, query| {
			let (delta, sector_move_event_channel, sound_queue) = resources;

			{
				let (mut query_world, world) = world.split_for_query(&query);

				for (entity, (sector_ref, mut plat_active, mut sector_move)) in
					query.iter_entities_mut(&mut query_world)
				{
					let map_dynamic = world
						.get_component::<MapDynamic>(sector_ref.map_entity)
						.unwrap();

					if sector_move.velocity == 0.0 {
						plat_active.wait_timer.tick(**delta);

						if plat_active.wait_timer.is_zero() {
							if let Some(sound) = &plat_active.start_sound {
								sound_queue.push((sound.clone(), entity));
							}

							let sector_dynamic = &map_dynamic.sectors[sector_ref.index];

							if sector_dynamic.interval.min == plat_active.low_height {
								sector_move.velocity = plat_active.speed;
								sector_move.target = plat_active.high_height;
							} else {
								sector_move.velocity = -plat_active.speed;
								sector_move.target = plat_active.low_height;
							}
						}
					}
				}
			}

			{
				let (mut sector_move_world, mut world) = world.split::<Write<SectorMove>>();
				let (mut plat_active_world, world) = world.split::<Write<PlatActive>>();

				for event in sector_move_event_channel.read(&mut sector_move_event_reader) {
					let mut sector_move = sector_move_world
						.get_component_mut::<SectorMove>(event.entity)
						.unwrap();

					if sector_move.velocity != 0.0 {
						let mut plat_active = plat_active_world
							.get_component_mut::<PlatActive>(event.entity)
							.unwrap();
						let sector_ref = world.get_component::<SectorRef>(event.entity).unwrap();
						let map_dynamic = world
							.get_component::<MapDynamic>(sector_ref.map_entity)
							.unwrap();

						match event.event_type {
							SectorMoveEventType::Collided => {
								if plat_active.can_reverse {
									if let Some(sound) = &plat_active.start_sound {
										sound_queue.push((sound.clone(), event.entity));
									}

									sector_move.velocity = -sector_move.velocity;

									if sector_move.velocity > 0.0 {
										sector_move.target = plat_active.high_height;
									} else {
										sector_move.target = plat_active.low_height;
									}
								}
							}
							SectorMoveEventType::TargetReached => {
								if let Some(sound) = &plat_active.finish_sound {
									sound_queue.push((sound.clone(), event.entity));
								}

								let sector_dynamic = &map_dynamic.sectors[sector_ref.index];
								sector_move.velocity = 0.0;
								sector_move.target = sector_dynamic.interval.min;

								if sector_dynamic.interval.min == plat_active.high_height {
									command_buffer.remove_component::<PlatActive>(event.entity);
								} else {
									plat_active.wait_timer.reset();
								}
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
						crate::doom::switch::activate(
							&plat_use.switch_params,
							command_buffer,
							sound_queue.as_mut(),
							linedef_ref.index,
							map,
							map_dynamic.as_mut(),
						);

						if plat_use.switch_params.retrigger_time.is_none() {
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
		SectorMove {
			velocity: 0.0,
			target: sector_dynamic.interval.min,
			sound: params.move_sound.clone(),
			sound_timer: Timer::new(params.move_sound_time),
		},
	);

	command_buffer.add_component(
		sector_dynamic.entity,
		PlatActive {
			speed: params.speed,
			wait_timer: Timer::new_zero(params.wait_time),
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
