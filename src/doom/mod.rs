//! Items specific to the implementation of Doom.

pub mod camera;
pub mod client;
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
		quadtree::Quadtree,
		spawn::SpawnMergerHandlerSet,
		time::{increment_game_time, DeltaTime, GameTime},
		video::{AsBytes, DrawTarget, RenderContext},
	},
	doom::{
		camera::{camera_system, movement_bob_system},
		client::{
			player_command_system, player_move_system, player_touch, player_use_system,
			player_weapon_system, Client,
		},
		components::{RandomTransformDef, SpawnPoint, Transform, TransformDef},
		data::{iwads::IWADINFO, FRAME_TIME},
		door::{door_active, door_linedef_touch, door_switch_use, door_use},
		draw::{
			finish_draw,
			map::draw_map,
			sprite::{draw_sprites, SpriteRender},
			start_draw,
			ui::draw_ui,
			world::draw_world,
			wsprite::{draw_weapon_sprites, WeaponSpriteRender},
		},
		exit::exit_switch_use,
		floor::{floor_active, floor_linedef_touch, floor_switch_use},
		health::apply_damage,
		hud::{arms_stat, health_stat},
		image::{import_palette, import_patch, Image, ImageData, Palette},
		light::{light_flash_system, light_glow_system},
		map::{
			load::import_map,
			textures::{
				import_flat, import_pnames, import_textures, import_wall, PNames, Textures,
			},
			LinedefRef, LinedefRefDef, Map, MapDynamic, SectorRef, SectorRefDef,
		},
		physics::{physics, BoxCollider},
		plat::{plat_active, plat_linedef_touch, plat_switch_use},
		sectormove::sector_move_system,
		sound::{
			import_raw_sound, import_sound, sound_playing_system, start_sound_system, RawSound,
			Sound,
		},
		sprite::{import_sprite, Sprite},
		state::{
			entity::{
				next_entity_state, remove_entity, set_blocks_types, set_entity_sprite,
				set_solid_type,
			},
			state,
			weapon::{
				extra_light, line_attack, next_weapon_state, projectile_touch, radius_attack,
				set_weapon_sprite, set_weapon_state, spawn_projectile, spray_attack,
				weapon_position, weapon_ready, weapon_refire,
			},
		},
		switch::switch_active_system,
		template::{
			import_entity, import_weapon, EntityTemplate, EntityTemplateRef, EntityTemplateRefDef,
			WeaponTemplate,
		},
		texture::{texture_animation_system, texture_scroll_system},
		ui::{import_font, Font, UiImage, UiParams, UiTransform},
		wad::{IWADInfo, WadLoader},
	},
};
use anyhow::{bail, Context};
use clap::ArgMatches;
use crossbeam_channel::Sender;
use legion::{
	component,
	serialize::{set_entity_serializer, Canon},
	systems::{CommandBuffer, ResourceSet},
	Entity, IntoQuery, Read, Registry, Resources, Schedule, World, Write,
};
use nalgebra::Vector2;
use relative_path::RelativePath;
use serde::{de::DeserializeSeed, Deserialize, Serialize};
use std::{
	fs::File,
	io::{BufReader, BufWriter, Write as IOWrite},
	path::Path,
	time::Instant,
};
use vulkano::{
	format::Format,
	image::{Dimensions, ImmutableImage, MipmapsCount},
	sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
};

pub fn import(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let function = match path.extension() {
		Some("entity") => import_entity,
		Some("flat") => import_flat,
		Some("font") => import_font,
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

	resources.insert(data::get_bindings());

	// Asset types
	let mut asset_storage = AssetStorage::new(import, WadLoader::new());
	asset_storage.add_storage::<EntityTemplate>(false);
	asset_storage.add_storage::<Font>(false);
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

	// Component types
	{
		let (mut handler_set, mut registry) =
			<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

		registry.register::<SpawnPoint>("SpawnPoint".into());
		handler_set.register_clone::<SpawnPoint>();

		registry.register::<Transform>("Transform".into());
		handler_set.register_spawn::<TransformDef, Transform>();
		handler_set.register_spawn::<RandomTransformDef, Transform>();

		registry.register::<EntityTemplateRef>("EntityTemplateRef".into());
		handler_set.register_spawn::<EntityTemplateRefDef, EntityTemplateRef>();

		registry.register::<MapDynamic>("MapDynamic".into());
		handler_set.register_clone::<MapDynamic>();

		registry.register::<LinedefRef>("LinedefRef".into());
		handler_set.register_clone::<LinedefRef>();
		handler_set.register_spawn::<LinedefRefDef, LinedefRef>();

		registry.register::<SectorRef>("SectorRef".into());
		handler_set.register_clone::<SectorRef>();
		handler_set.register_spawn::<SectorRefDef, SectorRef>();

		registry.register::<SpriteRender>("SpriteRender".into());
		handler_set.register_clone::<SpriteRender>();

		registry.register::<WeaponSpriteRender>("WeaponSpriteRender".into());
		handler_set.register_clone::<WeaponSpriteRender>();

		handler_set.register_clone::<UiTransform>();

		handler_set.register_clone::<UiImage>();
	}

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

#[rustfmt::skip]
pub fn init_update_systems(resources: &mut Resources) -> anyhow::Result<Schedule> {
	Ok(Schedule::builder()
		.add_thread_local(player_command_system(resources)).flush()
		.add_thread_local(player_move_system(resources)).flush()
		.add_thread_local(player_weapon_system(resources)).flush()
		.add_thread_local(player_use_system(resources)).flush()

		.add_thread_local(physics(resources)).flush()
		.add_thread_local(door_linedef_touch(resources)).flush()
		.add_thread_local(floor_linedef_touch(resources)).flush()
		.add_thread_local(plat_linedef_touch(resources)).flush()
		.add_thread_local(player_touch(resources)).flush()
		.add_thread_local(projectile_touch(resources)).flush()

		.add_thread_local(movement_bob_system(resources)).flush()
		.add_thread_local(camera_system(resources)).flush()
		.add_thread_local(door_use(resources)).flush()
		.add_thread_local(door_switch_use(resources)).flush()
		.add_thread_local(exit_switch_use(resources)).flush()
		.add_thread_local(floor_switch_use(resources)).flush()
		.add_thread_local(plat_switch_use(resources)).flush()
		.add_thread_local(sector_move_system(resources)).flush()
		.add_thread_local(door_active(resources)).flush()
		.add_thread_local(floor_active(resources)).flush()
		.add_thread_local(plat_active(resources)).flush()
		.add_thread_local(light_flash_system(resources)).flush()
		.add_thread_local(light_glow_system(resources)).flush()
		.add_thread_local(switch_active_system(resources)).flush()
		.add_thread_local(texture_animation_system(resources)).flush()
		.add_thread_local(texture_scroll_system(resources)).flush()
		.add_thread_local(apply_damage(resources)).flush()
		.add_thread_local_fn({
			let actions = Schedule::builder()
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
				.build();

			state(resources, actions)
		})
		.add_thread_local(health_stat(resources)).flush()
		.add_thread_local(arms_stat(resources)).flush()
		.add_thread_local(increment_game_time()).flush()
		.build())
}

pub fn init_draw_systems(resources: &mut Resources) -> anyhow::Result<Schedule> {
	Ok(Schedule::builder()
		.add_thread_local(start_draw(resources)?)
		.add_thread_local(draw_world(resources)?)
		.add_thread_local(draw_map(resources)?)
		.add_thread_local(draw_sprites(resources)?)
		.add_thread_local(draw_weapon_sprites(resources)?)
		.add_thread_local(draw_ui(resources)?)
		.add_thread_local(finish_draw(resources)?)
		.build())
}

#[rustfmt::skip]
pub fn init_sound_systems(resources: &mut Resources) -> anyhow::Result<Schedule> {
	Ok(Schedule::builder()
		.add_thread_local(start_sound_system(resources)).flush()
		.add_thread_local(sound_playing_system(resources)).flush()
		.build())
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
	log::info!("Clearing game...");
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

pub fn new_game(map: &str, world: &mut World, resources: &mut Resources) -> anyhow::Result<()> {
	clear_game(world, resources);

	log::info!("Starting map {}...", map);
	let map_lower = map.to_ascii_lowercase();
	let start_time = Instant::now();
	resources.insert(GameTime::default());

	log::info!("Loading map...");
	let map_handle: AssetHandle<Map> = {
		let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
		asset_storage.load(&format!("{}.map", map_lower))
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
				.load(&RelativePath::new(&map_lower).with_extension("things"))?,
		)?
	};
	spawn::spawn_things(things, world, resources)?;

	// Spawn player
	let entity = spawn::spawn_player(world, resources, 1)?;
	resources.insert(Client {
		entity: Some(entity),
		..Client::default()
	});

	log::info!("Processing assets...");
	{
		let (render_context, mut asset_storage) =
			<(Read<RenderContext>, Write<AssetStorage>)>::fetch_mut(resources);

		// Palette
		let palette_handle: AssetHandle<Palette> = asset_storage.load("playpal.palette");

		// Images
		asset_storage.process::<Image, _>(|data, asset_storage| {
			let image_data: ImageData = *data.downcast().ok().unwrap();
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
				MipmapsCount::One,
				Format::R8G8B8A8Unorm,
				render_context.queues().graphics.clone(),
			)?;

			Ok(crate::doom::image::Image {
				image,
				offset: Vector2::new(image_data.offset[0] as f32, image_data.offset[1] as f32),
			})
		});
	}

	log::debug!(
		"Loading took {} s",
		(Instant::now() - start_time).as_secs_f32()
	);

	Ok(())
}

pub fn change_map(map: &str, world: &mut World, resources: &mut Resources) -> anyhow::Result<()> {
	// TODO continue existing game
	new_game(map, world, resources)
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

	let result = ASSET_SERIALIZER.set(&mut asset_storage, || -> anyhow::Result<()> {
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
		Ok(())
	});

	match result {
		Ok(_) => log::info!("Save complete."),
		Err(err) => log::error!("{:?}", err),
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
	};

	match result {
		Ok(saved_resources) => {
			resources.insert(saved_resources.client);
			resources.insert(saved_resources.game_time);

			let quadtree = create_quadtree(world, resources);
			resources.insert(quadtree);

			log::info!("Load complete.");
		}
		Err(err) => log::error!("{:?}", err),
	}
}
