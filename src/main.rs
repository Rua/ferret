//#![allow(unused)]
//#![warn(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate vulkano;

mod assets;
mod audio;
mod commands;
mod components;
mod configvars;
mod doom;
mod geometry;
mod logger;
mod renderer;
//mod net;
//mod protocol;
mod stdin;

use crate::{
	audio::Audio,
	commands::CommandSender,
	components::TransformComponent,
	logger::Logger,
	renderer::video::Video,
};
use specs::{World, WorldExt};
use std::{
	error::Error,
	sync::mpsc::{self, Receiver},
	time::{Duration, Instant},
};
use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	platform::desktop::EventLoopExtDesktop,
};

fn main() -> Result<(), Box<dyn Error>> {
	Logger::init().unwrap();
	let mut main_loop = MainLoop::new()?;
	main_loop.start()?;

	Ok(())
}

struct MainLoop {
	audio: Audio,
	command_receiver: Receiver<Vec<String>>,
	command_sender: CommandSender,
	event_loop: EventLoop<()>,
	old_time: Instant,
	should_quit: bool,
	video: Video,
	world: World,
}

impl MainLoop {
	fn new() -> Result<MainLoop, Box<dyn Error>> {
		let (command_sender, command_receiver) = mpsc::channel();
		let command_sender = CommandSender::new(command_sender);

		match stdin::spawn(command_sender.clone()) {
			Ok(_) => (),
			Err(err) => {
				return Err(Box::from(format!("Could not start stdin thread: {}", err)));
			}
		};

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

		let mut world = World::new();
		world.register::<TransformComponent>();


		let mut loader = doom::wad::WadLoader::new();
		loader.add("doom.wad")?;
		loader.add("doom.gwa")?;
		world.insert(loader);

		Ok(MainLoop {
			audio,
			command_receiver,
			command_sender,
			event_loop,
			old_time: Instant::now(),
			should_quit: false,
			video,
			world,
		})
	}

	fn start(&mut self) -> Result<(), Box<dyn Error>> {
		self.old_time = Instant::now();

		while !self.should_quit {
			let mut delta;
			let mut new_time;

			// Busy-loop until there is at least a millisecond of delta
			while {
				new_time = Instant::now();
				delta = new_time - self.old_time;
				delta.as_millis() < 1
			} {}

			self.frame(delta)?;
			self.old_time = new_time;
		}

		Ok(())
	}

	fn frame(&mut self, delta: Duration) -> Result<(), Box<dyn Error>> {
		let sender2 = self.command_sender.clone();
		self.event_loop	.run_return(
			|event, _, control_flow| match event {
				Event::WindowEvent {
					event,
					window_id: _,
				} => match event {
					WindowEvent::CloseRequested => {
						sender2.send("quit");
						*control_flow = ControlFlow::Exit;
					}
					_ => {}
				},
				Event::EventsCleared => {
					*control_flow = ControlFlow::Exit;
				}
				_ => {}
			}
		);

		// Execute console commands
		while let Some(args) = self.command_receiver.try_iter().next() {
			match args[0].as_str() {
				"map" => doom::map::spawn_map_entities(&mut self.world, "E1M1")?,
				"quit" => self.should_quit = true,
				_ => debug!("Received invalid command: {}", args[0]),
			}
		}

		if self.should_quit {
			return Ok(());
		}

		self.video.draw_frame().unwrap();

		Ok(())
	}
}
