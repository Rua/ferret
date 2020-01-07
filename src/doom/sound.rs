use crate::assets::{AssetFormat, DataSource};
use byteorder::{ReadBytesExt, LE};
use std::{
	error::Error,
	io::{Cursor, Read},
};

pub struct DoomSound {
	sampling_rate: u16,
	samples: Vec<u8>,
}

#[derive(Clone, Copy)]
pub struct DoomSoundFormat;

impl AssetFormat for DoomSoundFormat {
	type Asset = DoomSound;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(name)?);
		let signature = reader.read_u16::<LE>()?;

		if signature != 3 {
			return Err(Box::from("No Doom sound file signature found"));
		}

		let sampling_rate = reader.read_u16::<LE>()?;
		let sample_count = reader.read_u32::<LE>()? as usize;

		let mut samples = vec![0u8; sample_count];
		reader.read_exact(&mut samples)?;

		// Remove padding bytes at start and end
		if samples.ends_with(&[samples[sample_count - 17]; 16]) {
			samples.drain(sample_count - 17..);
		}

		if samples.starts_with(&[samples[16]; 16]) {
			samples.drain(..16);
		}

		Ok(DoomSound {
			sampling_rate,
			samples,
		})
	}
}
