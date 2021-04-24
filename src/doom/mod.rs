//! Items specific to the implementation of Doom.

pub mod camera;
pub mod cheats;
pub mod client;
pub mod commands;
pub mod components;
pub mod data;
pub mod door;
pub mod draw;
pub mod exit;
pub mod floor;
pub mod health;
pub mod hud;
pub mod image;
pub mod input;
pub mod light;
pub mod map;
pub mod physics;
pub mod plat;
pub mod sectormove;
pub mod sound;
pub mod spawn;
pub mod sprite;
pub mod state;
pub mod switch;
pub mod template;
pub mod texture;
pub mod trace;
pub mod ui;
pub mod wad;

use crate::{
	common::{
		assets::{AssetHandle, AssetStorage, ImportData, ASSET_SERIALIZER},
		geometry::AABB2,
		input::InputState,
		quadtree::Quadtree,
		time::{DeltaTime, GameTime},
		video::{DrawTarget, RenderContext},
	},
	doom::{
		camera::{camera_move, movement_bob},
		client::{
			player_command_system, player_move_system, player_touch, player_use,
			player_weapon_system, Client, UseEvent,
		},
		components::{clear_event, register_components, Transform},
		data::{iwads::IWADINFO, FRAME_TIME},
		door::{door_active, door_linedef_touch, door_switch_use, door_use},
		draw::{
			finish_draw, map::draw_map, sprite::draw_sprites, start_draw, ui::draw_ui,
			world::draw_world, wsprite::draw_weapon_sprites,
		},
		exit::exit_switch_use,
		floor::{floor_active, floor_linedef_touch, floor_switch_use},
		health::{apply_damage, DamageEvent},
		hud::{ammo_stat, arms_stat, health_stat},
		image::{import_palette, import_patch, process_images, Image, ImageData, Palette},
		light::{light_flash_system, light_glow_system},
		map::{
			load::import_map,
			textures::{
				import_flat, import_pnames, import_textures, import_wall, PNames, Textures,
			},
			LinedefRef, Map, MapDynamic, SectorRef,
		},
		physics::{physics, BoxCollider, StepEvent, TouchEvent},
		plat::{plat_active, plat_linedef_touch, plat_switch_use},
		sectormove::sector_move_system,
		sound::{
			import_raw_sound, import_sound, start_sound, update_sound, RawSound, Sound,
			StartSoundEvent,
		},
		sprite::{import_sprite, Sprite},
		state::{
			entity::{
				next_entity_state, remove_entity, set_blocks_types, set_entity_sprite,
				set_solid_type, EntityStateEvent,
			},
			state,
			weapon::{
				change_ammo_count, extra_light, line_attack, next_weapon_state, projectile_touch,
				radius_attack, set_weapon_sprite, set_weapon_state, spawn_projectile, spray_attack,
				weapon_position, weapon_ready, weapon_refire, WeaponStateEvent,
			},
		},
		switch::switch_active_system,
		template::{
			import_ammo, import_entity, import_weapon, AmmoTemplate, EntityTemplate, WeaponTemplate,
		},
		texture::{texture_animation_system, texture_scroll_system},
		ui::{import_font, import_hexfont, Font, HexFont, UiParams},
		wad::{IWADInfo, WadLoader},
	},
};
use anyhow::{bail, Context};
use chrono::Local;
use clap::ArgMatches;
use crossbeam_channel::Sender;
use legion::{
	component,
	serialize::{set_entity_serializer, Canon},
	systems::{Builder, CommandBuffer, ResourceSet},
	Entity, IntoQuery, Read, Registry, Resources, Schedule, World, Write,
};
use relative_path::RelativePath;
use serde::{de::DeserializeSeed, Deserialize, Serialize};
use std::{
	fs::File,
	io::{BufReader, BufWriter, Write as IOWrite},
	path::Path,
};
use vulkano::{
	sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
	sync::GpuFuture,
};

pub fn import(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let function = match path.extension() {
		Some("ammo") => import_ammo,
		Some("entity") => import_entity,
		Some("flat") => import_flat,
		Some("font") => import_font,
		Some("hex") => import_hexfont,
		Some("map") => import_map,
		Some("palette") => import_palette,
		Some("patch") => import_patch,
		Some("sound") => import_sound,
		Some("rawsound") => import_raw_sound,
		Some("sprite") => import_sprite,
		Some("texture") => import_wall,
		Some("weapon") => import_weapon,
		Some(ext) => bail!("Unsupported file extension: {}", ext),
		None => match path.file_name() {
			Some("pnames") => import_pnames,
			Some("texture1") | Some("texture2") => import_textures,
			Some(name) => bail!("File has no extension: {}", name),
			None => bail!("Path ends in '..'"),
		},
	};

	function(path, asset_storage)
}

pub fn init_resources(resources: &mut Resources, arg_matches: &ArgMatches) -> anyhow::Result<()> {
	resources.insert(DeltaTime(FRAME_TIME));

	let dimensions = <Read<DrawTarget>>::fetch(resources).dimensions();
	resources.insert(UiParams::new(dimensions));

	let device = <Read<RenderContext>>::fetch(resources).device().clone();
	resources.insert(
		Sampler::new(
			device,
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

	{
		let mut input_state = <Write<InputState>>::fetch_mut(resources);
		input_state.bindings = data::get_bindings();
	}

	// Asset types
	let mut asset_storage = AssetStorage::new(import, WadLoader::new());
	asset_storage.add_storage::<AmmoTemplate>(false);
	asset_storage.add_storage::<EntityTemplate>(false);
	asset_storage.add_storage::<Font>(false);
	asset_storage.add_storage::<HexFont>(true);
	asset_storage.add_storage::<Image>(true);
	asset_storage.add_storage::<ImageData>(false);
	asset_storage.add_storage::<Map>(false);
	asset_storage.add_storage::<Palette>(false);
	asset_storage.add_storage::<PNames>(false);
	asset_storage.add_storage::<RawSound>(false);
	asset_storage.add_storage::<Sound>(false);
	asset_storage.add_storage::<Sprite>(false);
	asset_storage.add_storage::<Textures>(false);
	asset_storage.add_storage::<WeaponTemplate>(false);
	resources.insert(asset_storage);

	register_components(resources);

	log::info!("Engine initialised.");
	log::info!("Type \"help\" to see available commands.");
	log::info!("--------------------------------");

	// Load IWAD and PWADs
	load_wads(resources, &arg_matches)?;

	// Select map
	let map = if let Some(map) = arg_matches.value_of("map") {
		map
	} else {
		<Read<IWADInfo>>::fetch(resources).map
	};

	let command_sender = <Read<Sender<String>>>::fetch(resources);
	command_sender.send(format!("new {}", map)).ok();
	command_sender.send("save foo".into()).ok();

	Ok(())
}

pub fn add_update_systems(builder: &mut Builder, resources: &mut Resources) -> anyhow::Result<()> {
	#[rustfmt::skip]
	builder
		.add_thread_local(player_command_system(resources)).flush()
		.add_thread_local(player_move_system(resources)).flush()
		.add_thread_local(player_weapon_system(resources)).flush()

		.add_system(player_use(resources)).flush()
		.add_system(door_use(resources))
		.add_system(door_switch_use(resources))
		.add_system(exit_switch_use(resources))
		.add_system(floor_switch_use(resources))
		.add_system(plat_switch_use(resources))
		.add_system(clear_event::<UseEvent>())
		.flush()

		.add_system(physics(resources)).flush()
		.add_system(door_linedef_touch(resources))
		.add_system(floor_linedef_touch(resources))
		.add_system(plat_linedef_touch(resources))
		.add_system(player_touch(resources))
		.add_system(projectile_touch(resources))
		.add_system(movement_bob(resources))
		.add_system(camera_move(resources))
		.add_system(clear_event::<StepEvent>())
		.add_system(clear_event::<TouchEvent>())
		.flush()

		.add_thread_local(sector_move_system(resources)).flush()
		.add_thread_local(door_active(resources)).flush()
		.add_thread_local(floor_active(resources)).flush()
		.add_thread_local(plat_active(resources)).flush()
		.add_thread_local(light_flash_system(resources)).flush()
		.add_thread_local(light_glow_system(resources)).flush()
		.add_thread_local(switch_active_system(resources)).flush()
		.add_thread_local(texture_animation_system(resources)).flush()
		.add_thread_local(texture_scroll_system(resources)).flush()
		
		.add_system(apply_damage(resources))
		.add_system(clear_event::<DamageEvent>())
		.flush()

		.add_thread_local_fn({
			let actions = Schedule::builder()
				.add_system(change_ammo_count(resources))
				.add_system(extra_light(resources))
				.add_system(next_entity_state(resources))
				.add_system(remove_entity(resources))
				.add_system(set_blocks_types(resources))
				.add_system(set_entity_sprite(resources))
				.add_system(set_solid_type(resources))
				.add_system(next_weapon_state(resources))
				.add_system(line_attack(resources))
				.add_system(radius_attack(resources))
				.add_system(set_weapon_sprite(resources))
				.add_system(set_weapon_state(resources))
				.add_system(spawn_projectile(resources))
				.add_system(spray_attack(resources))
				.add_system(weapon_position(resources))
				.add_system(weapon_ready(resources))
				.add_system(weapon_refire(resources))
				.add_system(clear_event::<EntityStateEvent>())
				.add_system(clear_event::<WeaponStateEvent>())
				.flush()
				.build();

			state(resources, actions)
		})
		.add_thread_local(ammo_stat(resources)).flush()
		.add_thread_local(health_stat(resources)).flush()
		.add_thread_local(arms_stat(resources)).flush();

	Ok(())
}

pub fn add_output_systems(builder: &mut Builder, resources: &mut Resources) -> anyhow::Result<()> {
	#[rustfmt::skip]
	builder
		.add_thread_local(start_draw(resources)?)
		.add_thread_local(draw_world(resources)?)
		.add_thread_local(draw_map(resources)?)
		.add_thread_local(draw_sprites(resources)?)
		.add_thread_local(draw_weapon_sprites(resources)?)
		.add_thread_local(draw_ui(resources)?)
		.add_thread_local(finish_draw(resources)?)
		.add_system(start_sound(resources))
		.add_system(clear_event::<StartSoundEvent>()).flush()
		.add_system(update_sound(resources)).flush();

	Ok(())
}

fn load_wads(resources: &mut Resources, arg_matches: &ArgMatches) -> anyhow::Result<()> {
	// Determine IWAD
	let mut iter = IWADINFO
		.iter()
		.enumerate()
		.flat_map(|(i, info)| info.files.iter().map(move |file| (i, *file)));

	let (index, iwad_path) = if let Some(iwad) = arg_matches.value_of("iwad") {
		let iwad_path = Path::new(iwad);
		let iwad_file: &str = iwad_path
			.file_name()
			.with_context(|| format!("IWAD path \"{}\" does not contain a file name.", iwad))?
			.to_str()
			.unwrap();

		if let Some((index, _)) = iter.find(|(_, file)| *file == iwad_file) {
			(index, iwad_path)
		} else {
			bail!("IWAD \"{}\" is not a recognised game IWAD.", iwad);
		}
	} else {
		iter.map(|(i, file)| (i, Path::new(file)))
			.find(|(_, file)| file.is_file())
			.with_context(|| {
				format!("No recognised game IWAD found. Try specifying one with the \"-i\" command line option.")
			})?
	};

	resources.insert(IWADINFO[index].clone());

	// Add IWAD and PWADs to loader
	{
		let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
		let loader = asset_storage
			.source_mut()
			.downcast_mut::<WadLoader>()
			.expect("AssetStorage source was not of type WadLoader");

		let mut add_with_gwa = |path: &Path| -> anyhow::Result<()> {
			loader
				.add(&path)
				.context(format!("Couldn't load WAD \"{}\"", path.display()))?;

			// Try to load the .gwa file as well if present
			if let Some(extension) = path.extension() {
				if extension == "wad" {
					let path = path.with_extension("gwa");

					if path.is_file() {
						loader
							.add(&path)
							.context(format!("Couldn't load WAD \"{}\"", path.display()))?;
					}
				}
			}

			Ok(())
		};

		add_with_gwa(iwad_path)?;

		if let Some(iter) = arg_matches.values_of("PWADS") {
			for pwad in iter.map(Path::new) {
				add_with_gwa(pwad)?;
			}
		}
	}

	Ok(())
}

macro_rules! game_entities {
	() => {
		component::<Transform>()
			| component::<MapDynamic>()
			| component::<LinedefRef>()
			| component::<SectorRef>()
	};
}

pub fn clear_game(world: &mut World, resources: &mut Resources) {
	log::debug!("Clearing game...");
	let mut command_buffer = CommandBuffer::new(world);
	command_buffer.exec_mut(|_, resources| {
		resources.remove::<Client>();
		resources.remove::<GameTime>();
		resources.remove::<Quadtree>();
	});
	for &entity in <Entity>::query().filter(game_entities!()).iter(world) {
		command_buffer.remove(entity);
	}
	command_buffer.flush(world, resources);
}

pub fn create_quadtree(world: &World, resources: &Resources) -> Quadtree {
	let asset_storage = <Read<AssetStorage>>::fetch(resources);
	let map_dynamic = <&MapDynamic>::query()
		.iter(world)
		.next()
		.expect("No MapDynamic entity found");
	let map = asset_storage.get(&map_dynamic.map).unwrap();
	let mut quadtree = Quadtree::new(map.bbox);

	for (&entity, box_collider, transform) in
		<(Entity, &BoxCollider, &Transform)>::query().iter(world)
	{
		quadtree.insert(
			entity,
			&AABB2::from_radius(box_collider.radius).offset(transform.position.fixed_resize(0.0)),
		);
	}

	quadtree
}

pub fn new_game(map: &str, world: &mut World, resources: &mut Resources) {
	let map = map.to_ascii_lowercase();
	log::info!("Starting new game on map {}...", map);

	clear_game(world, resources);

	let result = || -> anyhow::Result<()> {
		resources.insert(GameTime::default());

		log::info!("Loading map...");
		let map_handle: AssetHandle<Map> = {
			let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
			asset_storage.load(&format!("{}.map", map))
		};
		spawn::spawn_map_entities(world, resources, &map_handle)?;

		let quadtree = create_quadtree(world, resources);
		resources.insert(quadtree);

		log::info!("Spawning entities...");
		let things = {
			let asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
			map::load::build_things(
				&asset_storage
					.source()
					.load(&RelativePath::new(&map).with_extension("things"))?,
			)?
		};
		spawn::spawn_things(things, world, resources)?;

		// Spawn player
		let entity = spawn::spawn_player(world, resources, 1)?;
		resources.insert(Client {
			entity: Some(entity),
			..Client::default()
		});

		{
			let (render_context, mut asset_storage) =
				<(Read<RenderContext>, Write<AssetStorage>)>::fetch_mut(resources);
			process_images(&render_context, &mut asset_storage);
		}

		Ok(())
	}();

	match result {
		Ok(_) => log::info!("Game started."),
		Err(err) => log::error!("{:?}", err),
	}
}

pub fn change_map(map: &str, world: &mut World, resources: &mut Resources) {
	if !resources.contains::<GameTime>() {
		log::error!("Can't change map, not currently in a game.");
		return;
	}

	// TODO continue existing game
	new_game(map, world, resources);
}

#[derive(Serialize, Deserialize)]
struct SavedResources {
	client: Client,
	game_time: GameTime,
}

pub fn save_game(name: &str, world: &mut World, resources: &mut Resources) {
	if !resources.contains::<GameTime>() {
		log::error!("Can't save game, not currently in a game.");
		return;
	}

	let name = format!("{}.sav", name);
	log::info!("Saving game to \"{}\"...", name);

	let (canon, client, game_time, registry, mut asset_storage) = <(
		Read<Canon>,
		Read<Client>,
		Read<GameTime>,
		Read<Registry<String>>,
		Write<AssetStorage>,
	)>::fetch_mut(resources);

	let saved_resources = SavedResources {
		client: client.clone(),
		game_time: *game_time,
	};

	let result = ASSET_SERIALIZER
		.set(&mut asset_storage, || -> anyhow::Result<()> {
			let mut file = BufWriter::new(
				File::create(&name)
					.with_context(|| format!("Couldn't open \"{}\" for writing", name))?,
			);
			let mut serializer = rmp_serde::encode::Serializer::new(&mut file);

			set_entity_serializer(&*canon, || saved_resources.serialize(&mut serializer))
				.context("Couldn't serialize resources")?;
			world
				.as_serializable(game_entities!(), &*registry, &*canon)
				.serialize(&mut serializer)
				.context("Couldn't serialize world")?;

			file.flush().context("Couldn't flush file")?;
			log::info!("Game saved.");
			Ok(())
		})
		.context("Couldn't save game");

	if let Err(err) = result {
		log::error!("{:?}", err);
	}
}

pub fn load_game(name: &str, world: &mut World, resources: &mut Resources) {
	clear_game(world, resources);

	let name = format!("{}.sav", name);
	log::info!("Loading game from \"{}\"...", name);

	let result = {
		let (canon, registry, mut asset_storage) =
			<(Read<Canon>, Read<Registry<String>>, Write<AssetStorage>)>::fetch_mut(resources);

		ASSET_SERIALIZER.set(&mut asset_storage, || -> anyhow::Result<_> {
			let mut file = BufReader::new(
				File::open(&name)
					.with_context(|| format!("Couldn't open \"{}\" for reading", name))?,
			);
			let mut deserializer = rmp_serde::decode::Deserializer::new(&mut file);
			let saved_resources =
				set_entity_serializer(&*canon, || SavedResources::deserialize(&mut deserializer))
					.context("Couldn't deserialize resources")?;
			registry
				.as_deserialize_into_world(world, &*canon)
				.deserialize(&mut deserializer)
				.context("Couldn't deserialize world")?;
			Ok(saved_resources)
		})
	}
	.context("Couldn't load game");

	match result {
		Ok(saved_resources) => {
			resources.insert(saved_resources.client);
			resources.insert(saved_resources.game_time);

			let quadtree = create_quadtree(world, resources);
			resources.insert(quadtree);

			log::info!("Game loaded.");
		}
		Err(err) => log::error!("{:?}", err),
	}
}

pub fn take_screenshot(resources: &Resources) {
	let result = || -> anyhow::Result<_> {
		let (draw_target, render_context) =
			<(Read<DrawTarget>, Read<RenderContext>)>::fetch(resources);
		let (buffer, dimensions, future) = draw_target.copy_to_cpu(&render_context)?;
		future.then_signal_fence_and_flush()?.wait(None)?;

		let filename = Local::now()
			.format("screenshot %Y-%m-%d %H-%M-%S %f.png")
			.to_string();
		let mut encoder = png::Encoder::new(
			BufWriter::new(File::create(&filename)?),
			dimensions[0],
			dimensions[1],
		);
		encoder.set_color(png::ColorType::RGBA);
		encoder.set_depth(png::BitDepth::Eight);
		let mut writer = encoder.write_header()?;
		writer.write_image_data(&buffer.read()?)?;
		log::info!("Screenshot saved to \"{}\"", filename);
		Ok(())
	}()
	.context("Couldn't take screenshot");

	if let Err(err) = result {
		log::error!("{:?}", err);
	}
}
