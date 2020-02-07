use std::error::Error;

pub struct Audio {}

impl Audio {
	pub fn new() -> Result<Audio, Box<dyn Error + Send + Sync>> {
		Ok(Audio {})
	}
}

impl Drop for Audio {
	fn drop(&mut self) {}
}

pub struct Sound {
	pub sampling_rate: u16,
	pub samples: Vec<u8>,
}
