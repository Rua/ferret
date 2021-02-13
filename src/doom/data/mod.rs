pub mod anims;
mod bindings;
mod fonts;
pub mod iwads;
mod linedefs;
mod mobjs;
mod sectors;
pub mod sounds;
mod weapons;

pub use bindings::get_bindings;
pub use fonts::FONTS;
pub use linedefs::LINEDEFS;
pub use mobjs::{DOOMEDNUMS, MOBJS};
pub use sectors::SECTORS;
pub use weapons::WEAPONS;

use once_cell::sync::Lazy;
use std::time::Duration;

pub const FRAME_RATE: f32 = 35.0;
pub const FRAME_TIME: Duration = Duration::from_nanos(28_571_429); // 1/35 sec

pub const FORWARD_ACCEL: f32 = (50.0 * 2048.0 / 65536.0) * FRAME_RATE * FRAME_RATE;
pub const STRAFE_ACCEL: f32 = (40.0 * 2048.0 / 65536.0) * FRAME_RATE * FRAME_RATE;

pub const GRAVITY: f32 = 1.0 * FRAME_RATE * FRAME_RATE;

pub static FRICTION: Lazy<f32> = Lazy::new(|| 0.90625f32.powf(FRAME_RATE));
