use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Action {
	Attack,
	SwitchWeapon(u8),
	Use,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Axis {
	Forward,
	Pitch,
	Strafe,
	Yaw,
}
