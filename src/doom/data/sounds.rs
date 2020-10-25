use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct SoundData {
	pub sounds: &'static [&'static str],
	pub global: bool,
}

lazy_static! {
	pub static ref SOUNDS: Vec<SoundData> = vec![
		SoundData {
			sounds: &[
				"dspodth1.rawsound",
				"dspodth2.rawsound",
				"dspodth3.rawsound",
			],
			global: false,
		},
		SoundData {
			sounds: &["dsbgth1.rawsound", "dsbgth2.rawsound"],
			global: false,
		},
		SoundData {
			sounds: &["dscybdth.rawsound"],
			global: true,
		},
		SoundData {
			sounds: &["dsspidth.rawsound"],
			global: true,
		},
		SoundData {
			sounds: &["dsbosdth.rawsound"],
			global: true,
		},
		SoundData {
			sounds: &["dsbospn.rawsound"],
			global: true,
		},
	];
}
