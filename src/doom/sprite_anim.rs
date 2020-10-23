use crate::{
	common::assets::AssetStorage,
	doom::{
		entitytemplate::EntityTemplateRef,
		sprite::SpriteRender,
		state::{state_trigger, State},
	},
};
use legion::{systems::Runnable, IntoQuery, Resources, SystemBuilder};

pub fn sprite_anim_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("sprite_anim_system")
		.read_resource::<AssetStorage>()
		.with_query(<((&EntityTemplateRef, &State), &mut SpriteRender)>::query())
		.build(move |_command_buffer, world, resources, query| {
			let asset_storage = resources;

			for (state_data, sprite_render) in query.iter_mut(world) {
				state_trigger(state_data, asset_storage, |state_info| {
					*sprite_render = state_info.sprite.clone();
				});
			}
		})
}
