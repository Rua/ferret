use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		frame::FrameState,
		spawn::{ComponentAccessor, SpawnFrom},
		time::Timer,
	},
	doom::{
		map::spawn::SpawnContext,
		physics::{BoxCollider, SolidBits},
		psprite::WeaponSpriteRender,
		sound::Sound,
		sprite::SpriteRender,
		template::{EntityTemplateRef, WeaponTemplate},
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

	pub blocks_types: Option<SolidBits>,
	pub sound: Option<AssetHandle<Sound>>,
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
	SystemBuilder::new("state_set_system")
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
					if let Some(blocks_types) = &state_info.blocks_types {
						box_collider.blocks_types = *blocks_types;
					}
				});
			}
		})
}

pub fn sound_play_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("sound_play_system")
		.read_resource::<AssetStorage>()
		.write_resource::<Vec<(AssetHandle<Sound>, Entity)>>()
		.with_query(<((&EntityTemplateRef, &State), Entity)>::query())
		.build(move |_command_buffer, world, resources, query| {
			let (asset_storage, sound_queue) = resources;

			for (state_data, entity) in query.iter_mut(world) {
				state_trigger(state_data, asset_storage, |state_info| {
					if let Some(sound) = &state_info.sound {
						sound_queue.push((sound.clone(), *entity));
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

#[derive(Clone, Debug, Default)]
pub struct WeaponStateInfo {
	pub time: Option<Duration>,
	pub next: Option<(StateName, usize)>,
	pub remove: bool,

	pub sound: Option<AssetHandle<Sound>>,
	pub sprite: Option<SpriteRender>,
}

#[derive(Clone, Debug)]
pub struct WeaponState {
	pub template: AssetHandle<WeaponTemplate>,
	pub state: State,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WeaponStateDef;

impl SpawnFrom<WeaponStateDef> for WeaponState {
	fn spawn(
		_component: &WeaponStateDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> WeaponState {
		let (asset_storage, frame_state) =
			<(Read<AssetStorage>, Read<FrameState>)>::fetch(resources);

		let template = asset_storage
			.handle_for::<WeaponTemplate>("shotgun")
			.unwrap();

		WeaponState {
			state: State {
				timer: Timer::new_elapsed(frame_state.time, Duration::default()),
				action: StateAction::Set((StateName::from("up").unwrap(), 0)),
			},
			template,
		}
	}
}

pub fn weapon_state_set_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("weapon_state_set_system")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameState>()
		.with_query(<(Entity, &mut WeaponState)>::query())
		.build(move |command_buffer, world, resources, query| {
			let (asset_storage, frame_state) = resources;

			for (entity, weapon_state) in query.iter_mut(world) {
				if let StateAction::Wait(state_name) = weapon_state.state.action {
					if weapon_state.state.timer.is_elapsed(frame_state.time) {
						weapon_state.state.action = StateAction::Set(state_name);
					}
				}

				if let StateAction::Set(state_name) = weapon_state.state.action {
					let states = &asset_storage.get(&weapon_state.template).unwrap().states;
					let state_info = states
						.get(&state_name.0)
						.and_then(|x| x.get(state_name.1))
						.unwrap_or_else(|| panic!("Invalid state {:?}", state_name));

					if state_info.remove {
						weapon_state.state.action = StateAction::None;
						command_buffer.remove(*entity);
					}
				}
			}
		})
}

pub fn weapon_state_next_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("weapon_state_next_system")
		.read_resource::<AssetStorage>()
		.with_query(<&mut WeaponState>::query())
		.build(move |_command_buffer, world, resources, query| {
			let asset_storage = resources;

			for weapon_state in query.iter_mut(world) {
				if let StateAction::Set(state_name) = weapon_state.state.action {
					let states = &asset_storage.get(&weapon_state.template).unwrap().states;
					let state_info = states
						.get(&state_name.0)
						.and_then(|x| x.get(state_name.1))
						.unwrap_or_else(|| panic!("Invalid state {:?}", state_name));

					if let Some(time) = state_info.time {
						weapon_state.state.timer.restart_with(time);
						weapon_state.state.action =
							StateAction::Wait(state_info.next.unwrap_or_else(|| {
								(
									state_name.0,
									(state_name.1 + 1) % states[&state_name.0].len(),
								)
							}));
					} else {
						weapon_state.state.action = StateAction::None;
					}
				}
			}
		})
}

pub fn weapon_state_trigger<F>(weapon_state: &WeaponState, asset_storage: &AssetStorage, func: F)
where
	F: FnOnce(&WeaponStateInfo),
{
	if let StateAction::Set(state_name) = weapon_state.state.action {
		let states = &asset_storage.get(&weapon_state.template).unwrap().states;
		let state_info = &states[&state_name.0][state_name.1];
		func(state_info);
	}
}

pub fn weapon_sprite_anim_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("weapon_sprite_anim_system")
		.read_resource::<AssetStorage>()
		.with_query(<(&WeaponState, &mut WeaponSpriteRender)>::query())
		.build(move |_command_buffer, world, resources, query| {
			let asset_storage = resources;

			for (weapon_state, weapon_sprite_render) in query.iter_mut(world) {
				weapon_state_trigger(weapon_state, asset_storage, |state_info| {
					if let Some(sprite) = &state_info.sprite {
						weapon_sprite_render.slots[0] = Some(sprite.clone());
					}
				});
			}
		})
}
