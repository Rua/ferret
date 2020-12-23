use crate::{
	common::{assets::AssetStorage, geometry::Angle},
	doom::{
		data::FRAME_TIME,
		draw::sprite::SpriteRender,
		sound::StartSound,
		state::{
			weapon::{
				ExtraLight, LineAttack, NextWeaponState, SetWeaponSprite, SetWeaponState,
				SpawnProjectile, WeaponPosition, WeaponReFire, WeaponReady, WeaponSpriteSlot,
				WeaponSpriteSlotDef,
			},
			EntityDef, StateName,
		},
		template::WeaponTemplate,
		WadMode,
	},
};
use legion::{systems::ResourceSet, Read, Resources, World, Write};
use nalgebra::Vector2;
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
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pung.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pung.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pung.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Bob,
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pung.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pung.sprite"),
							frame: 2,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						LineAttack {
							count: 1,
							damage_range: (1..=10).into(),
							damage_multiplier: 2.0,
							distance: 64.0,
							spread: Vector2::new(
								Angle::from_units(1.0 / 64.0),
								Angle(0),
							),
							accurate_until_refire: false,
							sparks: false,
							hit_sound: Some(asset_storage.load("dspunch.sound")),
							miss_sound: None,
						},
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pung.sprite"),
							frame: 3,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pung.sprite"),
							frame: 2,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pung.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReFire,
					));
					world
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
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Bob,
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						LineAttack {
							count: 1,
							damage_range: (1..=3).into(),
							damage_multiplier: 5.0,
							distance: 2000.0,
							spread: Vector2::new(
								Angle::from_units(1.0 / 64.0),
								Angle(0),
							),
							accurate_until_refire: true,
							sparks: true,
							hit_sound: None,
							miss_sound: None,
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						StartSound(asset_storage.load("dspistol.sound")),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlot::Flash,
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisg.sprite"),
							frame: 2,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisf.sprite"),
							frame: 0,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0625),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(None),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0),
					));
					world
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
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Bob,
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						LineAttack {
							count: 7,
							damage_range: (1..=3).into(),
							damage_multiplier: 5.0,
							distance: 2000.0,
							spread: Vector2::new(
								Angle::from_units(1.0 / 64.0),
								Angle(0),
							),
							accurate_until_refire: false,
							sparks: true,
							hit_sound: None,
							miss_sound: None,
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						StartSound(asset_storage.load("dsshotgn.sound")),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlot::Flash,
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 2,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 3,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 2,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtf.sprite"),
							frame: 0,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0625),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtf.sprite"),
							frame: 1,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.125),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(None),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0),
					));
					world
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
			let mut states = HashMap::with_capacity(10);
			states.insert(StateName::from("up").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Bob,
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						LineAttack {
							count: 1,
							damage_range: (1..=3).into(),
							damage_multiplier: 5.0,
							distance: 2000.0,
							spread: Vector2::new(
								Angle::from_units(1.0 / 64.0),
								Angle(0),
							),
							accurate_until_refire: true,
							sparks: true,
							hit_sound: None,
							miss_sound: None,
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						StartSound(asset_storage.load("dspistol.sound")),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlot::Flash,
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						LineAttack {
							count: 1,
							damage_range: (1..=3).into(),
							damage_multiplier: 5.0,
							distance: 2000.0,
							spread: Vector2::new(
								Angle::from_units(1.0 / 64.0),
								Angle(0),
							),
							accurate_until_refire: true,
							sparks: true,
							hit_sound: None,
							miss_sound: None,
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						StartSound(asset_storage.load("dspistol.sound")),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlot::Flash,
						SetWeaponState((StateName::from("flash2").unwrap(), 0)),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 0 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgf.sprite"),
							frame: 0,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0625),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(None),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0),
					));
					world
				},
			]);
			states.insert(StateName::from("flash2").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("flash2").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgf.sprite"),
							frame: 1,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.125),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(None),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0),
					));
					world
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
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Bob,
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlot::Flash,
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 12 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SpawnProjectile("rocket".into()),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 0 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misf.sprite"),
							frame: 0,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0625),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misf.sprite"),
							frame: 1,
							full_bright: true,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misf.sprite"),
							frame: 2,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.125),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misf.sprite"),
							frame: 3,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.125),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(None),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0),
					));
					world
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	};
	asset_storage.insert_with_name("missile", template);

	let template = WeaponTemplate {
		name: Some("chainsaw"),
		states: {
			let mut states = HashMap::with_capacity(8);
			states.insert(StateName::from("up").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 0 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 2,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						StartSound(asset_storage.load("dssawup.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 2,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 2,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 2,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						StartSound(asset_storage.load("dssawidl.sound")),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Bob,
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReady,
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 3,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Bob,
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						LineAttack {
							count: 1,
							damage_range: (1..=10).into(),
							damage_multiplier: 2.0,
							distance: 65.0,
							spread: Vector2::new(
								Angle::from_units(1.0 / 64.0),
								Angle(0),
							),
							accurate_until_refire: false,
							sparks: true,
							hit_sound: Some(asset_storage.load("dssawhit.sound")),
							miss_sound: Some(asset_storage.load("dssawful.sound")),
						},
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						LineAttack {
							count: 1,
							damage_range: (1..=10).into(),
							damage_multiplier: 2.0,
							distance: 65.0,
							spread: Vector2::new(
								Angle::from_units(1.0 / 64.0),
								Angle(0),
							),
							accurate_until_refire: false,
							sparks: true,
							hit_sound: Some(asset_storage.load("dssawhit.sound")),
							miss_sound: Some(asset_storage.load("dssawful.sound")),
						},
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 0 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReFire,
					));
					world
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	};
	asset_storage.insert_with_name("chainsaw", template);

	if wad_mode < WadMode::Doom1 {
		return;
	}

	let template = WeaponTemplate {
		name: Some("plasma"),
		states: {
			let mut states = HashMap::with_capacity(9);
			states.insert(StateName::from("up").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Bob,
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SpawnProjectile("plasma".into()),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlot::Flash,
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 20 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsf.sprite"),
							frame: 0,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0625),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(None),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0),
					));
					world
				},
			]);
			states.insert(StateName::from("flash2").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash2").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsf.sprite"),
							frame: 1,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0625),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(None),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0),
					));
					world
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
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Bob,
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 20 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						StartSound(asset_storage.load("dsbfg.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlot::Flash,
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SpawnProjectile("bfg".into()),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 20 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 11 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgf.sprite"),
							frame: 0,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0625),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgf.sprite"),
							frame: 1,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.125),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(None),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0),
					));
					world
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	};
	asset_storage.insert_with_name("bfg", template);

	if wad_mode < WadMode::Doom2 {
		return;
	}

	let template = WeaponTemplate {
		name: Some("supershotgun"),
		states: {
			let mut states = HashMap::with_capacity(16);
			states.insert(StateName::from("up").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponPosition::Bob,
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						LineAttack {
							count: 20,
							damage_range: (1..=3).into(),
							damage_multiplier: 5.0,
							distance: 2000.0,
							spread: Vector2::new(
								Angle::from_units(1.0 / 32.0),
								Angle::from_units(1.0 / 50.526199853),
							),
							accurate_until_refire: false,
							sparks: true,
							hit_sound: None,
							miss_sound: None,
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						StartSound(asset_storage.load("dsdshtgn.sound")),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlot::Flash,
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 2,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 3,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						StartSound(asset_storage.load("dsdbopn.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 4,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 5,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						StartSound(asset_storage.load("dsdbload.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 6,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 7,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						StartSound(asset_storage.load("dsdbcls.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 8,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0625),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 9,
							full_bright: true,
						})),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.125),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						SetWeaponSprite(None),
					));
					world.push((
						EntityDef,
						WeaponSpriteSlotDef,
						ExtraLight(0.0),
					));
					world
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	};
	asset_storage.insert_with_name("supershotgun", template);
}
