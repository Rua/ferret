use crate::assets::{AssetFormat, DataSource};
use serde::Deserialize;
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

#[derive(Deserialize)]
struct Header {
	signature: u16,
	sampling_rate: u16,
	sample_count: u32,
}

impl AssetFormat for DoomSoundFormat {
	type Asset = DoomSound;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(name)?);
		let header: Header = bincode::deserialize_from(&mut data)?;

		if header.signature != 3 {
			return Err(Box::from("No Doom sound file signature found"));
		}

		let mut samples = vec![0u8; header.sample_count as usize];
		data.read_exact(&mut samples)?;

		// Remove padding bytes at start and end
		if samples.ends_with(&[samples[header.sample_count as usize - 17]; 16]) {
			samples.drain(header.sample_count as usize - 17..);
		}

		if samples.starts_with(&[samples[16]; 16]) {
			samples.drain(..16);
		}

		Ok(DoomSound {
			sampling_rate: header.sampling_rate,
			samples: samples,
		})
	}
}
