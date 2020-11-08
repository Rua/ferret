use std::time::Duration;

/// A timer that elapses at the specified time
#[derive(Clone, Copy, Debug)]
pub struct Timer {
	target_time: Duration,
	wait_time: Duration,
}

impl Timer {
	/// Creates a new timer with the specified wait time, and sets it to elapse this duration
	/// after the current time.
	pub fn new(current_time: Duration, wait_time: Duration) -> Self {
		Self {
			target_time: current_time + wait_time,
			wait_time,
		}
	}

	/// Creates a new timer with the specified wait time, and sets it to elapse immediately.
	pub fn new_elapsed(current_time: Duration, wait_time: Duration) -> Self {
		Self {
			target_time: current_time,
			wait_time,
		}
	}

	pub fn is_elapsed(&self, current_time: Duration) -> bool {
		current_time >= self.target_time
	}

	pub fn restart(&mut self, current_time: Duration) {
		self.target_time = current_time + self.wait_time;
	}

	pub fn restart_with(&mut self, current_time: Duration, wait_time: Duration) {
		self.wait_time = wait_time;
		self.target_time = current_time + self.wait_time;
	}

	pub fn set_target(&mut self, target_time: Duration) {
		self.target_time = target_time;
	}
}
