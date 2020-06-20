use crate::{
	assets::AssetStorage,
	doom::map::{LinedefRef, MapDynamic},
};
use legion::prelude::{EntityStore, IntoQuery, Read, Runnable, SystemBuilder, Write};
use nalgebra::Vector2;
use std::time::Duration;

pub fn texture_animation_system() -> Box<dyn Runnable> {
	SystemBuilder::new("texture_animation_system")
		.read_resource::<AssetStorage>()
		.read_resource::<Duration>()
		.with_query(<Write<MapDynamic>>::query())
		.build_thread_local(move |_, world, resources, query| {
			let (asset_storage, delta) = resources;

			for mut map_dynamic in query.iter_mut(world) {
				let map_dynamic = map_dynamic.as_mut();

				for (handle, anim_state) in map_dynamic.anim_states_flat.iter_mut() {
					if let Some(time_left) = anim_state.time_left.checked_sub(**delta) {
						anim_state.time_left = time_left;
					} else {
						let map = asset_storage.get(&map_dynamic.map).unwrap();
						let anim = &map.anims_flat[handle];
						anim_state.frame = (anim_state.frame + 1) % anim.frames.len();
						anim_state.time_left = anim.frame_time;
					}
				}

				for (handle, anim_state) in map_dynamic.anim_states_wall.iter_mut() {
					if let Some(time_left) = anim_state.time_left.checked_sub(**delta) {
						anim_state.time_left = time_left;
					} else {
						let map = asset_storage.get(&map_dynamic.map).unwrap();
						let anim = &map.anims_wall[handle];
						anim_state.frame = (anim_state.frame + 1) % anim.frames.len();
						anim_state.time_left = anim.frame_time;
					}
				}
			}
		})
}

pub fn texture_scroll_system() -> Box<dyn Runnable> {
	SystemBuilder::new("texture_scroll_system")
		.read_resource::<Duration>()
		.with_query(<(Read<LinedefRef>, Read<TextureScroll>)>::query())
		.write_component::<MapDynamic>()
		.build_thread_local(move |_, world, delta, query| {
			let (query_world, mut map_dynamic_world) = world.split_for_query(&query);

			// Scroll textures
			for (linedef_ref, texture_scroll) in query.iter(&query_world) {
				let mut map_dynamic = map_dynamic_world
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
