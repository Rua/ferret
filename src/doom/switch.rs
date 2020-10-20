use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		audio::Sound,
		frame::FrameState,
		time::Timer,
	},
	doom::{
		image::Image,
		map::{textures::TextureType, LinedefRef, Map, MapDynamic, SidedefSlot},
	},
};
use legion::{
	systems::{CommandBuffer, Runnable},
	Entity, IntoQuery, SystemBuilder,
};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct SwitchParams {
	pub sound: Option<AssetHandle<Sound>>,
	pub retrigger_time: Option<Duration>,
}

#[derive(Clone, Debug)]
pub struct SwitchActive {
	pub sound: Option<AssetHandle<Sound>>,
	pub texture: AssetHandle<Image>,
	pub texture_slot: SidedefSlot,
	pub timer: Timer,
}

pub fn switch_active_system() -> impl Runnable {
	SystemBuilder::new("switch_active_system")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameState>()
		.write_resource::<Vec<(AssetHandle<Sound>, Entity)>>()
		.with_query(<(Entity, &LinedefRef, &mut SwitchActive)>::query())
		.with_query(<&mut MapDynamic>::query())
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, frame_state, sound_queue) = resources;

			let (mut world0, mut world) = world.split_for_query(&queries.0);

			for (entity, linedef_ref, switch_active) in queries.0.iter_mut(&mut world0) {
				if switch_active.timer.is_elapsed(frame_state.time) {
					let map_dynamic = queries
						.1
						.get_mut(&mut world, linedef_ref.map_entity)
						.unwrap();
					let linedef_dynamic = &mut map_dynamic.linedefs[linedef_ref.index];
					let sidedef_dynamic = linedef_dynamic.sidedefs[0].as_mut().unwrap();
					let map = asset_storage.get(&map_dynamic.map).unwrap();
					let linedef = &map.linedefs[linedef_ref.index];
					let sidedef = linedef.sidedefs[0].as_ref().unwrap();
					let sector_entity = map_dynamic.sectors[sidedef.sector_index].entity;

					sidedef_dynamic.textures[switch_active.texture_slot as usize] =
						TextureType::Normal(switch_active.texture.clone());

					if let Some(sound) = &switch_active.sound {
						sound_queue.push((sound.clone(), sector_entity));
					}

					command_buffer.remove_component::<SwitchActive>(*entity);
				}
			}
		})
}

pub fn activate(
	params: &SwitchParams,
	command_buffer: &mut CommandBuffer,
	sound_queue: &mut Vec<(AssetHandle<Sound>, Entity)>,
	frame_state: &FrameState,
	linedef_index: usize,
	map: &Map,
	map_dynamic: &mut MapDynamic,
) {
	let linedef = &map.linedefs[linedef_index];
	let sidedef = linedef.sidedefs[0].as_ref().unwrap();
	let linedef_dynamic = &mut map_dynamic.linedefs[linedef_index];
	let sidedef_dynamic = linedef_dynamic.sidedefs[0].as_mut().unwrap();

	for slot in [SidedefSlot::Top, SidedefSlot::Middle, SidedefSlot::Bottom]
		.iter()
		.copied()
	{
		if let TextureType::Normal(texture) = &mut sidedef_dynamic.textures[slot as usize] {
			if let Some(new) = map.switches.get(texture) {
				// Change texture
				let old = std::mem::replace(texture, new.clone());

				// Play sound
				if let Some(sound) = &params.sound {
					let sector_entity = map_dynamic.sectors[sidedef.sector_index].entity;
					sound_queue.push((sound.clone(), sector_entity));
				}

				if let Some(time_left) = params.retrigger_time {
					command_buffer.add_component(
						linedef_dynamic.entity,
						SwitchActive {
							sound: params.sound.clone(),
							texture: old,
							texture_slot: slot,
							timer: Timer::new(frame_state.time, time_left),
						},
					);
				}

				break;
			}
		}
	}
}
