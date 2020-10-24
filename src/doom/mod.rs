pub mod camera;
pub mod client;
pub mod components;
pub mod data;
pub mod door;
pub mod entitytemplate;
pub mod floor;
pub mod image;
pub mod input;
pub mod light;
pub mod map;
pub mod physics;
pub mod plat;
pub mod psprite;
pub mod render;
pub mod sectormove;
pub mod sound;
pub mod sprite;
pub mod state;
pub mod switch;
pub mod texture;
pub mod ui;
pub mod wad;

use crate::{
	common::assets::{AssetStorage, ImportData},
	doom::{
		image::{import_palette, import_patch},
		map::{
			load::import_map,
			textures::{import_flat, import_pnames, import_textures, import_wall},
		},
		sound::{import_raw_sound, import_sound},
		sprite::import_sprite,
	},
};
use anyhow::bail;
use relative_path::RelativePath;

pub fn import(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let function = match path.extension() {
		Some("flat") => import_flat,
		Some("map") => import_map,
		Some("palette") => import_palette,
		Some("patch") => import_patch,
		Some("sound") => import_sound,
		Some("rawsound") => import_raw_sound,
		Some("sprite") => import_sprite,
		Some("texture") => import_wall,
		Some(ext) => bail!("Unsupported file extension: {}", ext),
		None => match path.file_name() {
			Some("pnames") => import_pnames,
			Some("texture1") | Some("texture2") => import_textures,
			Some(name) => bail!("File has no extension: {}", name),
			None => bail!("Path ends in '..'"),
		},
	};

	function(path, asset_storage)
}
