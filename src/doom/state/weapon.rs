use crate::{
	common::{frame::FrameState, geometry::Angle},
	doom::{
		camera::{Camera, MovementBob},
		client::Client,
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
					if let StateAction::None = weapon_state.state.action {
						weapon_state
							.state
							.timer
							.restart_with(frame_state.time, next_state.time);
						weapon_state.state.action = StateAction::Wait(next_state.state);
					}
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

#[derive(Clone, Copy, Debug)]
pub enum WeaponPosition {
	Bob,
	Down,
	Up,
}

pub fn weapon_position_system(_resources: &mut Resources) -> impl Runnable {
	const DOWN_SPEED: f32 = 6.0;
	const UP_SPEED: f32 = -6.0;

	SystemBuilder::new("weapon_position_system")
		.read_resource::<FrameState>()
		.with_query(<(Entity, &Entity, &WeaponPosition)>::query())
		.with_query(
			<(
				&Camera,
				&MovementBob,
				&mut WeaponState,
				&mut WeaponSpriteRender,
			)>::query()
			.filter(component::<WeaponState>()),
		)
		.build(move |command_buffer, world, resources, queries| {
			let frame_state = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, weapon_position) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok((camera, movement_bob, weapon_state, weapon_sprite_render)) =
					queries.1.get_mut(&mut world, target)
				{
					match weapon_position {
						WeaponPosition::Bob => {
							let mut angle = Angle::from_units(
								frame_state.time.as_secs_f64()
									/ camera.weapon_bob_period.as_secs_f64(),
							);
							weapon_sprite_render.position[0] =
								movement_bob.amplitude * angle.cos() as f32;

							angle.0 &= 0x7FFF_FFFF;
							weapon_sprite_render.position[1] =
								movement_bob.amplitude * angle.sin() as f32;
						}
						WeaponPosition::Down => {
							weapon_sprite_render.position[1] += DOWN_SPEED;

							if weapon_sprite_render.position[1] >= 96.0 {
								weapon_sprite_render.position[1] = 96.0;

								if let Some(switch_to) = weapon_state.switch_to.take() {
									let state_name = (StateName::from("up").unwrap(), 0);
									weapon_state.state.action = StateAction::Set(state_name);
									weapon_state.current = switch_to;
								}
							}
						}
						WeaponPosition::Up => {
							weapon_sprite_render.position[1] += UP_SPEED;

							if weapon_sprite_render.position[1] <= 0.0 {
								weapon_sprite_render.position[1] = 0.0;
								let state_name = (StateName::from("ready").unwrap(), 0);
								weapon_state.state.action = StateAction::Set(state_name);
							}
						}
					}
				}
			}
		})
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WeaponReady;

pub fn weapon_ready_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("weapon_ready_system")
		.read_resource::<Client>()
		.with_query(<(Entity, &Entity, &WeaponReady)>::query())
		.with_query(<&mut WeaponState>::query().filter(component::<WeaponState>()))
		.build(move |command_buffer, world, resources, queries| {
			let client = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, WeaponReady) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(weapon_state) = queries.1.get_mut(&mut world, target) {
					if weapon_state.switch_to.is_some() {
						let state_name = (StateName::from("down").unwrap(), 0);
						weapon_state.state.action = StateAction::Set(state_name);
					} else if client.command.attack && !client.previous_command.attack {
						let state_name = (StateName::from("attack").unwrap(), 0);
						weapon_state.state.action = StateAction::Set(state_name);
					}
				}
			}
		})
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WeaponReFire;

pub fn weapon_refire_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("weapon_refire_system")
		.read_resource::<Client>()
		.with_query(<(Entity, &Entity, &WeaponReFire)>::query())
		.with_query(<&mut WeaponState>::query().filter(component::<WeaponState>()))
		.build(move |command_buffer, world, resources, queries| {
			let client = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, WeaponReFire) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(weapon_state) = queries.1.get_mut(&mut world, target) {
					if client.command.attack {
						let state_name = (StateName::from("attack").unwrap(), 0);
						weapon_state.state.action = StateAction::Set(state_name);
					}
				}
			}
		})
}
