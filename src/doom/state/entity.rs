use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom, SpawnMergerHandlerSet},
		time::{GameTime, Timer},
	},
	doom::{
		draw::sprite::SpriteRender,
		physics::{BoxCollider, SolidBits, SolidType},
		sound::StartSoundEventEntity,
		spawn::spawn_helper,
		state::{State, StateAction, StateName, StateSystemsRun},
		template::{EntityTemplate, EntityTemplateRef},
	},
};
use legion::{
	component,
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Read, Registry, Resources, SystemBuilder, Write,
};
use rand::{distributions::Uniform, thread_rng, Rng};
use std::{sync::atomic::Ordering, time::Duration};

#[derive(Clone, Copy, Debug)]
pub struct EntityStateEvent {
	pub entity: Entity,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EntityStateEventDef;

impl SpawnFrom<EntityStateEventDef> for EntityStateEvent {
	fn spawn(
		_component: &EntityStateEventDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		<Read<SpawnContext<EntityStateEvent>>>::fetch(resources).0
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StateDef;

impl SpawnFrom<StateDef> for State {
	fn spawn(_component: &StateDef, _accessor: ComponentAccessor, resources: &Resources) -> Self {
		let (asset_storage, game_time, template_handle) = <(
			Read<AssetStorage>,
			Read<GameTime>,
			Read<SpawnContext<AssetHandle<EntityTemplate>>>,
		)>::fetch(resources);
		let template = asset_storage.get(&template_handle.0).unwrap();

		let spawn_state_name = (StateName::from("spawn").unwrap(), 0);
		template
			.states
			.get(&spawn_state_name.0)
			.and_then(|x| x.get(spawn_state_name.1))
			.expect("Entity template has no spawn state");

		State {
			timer: Timer::new_elapsed(*game_time, Duration::default()),
			action: StateAction::Set(spawn_state_name),
		}
	}
}

pub fn entity_state(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	handler_set.register_spawn::<StateDef, State>();
	registry.register::<State>("State".into());

	handler_set.register_spawn::<EntityStateEventDef, EntityStateEvent>();

	SystemBuilder::new("set_entity_state")
		.read_resource::<GameTime>()
		.read_resource::<StateSystemsRun>()
		.with_query(<(Entity, &EntityTemplateRef, &mut State)>::query())
		.build(move |command_buffer, world, resources, query| {
			let (game_time, state_systems_run) = resources;

			for (&entity, template_ref, state) in query.iter_mut(world) {
				if let StateAction::Wait(state_name) = state.action {
					if state.timer.is_elapsed(**game_time) {
						state.action = StateAction::Set(state_name);
					}
				}

				if let StateAction::Set(state_name) = state.action {
					state_systems_run.0.store(true, Ordering::Relaxed);
					state.action = StateAction::None;
					let handle = template_ref.0.clone();

					command_buffer.exec_mut(move |world, resources| {
						resources.insert(SpawnContext(EntityStateEvent { entity }));
						resources.insert(SpawnContext(StartSoundEventEntity(Some(entity))));
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
				resources.remove::<SpawnContext<EntityStateEvent>>();
				resources.remove::<SpawnContext<StartSoundEventEntity>>();
			});
		})
}

#[derive(Clone, Copy, Debug)]
pub struct NextState {
	pub time: Duration,
	pub state: (StateName, usize),
}

#[derive(Clone, Debug)]
pub struct NextStateRandomTimeDef {
	pub time: Uniform<Duration>,
	pub state: (StateName, usize),
}

impl SpawnFrom<NextStateRandomTimeDef> for NextState {
	fn spawn(
		component: &NextStateRandomTimeDef,
		_accessor: ComponentAccessor,
		_resources: &Resources,
	) -> Self {
		NextState {
			time: thread_rng().sample(component.time),
			state: component.state,
		}
	}
}

pub fn next_entity_state(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<NextState>();
	handler_set.register_spawn::<NextStateRandomTimeDef, NextState>();

	SystemBuilder::new("next_entity_state")
		.read_resource::<GameTime>()
		.with_query(<(&EntityStateEvent, &NextState)>::query())
		.with_query(<&mut State>::query())
		.build(move |_command_buffer, world, resources, queries| {
			let game_time = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, next_state) in queries.0.iter(&world0) {
				if let Ok(state) = queries.1.get_mut(&mut world, event.entity) {
					if let StateAction::None = state.action {
						state.timer.restart_with(**game_time, next_state.time);
						state.action = StateAction::Wait(next_state.state);
					}
				}
			}
		})
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RemoveEntity;

pub fn remove_entity(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<RemoveEntity>();

	SystemBuilder::new("remove_entity")
		.with_query(<(&EntityStateEvent, &RemoveEntity)>::query())
		.with_query(<&State>::query())
		.build(move |command_buffer, world, _resources, queries| {
			for (&event, RemoveEntity) in queries.0.iter(world) {
				if let Ok(_) = queries.1.get(world, event.entity) {
					command_buffer.remove(event.entity);
				}
			}
		})
}

#[derive(Clone, Copy, Debug)]
pub struct SetBlocksTypes(pub SolidBits);

pub fn set_blocks_types(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<SetBlocksTypes>();

	SystemBuilder::new("set_blocks_types")
		.with_query(<(&EntityStateEvent, &SetBlocksTypes)>::query())
		.with_query(<&mut BoxCollider>::query().filter(component::<State>()))
		.build(move |_command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, &SetBlocksTypes(blocks_types)) in queries.0.iter(&world0) {
				if let Ok(box_collider) = queries.1.get_mut(&mut world, event.entity) {
					box_collider.blocks_types = blocks_types;
				}
			}
		})
}

#[derive(Clone, Debug)]
pub struct SetEntitySprite(pub SpriteRender);

pub fn set_entity_sprite(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<SetEntitySprite>();

	SystemBuilder::new("set_entity_sprite")
		.with_query(<(&EntityStateEvent, &SetEntitySprite)>::query())
		.with_query(<&mut SpriteRender>::query().filter(component::<State>()))
		.build(move |_command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, SetEntitySprite(sprite)) in queries.0.iter(&world0) {
				if let Ok(sprite_render) = queries.1.get_mut(&mut world, event.entity) {
					*sprite_render = sprite.clone();
				}
			}
		})
}

#[derive(Clone, Copy, Debug)]
pub struct SetSolidType(pub SolidType);

pub fn set_solid_type(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<SetSolidType>();

	SystemBuilder::new("set_solid_type")
		.with_query(<(&EntityStateEvent, &SetSolidType)>::query())
		.with_query(<&mut BoxCollider>::query().filter(component::<State>()))
		.build(move |_command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, &SetSolidType(solid_type)) in queries.0.iter(&world0) {
				if let Ok(box_collider) = queries.1.get_mut(&mut world, event.entity) {
					box_collider.solid_type = solid_type;
				}
			}
		})
}
