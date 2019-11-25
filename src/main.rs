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
mod configvars;
mod doom;
mod geometry;
mod input;
mod logger;
mod renderer;
//mod net;
//mod protocol;
mod stdin;

use crate::{
	assets::AssetStorage,
	audio::Audio,
	commands::CommandSender,
	input::{Axis, Bindings, Button, InputState, MouseAxis},
	logger::Logger,
	renderer::{texture::Texture, video::Video},
};
use specs::{world::Builder, RunNow, World, WorldExt};
use std::{error::Error, sync::mpsc, time::Instant};
use winit::{
	event::{Event, MouseButton, VirtualKeyCode, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	platform::desktop::EventLoopExtDesktop,
};

fn main() -> Result<(), Box<dyn Error>> {
	Logger::init().unwrap();

	let (command_sender, command_receiver) = mpsc::channel();
	let command_sender = CommandSender::new(command_sender);

	match stdin::spawn(command_sender.clone()) {
		Ok(_) => (),
		Err(err) => {
			return Err(Box::from(format!("Could not start stdin thread: {}", err)));
		}
	};

	let _sdl = match sdl2::init() {
		Ok(val) => val,
		Err(err) => {
			return Err(Box::from(format!("Could not initialise SDL: {}", err)));
		}
	};

	let mut event_loop = EventLoop::new();

	let (video, _debug_callback) = match Video::new(&event_loop) {
		Ok(val) => val,
		Err(err) => {
			return Err(Box::from(format!(
				"Could not initialise video system: {}",
				err
			)));
		}
	};

	let audio = match Audio::new() {
		Ok(val) => val,
		Err(err) => {
			return Err(Box::from(format!(
				"Could not initialise audio system: {}",
				err
			)));
		}
	};

	let mut loader = doom::wad::WadLoader::new();
	loader.add("doom.wad")?;
	loader.add("doom.gwa")?;

	let mut bindings = Bindings::new();
	bindings.bind_action(
		doom::input::Action::Attack,
		Button::Mouse(MouseButton::Left),
	);
	bindings.bind_action(doom::input::Action::Use, Button::Key(VirtualKeyCode::Space));
	bindings.bind_action(doom::input::Action::Use, Button::Mouse(MouseButton::Middle));
	bindings.bind_axis(
		doom::input::Axis::Forward,
		Axis::Emulated {
			pos: Button::Key(VirtualKeyCode::W),
			neg: Button::Key(VirtualKeyCode::S),
		},
	);
	bindings.bind_axis(
		doom::input::Axis::Strafe,
		Axis::Emulated {
			pos: Button::Key(VirtualKeyCode::A),
			neg: Button::Key(VirtualKeyCode::D),
		},
	);
	bindings.bind_axis(
		doom::input::Axis::Yaw,
		Axis::Mouse {
			axis: MouseAxis::X,
			scale: 1.0,
		},
	);
	bindings.bind_axis(
		doom::input::Axis::Pitch,
		Axis::Mouse {
			axis: MouseAxis::Y,
			scale: 1.0,
		},
	);
	//println!("{}", serde_json::to_string(&bindings)?);

	let mut world = World::new();
	world.register::<doom::components::MapComponent>();
	world.register::<doom::components::TransformComponent>();
	world.insert(video);
	world.insert(audio);
	world.insert(loader);
	world.insert(InputState::new());
	world.insert(None as Option<doom::input::UserCommand>);
	world.insert(bindings);
	world.insert(AssetStorage::<Texture>::new());

	let mut render_system = doom::render::RenderSystem::new(&world)?;

	command_sender.send("map E1M1");

	let mut should_quit = false;
	let mut old_time = Instant::now();

	while !should_quit {
		let mut delta;
		let mut new_time;

		// Busy-loop until there is at least a millisecond of delta
		while {
			new_time = Instant::now();
			delta = new_time - old_time;
			delta.as_millis() < 1
		} {}

		old_time = new_time;
		//println!("{} fps", 1.0/delta.as_secs_f32());

		// Process events from the system
		{
			let mut input_state = world.fetch_mut::<InputState>();
			input_state.reset();
		}

		event_loop.run_return(|event, _, control_flow| {
			let mut input_state = world.fetch_mut::<InputState>();
			input_state.process_event(&event);

			match event {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::CloseRequested => {
						command_sender.send("quit");
						*control_flow = ControlFlow::Exit;
					}
					_ => {}
				},
				Event::EventsCleared => {
					*control_flow = ControlFlow::Exit;
				}
				_ => {}
			}
		});

		// Execute console commands
		while let Some(args) = command_receiver.try_iter().next() {
			match args[0].as_str() {
				"map" => {
					let name = &args[1];
					info!("Loading map {}...", name);
					let map = doom::map::from_wad(name, &world)?;
					world
						.create_entity()
						.with(doom::components::MapComponent { map })
						.build();

					doom::map::spawn_map_entities(&mut world, name)?;
				}
				"quit" => should_quit = true,
				_ => debug!("Received invalid command: {}", args[0]),
			}
		}

		if should_quit {
			return Ok(());
		}

		// Send user commands
		doom::input::UserCommandSenderSystem.run_now(&world);

		// Draw frame
		render_system.run_now(&world);
	}

	Ok(())
}
