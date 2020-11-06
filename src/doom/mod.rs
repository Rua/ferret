pub mod camera;
pub mod client;
pub mod components;
pub mod data;
pub mod door;
pub mod draw;
pub mod floor;
pub mod health;
pub mod image;
pub mod input;
pub mod light;
pub mod map;
pub mod physics;
pub mod plat;
pub mod sectormove;
pub mod sound;
pub mod sprite;
pub mod state;
pub mod switch;
pub mod template;
pub mod texture;
pub mod ui;
pub mod wad;

use crate::{
	common::{
		assets::{AssetHandle, AssetStorage, ImportData},
		frame::frame_state_system,
		quadtree::Quadtree,
		spawn::SpawnMergerHandlerSet,
		video::{AsBytes, DrawTarget, RenderContext},
	},
	doom::{
		camera::{camera_system, movement_bob_system},
		client::{
			player_attack_system, player_command_system, player_move_system, player_use_system,
			player_weapon_system, Client,
		},
		components::{SpawnPoint, Transform, TransformDef, Velocity, VelocityDef},
		data::FRAME_TIME,
		door::{door_active_system, door_switch_system, door_touch_system, door_use_system},
		draw::{
			finish_draw,
			map::draw_map,
			sprite::{draw_sprites, SpriteRender},
			start_draw,
			ui::draw_ui,
			world::draw_world,
			wsprite::{draw_weapon_sprites, WeaponSpriteRender},
		},
		floor::{floor_active_system, floor_switch_system, floor_touch_system},
		health::damage_system,
		image::{import_palette, import_patch, Image, ImageData, Palette},
		light::{light_flash_system, light_glow_system},
		map::{
			load::import_map,
			textures::{
				import_flat, import_pnames, import_textures, import_wall, PNames, Textures,
			},
			LinedefRef, Map, MapDynamic, SectorRef,
		},
		physics::physics_system,
		plat::{plat_active_system, plat_switch_system, plat_touch_system},
		sectormove::sector_move_system,
		sound::{
			import_raw_sound, import_sound, sound_playing_system, start_sound_system, RawSound,
			Sound,
		},
		sprite::{import_sprite, Sprite},
		state::{
			entity::{
				blocks_types_system, next_state_system, remove_entity_system, set_sprite_system,
			},
			state_system,
			weapon::{
				next_weapon_state_system, set_weapon_sprite_system, weapon_position_system,
				weapon_ready_system, weapon_refire_system,
			},
		},
		switch::switch_active_system,
		template::{EntityTemplate, EntityTemplateRef, EntityTemplateRefDef, WeaponTemplate},
		texture::{texture_animation_system, texture_scroll_system},
		ui::UiParams,
		wad::WadLoader,
	},
};
use anyhow::{bail, Context};
use clap::ArgMatches;
use crossbeam_channel::Sender;
use legion::{systems::ResourceSet, Read, Resources, Schedule, World, Write};
use nalgebra::Vector2;
use relative_path::RelativePath;
use std::{path::PathBuf, time::Instant};
use vulkano::{
	format::Format,
	image::{Dimensions, ImmutableImage},
	sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
};

pub fn import(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let function = match path.extension() {
		Some("flat") => import_flat,
		Some("map") => import_map,
		Some("palette") => import_palette,
		Some("patch") => import_patch,
		Some("sound") => import_sound,
		Some("rawsound") => import_raw_sound,
		Some("sprite") => import_sprite,
		Some("texture") => import_wall,
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
	resources.insert(Client::default());

	let mut loader = WadLoader::new();
	load_wads(&mut loader, &arg_matches)?;

	let wad_mode = match loader
		.wads()
		.next()
		.unwrap()
		.file_name()
		.unwrap()
		.to_str()
		.unwrap()
	{
		"doom1.wad" => WadMode::Doom1SW,
		"doom.wad" | "doomu.wad" => WadMode::Doom1,
		"doom2.wad" | "tnt.wad" | "plutonia.wad" => WadMode::Doom2,
		x => bail!("The IWAD \"{}\" is not recognised", x),
	};
	resources.insert(wad_mode);

	// Asset types
	let mut asset_storage = AssetStorage::new(import, loader);
	asset_storage.add_storage::<EntityTemplate>(false);
	asset_storage.add_storage::<WeaponTemplate>(false);
	asset_storage.add_storage::<Image>(true);
	asset_storage.add_storage::<ImageData>(false);
	asset_storage.add_storage::<Palette>(false);
	asset_storage.add_storage::<Map>(false);
	asset_storage.add_storage::<PNames>(false);
	asset_storage.add_storage::<Textures>(false);
	asset_storage.add_storage::<Sprite>(false);
	asset_storage.add_storage::<RawSound>(false);
	asset_storage.add_storage::<Sound>(false);
	resources.insert(asset_storage);

	// Component types
	{
		let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
		handler_set.register_clone::<SpawnPoint>();
		handler_set.register_spawn::<TransformDef, Transform>();
		handler_set.register_from::<VelocityDef, Velocity>();
		handler_set.register_spawn::<EntityTemplateRefDef, EntityTemplateRef>();
		handler_set.register_clone::<LinedefRef>();
		handler_set.register_clone::<MapDynamic>();
		handler_set.register_clone::<SectorRef>();
		handler_set.register_clone::<SpriteRender>();
		handler_set.register_clone::<WeaponSpriteRender>();
	}

	// Select map
	let map = if let Some(map) = arg_matches.value_of("map") {
		map
	} else {
		match *<Read<WadMode>>::fetch(resources) {
			WadMode::Doom1SW | WadMode::Doom1 => "E1M1",
			WadMode::Doom2 => "MAP01",
		}

		/*if wad == "doom.wad" || wad == "doom1.wad" || wad == "doomu.wad" {
			"E1M1"
		} else if wad == "doom2.wad" || wad == "tnt.wad" || wad == "plutonia.wad" {
			"MAP01"
		} else {
			bail!("No default map is known for this IWAD. Try specifying one with the \"-m\" option.")
		}*/
	};
	<Read<Sender<String>>>::fetch(resources)
		.send(format!("map {}", map))
		.ok();

	Ok(())
}

#[rustfmt::skip]
pub fn init_update_systems(resources: &mut Resources) -> anyhow::Result<Schedule> {
	Ok(Schedule::builder()
		.add_thread_local(player_command_system(resources)).flush()
		.add_thread_local(player_move_system(resources)).flush()
		.add_thread_local(player_weapon_system(resources)).flush()
		.add_thread_local(player_attack_system(resources)).flush()
		.add_thread_local(player_use_system(resources)).flush()
		.add_thread_local(physics_system(resources)).flush()
		.add_thread_local(movement_bob_system(resources)).flush()
		.add_thread_local(camera_system(resources)).flush()
		.add_thread_local(door_use_system(resources)).flush()
		.add_thread_local(door_switch_system(resources)).flush()
		.add_thread_local(door_touch_system(resources)).flush()
		.add_thread_local(floor_switch_system(resources)).flush()
		.add_thread_local(floor_touch_system(resources)).flush()
		.add_thread_local(plat_switch_system(resources)).flush()
		.add_thread_local(plat_touch_system(resources)).flush()
		.add_thread_local(sector_move_system(resources)).flush()
		.add_thread_local(door_active_system(resources)).flush()
		.add_thread_local(floor_active_system(resources)).flush()
		.add_thread_local(plat_active_system(resources)).flush()
		.add_thread_local(light_flash_system(resources)).flush()
		.add_thread_local(light_glow_system(resources)).flush()
		.add_thread_local(switch_active_system(resources)).flush()
		.add_thread_local(texture_animation_system(resources)).flush()
		.add_thread_local(texture_scroll_system(resources)).flush()
		.add_thread_local(damage_system(resources)).flush()
		.add_thread_local_fn({
			let actions = Schedule::builder()
				.add_system(blocks_types_system(resources))
				.add_system(next_state_system(resources))
				.add_system(remove_entity_system(resources))
				.add_system(set_sprite_system(resources))
				.add_system(next_weapon_state_system(resources))
				.add_system(set_weapon_sprite_system(resources))
				.add_system(weapon_position_system(resources))
				.add_system(weapon_ready_system(resources))
				.add_system(weapon_refire_system(resources))
				.build();

			state_system(resources, actions)
		})
		.add_thread_local(frame_state_system(FRAME_TIME)).flush()
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

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum WadMode {
	Doom1SW,
	Doom1,
	Doom2,
}

fn load_wads(loader: &mut WadLoader, arg_matches: &ArgMatches) -> anyhow::Result<()> {
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

pub fn load_map(name: &str, world: &mut World, resources: &mut Resources) -> anyhow::Result<()> {
	log::info!("Starting map {}...", name);
	let name_lower = name.to_ascii_lowercase();
	let start_time = Instant::now();

	log::info!("Loading entity data...");
	data::mobjs::load(resources);
	data::weapons::load(resources);
	data::sectors::load(resources);
	data::linedefs::load(resources);

	log::info!("Loading map...");
	let map_handle: AssetHandle<Map> = {
		let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
		asset_storage.load(&format!("{}.map", name_lower))
	};

	// Create quadtree
	let bbox = {
		let asset_storage = <Read<AssetStorage>>::fetch(resources);
		let map = asset_storage.get(&map_handle).unwrap();
		map.bbox.clone()
	};
	resources.insert(Quadtree::new(bbox));

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
		map::load::build_things(
			&asset_storage
				.source()
				.load(&RelativePath::new(&name_lower).with_extension("things"))?,
		)?
	};
	map::spawn::spawn_map_entities(world, resources, &map_handle)?;
	map::spawn::spawn_things(things, world, resources)?;

	// Spawn player
	let entity = map::spawn::spawn_player(world, resources, 1)?;
	<Write<Client>>::fetch_mut(resources).entity = Some(entity);

	log::debug!(
		"Loading took {} s",
		(Instant::now() - start_time).as_secs_f32()
	);

	Ok(())
}
