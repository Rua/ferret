use std::path::PathBuf;

#[inline]
pub fn config_dir() -> PathBuf {
	let mut path = dirs::config_dir().unwrap_or_default();
	path.push("ferret");
	path
}

#[inline]
pub fn data_dir() -> PathBuf {
	let mut path = dirs::data_dir().unwrap_or_default();
	path.push("ferret");
	path
}

#[inline]
pub fn screenshot_dir() -> PathBuf {
	dirs::picture_dir().unwrap_or_default()
}
