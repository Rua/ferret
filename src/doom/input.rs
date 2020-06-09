#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum BoolInput {
	Attack,
	//SwitchWeapon(u8),
	Use,
	Walk,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum FloatInput {
	Forward,
	Pitch,
	Strafe,
	Yaw,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct UserCommand {
	pub attack: bool,
	//pub action_switch_weapon: Option<u8>,
	pub r#use: bool,
	pub forward: f32,
	pub pitch: f32,
	pub strafe: f32,
	pub yaw: f32,
}
