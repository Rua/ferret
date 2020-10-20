use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct Timer {
	target_time: Duration,
	wait_time: Duration,
}

impl Timer {
	pub fn new(current_time: Duration, wait_time: Duration) -> Self {
		Self {
			target_time: current_time + wait_time,
			wait_time,
		}
	}

	pub fn is_elapsed(&self, current_time: Duration) -> bool {
		current_time >= self.target_time
	}

	pub fn restart(&mut self) {
		self.target_time += self.wait_time;
	}

	pub fn restart_with(&mut self, wait_time: Duration) {
		self.wait_time = wait_time;
		self.target_time += self.wait_time;
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct OldTimer {
	time: Duration,
	time_left: Duration,
}

impl OldTimer {
	pub fn new(time: Duration) -> OldTimer {
		OldTimer {
			time,
			time_left: time,
		}
	}

	pub fn new_zero(time: Duration) -> OldTimer {
		OldTimer {
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

	pub fn set_zero(&mut self) {
		self.time_left = Duration::default();
	}
}
