use crate::{
	assets::{AssetHandle, AssetStorage},
	audio::Sound,
	doom::{
		client::{UseAction, UseEvent},
		map::{LinedefRef, Map, MapDynamic, SectorRef},
	},
	geometry::Side,
};
use shrev::{EventChannel, ReaderId};
use specs::{
	Component, DenseVecStorage, Entities, Entity, Join, ReadExpect, ReadStorage, RunNow, World,
	WriteExpect, WriteStorage,
};
use specs_derive::Component;
use std::time::Duration;

pub struct DoorUpdateSystem {
	use_event_reader: ReaderId<UseEvent>,
}

impl DoorUpdateSystem {
	pub fn new(use_event_reader: ReaderId<UseEvent>) -> DoorUpdateSystem {
		DoorUpdateSystem { use_event_reader }
	}
}

impl<'a> RunNow<'a> for DoorUpdateSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (
			entities,
			delta,
			use_event_channel,
			map_asset_storage,
			mut sound_queue,
			linedef_ref_component,
			sector_ref_component,
			use_action_component,
			mut door_active_component,
			mut map_dynamic_component,
		) = world.system_data::<(
			Entities,
			ReadExpect<Duration>,
			ReadExpect<EventChannel<UseEvent>>,
			ReadExpect<AssetStorage<Map>>,
			WriteExpect<Vec<(AssetHandle<Sound>, Entity)>>,
			ReadStorage<LinedefRef>,
			ReadStorage<SectorRef>,
			ReadStorage<UseAction>,
			WriteStorage<DoorActive>,
			WriteStorage<MapDynamic>,
		)>();

		for use_event in use_event_channel.read(&mut self.use_event_reader) {
			if let Some(UseAction::DoorUse(door_use)) =
				use_action_component.get(use_event.linedef_entity)
			{
				let linedef_ref = linedef_ref_component.get(use_event.linedef_entity).unwrap();
				let map_dynamic = map_dynamic_component.get(linedef_ref.map_entity).unwrap();
				let map = map_asset_storage.get(&map_dynamic.map).unwrap();
				let linedef = &map.linedefs[linedef_ref.index];

				if let Some(back_sidedef) = &linedef.sidedefs[Side::Left as usize] {
					let sector_index = back_sidedef.sector_index;
					let sector = &map.sectors[sector_index];

					if let Some(open_height) = sector
						.neighbours
						.iter()
						.map(|index| map_dynamic.sectors[*index].interval.max)
						.min_by(|x, y| x.partial_cmp(y).unwrap())
					{
						let open_height = open_height - 4.0;
						let sector_entity = map_dynamic.sectors[sector_index].entity;

						if let Some(door_active) = door_active_component.get_mut(sector_entity) {
							match door_active.state {
								DoorState::Closing => {
									// Re-open the door
									door_active.state = DoorState::Closed;
									door_active.time_left = door_use.wait_time;
								}
								DoorState::Opening | DoorState::Open => {
									// Close the door early
									door_active.state = DoorState::Open;
									door_active.time_left = Duration::default();
								}
								DoorState::Closed => unreachable!(),
							}
						} else {
							door_active_component
								.insert(
									sector_entity,
									DoorActive {
										open_sound: door_use.open_sound.clone(),
										open_height,

										close_sound: door_use.close_sound.clone(),
										close_height: map_dynamic.sectors[sector_index]
											.interval
											.min,

										state: DoorState::Closed,
										speed: door_use.speed,
										time_left: door_use.wait_time,
									},
								)
								.unwrap();
						}
					} else {
						log::error!(
							"Used door linedef {}, sector {}, has no neighbouring sectors",
							linedef_ref.index,
							sector_index
						);
					}
				} else {
					log::error!("Used door linedef {} has no back sector", linedef_ref.index);
				}
			}
		}

		let mut done = Vec::new();

		for (entity, sector_ref, door_active) in
			(&entities, &sector_ref_component, &mut door_active_component).join()
		{
			let map_dynamic = map_dynamic_component
				.get_mut(sector_ref.map_entity)
				.unwrap();
			let sector_dynamic = &mut map_dynamic.sectors[sector_ref.index];

			match door_active.state {
				DoorState::Closed => {
					door_active.state = DoorState::Opening;

					// Play sound
					sound_queue.push((door_active.open_sound.clone(), entity));
				}
				DoorState::Opening => {
					sector_dynamic.interval.max += door_active.speed * delta.as_secs_f32();

					if sector_dynamic.interval.max > door_active.open_height {
						sector_dynamic.interval.max = door_active.open_height;
						door_active.state = DoorState::Open;
					}
				}
				DoorState::Open => {
					if let Some(new_time) = door_active.time_left.checked_sub(*delta) {
						door_active.time_left = new_time;
					} else {
						door_active.state = DoorState::Closing;

						// Play sound
						sound_queue.push((door_active.close_sound.clone(), entity));
					}
				}
				DoorState::Closing => {
					sector_dynamic.interval.max -= door_active.speed * delta.as_secs_f32();

					if sector_dynamic.interval.max < door_active.close_height {
						done.push(entity);
					}
				}
			}
		}

		for entity in done {
			door_active_component.remove(entity);
		}
	}
}

#[derive(Clone, Debug)]
pub struct DoorUse {
	pub open_sound: AssetHandle<Sound>,
	pub close_sound: AssetHandle<Sound>,
	pub speed: f32,
	pub wait_time: Duration,
}

#[derive(Clone, Component, Debug)]
pub struct DoorActive {
	pub open_sound: AssetHandle<Sound>,
	pub open_height: f32,

	pub close_sound: AssetHandle<Sound>,
	pub close_height: f32,

	pub state: DoorState,
	pub speed: f32,
	pub time_left: Duration,
}

#[derive(Clone, Copy, Debug)]
pub enum DoorState {
	Closed,
	Opening,
	Open,
	Closing,
}
