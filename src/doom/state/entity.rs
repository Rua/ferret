use crate::{
	common::frame::FrameState,
	doom::{
		physics::{BoxCollider, SolidBits},
		render::sprite::SpriteRender,
		state::{State, StateAction, StateName},
	},
};
use legion::{component, systems::Runnable, Entity, IntoQuery, Resources, SystemBuilder};
use std::time::Duration;

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
