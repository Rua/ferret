use crate::{
	common::{frame::FrameState, spawn::SpawnMergerHandlerSet},
	doom::{
		draw::sprite::SpriteRender,
		physics::{BoxCollider, SolidBits},
		state::{State, StateAction, StateName},
	},
};
use legion::{
	component,
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Resources, SystemBuilder, Write,
};
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct NextState {
	pub time: Duration,
	pub state: (StateName, usize),
}

pub fn next_entity_state(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<NextState>();

	SystemBuilder::new("next_entity_state")
		.read_resource::<FrameState>()
		.with_query(<(Entity, &Entity, &NextState)>::query())
		.with_query(<&mut State>::query())
		.build(move |command_buffer, world, resources, queries| {
			let frame_state = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, next_state) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(state) = queries.1.get_mut(&mut world, target) {
					if let StateAction::None = state.action {
						state.timer.restart_with(frame_state.time, next_state.time);
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
pub struct SetBlocksTypes(pub SolidBits);

pub fn set_blocks_types(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<SetBlocksTypes>();

	SystemBuilder::new("set_blocks_types")
		.with_query(<(Entity, &Entity, &SetBlocksTypes)>::query())
		.with_query(<&mut BoxCollider>::query().filter(component::<State>()))
		.build(move |command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, SetBlocksTypes(blocks_types)) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(box_collider) = queries.1.get_mut(&mut world, target) {
					box_collider.blocks_types = *blocks_types;
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
		.with_query(<(Entity, &Entity, &SetEntitySprite)>::query())
		.with_query(<&mut SpriteRender>::query().filter(component::<State>()))
		.build(move |command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, SetEntitySprite(sprite)) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(sprite_render) = queries.1.get_mut(&mut world, target) {
					*sprite_render = sprite.clone();
				}
			}
		})
}
