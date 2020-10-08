use crate::{
	common::{
		assets::AssetStorage,
		time::{FrameTime, Timer},
	},
	doom::{entitytemplate::EntityTemplateRef, sprite::SpriteRender},
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
	pub current: StateName,
	pub timer: Option<Timer>,
}

pub fn state_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("state_system")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameTime>()
		.with_query(<(Entity, &EntityTemplateRef, &mut SpriteRender, &mut State)>::query())
		.build(move |command_buffer, world, resources, query| {
			let (asset_storage, frame_time) = resources;

			for (entity, template_ref, sprite_render, state) in query.iter_mut(world) {
				let states = &asset_storage.get(&template_ref.0).unwrap().states;
				let State { current, timer } = state;
				timer.as_mut().map(|t| t.tick(frame_time.delta));

				while timer.map_or(false, |t| t.is_zero()) {
					if let Some(new_state_name) = states[current].next.unwrap().1 {
						let new_state = states
							.get(&new_state_name)
							.expect("Invalid next state name");
						*current = new_state_name;
						*sprite_render = new_state.sprite.clone();
						*timer = new_state.next.map(|(time, _)| Timer::new(time));
					} else {
						// Delete the entity if the next state is None
						*timer = None;
						command_buffer.remove(*entity);
					}
				}
			}
		})
}
