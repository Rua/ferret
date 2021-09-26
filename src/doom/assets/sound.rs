pub use crate::common::sound::{RawSound, Sound};
use crate::{
	common::assets::{AssetStorage, ImportData},
	doom::data::sounds::SOUNDS,
};
use anyhow::ensure;
use byteorder::{ReadBytesExt, LE};
use relative_path::RelativePath;
use std::io::{Cursor, Read};

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
			sounds: [asset_storage.load::<RawSound>(path.as_str())]
				.into_iter() // TODO change to into() once this is supported by SmallVec
				.collect(),
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
