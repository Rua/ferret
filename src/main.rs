//! Ferret, a Doom-compatible game engine.

mod common;
mod doom;

use crate::common::{
	assets::AssetStorage,
	input::InputState,
	spawn::SpawnMergerHandlerSet,
	video::{DrawTarget, PresentTarget, RenderContext},
};
use anyhow::Context;
use clap::{App, Arg};
use crossbeam_channel::Sender;
use legion::{serialize::Canon, systems::ResourceSet, Read, Registry, Resources, World, Write};
use nalgebra::Vector2;
use std::time::{Duration, Instant};
use winit::{
	event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	platform::run_return::EventLoopExtRunReturn,
};

fn main() -> anyhow::Result<()> {
	let arg_matches = App::new(clap::crate_name!())
		.about(clap::crate_description!())
		.version(clap::crate_version!())
		.arg(
			Arg::with_name("PWADS")
				.help("PWAD files to add")
				.multiple(true),
		)
		.arg(
			Arg::with_name("iwad")
				.help("IWAD file to use instead of the default")
				.short("i")
				.long("iwad")
				.value_name("FILE"),
		)
		.arg(
			Arg::with_name("map")
				.help("Map to load at startup")
				.short("m")
				.long("map")
				.value_name("NAME"),
		)
		.arg(
			Arg::with_name("log-level")
				.help("Highest log level to display")
				.long("log-level")
				.value_name("LEVEL")
				.possible_values(&["ERROR", "WARN", "INFO", "DEBUG", "TRACE"]),
		)
		.get_matches();

	common::logger::init(&arg_matches)?;

	// Set up resources
	let mut resources = Resources::default();

	let (command_sender, command_receiver) = common::commands::init()?;
	resources.insert(command_sender);

	let mut event_loop = EventLoop::new();
	let (render_context, _debug_callback) =
		RenderContext::new(&event_loop).context("Could not create RenderContext")?;
	let present_target = PresentTarget::new(
		render_context.surface().clone(),
		render_context.device().clone(),
	)
	.context("Couldn't create PresentTarget")?;
	let draw_target = DrawTarget::new(&render_context, present_target.dimensions())
		.context("Couldn't create DrawTarget")?;

	resources.insert(draw_target);
	resources.insert(present_target);
	resources.insert(render_context);

	resources.insert(common::sound::init()?);
	resources.insert(InputState::new());
	resources.insert(SpawnMergerHandlerSet::new());
	resources.insert(Registry::<String>::default());
	resources.insert(Canon::default());

	doom::init_resources(&mut resources, &arg_matches)?;

	let mut update_systems =
		doom::init_update_systems(&mut resources).context("Couldn't initialise update systems")?;
	let mut draw_systems =
		doom::init_draw_systems(&mut resources).context("Couldn't initialise draw systems")?;
	let mut sound_systems =
		doom::init_sound_systems(&mut resources).context("Couldn't initialise sound systems")?;

	// Create world
	let mut world = World::default();

	{
		let mut asset_storage = <Write<AssetStorage>>::fetch_mut(&mut resources);

		world.extend(vec![
			(
				doom::ui::UiTransform {
					position: Vector2::new(0.0, 168.0),
					depth: 1.0,
					alignment: [doom::ui::UiAlignment::Near, doom::ui::UiAlignment::Far],
					size: Vector2::new(320.0, 32.0),
					stretch: [true, false],
				},
				doom::ui::UiImage {
					image: asset_storage.load("floor7_2.flat"),
				},
			),
			(
				doom::ui::UiTransform {
					position: Vector2::new(0.0, 168.0),
					depth: 2.0,
					alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
					size: Vector2::new(320.0, 32.0),
					stretch: [false; 2],
				},
				doom::ui::UiImage {
					image: asset_storage.load("stbar.patch"),
				},
			),
			(
				doom::ui::UiTransform {
					position: Vector2::new(104.0, 168.0),
					depth: 3.0,
					alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
					size: Vector2::new(40.0, 32.0),
					stretch: [false; 2],
				},
				doom::ui::UiImage {
					image: asset_storage.load("starms.patch"),
				},
			),
			(
				doom::ui::UiTransform {
					position: Vector2::new(143.0, 168.0),
					depth: 10.0,
					alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
					size: Vector2::new(24.0, 29.0),
					stretch: [false; 2],
				},
				doom::ui::UiImage {
					image: asset_storage.load("stfst00.patch"),
				},
			),
		]);
	}

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
		event_loop.run_return(|event, _, control_flow| {
			let (command_sender, render_context, mut input_state, mut present_target) =
				<(
					Read<Sender<String>>,
					Read<RenderContext>,
					Write<InputState>,
					Write<PresentTarget>,
				)>::fetch_mut(&mut resources);

			input_state.process_event(&event);

			match event {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::CloseRequested => {
						command_sender.send("quit".to_owned()).ok();
						*control_flow = ControlFlow::Exit;
					}
					WindowEvent::Resized(new_size) => {
						present_target.window_resized(new_size.into());
					}
					WindowEvent::MouseInput {
						state: ElementState::Pressed,
						..
					} => {
						let window = render_context.surface().window();
						if let Err(err) = window.set_cursor_grab(true) {
							log::warn!("Couldn't grab cursor: {}", err);
						}
						window.set_cursor_visible(false);
						input_state.set_mouse_delta_enabled(true);
					}
					WindowEvent::Focused(false)
					| WindowEvent::KeyboardInput {
						input:
							KeyboardInput {
								state: ElementState::Pressed,
								virtual_keycode: Some(VirtualKeyCode::Escape),
								..
							},
						..
					} => {
						let window = render_context.surface().window();
						if let Err(err) = window.set_cursor_grab(false) {
							log::warn!("Couldn't release cursor: {}", err);
						}
						window.set_cursor_visible(true);
						input_state.set_mouse_delta_enabled(false);
					}
					_ => {}
				},
				Event::RedrawEventsCleared => {
					*control_flow = ControlFlow::Exit;
				}
				_ => {}
			}
		});

		// Execute console commands
		while let Some(command) = command_receiver.try_iter().next() {
			// Split into tokens
			let tokens = match common::commands::tokenize(&command) {
				Ok(tokens) => tokens,
				Err(e) => {
					log::error!("Invalid syntax: {}", e);
					continue;
				}
			};

			// Split further into subcommands
			for args in tokens.split(|tok| tok == ";") {
				match args[0].as_str() {
					"load" => doom::load_game(&args[1], &mut world, &mut resources),
					"map" => doom::load_map(&args[1], &mut world, &mut resources)?,
					"quit" => should_quit = true,
					"save" => doom::save_game(&args[1], &mut world, &mut resources),
					_ => log::error!("Unknown command: {}", args[0]),
				}
			}
		}

		if should_quit {
			return Ok(());
		}

		// Run game frames
		leftover_time += delta;

		if leftover_time >= doom::data::FRAME_TIME {
			update_systems.execute(&mut world, &mut resources);
			leftover_time -= doom::data::FRAME_TIME;

			let mut input_state = <Write<InputState>>::fetch_mut(&mut resources);
			input_state.reset();
		}

		// Update video and sound
		draw_systems.execute(&mut world, &mut resources);
		sound_systems.execute(&mut world, &mut resources);
	}

	Ok(())
}
