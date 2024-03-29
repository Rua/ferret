//! Ferret, a Doom-compatible game engine.

mod common;
mod doom;

use crate::common::{
	assets::AssetStorage,
	console::{execute_commands, update_console},
	dirs::config_dir,
	input::{InputState, RepeatTracker},
	spawn::{spawn_helper, SpawnMergerHandlerSet},
	time::increment_game_time,
	video::{DrawTarget, PresentTarget, RenderContext},
};
use anyhow::Context;
use clap::{App, Arg};
use crossbeam_channel::Sender;
use legion::{
	serialize::Canon,
	systems::{ResourceSet, Runnable},
	Read, Registry, Resources, Schedule, SystemBuilder, World, Write,
};
use nalgebra::Vector2;
use std::{
	fs::File,
	io::{BufWriter, Write as _},
	time::{Duration, Instant},
};
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

	let (log_sender, log_receiver) = crossbeam_channel::unbounded();
	common::logger::init(&arg_matches, log_sender)?;
	log::info!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");

	// Set up resources
	let mut resources = Resources::default();

	let (command_sender, command_receiver) = common::console::init()?;
	command_sender.send("exec config.cfg".into()).unwrap();
	resources.insert(command_sender.clone());

	let event_loop = EventLoop::new();
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
	resources.insert(InputState::new(
		doom::input::bool_values(),
		doom::input::float_values(),
		command_sender,
	));
	resources.insert(SpawnMergerHandlerSet::new());
	resources.insert(Registry::<String>::default());
	resources.insert(Canon::default());

	doom::init_resources(&mut resources, &arg_matches)?;

	#[rustfmt::skip]
	let mut input_systems = {
		Schedule::builder()
			.add_thread_local(process_events(event_loop)).flush()
			.add_thread_local_fn(execute_commands(
				command_receiver,
				doom::commands::commands(),
			)).flush()
			.build()
	};

	#[rustfmt::skip]
	let mut update_systems = {
		let mut builder = Schedule::builder();
		doom::game::add_update_systems(&mut builder, &mut resources)
			.context("Couldn't initialise update systems")?;
		builder
			.add_thread_local(increment_game_time()).flush()
			.build()
	};

	#[rustfmt::skip]
	let mut output_systems = {
		let mut builder = Schedule::builder();
		builder.add_system(update_console(log_receiver));
		doom::add_output_systems(&mut builder, &mut resources)
			.context("Couldn't initialise output systems")?;
		builder.build()
	};

	// Create world
	let mut world = World::default();

	{
		let mut asset_storage = <Write<AssetStorage>>::fetch_mut(&mut resources);
		let hexfont = asset_storage.load::<doom::assets::font::HexFont>("console.hex");

		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(0.0, 0.0),
				depth: 1.0,
				alignment: [doom::ui::UiAlignment::Near, doom::ui::UiAlignment::Near],
				size: Vector2::new(320.0, 100.0),
				stretch: [true, false],
			},
			doom::ui::UiHexFontText {
				lines: Vec::new(),
				font: hexfont,
			},
			doom::ui::Hidden,
		));
	}

	{
		let hud_handle = <Write<AssetStorage>>::fetch_mut(&mut resources)
			.load::<doom::assets::template::EntityTemplate>("hud.entity");
		let asset_storage = <Read<AssetStorage>>::fetch(&resources);
		let hud_template = asset_storage.get(&hud_handle).unwrap();
		spawn_helper(&hud_template.world, &mut world, &resources);
	}

	doom::assets::process_assets(&mut resources);

	// Run the input once to execute pending commands
	input_systems.execute(&mut world, &mut resources);

	{
		// Set default bindings if there aren't any yet
		let mut input_state = <Write<InputState>>::fetch_mut(&mut resources);
		if input_state.bindings.is_empty() {
			input_state.bindings = doom::data::default_bindings();
		}
	}

	let mut old_time = Instant::now();
	let mut leftover_time = Duration::ZERO;

	while !resources.contains::<ShouldQuit>() {
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

		input_systems.execute(&mut world, &mut resources);

		if resources.contains::<ShouldQuit>() {
			break;
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
		output_systems.execute(&mut world, &mut resources);
	}

	// Write configuration
	{
		let mut path = config_dir();
		path.push("config.cfg");
		let result = File::create(&path)
			.with_context(|| format!("Couldn't open \"{}\" for writing", path.display()))
			.and_then(|file| {
				let mut file = BufWriter::new(file);
				writeln!(file, "// Auto-generated by Ferret")?;
				let input_state = <Read<InputState>>::fetch(&resources);
				input_state
					.bindings
					.write(&mut file)
					.with_context(|| format!("Couldn't write \"{}\"", path.display()))
			});

		if let Err(e) = result {
			log::error!("{:?}", e);
		}
	}

	Ok(())
}

#[derive(Clone, Copy, Debug, Default)]
struct ShouldQuit;

fn process_events(mut event_loop: EventLoop<()>) -> impl Runnable {
	let mut repeat_tracker = RepeatTracker::new();

	SystemBuilder::new("process_events")
		.read_resource::<Sender<String>>()
		.read_resource::<RenderContext>()
		.write_resource::<InputState>()
		.write_resource::<PresentTarget>()
		.build(move |_command_buffer, _world, resources, _queries| {
			event_loop.run_return(|event, _, control_flow| {
				if repeat_tracker.is_repeat(&event) {
					return;
				}

				let (command_sender, render_context, input_state, present_target) = resources;

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
			})
		})
}
