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

pub struct DoomSoundFormat;

impl AssetFormat for DoomSoundFormat {
	type Asset = DoomSound;

	fn import(
		&self,
		name: &str,
		source: &mut impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error>> {
		let mut data = Cursor::new(source.load(name)?);
		let signature = data.read_u16::<LE>()?;

		if signature != 3 {
			return Err(Box::from("No Doom sound file signature found"));
		}

		let sampling_rate = data.read_u16::<LE>()?;
		let num_samples = data.read_u32::<LE>()? as usize;
		let mut samples = vec![0u8; num_samples as usize];

		data.read_exact(&mut samples)?;

		// Remove padding bytes at start and end
		if samples.ends_with(&[samples[num_samples - 17]; 16]) {
			samples.drain(num_samples - 17..);
		}

		if samples.starts_with(&[samples[16]; 16]) {
			samples.drain(..16);
		}

		Ok(DoomSound {
			sampling_rate: sampling_rate,
			samples: samples,
		})
	}
}
