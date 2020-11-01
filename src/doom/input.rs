#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum BoolInput {
	Attack,
	Use,
	Walk,
	Weapon1,
	Weapon2,
	Weapon3,
	Weapon4,
	Weapon5,
	Weapon6,
	Weapon7,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum FloatInput {
	Forward,
	Pitch,
	Strafe,
	Yaw,
}

#[derive(Clone, Debug, Default)]
pub struct UserCommand {
	pub attack: bool,
	pub weapon: Option<String>,
	pub r#use: bool,
	pub forward: f32,
	pub pitch: f32,
	pub strafe: f32,
	pub yaw: f32,
}
