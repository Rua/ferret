use crate::{
	assets::{Asset, AssetHandle, AssetStorage, DataSource},
	audio::{Sound, SoundController, SoundSource},
	doom::components::SoundPlaying,
};
use byteorder::{ReadBytesExt, LE};
use rodio::Source;
use specs::{Entities, Entity, Join, ReadExpect, RunNow, World, WriteExpect, WriteStorage};
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

	let mut data = vec![0u8; sample_count - 32];
	let mut padding = [0u8; 16];
	reader.read_exact(&mut padding)?;
	reader.read_exact(&mut data)?;
	reader.read_exact(&mut padding)?;

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
		let (entities, sound_device, sound_storage, mut sound_queue, mut sound_playing_component) =
			world.system_data::<(
				Entities,
				ReadExpect<rodio::Device>,
				ReadExpect<AssetStorage<Sound>>,
				WriteExpect<Vec<(AssetHandle<Sound>, Entity)>>,
				WriteStorage<SoundPlaying>,
			)>();

		let mut to_remove = Vec::new();

		// Update currently playing sounds
		for (entity, sound_playing) in (&entities, &mut sound_playing_component).join() {
			if sound_playing.controller.is_done() {
				to_remove.push(entity);
			}
		}

		// Remove finished sounds
		for entity in to_remove {
			sound_playing_component.remove(entity);
		}

		// Play new sounds
		for (handle, entity) in sound_queue.drain(..) {
			let sound = sound_storage.get(&handle).unwrap();
			let (controller, source) = SoundController::new(SoundSource::new(&sound));

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
