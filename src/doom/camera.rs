use crate::{
	common::{assets::AssetHandle, frame::FrameState, geometry::Angle},
	doom::{
		components::Velocity,
		data::FRAME_RATE,
		physics::{StepEvent, TouchEvent},
		render::wsprite::WeaponSpriteRender,
		sound::{Sound, StartSound},
	},
};
use legion::{systems::Runnable, IntoQuery, Resources, SystemBuilder};
use nalgebra::{Vector2, Vector3};
use shrev::EventChannel;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Camera {
	pub base: Vector3<f32>,
	pub offset: Vector3<f32>,
	pub bob_period: Duration,
	pub weapon_bob_period: Duration,
	pub deviation_position: f32,
	pub deviation_velocity: f32,
	pub impact_sound: AssetHandle<Sound>,
}

#[derive(Clone, Debug)]
pub struct MovementBob {
	pub amplitude: f32,
	pub max: f32,
}

pub fn camera_system(resources: &mut Resources) -> impl Runnable {
	let mut step_event_reader = resources
		.get_mut::<EventChannel<StepEvent>>()
		.unwrap()
		.register_reader();
	let mut touch_event_reader = resources
		.get_mut::<EventChannel<TouchEvent>>()
		.unwrap()
		.register_reader();

	SystemBuilder::new("camera_system")
		.read_resource::<FrameState>()
		.read_resource::<EventChannel<StepEvent>>()
		.read_resource::<EventChannel<TouchEvent>>()
		.with_query(<&mut Camera>::query())
		.with_query(<(&MovementBob, &mut Camera, &mut WeaponSpriteRender)>::query())
		.build(move |command_buffer, world, resources, queries| {
			let (frame_state, step_event_channel, touch_event_channel) = resources;

			// Entity hitting the ground
			for touch_event in touch_event_channel.read(&mut touch_event_reader) {
				if let (Ok(mut camera), Some(collision)) = (
					queries.0.get_mut(world, touch_event.toucher),
					touch_event.collision,
				) {
					let down_speed = collision.normal[2] * collision.speed;

					if down_speed >= 8.0 * FRAME_RATE {
						camera.deviation_velocity = -down_speed / 8.0;
						command_buffer
							.push((touch_event.toucher, StartSound(camera.impact_sound.clone())));
					}
				}
			}

			// Entity stepping up
			for step_event in step_event_channel.read(&mut step_event_reader) {
				if let Ok(mut camera) = queries.0.get_mut(world, step_event.entity) {
					camera.deviation_position -= step_event.height;
					camera.deviation_velocity = -camera.deviation_position / 8.0 * FRAME_RATE;
				}
			}

			for (movement_bob, mut camera, weapon_sprite_render) in queries.1.iter_mut(world) {
				// Calculate deviation
				if camera.deviation_position != 0.0 || camera.deviation_velocity != 0.0 {
					const DEVIATION_ACCEL: f32 = 0.25 * FRAME_RATE * FRAME_RATE;
					camera.deviation_position +=
						camera.deviation_velocity * frame_state.delta_time.as_secs_f32();
					camera.deviation_velocity +=
						DEVIATION_ACCEL * frame_state.delta_time.as_secs_f32();

					if camera.deviation_position > 0.0 {
						// Hit the top
						camera.deviation_position = 0.0;
						camera.deviation_velocity = 0.0;
					} else if camera.deviation_position < -camera.base[2] * 0.5 {
						// Hit the bottom, bounce back up
						camera.deviation_position = -camera.base[2] * 0.5;

						if camera.deviation_velocity < 0.0 {
							camera.deviation_velocity = FRAME_RATE;
						}
					}
				}

				// Set camera position
				let angle = Angle::from_units(
					frame_state.time.as_secs_f64() / camera.bob_period.as_secs_f64(),
				); // TODO replace with div_duration_f64 once it's stable
				let bob = movement_bob.amplitude * 0.5 * angle.sin() as f32;
				camera.offset[2] = camera.deviation_position + bob;

				// Set weapon position
				let mut angle = Angle::from_units(
					frame_state.time.as_secs_f64() / camera.weapon_bob_period.as_secs_f64(),
				);
				weapon_sprite_render.position[0] =
					1.0 + movement_bob.amplitude * angle.cos() as f32;

				angle.0 &= 0x7FFF_FFFF;
				weapon_sprite_render.position[1] = movement_bob.amplitude * angle.sin() as f32;
			}
		})
}

pub fn movement_bob_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("movement_bob_system")
		.with_query(<(&Velocity, &mut MovementBob)>::query())
		.build(move |_command_buffer, world, _resources, query| {
			for (velocity, movement_bob) in query.iter_mut(world) {
				let velocity2: Vector2<f32> = velocity.velocity.fixed_resize(0.0) / FRAME_RATE;
				movement_bob.amplitude = (velocity2.norm_squared() * 0.25).min(movement_bob.max);
			}
		})
}
