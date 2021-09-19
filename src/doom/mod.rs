//! Items specific to the implementation of Doom.

pub mod assets;
pub mod commands;
pub mod components;
pub mod data;
pub mod draw;
pub mod game;
pub mod input;
pub mod sound;
pub mod ui;
pub mod wad;

use crate::{
	common::{
		assets::{AssetStorage, ASSET_SERIALIZER},
		console::check_resize_console,
		time::DeltaTime,
		video::{DrawTarget, RenderContext},
	},
	doom::{
		assets::register_assets,
		components::register_components,
		data::{iwads::IWADINFO, FRAME_TIME},
		draw::{check_recreate, draw, FramebufferResizeEvent},
		sound::{start_sound, update_sound, StartSoundEvent},
		ui::UiParams,
		wad::{IWADInfo, WadLoader},
	},
};
use anyhow::{bail, Context};
use chrono::Local;
use clap::ArgMatches;
use crossbeam_channel::Sender;
use legion::{
	component,
	storage::Component,
	systems::{Builder, ResourceSet, Runnable},
	Entity, IntoQuery, Read, Resources, SystemBuilder, Write,
};
use std::{any::type_name, fs::File, io::BufWriter, path::Path};
use vulkano::{
	sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
	sync::GpuFuture,
};

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

	register_assets(resources);
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

	Ok(())
}

pub fn add_output_systems(builder: &mut Builder, resources: &mut Resources) -> anyhow::Result<()> {
	#[rustfmt::skip]
	builder
		.add_system(check_recreate())
		.flush()
		.add_system(check_resize_console())
		.add_system(clear_event::<FramebufferResizeEvent>())

		.add_thread_local_fn(draw(resources)?)
		.add_system(start_sound(resources))
		.add_system(clear_event::<StartSoundEvent>())
		.add_system(update_sound(resources));

	Ok(())
}

fn load_wads(resources: &mut Resources, arg_matches: &ArgMatches) -> anyhow::Result<()> {
	// Determine IWAD
	let mut iter = IWADINFO
		.iter()
		.enumerate()
		.flat_map(|(i, info)| info.files.iter().map(move |file| (i, *file)));

	let mut dir = dirs::data_dir().unwrap_or_default();
	dir.push("ferret");

	let (index, iwad_path) = if let Some(iwad) = arg_matches.value_of("iwad") {
		let iwad_path = dir.join(iwad);
		let iwad_file: &str = iwad_path
			.file_name()
			.with_context(|| format!("IWAD path \"{}\" does not contain a file name.", iwad))?
			.to_str()
			.unwrap();

		if let Some((index, _)) = iter.find(|(_, file)| *file == iwad_file) {
			(index, iwad_path)
		} else {
			bail!("File \"{}\" is not a recognised game IWAD.", iwad);
		}
	} else {
		iter.map(|(i, file)| (i, dir.join(file)))
			.find(|(_, file)| file.is_file())
			.with_context(|| {
				format!("No recognised game IWAD found in \"{}\". Try specifying one with the \"-i\" command line option.", dir.display())
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

		add_with_gwa(&iwad_path)?;

		if let Some(iter) = arg_matches.values_of("PWADS") {
			for pwad in iter.map(|file| dir.join(file)) {
				add_with_gwa(&pwad)?;
			}
		}
	}

	Ok(())
}

pub fn take_screenshot(resources: &Resources) {
	let result = || -> anyhow::Result<_> {
		let (draw_target, render_context) =
			<(Read<DrawTarget>, Read<RenderContext>)>::fetch(resources);
		let (buffer, dimensions, future) = draw_target.copy_to_cpu(&render_context)?;
		future.then_signal_fence_and_flush()?.wait(None)?;

		let mut path = dirs::picture_dir().unwrap_or_default();
		path.push(
			Local::now()
				.format("Ferret %Y-%m-%d %H-%M-%S %f.png")
				.to_string(),
		);

		let mut encoder = png::Encoder::new(
			BufWriter::new(File::create(&path)?),
			dimensions[0],
			dimensions[1],
		);
		encoder.set_color(png::ColorType::Rgba);
		encoder.set_depth(png::BitDepth::Eight);
		let mut writer = encoder.write_header()?;
		writer.write_image_data(&buffer.read()?)?;
		log::info!("Screenshot saved to \"{}\"", path.display());
		Ok(())
	}()
	.context("Couldn't take screenshot");

	if let Err(err) = result {
		log::error!("{:?}", err);
	}
}

pub fn clear_event<E: Component>() -> impl Runnable {
	SystemBuilder::new(format!("clear_event::<{}>", type_name::<E>()))
		.with_query(<Entity>::query().filter(component::<E>()))
		.build(move |command_buffer, world, _resources, query| {
			for &entity in query.iter(world) {
				command_buffer.remove(entity);
			}
		})
}
