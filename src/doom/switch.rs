use crate::{
	assets::{AssetHandle, AssetStorage},
	audio::Sound,
	doom::map::{
		textures::{TextureType, Wall},
		LinedefRef, Map, MapDynamic, SidedefSlot,
	},
};
use legion::prelude::{
	CommandBuffer, Entity, EntityStore, IntoQuery, Read, Runnable, SystemBuilder, Write,
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
	pub texture: AssetHandle<Wall>,
	pub texture_slot: SidedefSlot,
	pub time_left: Duration,
}

pub fn switch_active_system() -> Box<dyn Runnable> {
	SystemBuilder::new("switch_active_system")
		.read_resource::<AssetStorage>()
		.read_resource::<Duration>()
		.write_resource::<Vec<(AssetHandle<Sound>, Entity)>>()
		.with_query(<(Read<LinedefRef>, Write<SwitchActive>)>::query())
		.write_component::<MapDynamic>()
		.build_thread_local(move |command_buffer, world, resources, query| {
			let (asset_storage, delta, sound_queue) = resources;
			let (mut query_world, mut world) = world.split_for_query(&query);
			let (mut map_dynamic_world, _world) = world.split::<Write<MapDynamic>>();

			for (entity, (linedef_ref, mut switch_active)) in
				query.iter_entities_mut(&mut query_world)
			{
				if let Some(new_time) = switch_active.time_left.checked_sub(**delta) {
					switch_active.time_left = new_time;
				} else {
					let mut map_dynamic = map_dynamic_world
						.get_component_mut::<MapDynamic>(linedef_ref.map_entity)
						.unwrap();
					let map_dynamic = map_dynamic.as_mut();
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

					command_buffer.remove_component::<SwitchActive>(entity);
				}
			}
		})
}

pub fn activate(
	params: &SwitchParams,
	command_buffer: &mut CommandBuffer,
	sound_queue: &mut Vec<(AssetHandle<Sound>, Entity)>,
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
							time_left,
						},
					);
				}

				break;
			}
		}
	}
}
