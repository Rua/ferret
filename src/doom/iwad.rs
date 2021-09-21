use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct IWADInfo {
	pub files: &'static [&'static str],
	pub name: &'static str,
	pub map: &'static str,
	pub weapons: &'static [&'static str],
	pub maps: HashMap<&'static str, MapInfo>,
}

#[derive(Clone, Debug)]
pub struct MapInfo {
	pub name: &'static str,
	pub sky: &'static str,
	pub music: &'static str,
	pub exit: Option<&'static str>,
	pub secret_exit: Option<&'static str>,
}
