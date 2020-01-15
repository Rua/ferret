mod assets;
mod audio;
mod commands;
mod component;
mod configvars;
mod doom;
mod geometry;
mod input;
mod logger;
mod renderer;
mod stdin;

use crate::{
	assets::{AssetFormat, AssetStorage},
	audio::Audio,
	component::EntityTemplate,
	input::{Axis, Bindings, Button, InputState, MouseAxis},
	logger::Logger,
	renderer::{
		texture::{Texture, TextureBuilder},
		video::Video,
	},
};
use specs::{world::Builder, ReadExpect, RunNow, World, WorldExt, WriteExpect};
use std::{
	error::Error,
	sync::mpsc,
	time::{Duration, Instant},
};
use vulkano::{format::Format, image::Dimensions};
use winit::{
	ElementState, Event, EventsLoop, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};

const FRAME_TIME: Duration = Duration::from_nanos(28571429); // 1/35 sec

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
	Logger::init().unwrap();

	let (command_sender, command_receiver) = mpsc::channel();

	match stdin::spawn(command_sender.clone()) {
		Ok(_) => (),
		Err(err) => {
			return Err(Box::from(format!("Could not start stdin thread: {}", err)));
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
	world.register::<doom::components::MapDynamic>();
	world.register::<doom::components::SpawnPoint>();
	world.register::<doom::components::SpriteRender>();
	world.register::<doom::components::Transform>();
	world.insert(AssetStorage::<EntityTemplate>::default());
	world.insert(AssetStorage::<doom::map::Map>::default());
	world.insert(AssetStorage::<doom::map::textures::Flat>::default());
	world.insert(AssetStorage::<doom::map::textures::WallTexture>::default());
	world.insert(AssetStorage::<doom::image::Palette>::default());
	world.insert(AssetStorage::<doom::sprite::Sprite>::default());
	world.insert(AssetStorage::<Texture>::default());
	world.insert(video);
	world.insert(audio);
	world.insert(loader);
	world.insert(InputState::new());
	world.insert(bindings);
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
				world.system_data::<(WriteExpect<InputState>, ReadExpect<Video>)>();
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
						if let Err(msg) = window.grab_cursor(true) {
							log::warn!("Couldn't grab cursor: {}", msg);
						}
						window.hide_cursor(true);
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
						let window = video.surface().window();
						if let Err(msg) = window.grab_cursor(false) {
							log::warn!("Couldn't release cursor: {}", msg);
						}
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
					log::error!("Invalid syntax: {}", e);
					continue;
				}
			};

			// Split further into subcommands
			for args in tokens.split(|tok| tok == ";") {
				match args[0].as_str() {
					"map" => {
						let name = &args[1];
						log::info!("Loading map {}...", name);

						// Load palette
						let palette_handle = {
							let (mut loader, mut palette_storage) = world.system_data::<(
								WriteExpect<doom::wad::WadLoader>,
								WriteExpect<AssetStorage<crate::doom::image::Palette>>,
							)>();
							let handle = palette_storage.load("PLAYPAL", &mut *loader);
							palette_storage.build_waiting(|intermediate| Ok(intermediate));
							handle
						};

						// Load entity type data
						let entity_types = doom::entities::EntityTypes::new(&world);
						world.insert(entity_types);

						{
							let (palette_storage, mut sprite_storage, mut texture_storage, video) =
								world.system_data::<(
									ReadExpect<AssetStorage<crate::doom::image::Palette>>,
									WriteExpect<AssetStorage<crate::doom::sprite::Sprite>>,
									WriteExpect<AssetStorage<Texture>>,
									ReadExpect<crate::renderer::video::Video>,
								)>();
							let palette = palette_storage.get(&palette_handle).unwrap();
							sprite_storage.build_waiting(|intermediate| {
								Ok(intermediate
									.build(
										video.queues().graphics.clone(),
										palette,
										&mut texture_storage,
									)?
									.0)
							});
						}

						// Load map
						let map = {
							let (
								mut loader,
								mut map_storage,
								mut flat_storage,
								mut wall_texture_storage,
							) = world.system_data::<(
								WriteExpect<doom::wad::WadLoader>,
								WriteExpect<AssetStorage<doom::map::Map>>,
								WriteExpect<AssetStorage<doom::map::textures::Flat>>,
								WriteExpect<AssetStorage<doom::map::textures::WallTexture>>,
							)>();
							let map = map_storage.load(name, &mut *loader);
							map_storage.build_waiting(|data| {
								doom::map::build_map(
									data,
									"SKY1",
									&mut *loader,
									&mut *flat_storage,
									&mut *wall_texture_storage,
								)
							});

							map
						};

						// Build textures
						{
							let (mut texture_storage, video) = world
								.system_data::<(WriteExpect<AssetStorage<Texture>>, ReadExpect<Video>)>(
								);
							texture_storage.build_waiting(|data| {
								Ok(data.build(video.queues().graphics.clone())?.0)
							});
						}

						{
							let (palette_storage, mut flat_storage, video) = world.system_data::<(
								ReadExpect<AssetStorage<doom::image::Palette>>,
								WriteExpect<AssetStorage<doom::map::textures::Flat>>,
								ReadExpect<Video>,
							)>();
							let palette = palette_storage.get(&palette_handle).unwrap();
							flat_storage.build_waiting(|image| {
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
								let builder = TextureBuilder::new()
									.with_data(data)
									.with_dimensions(Dimensions::Dim2d {
										width: image.size[0] as u32,
										height: image.size[1] as u32,
									})
									.with_format(Format::R8G8B8A8Unorm);

								Ok(builder.build(video.queues().graphics.clone())?.0)
							});
						}

						{
							let (palette_storage, mut wall_texture_storage, video) = world
								.system_data::<(
									ReadExpect<AssetStorage<doom::image::Palette>>,
									WriteExpect<AssetStorage<doom::map::textures::WallTexture>>,
									ReadExpect<Video>,
								)>();
							let palette = palette_storage.get(&palette_handle).unwrap();
							wall_texture_storage.build_waiting(|image| {
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

								let builder = TextureBuilder::new()
									.with_data(data)
									.with_dimensions(Dimensions::Dim2d {
										width: image.size[0] as u32,
										height: image.size[1] as u32,
									})
									.with_format(Format::R8G8B8A8Unorm);

								Ok(builder.build(video.queues().graphics.clone())?.0)
							});
						}

						// Generate model
						let map_model = {
							let map_storage =
								world.system_data::<ReadExpect<AssetStorage<doom::map::Map>>>();
							let map = map_storage.get(&map).unwrap();
							doom::map::meshes::make_model(&map, &world)?
						};

						// Create world entity
						world
							.create_entity()
							.with(doom::components::MapDynamic {
								map: map.clone(),
								map_model,
							})
							.build();

						// Spawn things
						let things = {
							let mut loader =
								world.system_data::<WriteExpect<doom::wad::WadLoader>>();
							doom::map::lumps::ThingsFormat.import(name, &mut *loader)?
						};
						doom::map::spawn_map_entities(things, &world, &map)?;
						let entity = doom::map::spawn_player(&world)?;
						world.insert(entity);
					}
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
