use crate::{
	doom::{components::Velocity, data::FRAME_RATE},
	geometry::Angle,
};
use derivative::Derivative;
use legion::prelude::{IntoQuery, Read, ResourceSet, Resources, World, Write};
use nalgebra::{Vector2, Vector3};
use std::time::Duration;

#[derive(Clone, Copy, Debug, Derivative)]
#[derivative(Default)]
pub struct Camera {
	#[derivative(Default(value = "Vector3::zeros()"))]
	pub base: Vector3<f32>,
	#[derivative(Default(value = "Vector3::zeros()"))]
	pub offset: Vector3<f32>,
	pub bob_angle: Angle,
	pub bob_max: f32,
	pub bob_period: Duration,
}

pub fn camera_system() -> Box<dyn FnMut(&mut World, &mut Resources)> {
	Box::new(|world, resources| {
		let delta = <Read<Duration>>::fetch(resources);

		for (velocity, mut camera) in <(Read<Velocity>, Write<Camera>)>::query().iter_mut(world) {
			let div = delta.as_secs_f64() / camera.bob_period.as_secs_f64(); // TODO replace with div_duration_f64 once it's stable
			camera.bob_angle += Angle::from_units(div);

			let velocity2 = Vector2::new(velocity.velocity[0], velocity.velocity[1]) / FRAME_RATE;
			let amplitude = (velocity2.norm_squared() * 0.25).min(camera.bob_max) * 0.5;
			let bob = amplitude * camera.bob_angle.sin() as f32;
			camera.offset[2] = bob;
		}
	})
}
