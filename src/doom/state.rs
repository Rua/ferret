use crate::{
	common::{assets::AssetStorage, frame::FrameState, time::OldTimer},
	doom::{entitytemplate::EntityTemplateRef, sprite::SpriteRender},
};
use arrayvec::ArrayString;
use legion::{systems::Runnable, Entity, IntoQuery, Resources, SystemBuilder};
use std::time::Duration;

pub type StateName = ArrayString<[u8; 16]>;

#[derive(Clone, Debug)]
pub struct StateDef {
	pub sprite: SpriteRender,
	pub next: Option<(Duration, Option<(StateName, usize)>)>,
}

#[derive(Clone, Debug)]
pub struct State {
	pub current: (StateName, usize),
	pub timer: Option<OldTimer>,
}

pub fn state_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("state_system")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameState>()
		.with_query(<(Entity, &EntityTemplateRef, &mut SpriteRender, &mut State)>::query())
		.build(move |_command_buffer, world, resources, query| {
			let (asset_storage, frame_state) = resources;

			for (_entity, template_ref, sprite_render, state) in query.iter_mut(world) {
				let states = &asset_storage.get(&template_ref.0).unwrap().states;
				let State { current, timer } = state;
				timer.as_mut().map(|t| t.tick(frame_state.delta_time));

				while timer.map_or(false, |t| t.is_zero()) {
					let new = if let Some(new) = states[&current.0][current.1].next.unwrap().1 {
						new
					} else {
						(current.0, (current.1 + 1) % states[&current.0].len())
					};

					let new_state = states
						.get(&new.0)
						.and_then(|x| x.get(new.1))
						.expect("Invalid next state name");
					*current = new;
					*sprite_render = new_state.sprite.clone();
					*timer = new_state.next.map(|(time, _)| OldTimer::new(time));
				}
			}
		})
}
