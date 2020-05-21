use crate::{
	assets::{AssetHandle, AssetStorage},
	audio::Sound,
	doom::{
		client::{UseAction, UseEvent},
		map::{
			textures::{TextureType, Wall},
			LinedefRef, MapDynamic, SectorRef, SidedefSlot,
		},
		physics::SectorTracer,
	},
	geometry::Side,
};
use legion::prelude::{
	CommandBuffer, Entity, IntoQuery, Read, ResourceSet, Resources, World, Write,
};
use shrev::{EventChannel, ReaderId};
use std::time::Duration;

pub fn door_update_system(
	mut use_event_reader: ReaderId<UseEvent>,
) -> Box<dyn FnMut(&mut World, &mut Resources)> {
	Box::new(move |world, resources| {
		let (asset_storage, delta, use_event_channel, mut sound_queue) = <(
			Read<AssetStorage>,
			Read<Duration>,
			Read<EventChannel<UseEvent>>,
			Write<Vec<(AssetHandle<Sound>, Entity)>>,
		)>::fetch_mut(resources);

		let tracer = SectorTracer { world };
		let mut command_buffer = CommandBuffer::new(world);

		for use_event in use_event_channel.read(&mut use_event_reader) {
			if let Some(UseAction::DoorUse(door_use)) = world
				.get_component::<UseAction>(use_event.linedef_entity)
				.as_deref()
			{
				let linedef_ref = world
					.get_component::<LinedefRef>(use_event.linedef_entity)
					.unwrap();
				let map_dynamic = world
					.get_component::<MapDynamic>(linedef_ref.map_entity)
					.unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let linedef = &map.linedefs[linedef_ref.index];

				if let Some(back_sidedef) = &linedef.sidedefs[Side::Left as usize] {
					let sector_index = back_sidedef.sector_index;
					let sector = &map.sectors[sector_index];
					let sector_entity = map_dynamic.sectors[sector_index].entity;

					if let Some(mut door_active) =
						unsafe { world.get_component_mut_unchecked::<DoorActive>(sector_entity) }
					{
						match door_active.state {
							DoorState::Closing => {
								// Re-open the door
								door_active.state = DoorState::Closed;
							}
							DoorState::Opening | DoorState::Open => {
								// Close the door early
								door_active.state = DoorState::Open;
								door_active.time_left = Duration::default();
							}
							DoorState::Closed => unreachable!(),
						}
					} else {
						if let Some(open_height) = sector
							.neighbours
							.iter()
							.map(|index| map_dynamic.sectors[*index].interval.max)
							.min_by(|x, y| x.partial_cmp(y).unwrap())
						{
							command_buffer.add_component(
								sector_entity,
								DoorActive {
									open_sound: door_use.open_sound.clone(),
									open_height: open_height - 4.0,

									close_sound: door_use.close_sound.clone(),
									close_height: map_dynamic.sectors[sector_index].interval.min,

									state: DoorState::Closed,
									speed: door_use.speed,
									time_left: door_use.wait_time,
									wait_time: door_use.wait_time,
								},
							);
						} else {
							log::error!(
								"Used door sector {} has no neighbouring sectors",
								sector_index
							);
						}
					}
				} else {
					log::error!("Used door linedef {} has no back sector", linedef_ref.index);
				}
			} else if let Some(UseAction::DoorSwitchUse(door_use)) = world
				.get_component::<UseAction>(use_event.linedef_entity)
				.as_deref()
			{
				// Skip if switch is already in active state
				if world.has_component::<SwitchActive>(use_event.linedef_entity) {
					continue;
				}

				let linedef_ref = world
					.get_component::<LinedefRef>(use_event.linedef_entity)
					.unwrap();
				let mut map_dynamic = unsafe {
					world
						.get_component_mut_unchecked::<MapDynamic>(linedef_ref.map_entity)
						.unwrap()
				};
				let map_dynamic = map_dynamic.as_mut();
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let linedef = &map.linedefs[linedef_ref.index];

				let mut used = false;

				// Activate all the doors with the same tag
				for (i, sector) in map
					.sectors
					.iter()
					.enumerate()
					.filter(|(_, s)| s.sector_tag == linedef.sector_tag)
				{
					let sector_entity = map_dynamic.sectors[i].entity;

					if world.has_component::<DoorActive>(sector_entity) {
						continue;
					} else {
						used = true;
					}

					if let Some(open_height) = sector
						.neighbours
						.iter()
						.map(|index| map_dynamic.sectors[*index].interval.max)
						.min_by(|x, y| x.partial_cmp(y).unwrap())
					{
						command_buffer.add_component(
							sector_entity,
							DoorActive {
								open_sound: door_use.open_sound.clone(),
								open_height: open_height - 4.0,

								close_sound: door_use.close_sound.clone(),
								close_height: map_dynamic.sectors[i].interval.min,

								state: DoorState::Closed,
								speed: door_use.speed,
								time_left: door_use.wait_time,
								wait_time: door_use.wait_time,
							},
						);
					} else {
						log::error!("Used door sector {}, has no neighbouring sectors", i);
					}
				}

				if used {
					// Flip the switch texture
					let sidedef = linedef.sidedefs[0].as_ref().unwrap();
					let linedef_dynamic = &mut map_dynamic.linedefs[linedef_ref.index];
					let sidedef_dynamic = linedef_dynamic.sidedefs[0].as_mut().unwrap();

					for slot in [SidedefSlot::Top, SidedefSlot::Middle, SidedefSlot::Bottom]
						.iter()
						.copied()
					{
						if let TextureType::Normal(texture) =
							&mut sidedef_dynamic.textures[slot as usize]
						{
							if let Some(new) = map.switches.get(texture) {
								// Change texture
								let old = std::mem::replace(texture, new.clone());

								// Play sound
								let sector_entity =
									map_dynamic.sectors[sidedef.sector_index].entity;
								sound_queue.push((door_use.switch_sound.clone(), sector_entity));

								command_buffer.add_component(
									use_event.linedef_entity,
									SwitchActive {
										sound: door_use.switch_sound.clone(),
										texture: old,
										texture_slot: slot,
										time_left: door_use.switch_time,
									},
								);

								break;
							}
						}
					}
				}
			}
		}

		for (entity, (sector_ref, mut door_active)) in unsafe {
			<(Read<SectorRef>, Write<DoorActive>)>::query().iter_entities_unchecked(world)
		} {
			let mut map_dynamic = unsafe {
				world
					.get_component_mut_unchecked::<MapDynamic>(sector_ref.map_entity)
					.unwrap()
			};
			let map_dynamic = map_dynamic.as_mut();
			let map = asset_storage.get(&map_dynamic.map).unwrap();
			let sector_dynamic = &mut map_dynamic.sectors[sector_ref.index];
			let sector = &map.sectors[sector_ref.index];

			match door_active.state {
				DoorState::Closed => {
					door_active.state = DoorState::Opening;

					// Play sound
					sound_queue.push((door_active.open_sound.clone(), entity));
				}
				DoorState::Opening => {
					let move_step = door_active.speed * delta.as_secs_f32();
					sector_dynamic.interval.max += move_step;

					if sector_dynamic.interval.max > door_active.open_height {
						sector_dynamic.interval.max = door_active.open_height;
						door_active.state = DoorState::Open;
						door_active.time_left = door_active.wait_time;
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
					let move_step = -door_active.speed * delta.as_secs_f32();
					let trace = tracer.trace(
						-sector_dynamic.interval.max,
						-1.0,
						move_step,
						sector.subsectors.iter().map(|i| &map.subsectors[*i]),
					);

					// TODO use fraction
					if trace.collision.is_some() {
						// Hit something on the way down, re-open the door
						door_active.state = DoorState::Closed;
					} else {
						sector_dynamic.interval.max += move_step;

						if sector_dynamic.interval.max < door_active.close_height {
							command_buffer.remove_component::<DoorActive>(entity);
						}
					}
				}
			}
		}

		for (entity, (linedef_ref, mut switch_active)) in unsafe {
			<(Read<LinedefRef>, Write<SwitchActive>)>::query().iter_entities_unchecked(world)
		} {
			if let Some(new_time) = switch_active.time_left.checked_sub(*delta) {
				switch_active.time_left = new_time;
			} else {
				let mut map_dynamic = unsafe {
					world
						.get_component_mut_unchecked::<MapDynamic>(linedef_ref.map_entity)
						.unwrap()
				};
				let map_dynamic = map_dynamic.as_mut();
				let linedef_dynamic = &mut map_dynamic.linedefs[linedef_ref.index];
				let sidedef_dynamic = linedef_dynamic.sidedefs[0].as_mut().unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let linedef = &map.linedefs[linedef_ref.index];
				let sidedef = linedef.sidedefs[0].as_ref().unwrap();
				let sector_entity = map_dynamic.sectors[sidedef.sector_index].entity;

				sidedef_dynamic.textures[switch_active.texture_slot as usize] =
					TextureType::Normal(switch_active.texture.clone());
				sound_queue.push((switch_active.sound.clone(), sector_entity));
				command_buffer.remove_component::<SwitchActive>(entity);
			}
		}

		command_buffer.write(world);
	})
}

#[derive(Clone, Debug)]
pub struct DoorUse {
	pub open_sound: AssetHandle<Sound>,
	pub close_sound: AssetHandle<Sound>,
	pub speed: f32,
	pub wait_time: Duration,
}

#[derive(Clone, Debug)]
pub struct DoorSwitchUse {
	pub open_sound: AssetHandle<Sound>,
	pub close_sound: AssetHandle<Sound>,
	pub switch_sound: AssetHandle<Sound>,
	pub switch_time: Duration,
	pub speed: f32,
	pub wait_time: Duration,
}

#[derive(Clone, Debug)]
pub struct DoorActive {
	pub open_sound: AssetHandle<Sound>,
	pub open_height: f32,

	pub close_sound: AssetHandle<Sound>,
	pub close_height: f32,

	pub state: DoorState,
	pub speed: f32,
	pub time_left: Duration,
	pub wait_time: Duration,
}

#[derive(Clone, Debug)]
pub struct SwitchActive {
	sound: AssetHandle<Sound>,
	texture: AssetHandle<Wall>,
	texture_slot: SidedefSlot,
	time_left: Duration,
}

#[derive(Clone, Copy, Debug)]
pub enum DoorState {
	Closed,
	Opening,
	Open,
	Closing,
}
