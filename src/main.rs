mod assets;
mod audio;
mod commands;
mod component;
mod configvars;
mod doom;
mod geometry;
mod input;
mod logger;
mod quadtree;
mod renderer;

use crate::{
	assets::{AssetHandle, AssetStorage, DataSource},
	audio::Sound,
	geometry::{AABB2, AABB3},
	input::{Axis, Bindings, Button, InputState, MouseAxis},
	quadtree::Quadtree,
	renderer::{AsBytes, RenderContext},
};
use anyhow::{bail, Context};
use clap::{App, Arg, ArgMatches};
use legion::{
	prelude::{Entity, IntoQuery, Read, ResourceSet, Resources, World, Write},
	systems::schedule::Builder,
};
use nalgebra::{Matrix4, Vector3};
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use shrev::EventChannel;
use std::{
	path::PathBuf,
	time::{Duration, Instant},
};
use vulkano::{
	format::Format,
	image::{Dimensions, ImmutableImage},
};
use winit::{
	event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	platform::desktop::EventLoopExtDesktop,
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

	logger::init(&arg_matches)?;

	// Set up resources
	let mut resources = Resources::default();

	let mut loader = doom::wad::WadLoader::new();
	load_wads(&mut loader, &arg_matches)?;
	resources.insert(loader);

	let (command_sender, command_receiver) = commands::init()?;
	let mut event_loop = EventLoop::new();

	let (render_context, _debug_callback) =
		RenderContext::new(&event_loop).context("Could not create rendering context")?;
	resources.insert(render_context);

	let sound_sender = audio::init()?;
	resources.insert(sound_sender);

	let bindings = get_bindings();
	resources.insert(bindings);

	resources.insert(AssetStorage::default());
	resources.insert(Pcg64Mcg::from_entropy());
	resources.insert(InputState::new());
	resources.insert(Vec::<(AssetHandle<Sound>, Entity)>::new());
	resources.insert(doom::client::Client::default());
	resources.insert(doom::data::FRAME_TIME);
	resources.insert(EventChannel::<doom::client::UseEvent>::new());

	// Select map
	let map =
		if let Some(map) = arg_matches.value_of("map") {
			map
		} else {
			let loader = <Read<doom::wad::WadLoader>>::fetch(&resources);
			let wad = loader.wads().next().unwrap().file_name().unwrap();

			if wad == "doom.wad" || wad == "doom1.wad" || wad == "doomu.wad" {
				"E1M1"
			} else if wad == "doom2.wad" || wad == "tnt.wad" || wad == "plutonia.wad" {
				"MAP01"
			} else {
				bail!("No default map is known for this IWAD. Try specifying one with the \"-m\" option.")
			}
		};
	command_sender.send(format!("map {}", map)).ok();

	// Create systems
	let mut render_system =
		doom::render::RenderSystem::new(&*resources.get::<RenderContext>().unwrap())
			.context("Couldn't create RenderSystem")?;
	let mut sound_system = doom::sound::sound_system();
	let mut update_dispatcher = Builder::default()
		.add_thread_local_fn(doom::client::player_command_system())
		.add_thread_local_fn(doom::client::player_move_system())
		.add_thread_local_fn(doom::client::player_use_system())
		.add_thread_local_fn(doom::camera::camera_system())
		.add_thread_local_fn(doom::physics::physics_system())
		.add_thread_local_fn(doom::door::door_use_system(&mut resources))
		.add_thread_local_fn(doom::door::door_active_system())
		.add_thread_local_fn(doom::door::switch_active_system())
		.add_thread_local_fn(doom::light::light_system())
		.add_thread_local_fn(doom::texture::texture_system())
		.build();

	// Create world
	let mut world = World::new();

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
			let (mut input_state, render_context) =
				<(Write<InputState>, Read<RenderContext>)>::fetch_mut(&mut resources);
			input_state.process_event(&event);

			match event {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::CloseRequested => {
						command_sender.send("quit".to_owned()).ok();
						*control_flow = ControlFlow::Exit;
					}
					WindowEvent::Resized(_) => {
						if let Err(msg) = render_system.recreate() {
							log::warn!("Error recreating swapchain: {}", msg);
						}
					}
					WindowEvent::MouseInput {
						state: ElementState::Pressed,
						..
					} => {
						let window = render_context.surface().window();
						if let Err(msg) = window.set_cursor_grab(true) {
							log::warn!("Couldn't grab cursor: {}", msg);
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
						if let Err(msg) = window.set_cursor_grab(false) {
							log::warn!("Couldn't release cursor: {}", msg);
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
			let tokens = match commands::tokenize(&command) {
				Ok(tokens) => tokens,
				Err(e) => {
					log::error!("Invalid syntax: {}", e);
					continue;
				}
			};

			// Split further into subcommands
			for args in tokens.split(|tok| tok == ";") {
				match args[0].as_str() {
					"map" => load_map(&args[1], &mut world, &mut resources)?,
					"quit" => should_quit = true,
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
			leftover_time -= doom::data::FRAME_TIME;

			update_dispatcher.execute(&mut world, &mut resources);

			// Reset input delta state
			{
				let mut input_state = resources.get_mut::<InputState>().unwrap();
				input_state.reset();
			}
		}

		// Update sound
		sound_system(&mut world, &mut resources);

		// Draw frame
		render_system
			.draw(&world, &resources)
			.context("Error while rendering")?;
	}

	Ok(())
}

fn load_wads(loader: &mut doom::wad::WadLoader, arg_matches: &ArgMatches) -> anyhow::Result<()> {
	let mut wads = Vec::new();
	const IWADS: [&str; 6] = ["doom2", "plutonia", "tnt", "doomu", "doom", "doom1"];

	let iwad = if let Some(iwad) = arg_matches.value_of("iwad") {
		PathBuf::from(iwad)
	} else if let Some(iwad) = IWADS
		.iter()
		.map(|p| PathBuf::from(format!("{}.wad", p)))
		.find(|p| p.is_file())
	{
		iwad
	} else {
		bail!("No iwad file found. Try specifying one with the \"-i\" command line option.")
	};

	wads.push(iwad);

	if let Some(iter) = arg_matches.values_of("PWADS") {
		wads.extend(iter.map(PathBuf::from));
	}

	for path in wads {
		loader
			.add(&path)
			.context(format!("Couldn't load {}", path.display()))?;

		// Try to load the .gwa file as well if present
		if let Some(extension) = path.extension() {
			if extension == "wad" {
				let path = path.with_extension("gwa");

				if path.is_file() {
					loader
						.add(&path)
						.context(format!("Couldn't load {}", path.display()))?;
				}
			}
		}
	}

	Ok(())
}

fn get_bindings() -> Bindings<doom::input::Action, doom::input::Axis> {
	let mut bindings = Bindings::new();
	bindings.bind_action(
		doom::input::Action::Attack,
		Button::Mouse(MouseButton::Left),
	);
	bindings.bind_action(doom::input::Action::Use, Button::Key(VirtualKeyCode::Space));
	bindings.bind_action(doom::input::Action::Use, Button::Mouse(MouseButton::Middle));
	bindings.bind_action(
		doom::input::Action::Walk,
		Button::Key(VirtualKeyCode::LShift),
	);
	bindings.bind_action(
		doom::input::Action::Walk,
		Button::Key(VirtualKeyCode::RShift),
	);
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

	bindings
}

fn load_map(name: &str, world: &mut World, resources: &mut Resources) -> anyhow::Result<()> {
	log::info!("Starting map {}...", name);
	let start_time = Instant::now();

	// Load palette
	let palette_handle: AssetHandle<doom::image::Palette> = {
		let (mut asset_storage, mut loader) =
			<(Write<AssetStorage>, Write<doom::wad::WadLoader>)>::fetch_mut(resources);
		let handle = asset_storage.load("PLAYPAL", &mut *loader);
		asset_storage.build_waiting::<doom::image::Palette, _>(|x, _| Ok(x));
		handle
	};

	// Load entity type data
	log::info!("Loading entity data...");
	let mobj_types = doom::data::MobjTypes::new(resources);
	let sector_types = doom::data::SectorTypes::new(resources);
	let linedef_types = doom::data::LinedefTypes::new(resources);

	resources.insert(mobj_types);
	resources.insert(sector_types);
	resources.insert(linedef_types);

	// Load sprite images
	{
		let (render_context, mut asset_storage, mut source) = <(
			Read<crate::renderer::RenderContext>,
			Write<AssetStorage>,
			Write<crate::doom::wad::WadLoader>,
		)>::fetch_mut(resources);
		asset_storage.build_waiting::<doom::sprite::Sprite, _>(|builder, asset_storage| {
			Ok(builder.build(asset_storage, &mut *source)?)
		});
		asset_storage.build_waiting::<doom::sprite::SpriteImage, _>(|image, asset_storage| {
			let palette = asset_storage.get(&palette_handle).unwrap();
			let data: Vec<_> = image
				.data
				.into_iter()
				.map(|pixel| {
					if pixel.a == 0xFF {
						palette[pixel.i as usize]
					} else {
						crate::doom::image::RGBAColor::default()
					}
				})
				.collect();

			// Create the image
			let matrix = Matrix4::new_translation(&Vector3::new(
				0.0,
				image.offset[0] as f32,
				image.offset[1] as f32,
			)) * Matrix4::new_nonuniform_scaling(&Vector3::new(
				0.0,
				image.size[0] as f32,
				image.size[1] as f32,
			));

			let (image, _future) = ImmutableImage::from_iter(
				data.as_bytes().iter().copied(),
				Dimensions::Dim2d {
					width: image.size[0] as u32,
					height: image.size[1] as u32,
				},
				Format::R8G8B8A8Unorm,
				render_context.queues().graphics.clone(),
			)?;

			Ok(crate::doom::sprite::SpriteImage { matrix, image })
		});
	}

	// Load sounds
	{
		let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
		asset_storage.build_waiting::<audio::Sound, _>(|intermediate, _| {
			doom::sound::build_sound(intermediate)
		});
	}

	// Load map
	log::info!("Loading map...");
	let map_handle = {
		let (mut asset_storage, mut loader) =
			<(Write<AssetStorage>, Write<doom::wad::WadLoader>)>::fetch_mut(resources);
		let map_handle = asset_storage.load(name, &mut *loader);
		asset_storage.build_waiting::<doom::map::Map, _>(|data, asset_storage| {
			doom::map::load::build_map(data, "SKY1", &mut *loader, asset_storage)
		});

		map_handle
	};

	// Build flats and wall textures
	{
		let (render_context, mut asset_storage) =
			<(Read<RenderContext>, Write<AssetStorage>)>::fetch_mut(resources);
		asset_storage.build_waiting::<doom::map::textures::Wall, _>(|image, asset_storage| {
			let palette = asset_storage.get(&palette_handle).unwrap();
			let data: Vec<_> = image
				.data
				.into_iter()
				.map(|pixel| {
					if pixel.a == 0xFF {
						palette[pixel.i as usize]
					} else {
						crate::doom::image::RGBAColor::default()
					}
				})
				.collect();

			// Create the image
			let (image, _future) = ImmutableImage::from_iter(
				data.as_bytes().iter().copied(),
				Dimensions::Dim2d {
					width: image.size[0] as u32,
					height: image.size[1] as u32,
				},
				Format::R8G8B8A8Unorm,
				render_context.queues().graphics.clone(),
			)?;

			Ok(image)
		});
		asset_storage.build_waiting::<doom::map::textures::Flat, _>(|image, asset_storage| {
			let palette = asset_storage.get(&palette_handle).unwrap();
			let data: Vec<_> = image
				.data
				.into_iter()
				.map(|pixel| {
					if pixel.a == 0xFF {
						palette[pixel.i as usize]
					} else {
						crate::doom::image::RGBAColor::default()
					}
				})
				.collect();

			let (image, _future) = ImmutableImage::from_iter(
				data.as_bytes().iter().copied(),
				Dimensions::Dim2d {
					width: image.size[0] as u32,
					height: image.size[1] as u32,
				},
				Format::R8G8B8A8Unorm,
				render_context.queues().graphics.clone(),
			)?;

			Ok(image)
		});
	}

	log::info!("Spawning entities...");

	// Spawn map entities and things
	let things = {
		let loader = <Write<doom::wad::WadLoader>>::fetch_mut(resources);
		doom::map::load::build_things(&loader.load(&format!("{}/+{}", name, 1))?)?
	};
	doom::map::spawn_map_entities(world, &resources, &map_handle)?;
	doom::map::spawn_things(things, world, resources, &map_handle)?;

	// Spawn player
	let entity = doom::map::spawn_player(world, resources)?;
	<Write<doom::client::Client>>::fetch_mut(resources).entity = Some(entity);

	// Create quadtree and add entities to it
	let bbox = {
		let asset_storage = <Read<AssetStorage>>::fetch(resources);
		let map = asset_storage.get(&map_handle).unwrap();
		map.bbox.clone()
	};
	let mut quadtree = Quadtree::new(bbox);

	for (entity, (box_collider, transform)) in <(
		Read<doom::physics::BoxCollider>,
		Read<doom::components::Transform>,
	)>::query()
	.iter_entities(world)
	{
		let bbox = AABB3::from_radius_height(box_collider.radius, box_collider.height);
		quadtree.insert(entity, &AABB2::from(&bbox.offset(transform.position)));
	}

	resources.insert(quadtree);

	log::debug!(
		"Loading took {} s",
		(Instant::now() - start_time).as_secs_f32()
	);

	Ok(())
}
