use crate::{
	common::time::{FrameTime, Timer},
	doom::render::sprite::SpriteRender,
};
use arrayvec::ArrayString;
use legion::{systems::Runnable, Entity, IntoQuery, Resources, SystemBuilder};
use std::{collections::HashMap, sync::Arc, time::Duration};

pub type StateName = ArrayString<[u8; 16]>;

#[derive(Clone, Debug)]
pub struct State {
	pub states: Arc<HashMap<StateName, StateDef>>,
	pub next: Option<(Timer, Option<StateName>)>,
	pub spawn_state: Option<StateName>,
	pub see_state: Option<StateName>,
	pub pain_state: Option<StateName>,
	pub melee_state: Option<StateName>,
	pub missile_state: Option<StateName>,
	pub death_state: Option<StateName>,
	pub xdeath_state: Option<StateName>,
	pub raise_state: Option<StateName>,
}

#[derive(Clone, Debug)]
pub struct StateDef {
	pub sprite: SpriteRender,
	pub next: Option<(Duration, Option<StateName>)>,
}

pub fn state_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("state_system")
		.read_resource::<FrameTime>()
		.with_query(<(Entity, &mut SpriteRender, &mut State)>::query())
		.build(move |command_buffer, world, resources, query| {
			let frame_time = resources;

			for (entity, sprite_render, state) in query.iter_mut(world) {
				let state = &mut *state;

				if let Some((timer, next)) = &mut state.next {
					timer.tick(frame_time.delta);

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
