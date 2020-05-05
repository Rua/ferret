use crate::{
	assets::AssetStorage,
	doom::map::{LinedefRef, Map, MapDynamic},
};
use nalgebra::Vector2;
use specs::{
	Component, DenseVecStorage, Join, ReadExpect, ReadStorage, RunNow, World, WriteStorage,
};
use specs_derive::Component;
use std::time::Duration;

#[derive(Default)]
pub struct TextureAnimSystem;

impl<'a> RunNow<'a> for TextureAnimSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (
			delta,
			map_storage,
			linedef_ref_component,
			texture_scroll_component,
			mut map_dynamic_component,
		) = world.system_data::<(
			ReadExpect<Duration>,
			ReadExpect<AssetStorage<Map>>,
			ReadStorage<LinedefRef>,
			ReadStorage<TextureScroll>,
			WriteStorage<MapDynamic>,
		)>();

		// Advance animations
		for map_dynamic in (&mut map_dynamic_component).join() {
			for (handle, anim_state) in &mut map_dynamic.anim_states_flat {
				if let Some(time_left) = anim_state.time_left.checked_sub(*delta) {
					anim_state.time_left = time_left;
				} else {
					let map = map_storage.get(&map_dynamic.map).unwrap();
					let anim = &map.anims_flat[handle];
					anim_state.frame = (anim_state.frame + 1) % anim.frames.len();
					anim_state.time_left = anim.frame_time;
				}
			}

			for (handle, anim_state) in &mut map_dynamic.anim_states_wall {
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
			(&linedef_ref_component, &texture_scroll_component).join()
		{
			let map_dynamic = map_dynamic_component
				.get_mut(linedef_ref.map_entity)
				.unwrap();
			let linedef_dynamic = &mut map_dynamic.linedefs[linedef_ref.index];
			linedef_dynamic.texture_offset += texture_scroll.speed * delta.as_secs_f32();
		}
	}
}

#[derive(Clone, Component, Copy, Debug)]
pub struct TextureScroll {
	pub speed: Vector2<f32>,
}
