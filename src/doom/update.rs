use crate::{
	doom::{
		components::Transform,
		input::{Action, Axis},
	},
	input::{Bindings, InputState},
};
use nalgebra::Vector2;
use specs::{Entity, ReadExpect, RunNow, SystemData, World, WriteStorage};

pub struct UpdateSystem;

impl<'a> RunNow<'a> for UpdateSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (entity, mut transform_storage, input_state, bindings) = <(
			ReadExpect<Entity>,
			WriteStorage<Transform>,
			ReadExpect<InputState>,
			ReadExpect<Bindings<Action, Axis>>,
		)>::fetch(world);
		let transform = transform_storage.get_mut(*entity).unwrap();

		transform.rotation[1] += (bindings.axis_value(&Axis::Pitch, &input_state) * 1e6) as i32;
		transform.rotation[1].0 =
			num_traits::clamp(transform.rotation[1].0, -0x40000000, 0x40000000);

		transform.rotation[2] -= (bindings.axis_value(&Axis::Yaw, &input_state) * 1e6) as i32;

		let axes = crate::geometry::angles_to_axes(transform.rotation);
		let mut move_dir = Vector2::new(
			bindings.axis_value(&Axis::Forward, &input_state) as f32,
			bindings.axis_value(&Axis::Strafe, &input_state) as f32,
		);
		let len = move_dir.norm();

		if len > 1.0 {
			move_dir /= len;
		}

		move_dir *= 20.0;

		transform.position += axes[0] * move_dir[0] + axes[1] * move_dir[1];
	}
}
