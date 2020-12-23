use crate::{
	common::{
		assets::AssetHandle, frame::FrameState, geometry::Angle, spawn::SpawnMergerHandlerSet,
	},
	doom::{
		data::FRAME_RATE,
		physics::{Physics, StepEvent},
		sound::Sound,
	},
};
use legion::{
	systems::{ResourceSet, Runnable},
	IntoQuery, Resources, SystemBuilder, Write,
};
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
	pub extra_light: f32,
}

pub fn camera_system(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut step_event_channel) =
		<(Write<SpawnMergerHandlerSet>, Write<EventChannel<StepEvent>>)>::fetch_mut(resources);

	handler_set.register_clone::<Camera>();
	let mut step_event_reader = step_event_channel.register_reader();

	SystemBuilder::new("camera_system")
		.read_resource::<FrameState>()
		.read_resource::<EventChannel<StepEvent>>()
		.with_query(<&mut Camera>::query())
		.with_query(<(&MovementBob, &mut Camera)>::query())
		.build(move |_command_buffer, world, resources, queries| {
			let (frame_state, step_event_channel) = resources;

			// Entity stepping up
			for step_event in step_event_channel.read(&mut step_event_reader) {
				if let Ok(mut camera) = queries.0.get_mut(world, step_event.entity) {
					camera.deviation_position -= step_event.height;
					camera.deviation_velocity = -camera.deviation_position / 8.0 * FRAME_RATE;
				}
			}

			for (movement_bob, mut camera) in queries.1.iter_mut(world) {
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
			}
		})
}

#[derive(Clone, Debug)]
pub struct MovementBob {
	pub amplitude: f32,
	pub max: f32,
}

pub fn movement_bob_system(resources: &mut Resources) -> impl Runnable {
	resources
		.get_mut::<SpawnMergerHandlerSet>()
		.expect("Required resource not present")
		.register_clone::<MovementBob>();

	SystemBuilder::new("movement_bob_system")
		.with_query(<(&Physics, &mut MovementBob)>::query())
		.build(move |_command_buffer, world, _resources, query| {
			for (physics, movement_bob) in query.iter_mut(world) {
				let velocity2: Vector2<f32> = physics.velocity.fixed_resize(0.0) / FRAME_RATE;
				movement_bob.amplitude = (velocity2.norm_squared() * 0.25).min(movement_bob.max);
			}
		})
}
