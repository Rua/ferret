use alto::{Alto, Context};
use sdl2::mixer;
use std::error::Error;

pub struct Audio {
	_al_context: Context,
}

impl Audio {
	pub fn init() -> Result<Audio, Box<dyn Error>> {
		let alto_context = Alto::load_default()?;

		// Open OpenAL
		let al_device = alto_context.open(None)?;
		let al_context = al_device.new_context(None)?;

		al_context.set_position([0.0, 0.0, 0.0])?;
		al_context.set_velocity([0.0, 0.0, 0.0])?;
		al_context.set_orientation(([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]))?;

		// Open SDL_Mixer
		mixer::open_audio(44100, mixer::AUDIO_S16SYS, 2, 1024)?;

		Ok(Audio {
			_al_context: al_context,
		})
	}
}

impl Drop for Audio {
	fn drop(&mut self) {
		mixer::close_audio();
	}
}
