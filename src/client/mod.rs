mod audio;
mod client_configvars;
mod input;
mod video;
mod vulkan;

use sdl2::{
	self,
	EventPump,
	event::Event,
};
use std::{
	error::Error,
	time::{Duration, Instant},
};
use crate::{
	client::{
		audio::Audio,
		input::Input,
		video::Video,
	},
	commands::CommandSender,
};

pub struct Client {
	audio: Audio,
	command_sender: CommandSender,
	event_pump: EventPump,
	input: Input,
	video: Video,

	real_time: Instant,
	should_quit: bool,
}

impl Client {
	pub fn new(command_sender: CommandSender) -> Result<Client, Box<dyn Error>> {
		let sdl = match sdl2::init() {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!("Could not initialise SDL: {}", err)));
			}
		};

		let video = match Video::init(&sdl) {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!("Could not initialise video system: {}", err)));
			}
		};

		let audio = match Audio::init() {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!("Could not initialise audio system: {}", err)));
			}
		};

		let input = Input::init();

		let event_pump = match sdl.event_pump() {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!("Could not start event loop: {}", err)));
			}
		};

		Ok(Client {
			audio,
			command_sender,
			event_pump,
			input,
			video,

			real_time: Instant::now(),
			should_quit: false,
		})
	}

	pub fn frame(&mut self, delta: Duration) {
		self.real_time += delta;

		for event in self.event_pump.poll_iter() {
			match event {
				Event::Quit {..} => self.command_sender.send("quit"),
				_ => {},
			}
		}

		self.send_update();
		self.video.draw_frame().unwrap();
	}

	pub fn quit(&mut self) {
		self.should_quit = true;
	}

	fn send_update(&mut self) {
	}
}
