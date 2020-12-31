use crate::{
	common::{
		assets::{AssetHandle, AssetStorage, ImportData},
		geometry::Angle,
		sound::{SoundController, SoundSource},
		spawn::SpawnMergerHandlerSet,
	},
	doom::{client::Client, components::Transform, data::sounds::SOUNDS},
};
use anyhow::ensure;
use byteorder::{ReadBytesExt, LE};
use crossbeam_channel::Sender;
use legion::{
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Registry, Resources, SystemBuilder, Write,
};
use nalgebra::Vector2;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use relative_path::RelativePath;
use rodio::Source;
use std::io::{Cursor, Read as IoRead};

pub use crate::common::sound::{RawSound, Sound};

pub fn import_sound(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let path = path.with_extension("rawsound");

	let sound = if let Some(sound_data) = SOUNDS
		.iter()
		.find(|sound_data| sound_data.sounds.contains(&path.as_str()))
	{
		Sound {
			sounds: sound_data
				.sounds
				.iter()
				.map(|sound| asset_storage.load::<RawSound>(sound))
				.collect(),
			global: sound_data.global,
		}
	} else {
		Sound {
			sounds: std::iter::once(asset_storage.load::<RawSound>(path.as_str())).collect(),
			global: false,
		}
	};

	Ok(Box::new(sound))
}

pub fn import_raw_sound(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let mut reader = Cursor::new(asset_storage.source().load(path)?);
	let signature = reader.read_u16::<LE>()?;

	ensure!(signature == 3, "No Doom sound file signature found");

	let sample_rate = reader.read_u16::<LE>()? as u32;
	let sample_count = reader.read_u32::<LE>()? as usize;

	// Read in the samples
	let mut data = vec![0u8; sample_count - 32];
	let mut padding = [0u8; 16];
	reader.read_exact(&mut padding)?;
	reader.read_exact(&mut data)?;
	reader.read_exact(&mut padding)?;

	// Convert to i16
	let data = data
		.into_iter()
		.map(|x| ((x ^ 0x80) as i16) << 8)
		.collect::<Vec<i16>>();

	Ok(Box::new(RawSound {
		sample_rate,
		data: data.into(),
	}))
}

#[derive(Clone, Debug)]
pub struct StartSound(pub AssetHandle<Sound>);

#[derive(Clone, Debug)]
pub struct SoundPlaying {
	pub controller: SoundController,
}

type SoundSender = Sender<Box<dyn Source<Item = f32> + Send>>;

pub fn start_sound_system(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);
	handler_set.register_clone::<StartSound>();
	let mut rng = Pcg64Mcg::from_entropy();

	SystemBuilder::new("start_sound_system")
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.read_resource::<SoundSender>()
		.with_query(<&Transform>::query())
		.with_query(<(Entity, &Entity, &StartSound)>::query())
		.with_query(<(&Transform, Option<&mut SoundPlaying>)>::query())
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, client, sound_sender) = resources;
			let client_transform = *queries.0.get(world, client.entity.unwrap()).unwrap();
			let (world1, mut world) = world.split_for_query(&queries.1);

			for (&entity, &target, start_sound) in queries.1.iter(&world1) {
				command_buffer.remove(entity);

				let sound = asset_storage.get(&start_sound.0).unwrap();
				let index = match sound.sounds.len() {
					0 => continue,
					1 => 0,
					len => rng.gen_range(0..len),
				};
				let raw_sound = asset_storage.get(&sound.sounds[index]).unwrap();

				let (controller, source) = SoundController::new(SoundSource::new(&raw_sound));
				let (transform, sound_playing) = queries.2.get_mut(&mut world, target).unwrap();

				// Set distance falloff and stereo panning
				let volumes = calculate_volumes(&client_transform, transform);
				controller.set_volumes(volumes.into());

				// Stop old sound on this entity, if any
				if let Some(mut sound_playing) = sound_playing {
					sound_playing.controller.stop();
					sound_playing.controller = controller;
				} else {
					command_buffer.add_component(target, SoundPlaying { controller });
				}

				sound_sender.send(Box::from(source.convert_samples())).ok();
			}
		})
}

pub fn sound_playing_system(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);
	handler_set.register_clone::<SoundPlaying>();

	SystemBuilder::new("sound_playing_system")
		.read_resource::<Client>()
		.with_query(<&Transform>::query())
		.with_query(<(Entity, &Transform, &mut SoundPlaying)>::query())
		.build(move |command_buffer, world, resources, queries| {
			let client = resources;
			let client_transform = *queries.0.get(world, client.entity.unwrap()).unwrap();

			for (&entity, transform, sound_playing) in queries.1.iter_mut(world) {
				if sound_playing.controller.is_done() {
					command_buffer.remove_component::<SoundPlaying>(entity);
					continue;
				}

				// Set distance falloff and stereo panning
				let volumes = calculate_volumes(&client_transform, transform);
				sound_playing.controller.set_volumes(volumes.into());
			}
		})
}

fn calculate_volumes(client_transform: &Transform, entity_transform: &Transform) -> Vector2<f32> {
	let to_entity_vec = entity_transform.position - client_transform.position;
	let distance = to_entity_vec.norm();

	// Calculate distance falloff
	const MIN_DIST: f32 = 160.0;
	const MAX_DIST: f32 = 1200.0;

	let distance_factor = if distance < MIN_DIST {
		1.0
	} else if distance > MAX_DIST {
		0.0
	} else {
		(MAX_DIST - distance) / (MAX_DIST - MIN_DIST)
	};

	// Calculate stereo panning
	const MAX_PAN: f32 = 0.75;

	let pan = if distance < 1.0 {
		0.0
	} else {
		let angle = client_transform.rotation[2]
			- Angle::from_radians(f64::atan2(to_entity_vec[1] as f64, to_entity_vec[0] as f64));
		MAX_PAN * angle.sin() as f32
	};

	let volumes = Vector2::new(
		1.0 - 0.25 * (pan + 1.0).powi(2),
		1.0 - 0.25 * (pan - 1.0).powi(2),
	);

	// Final result
	volumes * distance_factor
}
