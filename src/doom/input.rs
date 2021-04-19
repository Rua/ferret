use serde::{Deserialize, Serialize};

#[inline]
pub fn bool_values() -> impl IntoIterator<Item = &'static str> {
	std::array::IntoIter::new([
		"attack", "use", "walk", "weapon1", "weapon2", "weapon3", "weapon4", "weapon5", "weapon6",
		"weapon7",
	])
}

#[inline]
pub fn float_values() -> impl IntoIterator<Item = &'static str> {
	std::array::IntoIter::new(["forward", "strafe", "yaw", "pitch"])
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserCommand {
	pub attack: bool,
	pub weapon: Option<String>,
	pub r#use: bool,
	pub forward: f32,
	pub pitch: f32,
	pub strafe: f32,
	pub yaw: f32,
}
