use crate::input::{Bindings, InputState};
use serde::{Deserialize, Serialize};
use specs::{Read, RunNow, World, Write};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Action {
	Attack,
	SwitchWeapon(u8),
	Use,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Axis {
	Forward,
	Pitch,
	Strafe,
	Yaw,
}

#[derive(Clone, Copy, Debug)]
pub struct UserCommand {
	pub action_attack: bool,
	//pub action_switch_weapon: Option<u8>,
	pub action_use: bool,
	pub axis_forward: f32,
	pub axis_pitch: f32,
	pub axis_strafe: f32,
	pub axis_yaw: f32,
}

pub struct UserCommandSenderSystem;

impl<'a> RunNow<'a> for UserCommandSenderSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (bindings, input_state, mut command) = world.system_data::<(
			Read<Bindings<Action, Axis>>,
			Read<InputState>,
			Write<Option<UserCommand>>,
		)>();

		/*if command.is_some() {
			debug!("Command was not handled!");
		}*/

		*command = Some(UserCommand {
			action_attack: bindings.action_is_down(&Action::Attack, &input_state),
			action_use: bindings.action_is_down(&Action::Use, &input_state),
			axis_forward: bindings.axis_value(&Axis::Forward, &input_state) as f32,
			axis_pitch: bindings.axis_value(&Axis::Pitch, &input_state) as f32,
			axis_strafe: bindings.axis_value(&Axis::Strafe, &input_state) as f32,
			axis_yaw: bindings.axis_value(&Axis::Yaw, &input_state) as f32,
		});
	}
}
