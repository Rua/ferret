use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		frame::FrameState,
		spawn::{ComponentAccessor, SpawnFrom, SpawnMergerHandlerSet},
		time::Timer,
	},
	doom::{
		draw::wsprite::WeaponSpriteSlot,
		map::spawn::{spawn_helper, SpawnContext},
		template::{EntityTemplateRef, WeaponTemplate},
	},
};
use arrayvec::ArrayString;
use legion::{
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Read, Resources, Schedule, SystemBuilder, World, Write,
};
use std::{
	sync::atomic::{AtomicBool, Ordering},
	time::Duration,
};

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

#[derive(Clone, Copy, Debug)]
pub struct StateSpawnContext<T>(pub T);

#[derive(Clone, Copy, Debug, Default)]
pub struct EntityDef;

impl SpawnFrom<EntityDef> for Entity {
	fn spawn(_component: &EntityDef, _accessor: ComponentAccessor, resources: &Resources) -> Self {
		<Read<StateSpawnContext<Entity>>>::fetch(resources).0
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

#[derive(Default, Debug)]
struct StateSystemsRun(AtomicBool);

pub fn state(
	resources: &mut Resources,
	mut actions: Schedule,
) -> impl FnMut(&mut World, &mut Resources) {
	let mut schedule = Schedule::builder()
		.add_system(entity_state(resources))
		.add_system(weapon_state(resources))
		.build();

	move |world, resources| loop {
		resources.insert(StateSystemsRun::default());
		schedule.execute(world, resources);

		{
			let systems_run = <Read<StateSystemsRun>>::fetch(resources)
				.0
				.swap(false, Ordering::Relaxed);

			if !systems_run {
				resources.remove::<StateSystemsRun>();
				break;
			}
		}

		actions.execute(world, resources);
	}
}

pub fn entity_state(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_spawn::<StateDef, State>();
	handler_set.register_spawn::<EntityDef, Entity>();

	SystemBuilder::new("set_entity_state")
		.read_resource::<FrameState>()
		.read_resource::<StateSystemsRun>()
		.with_query(<(Entity, &EntityTemplateRef, &mut State)>::query())
		.build(move |command_buffer, world, resources, query| {
			let (frame_state, state_systems_run) = resources;

			for (&entity, template_ref, state) in query.iter_mut(world) {
				if let StateAction::Wait(state_name) = state.action {
					if state.timer.is_elapsed(frame_state.time) {
						state.action = StateAction::Set(state_name);
					}
				}

				if let StateAction::Set(state_name) = state.action {
					state_systems_run.0.store(true, Ordering::Relaxed);
					state.action = StateAction::None;
					let handle = template_ref.0.clone();

					command_buffer.exec_mut(move |world, resources| {
						resources.insert(StateSpawnContext(entity));
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
				resources.remove::<StateSpawnContext<Entity>>();
			});
		})
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WeaponSpriteSlotDef;

impl SpawnFrom<WeaponSpriteSlotDef> for WeaponSpriteSlot {
	fn spawn(
		_component: &WeaponSpriteSlotDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		<Read<StateSpawnContext<WeaponSpriteSlot>>>::fetch(resources).0
	}
}

#[derive(Clone, Debug)]
pub struct WeaponState {
	pub slots: [State; 2],
	pub current: AssetHandle<WeaponTemplate>,
	pub switch_to: Option<AssetHandle<WeaponTemplate>>,
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
			slots: [
				State {
					timer: Timer::new_elapsed(frame_state.time, Duration::default()),
					action: StateAction::Set((StateName::from("up").unwrap(), 0)),
				},
				State {
					timer: Timer::new_elapsed(frame_state.time, Duration::default()),
					action: StateAction::None,
				},
			],
			current,
			switch_to: None,
		}
	}
}

pub fn weapon_state(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_spawn::<WeaponStateDef, WeaponState>();
	handler_set.register_spawn::<WeaponSpriteSlotDef, WeaponSpriteSlot>();

	const SLOTS: [WeaponSpriteSlot; 2] = [WeaponSpriteSlot::Weapon, WeaponSpriteSlot::Flash];

	SystemBuilder::new("set_weapon_state")
		.read_resource::<FrameState>()
		.read_resource::<StateSystemsRun>()
		.with_query(<(Entity, &mut WeaponState)>::query())
		.build(move |command_buffer, world, resources, query| {
			let (frame_state, state_systems_run) = resources;

			for (&entity, weapon_state) in query.iter_mut(world) {
				for slot in SLOTS.iter().copied() {
					let state = &mut weapon_state.slots[slot as usize];

					if let StateAction::Wait(state_name) = state.action {
						if state.timer.is_elapsed(frame_state.time) {
							state.action = StateAction::Set(state_name);
						}
					}

					if let StateAction::Set(state_name) = state.action {
						state_systems_run.0.store(true, Ordering::Relaxed);
						state.action = StateAction::None;
						let handle = weapon_state.current.clone();

						command_buffer.exec_mut(move |world, resources| {
							resources.insert(StateSpawnContext(entity));
							resources.insert(StateSpawnContext(slot));
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
			}

			command_buffer.exec_mut(move |_world, resources| {
				resources.remove::<StateSpawnContext<Entity>>();
				resources.remove::<StateSpawnContext<WeaponSpriteSlot>>();
			});
		})
}
