use crate::{
	assets::{Asset, DataSource},
	audio::Sound,
};
use byteorder::{ReadBytesExt, LE};
use std::{
	error::Error,
	io::{Cursor, Read},
	sync::Arc,
};

impl Asset for Sound {
	type Data = Arc<Self>;
	type Intermediate = Vec<u8>;
	const NAME: &'static str = "Sound";

	fn import(
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Intermediate, Box<dyn Error + Send + Sync>> {
		source.load(name)
	}
}

pub fn build_sound(data: Vec<u8>) -> Result<Arc<Sound>, Box<dyn Error + Send + Sync>> {
	let mut reader = Cursor::new(data);
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

	Ok(Arc::new(Sound {
		sampling_rate,
		samples,
	}))
}
