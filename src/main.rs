mod common;
mod doom;

use crate::common::{
	assets::{AssetHandle, AssetStorage},
	audio::Sound,
	frame::{frame_state_system, FrameRng, FrameRngDef, FrameState},
	geometry::{AABB2, AABB3},
	input::InputState,
	quadtree::Quadtree,
	spawn::SpawnMergerHandlerSet,
	video::{AsBytes, DrawList, RenderContext, RenderTarget},
};
use anyhow::{bail, Context};
use clap::{App, Arg, ArgMatches};
use legion::{systems::ResourceSet, Entity, IntoQuery, Read, Resources, Schedule, World, Write};
use nalgebra::Vector2;
use rand::SeedableRng;
use relative_path::RelativePath;
use std::{
	path::PathBuf,
	sync::Mutex,
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
		doom::render::psprite::DrawPlayerSprites::new(&render_context, draw_list.render_pass())
			.context("Couldn't create DrawPlayerSprites")?,
	);
	draw_list.add_step(
		doom::render::ui::DrawUi::new(&render_context, draw_list.render_pass())
			.context("Couldn't create DrawUi")?,
	);

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

	resources.insert(InputState::new());
	resources.insert(Vec::<(AssetHandle<Sound>, Entity)>::new());
	resources.insert(doom::client::Client::default());

	let frame_state = FrameState {
		delta_time: doom::data::FRAME_TIME,
		time: Duration::default(),
		rng: Mutex::new(FrameRng::from_entropy()),
	};
	resources.insert(frame_state);

	let mut loader = doom::wad::WadLoader::new();
	load_wads(&mut loader, &arg_matches)?;

	// Select map
	let map =
		if let Some(map) = arg_matches.value_of("map") {
			map
		} else {
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

	// Asset types
	let mut asset_storage = AssetStorage::new(doom::import, loader);
	asset_storage.add_storage::<doom::entitytemplate::EntityTemplate>(false);
	asset_storage.add_storage::<doom::image::Image>(true);
	asset_storage.add_storage::<doom::image::ImageData>(false);
	asset_storage.add_storage::<doom::image::Palette>(false);
	asset_storage.add_storage::<doom::map::Map>(false);
	asset_storage.add_storage::<doom::map::textures::PNames>(false);
	asset_storage.add_storage::<doom::map::textures::Textures>(false);
	asset_storage.add_storage::<doom::sprite::Sprite>(false);
	asset_storage.add_storage::<doom::sound::Sound>(false);
	resources.insert(asset_storage);

	// Component types
	let mut handler_set = SpawnMergerHandlerSet::new();
	handler_set.register_spawn_from::<FrameRngDef, FrameRng>();
	handler_set.register_clone::<doom::camera::Camera>();
	handler_set.register_clone::<doom::client::UseAction>();
	handler_set.register_clone::<doom::client::User>();
	handler_set.register_clone::<doom::components::SpawnPoint>();
	handler_set
		.register_spawn_from::<doom::components::TransformDef, doom::components::Transform>(
		);
	handler_set.register_from::<doom::components::VelocityDef, doom::components::Velocity>();
	handler_set.register_clone::<doom::door::DoorActive>();
	handler_set.register_spawn_from::<doom::entitytemplate::EntityTemplateRefDef, doom::entitytemplate::EntityTemplateRef>();
	handler_set.register_clone::<doom::floor::FloorActive>();
	handler_set
		.register_spawn_from::<doom::light::LightFlashDef, doom::light::LightFlash>();
	handler_set.register_clone::<doom::light::LightGlow>();
	handler_set.register_clone::<doom::map::LinedefRef>();
	handler_set.register_clone::<doom::map::MapDynamic>();
	handler_set.register_clone::<doom::map::SectorRef>();
	handler_set.register_clone::<doom::physics::BoxCollider>();
	handler_set.register_clone::<doom::physics::TouchAction>();
	handler_set.register_clone::<doom::plat::PlatActive>();
	handler_set.register_clone::<doom::psprite::PlayerSpriteRender>();
	handler_set.register_clone::<doom::sectormove::CeilingMove>();
	handler_set.register_clone::<doom::sectormove::FloorMove>();
	handler_set.register_clone::<doom::sound::SoundPlaying>();
	handler_set.register_clone::<doom::sprite::SpriteRender>();
	handler_set.register_spawn_from::<doom::state::StateDef, doom::state::State>();
	handler_set.register_clone::<doom::switch::SwitchActive>();
	handler_set.register_clone::<doom::texture::TextureScroll>();
	resources.insert(handler_set);

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
		.add_thread_local(frame_state_system(doom::data::FRAME_TIME)).flush()
		.build();

	let mut output_dispatcher = Schedule::builder()
		.add_thread_local_fn(doom::render::render_system(draw_list))
		.add_thread_local_fn(doom::sound::sound_system())
		.build();

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
					"map" => load_map(&format!("{}", args[1]), &mut world, &mut resources)?,
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
			update_dispatcher.execute(&mut world, &mut resources);
			leftover_time -= doom::data::FRAME_TIME;

			let mut input_state = <Write<InputState>>::fetch_mut(&mut resources);
			input_state.reset();
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
	let name_lower = name.to_ascii_lowercase();
	let start_time = Instant::now();

	log::info!("Loading entity data...");
	doom::data::mobjs::load(resources);
	doom::data::sectors::load(resources);
	doom::data::linedefs::load(resources);

	log::info!("Loading map...");
	let map_handle = {
		let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
		asset_storage.load(&format!("{}.map", name_lower))
	};

	log::info!("Processing assets...");
	{
		let (render_context, mut asset_storage) =
			<(Read<RenderContext>, Write<AssetStorage>)>::fetch_mut(resources);

		// Palette
		let palette_handle: AssetHandle<doom::image::Palette> =
			asset_storage.load("playpal.palette");

		// Images
		asset_storage.process::<doom::image::Image, _>(|data, asset_storage| {
			let image_data: doom::image::ImageData = *data.downcast().ok().unwrap();
			let palette = asset_storage.get(&palette_handle).unwrap();
			let data: Vec<_> = image_data
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
					width: image_data.size[0] as u32,
					height: image_data.size[1] as u32,
				},
				Format::R8G8B8A8Unorm,
				render_context.queues().graphics.clone(),
			)?;

			Ok(crate::doom::image::Image {
				image,
				offset: Vector2::new(image_data.offset[0] as f32, image_data.offset[1] as f32),
			})
		});
	}

	log::info!("Spawning entities...");
	let things = {
		let asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
		doom::map::load::build_things(
			&asset_storage
				.source()
				.load(&RelativePath::new(&name_lower).with_extension("things"))?,
		)?
	};
	doom::map::spawn::spawn_map_entities(world, resources, &map_handle)?;
	doom::map::spawn::spawn_things(things, world, resources)?;

	// Spawn player
	let entity = doom::map::spawn::spawn_player(world, resources)?;
	<Write<doom::client::Client>>::fetch_mut(resources).entity = Some(entity);

	// Create quadtree and add entities to it
	let bbox = {
		let asset_storage = <Read<AssetStorage>>::fetch(resources);
		let map = asset_storage.get(&map_handle).unwrap();
		map.bbox.clone()
	};
	let mut quadtree = Quadtree::new(bbox);

	for (entity, box_collider, transform) in <(
		Entity,
		&doom::physics::BoxCollider,
		&doom::components::Transform,
	)>::query()
	.iter(world)
	{
		let bbox = AABB3::from_radius_height(box_collider.radius, box_collider.height);
		quadtree.insert(*entity, &AABB2::from(&bbox.offset(transform.position)));
	}

	resources.insert(quadtree);

	log::debug!(
		"Loading took {} s",
		(Instant::now() - start_time).as_secs_f32()
	);

	Ok(())
}
