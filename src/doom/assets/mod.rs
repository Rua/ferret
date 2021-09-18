pub mod font;
pub mod image;
pub mod map;
pub mod sound;
pub mod sprite;
pub mod template;

use crate::{
	common::assets::{AssetStorage, ImportData},
	doom::{
		assets::{
			font::{import_font, import_hexfont, Font, HexFont},
			image::{import_palette, import_patch, Image, ImageData, Palette},
			map::{
				load::import_map,
				textures::{
					import_flat, import_pnames, import_textures, import_wall, PNames, Textures,
				},
				Map,
			},
			sound::{import_raw_sound, import_sound, RawSound, Sound},
			sprite::{import_sprite, Sprite},
			template::{
				import_ammo, import_entity, import_weapon, AmmoTemplate, EntityTemplate,
				WeaponTemplate,
			},
		},
		wad::WadLoader,
	},
};
use anyhow::bail;
use legion::Resources;
use relative_path::RelativePath;

pub fn register_assets(resources: &mut Resources) {
	let mut asset_storage = AssetStorage::new(import, WadLoader::new());
	asset_storage.add_storage::<AmmoTemplate>(false);
	asset_storage.add_storage::<EntityTemplate>(false);
	asset_storage.add_storage::<Font>(false);
	asset_storage.add_storage::<HexFont>(true);
	asset_storage.add_storage::<Image>(true);
	asset_storage.add_storage::<ImageData>(false);
	asset_storage.add_storage::<Map>(false);
	asset_storage.add_storage::<Palette>(false);
	asset_storage.add_storage::<PNames>(false);
	asset_storage.add_storage::<RawSound>(false);
	asset_storage.add_storage::<Sound>(false);
	asset_storage.add_storage::<Sprite>(false);
	asset_storage.add_storage::<Textures>(false);
	asset_storage.add_storage::<WeaponTemplate>(false);
	resources.insert(asset_storage);
}

pub fn import(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let function = match path.extension() {
		Some("ammo") => import_ammo,
		Some("entity") => import_entity,
		Some("flat") => import_flat,
		Some("font") => import_font,
		Some("hex") => import_hexfont,
		Some("map") => import_map,
		Some("palette") => import_palette,
		Some("patch") => import_patch,
		Some("sound") => import_sound,
		Some("rawsound") => import_raw_sound,
		Some("sprite") => import_sprite,
		Some("texture") => import_wall,
		Some("weapon") => import_weapon,
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
