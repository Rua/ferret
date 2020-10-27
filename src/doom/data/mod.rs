pub mod anims;
mod bindings;
pub mod linedefs;
pub mod mobjs;
pub mod sectors;
pub mod sounds;
pub mod weapons;

pub use bindings::get_bindings;

use lazy_static::lazy_static;
use std::time::Duration;

pub const FRAME_RATE: f32 = 35.0;
pub const FRAME_TIME: Duration = Duration::from_nanos(28_571_429); // 1/35 sec

pub const FORWARD_ACCEL: f32 = (50.0 * 2048.0 / 65536.0) * FRAME_RATE * FRAME_RATE;
pub const STRAFE_ACCEL: f32 = (40.0 * 2048.0 / 65536.0) * FRAME_RATE * FRAME_RATE;

pub const GRAVITY: f32 = 1.0 * FRAME_RATE * FRAME_RATE;

lazy_static! {
	pub static ref FRICTION: f32 = 0.90625f32.powf(FRAME_RATE);
}
