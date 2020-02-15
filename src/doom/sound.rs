use crate::{
	assets::{Asset, AssetHandle, AssetStorage, DataSource},
	audio::{Sound, SoundController, SoundSource},
	doom::{
		client::Client,
		components::{SoundPlaying, Transform},
	},
	geometry::Angle,
};
use byteorder::{ReadBytesExt, LE};
use nalgebra::Vector2;
use rodio::Source;
use specs::{
	Entities, Entity, Join, ReadExpect, ReadStorage, RunNow, World, WriteExpect, WriteStorage,
};
use std::{
	error::Error,
	io::{Cursor, Read},
};

impl Asset for Sound {
	type Data = Self;
	type Intermediate = Vec<u8>;
	const NAME: &'static str = "Sound";

	fn import(
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Intermediate, Box<dyn Error + Send + Sync>> {
		source.load(name)
	}
}

pub fn build_sound(data: Vec<u8>) -> Result<Sound, Box<dyn Error + Send + Sync>> {
	let mut reader = Cursor::new(data);
	let signature = reader.read_u16::<LE>()?;

	if signature != 3 {
		return Err(Box::from("No Doom sound file signature found"));
	}

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

	Ok(Sound {
		sample_rate,
		data: data.into(),
	})
}

#[derive(Default)]
pub struct SoundSystem;

impl<'a> RunNow<'a> for SoundSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (
			entities,
			client,
			sound_device,
			sound_storage,
			transform_component,
			mut sound_queue,
			mut sound_playing_component,
		) = world.system_data::<(
			Entities,
			ReadExpect<Client>,
			ReadExpect<rodio::Device>,
			ReadExpect<AssetStorage<Sound>>,
			ReadStorage<Transform>,
			WriteExpect<Vec<(AssetHandle<Sound>, Entity)>>,
			WriteStorage<SoundPlaying>,
		)>();

		let mut to_remove = Vec::new();

		// Update currently playing sounds
		let client_transform = transform_component.get(client.entity.unwrap()).unwrap();

		for (entity, transform, sound_playing) in (
			&entities,
			&transform_component,
			&mut sound_playing_component,
		)
			.join()
		{
			if sound_playing.controller.is_done() {
				to_remove.push(entity);
				continue;
			}

			// Set distance falloff and stereo panning
			let volumes = calculate_volumes(client_transform, transform);
			sound_playing.controller.set_volumes(volumes.into());
		}

		// Remove finished sounds
		for entity in to_remove {
			sound_playing_component.remove(entity);
		}

		// Play new sounds
		for (handle, entity) in sound_queue.drain(..) {
			let sound = sound_storage.get(&handle).unwrap();
			let (controller, source) = SoundController::new(SoundSource::new(&sound));

			// Set distance falloff and stereo panning
			let transform = transform_component.get(entity).unwrap();
			let volumes = calculate_volumes(client_transform, transform);
			controller.set_volumes(volumes.into());

			// Insert component
			if let Ok(Some(sound_playing)) =
				sound_playing_component.insert(entity, SoundPlaying { controller })
			{
				// Stop old sound on this entity, if any
				sound_playing.controller.stop();
			}

			rodio::play_raw(&sound_device, source.convert_samples());
		}
	}
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
