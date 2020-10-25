use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct SoundData {
	pub sounds: &'static [&'static str],
}

lazy_static! {
	pub static ref SOUNDS: Vec<SoundData> = vec![
		SoundData {
			sounds: &[
				"dspodth1.rawsound",
				"dspodth2.rawsound",
				"dspodth3.rawsound"
			],
		},
		SoundData {
			sounds: &["dsbgth1.rawsound", "dsbgth2.rawsound"],
		}
	];
}
