use crate::{
	common::{
		assets::AssetStorage,
		frame::FrameState,
		spawn::{ComponentAccessor, SpawnFrom},
		time::Timer,
	},
	doom::{entitytemplate::EntityTemplateRef, map::spawn::SpawnContext, sprite::SpriteRender},
};
use arrayvec::ArrayString;
use legion::{
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Read, Resources, SystemBuilder,
};
use std::time::Duration;

pub type StateName = ArrayString<[u8; 16]>;

#[derive(Clone, Debug)]
pub struct StateInfo {
	pub sprite: SpriteRender,
	pub next: Option<(Duration, Option<(StateName, usize)>)>,
}

#[derive(Clone, Debug)]
pub struct State {
	pub current: (StateName, usize),
	pub timer: Option<Timer>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StateDef;

impl SpawnFrom<StateDef> for State {
	fn from_with_resources(
		_component: &StateDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> State {
		let (asset_storage, frame_state, spawn_context) =
			<(Read<AssetStorage>, Read<FrameState>, Read<SpawnContext>)>::fetch(resources);
		let template = asset_storage.get(&spawn_context.template_handle).unwrap();

		let spawn_state_name = (StateName::from("spawn").unwrap(), 0);
		let spawn_state = template
			.states
			.get(&spawn_state_name.0)
			.and_then(|x| x.get(spawn_state_name.1))
			.expect("Entity template has no spawn state");

		State {
			current: spawn_state_name,
			timer: spawn_state
				.next
				.map(|(time, _)| Timer::new(frame_state.time, time)),
		}
	}
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

				while timer.map_or(false, |t| t.is_elapsed(frame_state.time)) {
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

					if let Some((time, _)) = new_state.next {
						timer.as_mut().unwrap().restart_with(time);
					} else {
						*timer = None;
					}
				}
			}
		})
}
