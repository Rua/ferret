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
	assets::{AssetFormat, AssetStorage},
	audio::Audio,
	input::{Axis, Bindings, Button, InputState, MouseAxis},
	logger::Logger,
	renderer::{texture::Texture, video::Video},
};
use specs::{world::Builder, ReadExpect, RunNow, SystemData, World, WorldExt, WriteExpect};
use std::{
	error::Error,
	sync::mpsc,
	time::{Duration, Instant},
};
use winit::{
	ElementState, Event, EventsLoop, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};

const FRAME_TIME: Duration = Duration::from_nanos(28571429); // 1/35 sec

fn main() -> Result<(), Box<dyn Error>> {
	Logger::init().unwrap();

	let (command_sender, command_receiver) = mpsc::channel();

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

	let mut event_loop = EventsLoop::new();

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
			scale: 3.0,
		},
	);
	bindings.bind_axis(
		doom::input::Axis::Pitch,
		Axis::Mouse {
			axis: MouseAxis::Y,
			scale: 3.0,
		},
	);
	//println!("{}", serde_json::to_string(&bindings)?);

	let mut world = World::new();
	world.register::<doom::components::MapComponent>();
	world.register::<doom::components::SpawnPointComponent>();
	world.register::<doom::components::TransformComponent>();
	world.insert(video);
	world.insert(audio);
	world.insert(loader);
	world.insert(InputState::new());
	world.insert(bindings);
	world.insert(AssetStorage::<Texture>::new());
	world.insert(FRAME_TIME);

	let mut render_system = doom::render::RenderSystem::new(&world)?;

	command_sender.send("map E1M1".to_owned()).ok();

	let mut should_quit = false;
	let mut old_time = Instant::now();
	let mut leftover_time = Duration::default();

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
		event_loop.poll_events(|event| {
			let (mut input_state, video) =
				<(WriteExpect<InputState>, ReadExpect<Video>)>::fetch(&world);
			input_state.process_event(&event);

			match event {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::CloseRequested => {
						command_sender.send("quit".to_owned()).ok();
					}
					WindowEvent::MouseInput {
						state: ElementState::Pressed,
						..
					} => {
						let window = video.surface().window();
						window.grab_cursor(true).ok();
						window.hide_cursor(true);
						input_state.set_mouse_delta_enabled(true);
					}
					WindowEvent::KeyboardInput {
						input:
							KeyboardInput {
								state: ElementState::Pressed,
								virtual_keycode: Some(VirtualKeyCode::Escape),
								..
							},
						..
					} => {
						let window = video.surface().window();
						window.grab_cursor(false).ok();
						window.hide_cursor(false);
						input_state.set_mouse_delta_enabled(false);
					}
					WindowEvent::Focused(false) => {
						let window = video.surface().window();
						window.grab_cursor(false).ok();
						window.hide_cursor(false);
						input_state.set_mouse_delta_enabled(false);
					}
					_ => {}
				},
				_ => {}
			}
		});

		// Execute console commands
		while let Some(command) = command_receiver.try_iter().next() {
			// Split into tokens
			let tokens = match commands::tokenize(&command) {
				Ok(tokens) => tokens,
				Err(e) => {
					error!("Invalid syntax: {}", e);
					continue;
				}
			};

			// Split further into subcommands
			for args in tokens.split(|tok| tok == ";") {
				match args[0].as_str() {
					"map" => {
						let name = &args[1];
						info!("Loading map {}...", name);

						let map_data = {
							let mut loader = world.fetch_mut::<doom::wad::WadLoader>();
							doom::map::DoomMapFormat.import(name, &mut *loader)?
						};
						let map_model = doom::map::make_model(&map_data, &world)?;
						world
							.create_entity()
							.with(doom::components::MapComponent { map_model })
							.build();

						let things = {
							let mut loader = world.fetch_mut::<doom::wad::WadLoader>();
							doom::map::ThingsFormat.import(name, &mut *loader)?
						};
						doom::map::spawn_map_entities(things, &mut world, &map_data)?;
						let entity = doom::map::spawn_player(&mut world)?;
						world.insert(entity);
					}
					"quit" => should_quit = true,
					_ => error!("Unknown command: {}", args[0]),
				}
			}
		}

		if should_quit {
			return Ok(());
		}

		// Run game frames
		leftover_time += delta;

		if leftover_time >= FRAME_TIME {
			leftover_time -= FRAME_TIME;

			doom::update::UpdateSystem.run_now(&world);

			// Reset input delta state
			{
				let mut input_state = world.fetch_mut::<InputState>();
				input_state.reset();
			}
		}

		// Draw frame
		render_system.run_now(&world);
	}

	Ok(())
}
