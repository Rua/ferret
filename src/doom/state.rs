use crate::{common::timer::Timer, doom::render::sprite::SpriteRender};
use legion::{systems::Runnable, Entity, IntoQuery, Resources, SystemBuilder};
use std::{collections::HashMap, sync::Arc, time::Duration};

#[derive(Clone, Debug)]
pub struct State {
	pub states: Arc<HashMap<String, StateDef>>,
	pub next: Option<(Timer, Option<String>)>,
	pub spawn_state: Option<String>,
	pub see_state: Option<String>,
	pub pain_state: Option<String>,
	pub melee_state: Option<String>,
	pub missile_state: Option<String>,
	pub death_state: Option<String>,
	pub xdeath_state: Option<String>,
	pub raise_state: Option<String>,
}

#[derive(Clone, Debug)]
pub struct StateDef {
	pub sprite: SpriteRender,
	pub next: Option<(Duration, Option<String>)>,
}

pub fn state_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("state_system")
		.read_resource::<Duration>()
		.with_query(<(Entity, &mut SpriteRender, &mut State)>::query())
		.build(move |command_buffer, world, resources, query| {
			let delta = resources;

			for (entity, sprite_render, state) in query.iter_mut(world) {
				let state = &mut *state;

				if let Some((timer, next)) = &mut state.next {
					timer.tick(**delta);

					while timer.is_zero() {
						if let Some(new_state_name) = next {
							let new_state = state
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
