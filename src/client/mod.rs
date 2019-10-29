mod audio;
mod client_configvars;
mod input;
mod video;
mod vulkan;

use crate::{
	client::{audio::Audio, input::Input, video::Video},
	commands::CommandSender,
};
use specs::World;
use std::{
	error::Error,
	time::{Duration, Instant},
};
use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	platform::desktop::EventLoopExtDesktop,
};

pub struct Client {
	audio: Audio,
	command_sender: CommandSender,
	event_loop: EventLoop<()>,
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

		let event_loop = EventLoop::new();
		let video = match Video::init(&event_loop) {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!(
					"Could not initialise video system: {}",
					err
				)));
			}
		};

		let audio = match Audio::init() {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!(
					"Could not initialise audio system: {}",
					err
				)));
			}
		};

		let input = Input::init();

		Ok(Client {
			audio,
			command_sender,
			event_loop,
			input,
			video,

			real_time: Instant::now(),
			should_quit: false,
		})
	}

	pub fn frame(&mut self, delta: Duration, world: &mut World) {
		self.real_time += delta;
		let mut should_quit = false;

		self.event_loop
			.run_return(|event, _, control_flow| match event {
				Event::WindowEvent {
					event,
					window_id: _,
				} => match event {
					WindowEvent::CloseRequested => {
						should_quit = true;
						*control_flow = ControlFlow::Exit;
					}
					_ => {}
				},
				Event::EventsCleared => {
					*control_flow = ControlFlow::Exit;
				}
				_ => {}
			});

		if should_quit {
			self.command_sender.send("quit");
			return;
		}

		self.send_update();
		self.video.draw_frame().unwrap();
	}

	pub fn quit(&mut self) {
		self.should_quit = true;
	}

	fn send_update(&mut self) {}
}
