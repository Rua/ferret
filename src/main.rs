//! Ferret, a Doom-compatible game engine.

mod common;
mod doom;

use crate::common::{
	assets::AssetStorage,
	commands::execute_commands,
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

	let commands = doom::commands::commands();
	let mut execute_commands = execute_commands(command_receiver, commands);

	// Create world
	let mut world = World::default();

	{
		let mut asset_storage = <Write<AssetStorage>>::fetch_mut(&mut resources);

		world.push((
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
		));
		world.push((
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
		));

		// Ammo
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(2.0, 171.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(42.0, 20.0),
				stretch: [false; 2],
			},
			doom::ui::UiText {
				text: " 50".into(),
				font: asset_storage.load("sttnum.font"),
			},
			doom::hud::AmmoStat {
				ammo_type: None,
				show_max: false,
			},
		));

		// Health
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(48.0, 171.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(56.0, 20.0),
				stretch: [false; 2],
			},
			doom::ui::UiText {
				text: String::with_capacity(4),
				font: asset_storage.load("sttnum.font"),
			},
			doom::hud::HealthStat,
		));

		// Arms
		world.push((
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
		));

		// Weapon 2
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(111.0, 172.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(4.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiImage {
				image: asset_storage.load("stysnum0.patch"),
			},
			doom::hud::ArmsStat {
				weapons: vec!["pistol".into()],
				images: [
					asset_storage.load("stgnum2.patch"),
					asset_storage.load("stysnum2.patch"),
				],
			},
		));

		// Weapon 3
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(123.0, 172.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(4.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiImage {
				image: asset_storage.load("stysnum0.patch"),
			},
			doom::hud::ArmsStat {
				weapons: vec!["shotgun".into(), "supershotgun".into()],
				images: [
					asset_storage.load("stgnum3.patch"),
					asset_storage.load("stysnum3.patch"),
				],
			},
		));

		// Weapon 4
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(135.0, 172.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(4.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiImage {
				image: asset_storage.load("stysnum0.patch"),
			},
			doom::hud::ArmsStat {
				weapons: vec!["chaingun".into()],
				images: [
					asset_storage.load("stgnum4.patch"),
					asset_storage.load("stysnum4.patch"),
				],
			},
		));

		// Weapon 5
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(111.0, 182.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(4.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiImage {
				image: asset_storage.load("stysnum0.patch"),
			},
			doom::hud::ArmsStat {
				weapons: vec!["missile".into()],
				images: [
					asset_storage.load("stgnum5.patch"),
					asset_storage.load("stysnum5.patch"),
				],
			},
		));

		// Weapon 6
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(123.0, 182.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(4.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiImage {
				image: asset_storage.load("stysnum0.patch"),
			},
			doom::hud::ArmsStat {
				weapons: vec!["plasma".into()],
				images: [
					asset_storage.load("stgnum6.patch"),
					asset_storage.load("stysnum6.patch"),
				],
			},
		));

		// Weapon 7
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(135.0, 182.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(4.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiImage {
				image: asset_storage.load("stgnum7.patch"),
			},
			doom::hud::ArmsStat {
				weapons: vec!["bfg".into()],
				images: [
					asset_storage.load("stgnum7.patch"),
					asset_storage.load("stysnum7.patch"),
				],
			},
		));

		// Face
		world.push((
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
		));

		// Armor
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(179.0, 171.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(56.0, 20.0),
				stretch: [false; 2],
			},
			doom::ui::UiText {
				text: "  0%".into(),
				font: asset_storage.load("sttnum.font"),
			},
		));

		// Blue key
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(239.0, 171.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(7.0, 5.0),
				stretch: [false; 2],
			},
			doom::ui::UiImage {
				image: asset_storage.load("stkeys0.patch"),
			},
		));

		// Yellow key
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(239.0, 181.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(7.0, 5.0),
				stretch: [false; 2],
			},
			doom::ui::UiImage {
				image: asset_storage.load("stkeys1.patch"),
			},
		));

		// Red key
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(239.0, 191.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(7.0, 5.0),
				stretch: [false; 2],
			},
			doom::ui::UiImage {
				image: asset_storage.load("stkeys2.patch"),
			},
		));

		// Bullets current
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(276.0, 173.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(12.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiText {
				text: " 50".into(),
				font: asset_storage.load("stysnum.font"),
			},
			doom::hud::AmmoStat {
				ammo_type: Some(asset_storage.load("bullets.ammo")),
				show_max: false,
			},
		));

		// Bullets max
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(302.0, 173.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(12.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiText {
				text: "200".into(),
				font: asset_storage.load("stysnum.font"),
			},
			doom::hud::AmmoStat {
				ammo_type: Some(asset_storage.load("bullets.ammo")),
				show_max: true,
			},
		));

		// Shells current
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(276.0, 179.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(12.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiText {
				text: "  0".into(),
				font: asset_storage.load("stysnum.font"),
			},
			doom::hud::AmmoStat {
				ammo_type: Some(asset_storage.load("shells.ammo")),
				show_max: false,
			},
		));

		// Shells max
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(302.0, 179.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(12.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiText {
				text: " 50".into(),
				font: asset_storage.load("stysnum.font"),
			},
			doom::hud::AmmoStat {
				ammo_type: Some(asset_storage.load("shells.ammo")),
				show_max: true,
			},
		));

		// Rockets current
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(276.0, 185.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(12.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiText {
				text: "  0".into(),
				font: asset_storage.load("stysnum.font"),
			},
			doom::hud::AmmoStat {
				ammo_type: Some(asset_storage.load("rockets.ammo")),
				show_max: false,
			},
		));

		// Rockets max
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(302.0, 185.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(12.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiText {
				text: " 50".into(),
				font: asset_storage.load("stysnum.font"),
			},
			doom::hud::AmmoStat {
				ammo_type: Some(asset_storage.load("rockets.ammo")),
				show_max: true,
			},
		));

		// Cells current
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(276.0, 191.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(12.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiText {
				text: "  0".into(),
				font: asset_storage.load("stysnum.font"),
			},
			doom::hud::AmmoStat {
				ammo_type: Some(asset_storage.load("cells.ammo")),
				show_max: false,
			},
		));

		// Cells max
		world.push((
			doom::ui::UiTransform {
				position: Vector2::new(302.0, 191.0),
				depth: 10.0,
				alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
				size: Vector2::new(12.0, 6.0),
				stretch: [false; 2],
			},
			doom::ui::UiText {
				text: "300".into(),
				font: asset_storage.load("stysnum.font"),
			},
			doom::hud::AmmoStat {
				ammo_type: Some(asset_storage.load("cells.ammo")),
				show_max: true,
			},
		));
	}

	let mut old_time = Instant::now();
	let mut leftover_time = Duration::default();

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
		execute_commands(&mut world, &mut resources);

		if resources.contains::<ShouldQuit>() {
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

#[derive(Clone, Copy, Debug, Default)]
struct ShouldQuit;
