#![allow(unused_variables)]
use crate::{
	common::{assets::AssetStorage, geometry::Angle},
	doom::{
		assets::template::{AmmoTemplate, WeaponAmmo, WeaponTemplate},
		data::FRAME_TIME,
		draw::sprite::SpriteRender,
		game::{
			combat::{
				weapon::{
					ChangeAmmoCount, LineAttack, NextWeaponState, SetWeaponSprite, SetWeaponState,
					WeaponPosition, WeaponReFire, WeaponReady, WeaponSpriteSlot,
					WeaponStateEventDef, WeaponStateEventDefSlot,
				},
				ExtraLight, SpawnProjectile,
			},
			state::{entity::EntityStateEventDef, StateName},
		},
		sound::StartSoundEventDef,
	},
};
use legion::World;
use nalgebra::Vector2;
use once_cell::sync::Lazy;
use std::{collections::HashMap, default::Default};

#[rustfmt::skip]
pub static WEAPONS: Lazy<HashMap<&'static str, fn(&mut AssetStorage) -> WeaponTemplate>> = Lazy::new(|| {
	let mut weapons: HashMap<&'static str, fn(&mut AssetStorage) -> WeaponTemplate> = HashMap::new();

	weapons.insert("fist.weapon", |asset_storage| WeaponTemplate {
		name: "fist",
		states: {
			let mut states = HashMap::with_capacity(8);
			states.insert(StateName::from("up").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pung.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pung.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pung.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Bob,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
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
						EntityStateEventDef,
						LineAttack {
							count: 1,
							damage_range: (1..=10).into(),
							damage_multiplier: 2,
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
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 3),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 4),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pung.sprite"),
							frame: 1,
							full_bright: false,
						})),
						WeaponReFire,
					));
					world
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	});

	weapons.insert("pistol.weapon", |asset_storage| WeaponTemplate {
		name: "pistol",
		ammo: Some(WeaponAmmo {
			handle: asset_storage.load("bullets.ammo"),
			count: 1,
		}),
		states: {
			let mut states = HashMap::with_capacity(9);
			states.insert(StateName::from("up").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Bob,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
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
						EntityStateEventDef,
						LineAttack {
							count: 1,
							damage_range: (1..=3).into(),
							damage_multiplier: 5,
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
						WeaponStateEventDefSlot(WeaponSpriteSlot::Flash),
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world.push((
						StartSoundEventDef {
							handle: asset_storage.load("dspistol.sound"),
						},
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisg.sprite"),
							frame: 1,
							full_bright: false,
						})),
						ChangeAmmoCount,
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 3),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisg.sprite"),
							frame: 1,
							full_bright: false,
						})),
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0625),
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("pisf.sprite"),
							frame: 0,
							full_bright: true,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						SetWeaponSprite(None),
					));
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0),
					));
					world
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	});

	weapons.insert("shotgun.weapon", |asset_storage| WeaponTemplate {
		name: "shotgun",
		ammo: Some(WeaponAmmo {
			handle: asset_storage.load("shells.ammo"),
			count: 1,
		}),
		states: {
			let mut states = HashMap::with_capacity(15);
			states.insert(StateName::from("up").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Bob,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						ChangeAmmoCount,
					));
					world.push((
						StartSoundEventDef {
							handle: asset_storage.load("dsshotgn.sound"),
						},
					));
					world.push((
						WeaponStateEventDefSlot(WeaponSpriteSlot::Flash),
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world.push((
						EntityStateEventDef,
						LineAttack {
							count: 7,
							damage_range: (1..=3).into(),
							damage_multiplier: 5,
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
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 3),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 4),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 5),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 6),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 7),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 8),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0625),
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtf.sprite"),
							frame: 0,
							full_bright: true,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						ExtraLight(0.125),
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 2),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("shtf.sprite"),
							frame: 1,
							full_bright: true,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0),
					));
					world.push((
						WeaponStateEventDef,
						SetWeaponSprite(None),
					));
					world
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	});

	weapons.insert("chaingun.weapon", |asset_storage| WeaponTemplate {
		name: "chaingun",
		ammo: Some(WeaponAmmo {
			handle: asset_storage.load("bullets.ammo"),
			count: 1,
		}),
		states: {
			let mut states = HashMap::with_capacity(10);
			states.insert(StateName::from("up").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Bob,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						StartSoundEventDef {
							handle: asset_storage.load("dspistol.sound"),
						},
					));
					world.push((
						EntityStateEventDef,
						LineAttack {
							count: 1,
							damage_range: (1..=3).into(),
							damage_multiplier: 5,
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						ChangeAmmoCount,
					));
					world.push((
						WeaponStateEventDefSlot(WeaponSpriteSlot::Flash),
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						LineAttack {
							count: 1,
							damage_range: (1..=3).into(),
							damage_multiplier: 5,
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
						WeaponStateEventDefSlot(WeaponSpriteSlot::Flash),
						SetWeaponState((StateName::from("flash2").unwrap(), 0)),
					));
					world.push((
						StartSoundEventDef {
							handle: asset_storage.load("dspistol.sound"),
						},
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgg.sprite"),
							frame: 1,
							full_bright: false,
						})),
						ChangeAmmoCount,
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 0 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgg.sprite"),
							frame: 1,
							full_bright: false,
						})),
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgf.sprite"),
							frame: 0,
							full_bright: true,
						})),
					));
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0625),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						SetWeaponSprite(None),
					));
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0),
					));
					world
				},
			]);
			states.insert(StateName::from("flash2").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("flash2").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("chgf.sprite"),
							frame: 1,
							full_bright: true,
						})),
					));
					world.push((
						EntityStateEventDef,
						ExtraLight(0.125),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0),
					));
					world.push((
						WeaponStateEventDef,
						SetWeaponSprite(None),
					));
					world
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	});

	weapons.insert("missile.weapon", |asset_storage| WeaponTemplate {
		name: "missile",
		ammo: Some(WeaponAmmo {
			handle: asset_storage.load("rockets.ammo"),
			count: 1,
		}),
		states: {
			let mut states = HashMap::with_capacity(11);
			states.insert(StateName::from("up").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Bob,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDefSlot(WeaponSpriteSlot::Flash),
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						SpawnProjectile(asset_storage.load("rocket.entity")),
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 12 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misg.sprite"),
							frame: 1,
							full_bright: false,
						})),
						ChangeAmmoCount,
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 0 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misg.sprite"),
							frame: 1,
							full_bright: false,
						})),
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0625),
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misf.sprite"),
							frame: 0,
							full_bright: true,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 2),
						},
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
						EntityStateEventDef,
						ExtraLight(0.125),
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 3),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misf.sprite"),
							frame: 2,
							full_bright: true,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 4),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("misf.sprite"),
							frame: 3,
							full_bright: true,
						})),
					));
					world.push((
						EntityStateEventDef,
						ExtraLight(0.125),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0),
					));
					world.push((
						WeaponStateEventDef,
						SetWeaponSprite(None),
					));
					world
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	});

	weapons.insert("plasma.weapon", |asset_storage| WeaponTemplate {
		name: "plasma",
		ammo: Some(WeaponAmmo {
			handle: asset_storage.load("cells.ammo"),
			count: 1,
		}),
		states: {
			let mut states = HashMap::with_capacity(9);
			states.insert(StateName::from("up").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Bob,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						ChangeAmmoCount,
					));
					world.push((
						WeaponStateEventDefSlot(WeaponSpriteSlot::Flash),
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world.push((
						EntityStateEventDef,
						SpawnProjectile(asset_storage.load("plasma.entity")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 20 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsg.sprite"),
							frame: 1,
							full_bright: false,
						})),
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0625),
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsf.sprite"),
							frame: 0,
							full_bright: true,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						SetWeaponSprite(None),
					));
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0),
					));
					world
				},
			]);
			states.insert(StateName::from("flash2").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash2").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("plsf.sprite"),
							frame: 1,
							full_bright: true,
						})),
					));
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0625),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						SetWeaponSprite(None),
					));
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0),
					));
					world
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	});

	weapons.insert("bfg.weapon", |asset_storage| WeaponTemplate {
		name: "bfg",
		ammo: Some(WeaponAmmo {
			handle: asset_storage.load("cells.ammo"),
			count: 40,
		}),
		states: {
			let mut states = HashMap::with_capacity(10);
			states.insert(StateName::from("up").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Bob,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						StartSoundEventDef {
							handle: asset_storage.load("dsbfg.sound"),
						},
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 20 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world.push((
						WeaponStateEventDefSlot(WeaponSpriteSlot::Flash),
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						SpawnProjectile(asset_storage.load("bfg.entity")),
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 3),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 1,
							full_bright: false,
						})),
						ChangeAmmoCount,
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 20 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgg.sprite"),
							frame: 1,
							full_bright: false,
						})),
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0625),
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 11 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgf.sprite"),
							frame: 0,
							full_bright: true,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 2),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("bfgf.sprite"),
							frame: 1,
							full_bright: true,
						})),
					));
					world.push((
						EntityStateEventDef,
						ExtraLight(0.125),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0),
					));
					world.push((
						WeaponStateEventDef,
						SetWeaponSprite(None),
					));
					world
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	});

	weapons.insert("chainsaw.weapon", |asset_storage| WeaponTemplate {
		name: "chainsaw",
		states: {
			let mut states = HashMap::with_capacity(8);
			states.insert(StateName::from("up").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 0 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 2,
							full_bright: false,
						})),
					));
					world.push((
						StartSoundEventDef {
							handle: asset_storage.load("dssawup.sound"),
						},
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 2,
							full_bright: false,
						})),
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 2,
							full_bright: false,
						})),
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						StartSoundEventDef {
							handle: asset_storage.load("dssawidl.sound"),
						},
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 2,
							full_bright: false,
						})),
						WeaponPosition::Bob,
						WeaponReady,
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 3,
							full_bright: false,
						})),
						WeaponPosition::Bob,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						LineAttack {
							count: 1,
							damage_range: (1..=10).into(),
							damage_multiplier: 2,
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
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 0,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						LineAttack {
							count: 1,
							damage_range: (1..=10).into(),
							damage_multiplier: 2,
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
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 1,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 0 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sawg.sprite"),
							frame: 1,
							full_bright: false,
						})),
						WeaponReFire,
					));
					world
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	});

	weapons.insert("supershotgun.weapon", |asset_storage| WeaponTemplate {
		name: "supershotgun",
		ammo: Some(WeaponAmmo {
			handle: asset_storage.load("shells.ammo"),
			count: 2,
		}),
		states: {
			let mut states = HashMap::with_capacity(16);
			states.insert(StateName::from("up").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("up").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Up,
					));
					world
				},
			]);
			states.insert(StateName::from("down").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("down").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Down,
					));
					world
				},
			]);
			states.insert(StateName::from("ready").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponPosition::Bob,
						WeaponReady,
					));
					world
				},
			]);
			states.insert(StateName::from("attack").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 1),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 2),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 0,
							full_bright: false,
						})),
						ChangeAmmoCount,
					));
					world.push((
						EntityStateEventDef,
						LineAttack {
							count: 20,
							damage_range: (1..=3).into(),
							damage_multiplier: 5,
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
						WeaponStateEventDefSlot(WeaponSpriteSlot::Flash),
						SetWeaponState((StateName::from("flash").unwrap(), 0)),
					));
					world.push((
						StartSoundEventDef {
							handle: asset_storage.load("dsdshtgn.sound"),
						},
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 3),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 4),
						},
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
						StartSoundEventDef {
							handle: asset_storage.load("dsdbopn.sound"),
						},
					));
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 5),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 3,
							full_bright: false,
						})),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 6),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 7),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 5,
							full_bright: false,
						})),
					));
					world.push((
						StartSoundEventDef {
							handle: asset_storage.load("dsdbload.sound"),
						},
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 8),
						},
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
						WeaponStateEventDef,
						NextWeaponState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("attack").unwrap(), 9),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 7,
							full_bright: false,
						})),
					));
					world.push((
						StartSoundEventDef {
							handle: asset_storage.load("dsdbcls.sound"),
						},
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("ready").unwrap(), 0),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 0,
							full_bright: false,
						})),
						WeaponReFire,
					));
					world
				},
			]);
			states.insert(StateName::from("flash").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 1),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 8,
							full_bright: true,
						})),
					));
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0625),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						WeaponStateEventDef,
						NextWeaponState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("flash").unwrap(), 2),
						},
						SetWeaponSprite(Some(SpriteRender {
							sprite: asset_storage.load("sht2.sprite"),
							frame: 9,
							full_bright: true,
						})),
					));
					world.push((
						EntityStateEventDef,
						ExtraLight(0.125),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityStateEventDef,
						ExtraLight(0.0),
					));
					world.push((
						WeaponStateEventDef,
						SetWeaponSprite(None),
					));
					world
				},
			]);
			states
		},
		.. WeaponTemplate::default()
	});

	weapons
});

#[rustfmt::skip]
pub static AMMO: Lazy<HashMap<&'static str, fn(&mut AssetStorage) -> AmmoTemplate>> = Lazy::new(|| {
	let mut ammo: HashMap<&'static str, fn(&mut AssetStorage) -> AmmoTemplate> = HashMap::new();

	ammo.insert("bullets.ammo", |_asset_storage| AmmoTemplate {
		name: "bullets",
		.. AmmoTemplate::default()
	});

	ammo.insert("shells.ammo", |_asset_storage| AmmoTemplate {
		name: "shells",
		.. AmmoTemplate::default()
	});

	ammo.insert("rockets.ammo", |_asset_storage| AmmoTemplate {
		name: "rockets",
		.. AmmoTemplate::default()
	});

	ammo.insert("cells.ammo", |_asset_storage| AmmoTemplate {
		name: "cells",
		.. AmmoTemplate::default()
	});

	ammo
});
