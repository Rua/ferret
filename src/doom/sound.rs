use crate::{
	common::{
		assets::{AssetHandle, AssetStorage, ImportData},
		audio::{SoundController, SoundSource},
		geometry::Angle,
	},
	doom::{client::Client, components::Transform},
};
use anyhow::ensure;
use byteorder::{ReadBytesExt, LE};
use crossbeam_channel::Sender;
use legion::{
	systems::{CommandBuffer, ResourceSet},
	Entity, IntoQuery, Read, Resources, World, Write,
};
use nalgebra::Vector2;
use relative_path::RelativePath;
use rodio::Source;
use std::io::{Cursor, Read as IoRead};

pub use crate::common::audio::Sound;

pub fn import_sound(
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

	Ok(Box::new(Sound {
		sample_rate,
		data: data.into(),
	}))
}

pub fn sound_system() -> Box<dyn FnMut(&mut World, &mut Resources)> {
	Box::new(move |world, resources| {
		let (asset_storage, client, sound_sender, mut sound_queue) = <(
			Read<AssetStorage>,
			Read<Client>,
			Read<Sender<Box<dyn Source<Item = f32> + Send>>>,
			Write<Vec<(AssetHandle<Sound>, Entity)>>,
		)>::fetch_mut(resources);

		let mut command_buffer = CommandBuffer::new(world);

		{
			let client_transform = *<&Transform>::query()
				.get(world, client.entity.unwrap())
				.unwrap();

			// Play new sounds
			for (handle, entity) in sound_queue.drain(..) {
				let sound = asset_storage.get(&handle).unwrap();
				let (controller, source) = SoundController::new(SoundSource::new(&sound));
				let (transform, sound_playing) = <(&Transform, Option<&mut SoundPlaying>)>::query()
					.get_mut(world, entity)
					.unwrap();

				// Set distance falloff and stereo panning
				let volumes = calculate_volumes(&client_transform, transform);
				controller.set_volumes(volumes.into());

				// Stop old sound on this entity, if any
				if let Some(mut sound_playing) = sound_playing {
					sound_playing.controller.stop();
					sound_playing.controller = controller;
				} else {
					command_buffer.add_component(entity, SoundPlaying { controller });
				}

				sound_sender.send(Box::from(source.convert_samples())).ok();
			}

			// Update currently playing sounds
			for (entity, transform, sound_playing) in
				<(Entity, &Transform, &mut SoundPlaying)>::query().iter_mut(world)
			{
				if sound_playing.controller.is_done() {
					command_buffer.remove_component::<SoundPlaying>(*entity);
					continue;
				}

				// Set distance falloff and stereo panning
				let volumes = calculate_volumes(&client_transform, transform);
				sound_playing.controller.set_volumes(volumes.into());
			}
		}

		command_buffer.flush(world);
	})
}

fn calculate_volumes(client_transform: &Transform, entity_transform: &Transform) -> Vector2<f32> {
	let to_entity_vec = entity_transform.position - client_transform.position;

	// Calculate distance falloff
	const MIN_DIST: f32 = 160.0;
	const MAX_DIST: f32 = 1200.0;

	let distance = to_entity_vec.norm();
	let distance_factor = if distance < MIN_DIST {
		1.0
	} else if distance > MAX_DIST {
		0.0
	} else {
		(MAX_DIST - distance) / (MAX_DIST - MIN_DIST)
	};

	// Calculate stereo panning
	const MAX_PAN: f32 = 0.75;

	let angle = client_transform.rotation[2]
		- Angle::from_radians(f64::atan2(to_entity_vec[1] as f64, to_entity_vec[0] as f64));
	let pan = MAX_PAN * angle.sin() as f32;
	let volumes = Vector2::new(
		1.0 - 0.25 * (pan + 1.0).powi(2),
		1.0 - 0.25 * (pan - 1.0).powi(2),
	);

	// Final result
	volumes * distance_factor
}

pub struct SoundPlaying {
	pub controller: SoundController,
}
