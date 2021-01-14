use legion::{systems::Runnable, SystemBuilder};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct GameTime(pub Duration);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DeltaTime(pub Duration);

pub fn increment_game_time() -> impl Runnable {
	SystemBuilder::new("increment_game_time")
		.read_resource::<DeltaTime>()
		.write_resource::<GameTime>()
		.build(move |_command_buffer, _world, resources, _query| {
			let (delta_time, game_time) = resources;
			game_time.0 += delta_time.0;
		})
}

/// A timer that elapses at the specified time
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Timer {
	target_time: Duration,
	wait_time: Duration,
}

impl Timer {
	/// Creates a new timer with the specified wait time, and sets it to elapse this duration
	/// after the current time.
	pub fn new(game_time: GameTime, wait_time: Duration) -> Self {
		Self {
			target_time: game_time.0 + wait_time,
			wait_time,
		}
	}

	/// Creates a new timer with the specified wait time, and sets it to elapse immediately.
	pub fn new_elapsed(game_time: GameTime, wait_time: Duration) -> Self {
		Self {
			target_time: game_time.0,
			wait_time,
		}
	}

	pub fn is_elapsed(&self, game_time: GameTime) -> bool {
		game_time.0 >= self.target_time
	}

	pub fn restart(&mut self, game_time: GameTime) {
		self.target_time = game_time.0 + self.wait_time;
	}

	pub fn restart_with(&mut self, game_time: GameTime, wait_time: Duration) {
		self.wait_time = wait_time;
		self.target_time = game_time.0 + self.wait_time;
	}

	pub fn set_target(&mut self, target_time: GameTime) {
		self.target_time = target_time.0;
	}
}
