use crate::{
	common::assets::{AssetHandle, AssetStorage, ImportData},
	doom::{data::FONTS, image::Image},
};
use anyhow::Context;
use fnv::FnvHashMap;
use relative_path::RelativePath;

#[derive(Clone, Debug)]
pub struct Font {
	pub characters: FnvHashMap<char, AssetHandle<Image>>,
	pub spacing: FontSpacing,
}

#[derive(Clone, Copy, Debug)]
pub enum FontSpacing {
	FixedWidth { width: f32 },
	VariableWidth { space_width: f32 },
}

pub fn import_font(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let func = FONTS
		.get(path.as_str())
		.with_context(|| format!("Font \"{}\" not found", path))?;
	let template = func(asset_storage);
	Ok(Box::new(template))
}
