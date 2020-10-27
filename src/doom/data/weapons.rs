use crate::{
	common::assets::AssetStorage,
	doom::{
		data::FRAME_TIME,
		sprite::SpriteRender,
		state::{StateName, WeaponStateInfo},
		template::WeaponTemplate,
	},
	WadMode,
};
use legion::{systems::ResourceSet, Read, Resources, Write};
use std::{collections::HashMap, default::Default};

#[rustfmt::skip]
pub fn load(resources: &mut Resources) {
	let (wad_mode, mut asset_storage) = <(Read<WadMode>, Write<AssetStorage>)>::fetch_mut(resources);
	let wad_mode = *wad_mode;

	let template = WeaponTemplate {
		name: Some("fist"),
		states: {
			let mut states = HashMap::with_capacity(8);
			states.insert(StateName::from("up").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pung.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pung.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pung.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pung.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pung.sprite"), frame: 2, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(5 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pung.sprite"), frame: 3, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pung.sprite"), frame: 2, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(5 * FRAME_TIME),
					next: Some((StateName::from("ready").unwrap(), 0)),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pung.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	};
	asset_storage.insert_with_name("fist", template);

	let template = WeaponTemplate {
		name: Some("pistol"),
		states: {
			let mut states = HashMap::with_capacity(9);
			states.insert(StateName::from("up").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pisg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pisg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pisg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pisg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(6 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pisg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pisg.sprite"), frame: 2, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(5 * FRAME_TIME),
					next: Some((StateName::from("ready").unwrap(), 0)),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pisg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				WeaponStateInfo {
					time: Some(7 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("pisf.sprite"), frame: 0, full_bright: true}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(0 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 4, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	};
	asset_storage.insert_with_name("pistol", template);

	let template = WeaponTemplate {
		name: Some("shotgun"),
		states: {
			let mut states = HashMap::with_capacity(15);
			states.insert(StateName::from("up").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				WeaponStateInfo {
					time: Some(3 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(7 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(5 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(5 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 2, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 3, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(5 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 2, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(5 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(3 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(7 * FRAME_TIME),
					next: Some((StateName::from("ready").unwrap(), 0)),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtf.sprite"), frame: 0, full_bright: true}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(3 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtf.sprite"), frame: 1, full_bright: true}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(0 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 4, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	};
	asset_storage.insert_with_name("shotgun", template);

	let template = WeaponTemplate {
		name: Some("chaingun"),
		states: {
			let mut states = HashMap::with_capacity(8);
			states.insert(StateName::from("up").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("chgg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("chgg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("chgg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("chgg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("chgg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(0 * FRAME_TIME),
					next: Some((StateName::from("ready").unwrap(), 0)),
					sprite: Some(SpriteRender {sprite: asset_storage.load("chgg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				WeaponStateInfo {
					time: Some(5 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("chgf.sprite"), frame: 0, full_bright: true}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(0 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 4, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	};
	asset_storage.insert_with_name("chaingun", template);

	let template = WeaponTemplate {
		name: Some("missile"),
		states: {
			let mut states = HashMap::with_capacity(11);
			states.insert(StateName::from("up").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("misg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("misg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("misg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				WeaponStateInfo {
					time: Some(8 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("misg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(12 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("misg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(0 * FRAME_TIME),
					next: Some((StateName::from("ready").unwrap(), 0)),
					sprite: Some(SpriteRender {sprite: asset_storage.load("misg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				WeaponStateInfo {
					time: Some(3 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("misf.sprite"), frame: 0, full_bright: true}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("misf.sprite"), frame: 1, full_bright: true}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("misf.sprite"), frame: 2, full_bright: true}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("misf.sprite"), frame: 3, full_bright: true}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(0 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 4, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	};
	asset_storage.insert_with_name("missile", template);

	let template = WeaponTemplate {
		name: Some("plasma"),
		states: {
			let mut states = HashMap::with_capacity(7);
			states.insert(StateName::from("up").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("plsg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("plsg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("plsg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				WeaponStateInfo {
					time: Some(3 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("plsg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(20 * FRAME_TIME),
					next: Some((StateName::from("ready").unwrap(), 0)),
					sprite: Some(SpriteRender {sprite: asset_storage.load("plsg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("plsf.sprite"), frame: 0, full_bright: true}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(0 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 4, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	};
	asset_storage.insert_with_name("plasma", template);

	let template = WeaponTemplate {
		name: Some("bfg"),
		states: {
			let mut states = HashMap::with_capacity(10);
			states.insert(StateName::from("up").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("bfgg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("bfgg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("bfgg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				WeaponStateInfo {
					time: Some(20 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("bfgg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(10 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("bfgg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(10 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("bfgg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(20 * FRAME_TIME),
					next: Some((StateName::from("ready").unwrap(), 0)),
					sprite: Some(SpriteRender {sprite: asset_storage.load("bfgg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				WeaponStateInfo {
					time: Some(11 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("bfgf.sprite"), frame: 0, full_bright: true}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(6 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("bfgf.sprite"), frame: 1, full_bright: true}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(0 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 4, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	};
	asset_storage.insert_with_name("bfg", template);

	let template = WeaponTemplate {
		name: Some("chainsaw"),
		states: {
			let mut states = HashMap::with_capacity(7);
			states.insert(StateName::from("up").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sawg.sprite"), frame: 2, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sawg.sprite"), frame: 2, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sawg.sprite"), frame: 2, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sawg.sprite"), frame: 3, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sawg.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sawg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(0 * FRAME_TIME),
					next: Some((StateName::from("ready").unwrap(), 0)),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sawg.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	};
	asset_storage.insert_with_name("chainsaw", template);

	if wad_mode < WadMode::Doom2 {
		return;
	}

	let template = WeaponTemplate {
		name: Some("supershotgun"),
		states: {
			let mut states = HashMap::with_capacity(16);
			states.insert(StateName::from("up").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				WeaponStateInfo {
					time: Some(1 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				WeaponStateInfo {
					time: Some(3 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(7 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(7 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 1, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(7 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 2, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(7 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 3, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(7 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 4, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(7 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 5, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(6 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 6, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(6 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 7, full_bright: false}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(5 * FRAME_TIME),
					next: Some((StateName::from("ready").unwrap(), 0)),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 0, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				WeaponStateInfo {
					time: Some(5 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 8, full_bright: true}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(4 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("sht2.sprite"), frame: 9, full_bright: true}),
					.. WeaponStateInfo::default()
				},
				WeaponStateInfo {
					time: Some(0 * FRAME_TIME),
					sprite: Some(SpriteRender {sprite: asset_storage.load("shtg.sprite"), frame: 4, full_bright: false}),
					.. WeaponStateInfo::default()
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	};
	asset_storage.insert_with_name("supershotgun", template);
}
