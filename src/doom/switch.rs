use crate::{
	assets::{AssetHandle, AssetStorage},
	audio::Sound,
	doom::map::{
		textures::{TextureType, Wall},
		LinedefRef, Map, MapDynamic, SidedefSlot,
	},
};
use legion::prelude::{
	CommandBuffer, Entity, IntoQuery, Read, ResourceSet, Resources, World, Write,
};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct SwitchParams {
	pub sound: AssetHandle<Sound>,
	pub retrigger_time: Option<Duration>,
}

#[derive(Clone, Debug)]
pub struct SwitchActive {
	pub sound: AssetHandle<Sound>,
	pub texture: AssetHandle<Wall>,
	pub texture_slot: SidedefSlot,
	pub time_left: Duration,
}

pub fn switch_active_system() -> Box<dyn FnMut(&mut World, &mut Resources)> {
	Box::new(move |world, resources| {
		let (asset_storage, delta, mut sound_queue) = <(
			Read<AssetStorage>,
			Read<Duration>,
			Write<Vec<(AssetHandle<Sound>, Entity)>>,
		)>::fetch_mut(resources);

		let mut command_buffer = CommandBuffer::new(world);

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

pub fn activate(
	params: &SwitchParams,
	command_buffer: &mut CommandBuffer,
	sound_queue: &mut Vec<(AssetHandle<Sound>, Entity)>,
	linedef_index: usize,
	map: &Map,
	map_dynamic: &mut MapDynamic,
) -> bool {
	let linedef = &map.linedefs[linedef_index];
	let sidedef = linedef.sidedefs[0].as_ref().unwrap();
	let linedef_dynamic = &mut map_dynamic.linedefs[linedef_index];
	let sidedef_dynamic = linedef_dynamic.sidedefs[0].as_mut().unwrap();

	let mut activated = false;

	for slot in [SidedefSlot::Top, SidedefSlot::Middle, SidedefSlot::Bottom]
		.iter()
		.copied()
	{
		if let TextureType::Normal(texture) = &mut sidedef_dynamic.textures[slot as usize] {
			if let Some(new) = map.switches.get(texture) {
				activated = true;

				// Change texture
				let old = std::mem::replace(texture, new.clone());

				// Play sound
				let sector_entity = map_dynamic.sectors[sidedef.sector_index].entity;
				sound_queue.push((params.sound.clone(), sector_entity));

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

	activated
}
