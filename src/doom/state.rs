use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		frame::FrameState,
		spawn::{ComponentAccessor, SpawnFrom},
		time::Timer,
	},
	doom::{
		map::spawn::{spawn_helper, SpawnContext},
		physics::{BoxCollider, SolidBits},
		psprite::WeaponSpriteRender,
		sound::Sound,
		sprite::SpriteRender,
		template::{EntityTemplateRef, WeaponTemplate},
	},
};
use arrayvec::ArrayString;
use legion::{
	component,
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Read, Resources, SystemBuilder,
};
use std::time::Duration;

pub type StateName = ArrayString<[u8; 16]>;

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

#[derive(Clone, Debug)]
pub struct BlocksTypes(pub SolidBits);

pub fn blocks_types_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("blocks_types_system")
		.with_query(<(Entity, &Entity, &BlocksTypes)>::query())
		.with_query(<&mut BoxCollider>::query().filter(component::<State>()))
		.build(move |command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, BlocksTypes(blocks_types)) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(box_collider) = queries.1.get_mut(&mut world, target) {
					box_collider.blocks_types = *blocks_types;
				}
			}
		})
}

#[derive(Clone, Copy, Debug)]
pub struct NextState {
	pub time: Duration,
	pub state: (StateName, usize),
}

pub fn next_state_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("next_state_system")
		.read_resource::<FrameState>()
		.with_query(<(Entity, &Entity, &NextState)>::query())
		.with_query(<&mut State>::query())
		.build(move |command_buffer, world, resources, queries| {
			let frame_state = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, next_state) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(state) = queries.1.get_mut(&mut world, target) {
					state.timer.restart_with(frame_state.time, next_state.time);
					state.action = StateAction::Wait(next_state.state);
				}
			}
		})
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RemoveEntity;

pub fn remove_entity_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("remove_entity_system")
		.with_query(<(Entity, &Entity, &RemoveEntity)>::query())
		.with_query(<&State>::query())
		.build(move |command_buffer, world, _resources, queries| {
			for (&entity, &target, RemoveEntity) in queries.0.iter(world) {
				command_buffer.remove(entity);

				if let Ok(_) = queries.1.get(world, target) {
					command_buffer.remove(target);
				}
			}
		})
}

#[derive(Clone, Debug)]
pub struct SetSprite(pub SpriteRender);

pub fn set_sprite_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("set_sprite_system")
		.with_query(<(Entity, &Entity, &SetSprite)>::query())
		.with_query(<&mut SpriteRender>::query().filter(component::<State>()))
		.build(move |command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, SetSprite(sprite)) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(sprite_render) = queries.1.get_mut(&mut world, target) {
					*sprite_render = sprite.clone();
				}
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

			for (&entity, weapon_state) in query.iter_mut(world) {
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
						command_buffer.remove(entity);
					}
				}
			}
		})
}

pub fn weapon_state_next_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("weapon_state_next_system")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameState>()
		.with_query(<&mut WeaponState>::query())
		.build(move |_command_buffer, world, resources, query| {
			let (asset_storage, frame_state) = resources;

			for weapon_state in query.iter_mut(world) {
				if let StateAction::Set(state_name) = weapon_state.state.action {
					let states = &asset_storage.get(&weapon_state.template).unwrap().states;
					let state_info = states
						.get(&state_name.0)
						.and_then(|x| x.get(state_name.1))
						.unwrap_or_else(|| panic!("Invalid state {:?}", state_name));

					if let Some(time) = state_info.time {
						weapon_state
							.state
							.timer
							.restart_with(frame_state.time, time);
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
