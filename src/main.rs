#![feature(uniform_paths)]
#![feature(try_from)]

#![allow(unused)]
#![warn(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate vulkano;

mod client;
mod commands;
mod doom;
//mod doomtypes;
mod geometry;
mod logger;
mod model;
mod net;
mod protocol;
//mod palette;
//mod quaketypes;
mod server;
mod sprite;

use std::io;
use std::io::BufRead;
use std::panic;
use std::panic::AssertUnwindSafe;
use std::thread::Builder;

use crate::commands::{CommandSender, CommandUnion};
use crate::logger::Logger;

fn main() {
	Logger::init().unwrap();
	
	if true {
		let mut command_union = CommandUnion::new();
		command_union.add_commands(client::COMMANDS.keys());
		command_union.add_commands(server::COMMANDS.keys());
		let client_dispatcher = command_union.make_dispatcher(0);
		let server_dispatcher = command_union.make_dispatcher(1);
		
		match spawn_stdin(command_union.make_sender()) {
			Ok(_) => (),
			Err(err) => {
				error!("Could not start stdin thread: {}", err);
				return
			}
		};
		
		let server_thread = Builder::new()
			.name("server".to_owned())
			.spawn(move || {
				match panic::catch_unwind(AssertUnwindSafe(|| {
					server::server_main(server_dispatcher)
				})) {
					Ok(()) => debug!("Server thread terminated."),
					Err(_) => (),
				}
			});
		
		let server_thread = match server_thread {
			Ok(val) => val,
			Err(err) => {
				error!("Could not start server thread: {}", err);
				return
			}
		};
		
		client::client_main(client_dispatcher);
		debug!("Client thread terminated.");
		
		server_thread.join().ok();
	} else {
		let mut command_union = CommandUnion::new();
		command_union.add_commands(server::COMMANDS.keys());
		let dispatcher = command_union.make_dispatcher(0);
		
		match spawn_stdin(command_union.make_sender()) {
			Ok(_) => (),
			Err(err) => {
				error!("Could not start stdin thread: {}", err);
				return
			}
		};
		
		server::server_main(dispatcher);
	}
}

// Spawns a thread to read commands from stdin asynchronously
fn spawn_stdin(stdin_sender: CommandSender) -> Result<(), io::Error> {
	Builder::new()
		.name("stdin".to_owned())
		.spawn(move|| {
			let stdin = std::io::stdin();
			let stdin_buf = stdin.lock();
			
			for line_result in stdin_buf.lines() {
				match line_result {
					Ok(line) => {
						stdin_sender.push(&line);
					},
					Err(e) => {
						error!("Error: {}", e);
						break;
					},
				};
			}
		})?;
	
	Ok(())
}
