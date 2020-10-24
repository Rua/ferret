use crate::{
	common::{
		assets::AssetStorage,
		frame::FrameState,
		spawn::{ComponentAccessor, SpawnFrom},
		time::Timer,
	},
	doom::{
		entitytemplate::EntityTemplateRef,
		map::spawn::SpawnContext,
		physics::{BoxCollider, SolidMask},
		sprite::SpriteRender,
	},
};
use arrayvec::ArrayString;
use legion::{
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Read, Resources, SystemBuilder,
};
use std::time::Duration;

pub type StateName = ArrayString<[u8; 16]>;

#[derive(Clone, Debug, Default)]
pub struct StateInfo {
	pub time: Option<Duration>,
	pub next: Option<(StateName, usize)>,
	pub remove: bool,
	pub solid_mask: Option<SolidMask>,
	pub sprite: Option<SpriteRender>,
}

#[derive(Clone, Debug)]
pub struct State {
	pub timer: Timer,
	pub action: StateAction,
}

#[derive(Clone, Debug)]
pub enum StateAction {
	Set((StateName, usize)),
	Wait((StateName, usize)),
	None,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StateDef;

impl SpawnFrom<StateDef> for State {
	fn spawn(_component: &StateDef, _accessor: ComponentAccessor, resources: &Resources) -> State {
		let (asset_storage, frame_state, spawn_context) =
			<(Read<AssetStorage>, Read<FrameState>, Read<SpawnContext>)>::fetch(resources);
		let template = asset_storage.get(&spawn_context.template_handle).unwrap();

		let spawn_state_name = (StateName::from("spawn").unwrap(), 0);
		template
			.states
			.get(&spawn_state_name.0)
			.and_then(|x| x.get(spawn_state_name.1))
			.expect("Entity template has no spawn state");

		State {
			timer: Timer::new_elapsed(frame_state.time, Duration::default()),
			action: StateAction::Set(spawn_state_name),
		}
	}
}

pub fn state_set_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("state_timer_system")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameState>()
		.with_query(<(Entity, &EntityTemplateRef, &mut State)>::query())
		.build(move |command_buffer, world, resources, query| {
			let (asset_storage, frame_state) = resources;

			for (entity, template_ref, state) in query.iter_mut(world) {
				if let StateAction::Wait(state_name) = state.action {
					if state.timer.is_elapsed(frame_state.time) {
						state.action = StateAction::Set(state_name);
					}
				}

				if let StateAction::Set(state_name) = state.action {
					let states = &asset_storage.get(&template_ref.0).unwrap().states;
					let state_info = states
						.get(&state_name.0)
						.and_then(|x| x.get(state_name.1))
						.unwrap_or_else(|| panic!("Invalid state {:?}", state_name));

					if state_info.remove {
						state.action = StateAction::None;
						command_buffer.remove(*entity);
					}
				}
			}
		})
}

pub fn state_next_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("state_next_system")
		.read_resource::<AssetStorage>()
		.with_query(<(&EntityTemplateRef, &mut State)>::query())
		.build(move |_command_buffer, world, resources, query| {
			let asset_storage = resources;

			for (template_ref, state) in query.iter_mut(world) {
				if let StateAction::Set(state_name) = state.action {
					let states = &asset_storage.get(&template_ref.0).unwrap().states;
					let state_info = states
						.get(&state_name.0)
						.and_then(|x| x.get(state_name.1))
						.unwrap_or_else(|| panic!("Invalid state {:?}", state_name));

					if let Some(time) = state_info.time {
						state.timer.restart_with(time);
						state.action = StateAction::Wait(state_info.next.unwrap_or_else(|| {
							(
								state_name.0,
								(state_name.1 + 1) % states[&state_name.0].len(),
							)
						}));
					} else {
						state.action = StateAction::None;
					}
				}
			}
		})
}

pub fn state_trigger<F>(
	state_data: (&EntityTemplateRef, &State),
	asset_storage: &AssetStorage,
	func: F,
) where
	F: FnOnce(&StateInfo),
{
	let (template_ref, state) = state_data;
	if let StateAction::Set(state_name) = state.action {
		let states = &asset_storage.get(&template_ref.0).unwrap().states;
		let state_info = &states[&state_name.0][state_name.1];
		func(state_info);
	}
}

pub fn solid_mask_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("solid_mask_system")
		.read_resource::<AssetStorage>()
		.with_query(<((&EntityTemplateRef, &State), &mut BoxCollider)>::query())
		.build(move |_command_buffer, world, resources, query| {
			let asset_storage = resources;

			for (state_data, box_collider) in query.iter_mut(world) {
				state_trigger(state_data, asset_storage, |state_info| {
					if let Some(solid_mask) = &state_info.solid_mask {
						box_collider.solid_mask = *solid_mask;
					}
				});
			}
		})
}

pub fn sprite_anim_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("sprite_anim_system")
		.read_resource::<AssetStorage>()
		.with_query(<((&EntityTemplateRef, &State), &mut SpriteRender)>::query())
		.build(move |_command_buffer, world, resources, query| {
			let asset_storage = resources;

			for (state_data, sprite_render) in query.iter_mut(world) {
				state_trigger(state_data, asset_storage, |state_info| {
					if let Some(sprite) = &state_info.sprite {
						*sprite_render = sprite.clone();
					}
				});
			}
		})
}
