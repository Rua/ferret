use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Action {
	Attack,
	SwitchWeapon(u8),
	Use,
	Walk,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Axis {
	Forward,
	Pitch,
	Strafe,
	Yaw,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct UserCommand {
	pub action_attack: bool,
	//pub action_switch_weapon: Option<u8>,
	pub action_use: bool,
	pub axis_forward: f32,
	pub axis_pitch: f32,
	pub axis_strafe: f32,
	pub axis_yaw: f32,
}
