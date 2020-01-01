use std::error::Error;

pub struct Audio {}

impl Audio {
	pub fn new() -> Result<Audio, Box<dyn Error>> {
		Ok(Audio {})
	}
}

impl Drop for Audio {
	fn drop(&mut self) {}
}
