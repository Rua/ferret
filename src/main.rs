mod common;
mod doom;

use crate::common::{
	assets::{AssetHandle, AssetStorage, DataSource},
	audio::Sound,
	geometry::{AABB2, AABB3},
	input::InputState,
	quadtree::Quadtree,
	video::{AsBytes, DrawList, RenderContext, RenderTarget},
};
use anyhow::{bail, Context};
use clap::{App, Arg, ArgMatches};
use legion::prelude::{Entity, IntoQuery, Read, ResourceSet, Resources, Schedule, World, Write};
use nalgebra::{Vector2, Vector3};
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use std::{
	path::PathBuf,
	time::{Duration, Instant},
};
use vulkano::{
	format::Format,
	image::{Dimensions, ImmutableImage},
	sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
};
use winit::{
	event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
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

	common::logger::init(&arg_matches)?;

	// Set up resources
	let mut resources = Resources::default();

	let mut loader = doom::wad::WadLoader::new();
	load_wads(&mut loader, &arg_matches)?;
	resources.insert(loader);

	let (command_sender, command_receiver) = common::commands::init()?;
	let mut event_loop = EventLoop::new();

	let (render_context, _debug_callback) =
		RenderContext::new(&event_loop).context("Could not create RenderContext")?;
	let render_target = RenderTarget::new(
		render_context.surface().clone(),
		render_context.device().clone(),
	)
	.context("Couldn't create RenderTarget")?;

	let mut draw_list = DrawList::new(&render_context, render_target.dimensions())
		.context("Couldn't create DrawList")?;
	draw_list.add_step(
		doom::render::world::DrawWorld::new(&render_context)
			.context("Couldn't create DrawWorld")?,
	);
	draw_list.add_step(
		doom::render::map::DrawMap::new(draw_list.render_pass())
			.context("Couldn't create DrawMap")?,
	);
	draw_list.add_step(
		doom::render::sprite::DrawSprites::new(&render_context, draw_list.render_pass())
			.context("Couldn't create DrawSprites")?,
	);
	draw_list.add_step(
		doom::render::ui::DrawUi::new(&render_context, draw_list.render_pass())
			.context("Couldn't create DrawUi")?,
	);
	resources.insert(draw_list);

	resources.insert(
		Sampler::new(
			render_context.device().clone(),
			Filter::Nearest,
			Filter::Nearest,
			MipmapMode::Nearest,
			SamplerAddressMode::Repeat,
			SamplerAddressMode::Repeat,
			SamplerAddressMode::Repeat,
			0.0,
			1.0,
			0.0,
			0.0,
		)
		.context("Couldn't create texture sampler")?,
	);
	resources.insert(render_target);
	resources.insert(render_context);

	let sound_sender = common::audio::init()?;
	resources.insert(sound_sender);

	let bindings = doom::data::get_bindings();
	resources.insert(bindings);

	resources.insert(AssetStorage::default());
	resources.insert(Pcg64Mcg::from_entropy());
	resources.insert(InputState::new());
	resources.insert(Vec::<(AssetHandle<Sound>, Entity)>::new());
	resources.insert(doom::client::Client::default());
	resources.insert(doom::data::FRAME_TIME);

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
	#[rustfmt::skip]
	let mut update_dispatcher = Schedule::builder()
		.add_thread_local(doom::client::player_command_system()).flush()
		.add_thread_local(doom::client::player_move_system()).flush()
		.add_thread_local(doom::client::player_attack_system(&mut resources)).flush()
		.add_thread_local(doom::client::player_use_system(&mut resources)).flush()
		.add_thread_local(doom::physics::physics_system(&mut resources)).flush()
		.add_thread_local(doom::camera::camera_system(&mut resources)).flush()
		.add_thread_local(doom::door::door_use_system(&mut resources)).flush()
		.add_thread_local(doom::door::door_switch_system(&mut resources)).flush()
		.add_thread_local(doom::door::door_touch_system(&mut resources)).flush()
		.add_thread_local(doom::floor::floor_switch_system(&mut resources)).flush()
		.add_thread_local(doom::floor::floor_touch_system(&mut resources)).flush()
		.add_thread_local(doom::plat::plat_switch_system(&mut resources)).flush()
		.add_thread_local(doom::plat::plat_touch_system(&mut resources)).flush()
		.add_thread_local(doom::sectormove::sector_move_system(&mut resources)).flush()
		.add_thread_local(doom::door::door_active_system(&mut resources)).flush()
		.add_thread_local(doom::floor::floor_active_system(&mut resources)).flush()
		.add_thread_local(doom::plat::plat_active_system(&mut resources)).flush()
		.add_thread_local(doom::light::light_flash_system()).flush()
		.add_thread_local(doom::light::light_glow_system()).flush()
		.add_thread_local(doom::switch::switch_active_system()).flush()
		.add_thread_local(doom::texture::texture_animation_system()).flush()
		.add_thread_local(doom::texture::texture_scroll_system()).flush()
		.add_thread_local(doom::state::state_system(&mut resources)).flush()
		.build();

	let mut output_dispatcher = Schedule::builder()
		.add_thread_local_fn(doom::render::render_system())
		.add_thread_local_fn(doom::sound::sound_system())
		.build();

	// Create world
	let mut world = World::new();

	{
		let (mut asset_storage, mut loader) =
			<(Write<AssetStorage>, Write<doom::wad::WadLoader>)>::fetch_mut(&mut resources);

		world.insert(
			(),
			vec![
				/*(
					doom::ui::UiTransform {
						position: Vector3::new(0.0, 200.0 - 32.0, 0.0),
						alignment: [doom::ui::UiAlignment::Near, doom::ui::UiAlignment::Far],
						size: Vector2::new(320.0, 32.0),
						stretch: [true, false],
					},
					doom::ui::UiImage {
						image: asset_storage.load("FLOOR7_2", &mut *loader),
					},
				),*/
				(
					doom::ui::UiTransform {
						position: Vector3::new(0.0, 168.0, 1.0),
						alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
						size: Vector2::new(320.0, 32.0),
						stretch: [false; 2],
					},
					doom::ui::UiImage {
						image: asset_storage.load("STBAR", &mut *loader),
					},
				),
				(
					doom::ui::UiTransform {
						position: Vector3::new(104.0, 168.0, 2.0),
						alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
						size: Vector2::new(40.0, 32.0),
						stretch: [false; 2],
					},
					doom::ui::UiImage {
						image: asset_storage.load("STARMS", &mut *loader),
					},
				),
				(
					doom::ui::UiTransform {
						position: Vector3::new(143.0, 168.0, 10.0),
						alignment: [doom::ui::UiAlignment::Middle, doom::ui::UiAlignment::Far],
						size: Vector2::new(24.0, 29.0),
						stretch: [false; 2],
					},
					doom::ui::UiImage {
						image: asset_storage.load("STFST00", &mut *loader),
					},
				),
			],
		);
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
			let (mut input_state, render_context, mut render_target) =
				<(Write<InputState>, Read<RenderContext>, Write<RenderTarget>)>::fetch_mut(
					&mut resources,
				);
			input_state.process_event(&event);

			match event {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::CloseRequested => {
						command_sender.send("quit".to_owned()).ok();
						*control_flow = ControlFlow::Exit;
					}
					WindowEvent::Resized(new_size) => {
						render_target.window_resized(new_size.into());
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

		// Update video and sound
		output_dispatcher.execute(&mut world, &mut resources);
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
	doom::data::mobjs::load(resources);
	doom::data::sectors::load(resources);
	doom::data::linedefs::load(resources);

	// Load sprite images
	{
		let (render_context, mut asset_storage, mut source) = <(
			Read<RenderContext>,
			Write<AssetStorage>,
			Write<crate::doom::wad::WadLoader>,
		)>::fetch_mut(resources);
		asset_storage.build_waiting::<doom::sprite::Sprite, _>(|builder, asset_storage| {
			Ok(builder.build(asset_storage, &mut *source)?)
		});
		asset_storage.build_waiting::<doom::image::Image, _>(|image_raw, asset_storage| {
			let palette = asset_storage.get(&palette_handle).unwrap();
			let data: Vec<_> = image_raw
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
					width: image_raw.size[0] as u32,
					height: image_raw.size[1] as u32,
				},
				Format::R8G8B8A8Unorm,
				render_context.queues().graphics.clone(),
			)?;

			Ok(crate::doom::image::Image {
				image,
				offset: image_raw.offset,
			})
		});
	}

	// Load sounds
	{
		let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
		asset_storage.build_waiting::<common::audio::Sound, _>(|intermediate, _| {
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
