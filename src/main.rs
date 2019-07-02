#![allow(unused)]
#![warn(unused_must_use)]

#[macro_use]
extern crate downcast_rs;
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
mod net;
mod protocol;
mod palette;
mod server;
mod sprite;
mod stdin;

use crate::logger::Logger;

fn main() {
	Logger::init().unwrap();
	client::client_main();
}
