use crate::{
	assets::AssetStorage,
	doom::map::{LinedefRef, Map, MapDynamic},
};
use legion::prelude::{IntoQuery, Read, ResourceSet, Resources, World, Write};
use nalgebra::Vector2;
use std::time::Duration;

pub fn texture_anim_system() -> Box<dyn FnMut(&mut World, &mut Resources)> {
	Box::new(|world, resources| {
		let (delta, map_storage) = <(Read<Duration>, Read<AssetStorage<Map>>)>::fetch(resources);

		// Advance animations
		for mut map_dynamic in <Write<MapDynamic>>::query().iter_mut(world) {
			let map_dynamic = map_dynamic.as_mut();

			for (handle, anim_state) in map_dynamic.anim_states_flat.iter_mut() {
				if let Some(time_left) = anim_state.time_left.checked_sub(*delta) {
					anim_state.time_left = time_left;
				} else {
					let map = map_storage.get(&map_dynamic.map).unwrap();
					let anim = &map.anims_flat[handle];
					anim_state.frame = (anim_state.frame + 1) % anim.frames.len();
					anim_state.time_left = anim.frame_time;
				}
			}

			for (handle, anim_state) in map_dynamic.anim_states_wall.iter_mut() {
				if let Some(time_left) = anim_state.time_left.checked_sub(*delta) {
					anim_state.time_left = time_left;
				} else {
					let map = map_storage.get(&map_dynamic.map).unwrap();
					let anim = &map.anims_wall[handle];
					anim_state.frame = (anim_state.frame + 1) % anim.frames.len();
					anim_state.time_left = anim.frame_time;
				}
			}
		}

		// Scroll textures
		for (linedef_ref, texture_scroll) in
			<(Read<LinedefRef>, Read<TextureScroll>)>::query().iter(world)
		{
			let mut map_dynamic = world
				.get_component_mut::<MapDynamic>(linedef_ref.map_entity)
				.unwrap();
			let map_dynamic = map_dynamic.as_mut();
			let linedef_dynamic = &mut map_dynamic.linedefs[linedef_ref.index];
			linedef_dynamic.texture_offset += texture_scroll.speed * delta.as_secs_f32();
		}
	})
}

#[derive(Clone, Copy, Debug)]
pub struct TextureScroll {
	pub speed: Vector2<f32>,
}
