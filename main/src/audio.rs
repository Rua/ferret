use alto::{Alto, Context};
use std::error::Error;

pub struct Audio {
	_al_context: Context,
}

impl Audio {
	pub fn new() -> Result<Audio, Box<dyn Error>> {
		let alto_context = Alto::load_default()?;

		// Open OpenAL
		let al_device = alto_context.open(None)?;
		let al_context = al_device.new_context(None)?;

		al_context.set_position([0.0, 0.0, 0.0])?;
		al_context.set_velocity([0.0, 0.0, 0.0])?;
		al_context.set_orientation(([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]))?;

		Ok(Audio {
			_al_context: al_context,
		})
	}
}

impl Drop for Audio {
	fn drop(&mut self) {}
}
