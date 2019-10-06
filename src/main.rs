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

mod client;
mod commands;
mod components;
mod configvars;
mod doom;
mod geometry;
mod logger;
mod model;
//mod net;
//mod protocol;
mod palette;
mod server;
mod sprite;
mod stdin;

use std::{
	error::Error,
	sync::mpsc::{self, Receiver},
	time::{Duration, Instant},
};
use crate::{
	client::Client,
	commands::CommandSender,
	logger::Logger,
	server::Server,
};

fn main() -> Result<(), Box<dyn Error>> {
	Logger::init().unwrap();
	let mut main_loop = MainLoop::new()?;
	main_loop.start();

	Ok(())
}

struct MainLoop {
	client: Client,
	command_receiver: Receiver<Vec<String>>,
	old_time: Instant,
	server: Server,
	should_quit: bool,
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

		let mut client = Client::new(command_sender.clone())?;
		let mut server = Server::new()?;

		Ok(MainLoop{
			client,
			command_receiver,
			old_time: Instant::now(),
			server,
			should_quit: false,
		})
	}

	fn start(&mut self) {
		self.old_time = Instant::now();
		let mut new_time = Instant::now();
		let mut delta = new_time - self.old_time;

		while !self.should_quit {
			// Busy-loop until there is at least a millisecond of delta
			while {
				new_time = Instant::now();
				delta = new_time - self.old_time;
				delta.as_millis() < 1
			} {}

			self.frame(delta);
			self.old_time = new_time;
		}
	}

	fn frame(&mut self, delta: Duration) {
		// Execute console commands
		while let Some(args) = self.command_receiver.try_iter().next() {
			match args[0].as_str() {
				"quit" => self.should_quit = true,
				_ => debug!("Received invalid command: {}", args[0]),
			}
		}

		if self.should_quit {
			return;
		}

		self.client.frame(delta);
		self.server.frame(delta);
	}
}
