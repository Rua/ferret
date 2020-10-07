use crate::{
	common::{
		assets::AssetStorage,
		time::{FrameTime, Timer},
	},
	doom::{entitytemplate::EntityTemplateRef, render::sprite::SpriteRender},
};
use arrayvec::ArrayString;
use legion::{systems::Runnable, Entity, IntoQuery, Resources, SystemBuilder};
use std::time::Duration;

pub type StateName = ArrayString<[u8; 16]>;

#[derive(Clone, Debug)]
pub struct StateDef {
	pub sprite: SpriteRender,
	pub next: Option<(Duration, Option<StateName>)>,
}

#[derive(Clone, Debug)]
pub struct State {
	pub current: Option<StateName>,
	pub next: Option<(Timer, Option<StateName>)>,
}

pub fn state_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("state_system")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameTime>()
		.with_query(<(Entity, &EntityTemplateRef, &mut SpriteRender, &mut State)>::query())
		.build(move |command_buffer, world, resources, query| {
			let (asset_storage, frame_time) = resources;

			for (entity, template_ref, sprite_render, state) in query.iter_mut(world) {
				let state = &mut *state;

				if let Some((timer, next)) = &mut state.next {
					timer.tick(frame_time.delta);

					while timer.is_zero() {
						if let Some(new_state_name) = next {
							let template = asset_storage.get(&template_ref.0).unwrap();
							let new_state = template
								.states
								.get(new_state_name)
								.expect("Invalid next state name");

							*sprite_render = new_state.sprite.clone();

							if let Some((new_time, new_next)) = new_state.next.clone() {
								timer.set(new_time);
								*next = new_next;
							} else {
								state.next = None;
								break;
							}
						} else {
							// Delete the entity if the next state is None
							command_buffer.remove(*entity);
							break;
						}
					}
				}
			}
		})
}
