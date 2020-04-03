use std::time::Duration;

pub mod client;
pub mod components;
pub mod door;
pub mod entities;
pub mod image;
pub mod input;
pub mod light;
pub mod map;
pub mod physics;
pub mod render;
pub mod sound;
pub mod sprite;
pub mod update;
pub mod wad;

pub const FRAME_TIME: Duration = Duration::from_nanos(28571429); // 1/35 sec
