use crate::{
	common::{
		assets::AssetStorage,
		spawn::SpawnMergerHandlerSet,
		time::{DeltaTime, GameTime},
	},
	doom::map::{LinedefRef, MapDynamic},
};
use legion::{
	systems::{ResourceSet, Runnable},
	IntoQuery, Registry, Resources, SystemBuilder, Write,
};
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

pub fn texture_animation(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("texture_animation")
		.read_resource::<AssetStorage>()
		.read_resource::<GameTime>()
		.with_query(<&mut MapDynamic>::query())
		.build(move |_command_buffer, world, resources, query| {
			let (asset_storage, game_time) = resources;

			for map_dynamic in query.iter_mut(world) {
				for (handle, anim_state) in map_dynamic.anim_states.iter_mut() {
					if anim_state.timer.is_elapsed(**game_time) {
						let map = asset_storage.get(&map_dynamic.map).unwrap();
						let anim = &map.anims[handle];
						anim_state.frame = (anim_state.frame + 1) % anim.frames.len();
						anim_state.timer.restart(**game_time);
					}
				}
			}
		})
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TextureScroll {
	pub speed: Vector2<f32>,
}

pub fn texture_scroll(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<TextureScroll>("TextureScroll".into());
	handler_set.register_clone::<TextureScroll>();

	SystemBuilder::new("texture_scroll")
		.read_resource::<DeltaTime>()
		.with_query(<(&LinedefRef, &TextureScroll)>::query())
		.with_query(<&mut MapDynamic>::query())
		.build(move |_command_buffer, world, delta_time, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			// Scroll textures
			for (linedef_ref, texture_scroll) in queries.0.iter(&world0) {
				let map_dynamic = queries
					.1
					.get_mut(&mut world, linedef_ref.map_entity)
					.unwrap();
				let linedef_dynamic = &mut map_dynamic.linedefs[linedef_ref.index];
				linedef_dynamic.texture_offset += texture_scroll.speed * delta_time.0.as_secs_f32();
			}
		})
}
