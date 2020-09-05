use std::time::Duration;

#[derive(Clone, Copy, Debug, Default)]
pub struct Timer {
	time: Duration,
	time_left: Duration,
}

impl Timer {
	pub fn new(time: Duration) -> Timer {
		Timer {
			time,
			time_left: time,
		}
	}

	pub fn new_zero(time: Duration) -> Timer {
		Timer {
			time,
			time_left: Duration::default(),
		}
	}

	pub fn tick(&mut self, delta: Duration) {
		if let Some(new_time) = self.time_left.checked_sub(delta) {
			self.time_left = new_time;
		} else {
			self.time_left = Duration::default();
		}
	}

	pub fn is_zero(&self) -> bool {
		self.time_left == Duration::default()
	}

	pub fn reset(&mut self) {
		self.time_left = self.time;
	}

	pub fn set(&mut self, time: Duration) {
		self.time = time;
		self.time_left = time;
	}

	pub fn set_zero(&mut self) {
		self.time_left = Duration::default();
	}
}
