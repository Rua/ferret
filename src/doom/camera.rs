use crate::{
	assets::AssetHandle,
	audio::Sound,
	doom::{components::Velocity, data::FRAME_RATE, physics::TouchEvent},
	geometry::Angle,
};
use legion::prelude::{Entity, IntoQuery, Read, ResourceSet, Resources, World, Write};
use nalgebra::{Vector2, Vector3};
use shrev::EventChannel;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Camera {
	pub base: Vector3<f32>,
	pub offset: Vector3<f32>,
	pub bob_angle: Angle,
	pub bob_max: f32,
	pub bob_period: Duration,
	pub deviation_position: f32,
	pub deviation_velocity: f32,
	pub impact_sound: AssetHandle<Sound>,
}

pub fn camera_system(resources: &mut Resources) -> Box<dyn FnMut(&mut World, &mut Resources)> {
	let mut touch_event_reader = resources
		.get_mut::<EventChannel<TouchEvent>>()
		.unwrap()
		.register_reader();

	Box::new(move |world, resources| {
		let (delta, touch_event_channel, mut sound_queue) = <(
			Read<Duration>,
			Read<EventChannel<TouchEvent>>,
			Write<Vec<(AssetHandle<Sound>, Entity)>>,
		)>::fetch_mut(resources);

		// Handle entity hitting the ground
		for touch_event in touch_event_channel.read(&mut touch_event_reader) {
			if let (Some(mut camera), Some(collision)) = (
				world.get_component_mut::<Camera>(touch_event.toucher),
				touch_event.collision,
			) {
				let down_speed = collision.normal[2] * collision.speed;

				if down_speed >= 8.0 * FRAME_RATE {
					camera.deviation_velocity = -down_speed / 8.0;
					sound_queue.push((camera.impact_sound.clone(), touch_event.toucher));
				}
			}
		}

		// Set camera position
		for (velocity, mut camera) in <(Read<Velocity>, Write<Camera>)>::query().iter_mut(world) {
			camera.deviation_position += camera.deviation_velocity * delta.as_secs_f32();

			if camera.deviation_position > 0.0 {
				// Hit the top
				camera.deviation_position = 0.0;
				camera.deviation_velocity = 0.0;
			} else if camera.deviation_position < -camera.base[2] * 0.5 {
				// Hit the bottom, bounce back up
				camera.deviation_position = -camera.base[2] * 0.5;

				if camera.deviation_velocity <= 0.0 {
					camera.deviation_velocity = 0.0;
				}
			} else {
				camera.deviation_velocity += 0.25 / delta.as_secs_f32();
			}

			let div = delta.as_secs_f64() / camera.bob_period.as_secs_f64(); // TODO replace with div_duration_f64 once it's stable
			camera.bob_angle += Angle::from_units(div);

			let velocity2 = Vector2::new(velocity.velocity[0], velocity.velocity[1]) / FRAME_RATE;
			let amplitude = (velocity2.norm_squared() * 0.25).min(camera.bob_max) * 0.5;
			let bob = amplitude * camera.bob_angle.sin() as f32;

			camera.offset[2] = camera.deviation_position + bob;
		}
	})
}
