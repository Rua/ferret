use crate::{
	common::frame::FrameState,
	doom::{
		render::{sprite::SpriteRender, wsprite::WeaponSpriteRender},
		state::{StateAction, StateName, WeaponState},
	},
};
use legion::{component, systems::Runnable, Entity, IntoQuery, Resources, SystemBuilder};
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct NextWeaponState {
	pub time: Duration,
	pub state: (StateName, usize),
}

pub fn next_weapon_state_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("next_weapon_state_system")
		.read_resource::<FrameState>()
		.with_query(<(Entity, &Entity, &NextWeaponState)>::query())
		.with_query(<&mut WeaponState>::query())
		.build(move |command_buffer, world, resources, queries| {
			let frame_state = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, next_state) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(weapon_state) = queries.1.get_mut(&mut world, target) {
					weapon_state
						.state
						.timer
						.restart_with(frame_state.time, next_state.time);
					weapon_state.state.action = StateAction::Wait(next_state.state);
				}
			}
		})
}

#[derive(Clone, Debug)]
pub struct SetWeaponSprite(pub SpriteRender);

pub fn set_weapon_sprite_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("set_weapon_sprite_system")
		.with_query(<(Entity, &Entity, &SetWeaponSprite)>::query())
		.with_query(<&mut WeaponSpriteRender>::query().filter(component::<WeaponState>()))
		.build(move |command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, SetWeaponSprite(sprite)) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(weapon_sprite_render) = queries.1.get_mut(&mut world, target) {
					weapon_sprite_render.slots[0] = Some(sprite.clone());
				}
			}
		})
}
