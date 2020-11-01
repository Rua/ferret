use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		frame::FrameState,
		spawn::{ComponentAccessor, SpawnFrom},
		time::Timer,
	},
	doom::{
		map::spawn::{spawn_helper, SpawnContext},
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

pub mod entity;
pub mod weapon;

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

#[derive(Clone, Debug)]
pub struct WeaponState {
	pub state: State,
	pub current: AssetHandle<WeaponTemplate>,
	pub switch_to: Option<AssetHandle<WeaponTemplate>>,
}

#[derive(Clone, Copy, Debug)]
pub struct StateSpawnContext {
	pub entity: Entity,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EntityDef;

impl SpawnFrom<EntityDef> for Entity {
	fn spawn(_component: &EntityDef, _accessor: ComponentAccessor, resources: &Resources) -> Self {
		<Read<StateSpawnContext>>::fetch(resources).entity
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StateDef;

impl SpawnFrom<StateDef> for State {
	fn spawn(_component: &StateDef, _accessor: ComponentAccessor, resources: &Resources) -> Self {
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
		.read_resource::<FrameState>()
		.with_query(<(Entity, &EntityTemplateRef, &mut State)>::query())
		.build(move |command_buffer, world, resources, query| {
			let frame_state = resources;

			for (&entity, template_ref, state) in query.iter_mut(world) {
				if let StateAction::Wait(state_name) = state.action {
					if state.timer.is_elapsed(frame_state.time) {
						state.action = StateAction::Set(state_name);
					}
				}

				if let StateAction::Set(state_name) = state.action {
					state.action = StateAction::None;
					let handle = template_ref.0.clone();

					command_buffer.exec_mut(move |world, resources| {
						resources.insert(StateSpawnContext { entity });
						let asset_storage = <Read<AssetStorage>>::fetch(resources);
						let state_world = &asset_storage
							.get(&handle)
							.unwrap()
							.states
							.get(&state_name.0)
							.and_then(|x| x.get(state_name.1))
							.unwrap_or_else(|| panic!("Invalid state {:?}", state_name));

						spawn_helper(&state_world, world, resources);
					});
				}
			}

			command_buffer.exec_mut(move |_world, resources| {
				resources.remove::<StateSpawnContext>();
			});
		})
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

		let current = asset_storage
			.handle_for::<WeaponTemplate>("pistol")
			.unwrap();

		WeaponState {
			state: State {
				timer: Timer::new_elapsed(frame_state.time, Duration::default()),
				action: StateAction::Set((StateName::from("up").unwrap(), 0)),
			},
			current,
			switch_to: None,
		}
	}
}

pub fn weapon_state_set_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("weapon_state_set_system")
		.read_resource::<FrameState>()
		.with_query(<(Entity, &mut WeaponState)>::query())
		.build(move |command_buffer, world, resources, query| {
			let frame_state = resources;

			for (&entity, weapon_state) in query.iter_mut(world) {
				if let StateAction::Wait(state_name) = weapon_state.state.action {
					if weapon_state.state.timer.is_elapsed(frame_state.time) {
						weapon_state.state.action = StateAction::Set(state_name);
					}
				}

				if let StateAction::Set(state_name) = weapon_state.state.action {
					weapon_state.state.action = StateAction::None;
					let handle = weapon_state.current.clone();

					command_buffer.exec_mut(move |world, resources| {
						resources.insert(StateSpawnContext { entity });
						let asset_storage = <Read<AssetStorage>>::fetch(resources);
						let state_world = &asset_storage
							.get(&handle)
							.unwrap()
							.states
							.get(&state_name.0)
							.and_then(|x| x.get(state_name.1))
							.unwrap_or_else(|| panic!("Invalid state {:?}", state_name));

						spawn_helper(&state_world, world, resources);
					});
				}
			}
		})
}
