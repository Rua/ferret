use crate::{
	common::{assets::AssetStorage, frame::FrameState, spawn::SpawnMergerHandlerSet},
	doom::map::{LinedefRef, MapDynamic},
};
use legion::{
	systems::{ResourceSet, Runnable},
	IntoQuery, Resources, SystemBuilder, Write,
};
use nalgebra::Vector2;

pub fn texture_animation_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("texture_animation_system")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameState>()
		.with_query(<&mut MapDynamic>::query())
		.build(move |_command_buffer, world, resources, query| {
			let (asset_storage, frame_state) = resources;

			for map_dynamic in query.iter_mut(world) {
				for (handle, anim_state) in map_dynamic.anim_states.iter_mut() {
					if anim_state.timer.is_elapsed(frame_state.time) {
						let map = asset_storage.get(&map_dynamic.map).unwrap();
						let anim = &map.anims[handle];
						anim_state.frame = (anim_state.frame + 1) % anim.frames.len();
						anim_state.timer.restart(frame_state.time);
					}
				}
			}
		})
}

#[derive(Clone, Copy, Debug)]
pub struct TextureScroll {
	pub speed: Vector2<f32>,
}

pub fn texture_scroll_system(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<TextureScroll>();

	SystemBuilder::new("texture_scroll_system")
		.read_resource::<FrameState>()
		.with_query(<(&LinedefRef, &TextureScroll)>::query())
		.with_query(<&mut MapDynamic>::query())
		.build(move |_command_buffer, world, frame_state, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			// Scroll textures
			for (linedef_ref, texture_scroll) in queries.0.iter(&world0) {
				let map_dynamic = queries
					.1
					.get_mut(&mut world, linedef_ref.map_entity)
					.unwrap();
				let linedef_dynamic = &mut map_dynamic.linedefs[linedef_ref.index];
				linedef_dynamic.texture_offset +=
					texture_scroll.speed * frame_state.delta_time.as_secs_f32();
			}
		})
}
