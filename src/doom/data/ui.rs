use crate::{
	common::assets::AssetStorage,
	doom::{
		assets::template::EntityTemplate,
		ui::{
			hud::{AmmoStat, ArmsStat, HealthStat},
			UiAlignment, UiGameView, UiImage, UiText, UiTransform,
		},
	},
};
use legion::World;
use nalgebra::Vector2;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static UI: Lazy<HashMap<&'static str, fn(&mut AssetStorage) -> EntityTemplate>> =
	Lazy::new(|| {
		let mut ui: HashMap<&'static str, fn(&mut AssetStorage) -> EntityTemplate> = HashMap::new();

		ui.insert("hud.entity", |asset_storage| EntityTemplate {
			name: Some("hud"),
			world: {
				let mut world = World::default();

				// Game view
				world.push((
					UiTransform {
						position: Vector2::new(0.0, 0.0),
						depth: 0.0,
						alignment: [UiAlignment::Near, UiAlignment::Near],
						size: Vector2::new(320.0, 168.0),
						stretch: [true, true],
					},
					UiGameView,
				));

				// Tiled background
				world.push((
					UiTransform {
						position: Vector2::new(0.0, 168.0),
						depth: 1.0,
						alignment: [UiAlignment::Near, UiAlignment::Far],
						size: Vector2::new(320.0, 32.0),
						stretch: [true, false],
					},
					UiImage {
						image: asset_storage.load("floor7_2.flat"),
					},
				));

				// Main image
				world.push((
					UiTransform {
						position: Vector2::new(0.0, 168.0),
						depth: 2.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(320.0, 32.0),
						stretch: [false; 2],
					},
					UiImage {
						image: asset_storage.load("stbar.patch"),
					},
				));

				// Ammo
				world.push((
					UiTransform {
						position: Vector2::new(2.0, 171.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(42.0, 20.0),
						stretch: [false; 2],
					},
					UiText {
						text: " 50".into(),
						font: asset_storage.load("sttnum.font"),
					},
					AmmoStat {
						ammo_type: None,
						show_max: false,
					},
				));

				// Health
				world.push((
					UiTransform {
						position: Vector2::new(48.0, 171.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(56.0, 20.0),
						stretch: [false; 2],
					},
					UiText {
						text: String::with_capacity(4),
						font: asset_storage.load("sttnum.font"),
					},
					HealthStat,
				));

				// Arms
				world.push((
					UiTransform {
						position: Vector2::new(104.0, 168.0),
						depth: 3.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(40.0, 32.0),
						stretch: [false; 2],
					},
					UiImage {
						image: asset_storage.load("starms.patch"),
					},
				));

				// Weapon 2
				world.push((
					UiTransform {
						position: Vector2::new(111.0, 172.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(4.0, 6.0),
						stretch: [false; 2],
					},
					UiImage {
						image: asset_storage.load("stysnum0.patch"),
					},
					ArmsStat {
						weapons: vec!["pistol".into()],
						images: [
							asset_storage.load("stgnum2.patch"),
							asset_storage.load("stysnum2.patch"),
						],
					},
				));

				// Weapon 3
				world.push((
					UiTransform {
						position: Vector2::new(123.0, 172.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(4.0, 6.0),
						stretch: [false; 2],
					},
					UiImage {
						image: asset_storage.load("stysnum0.patch"),
					},
					ArmsStat {
						weapons: vec!["shotgun".into(), "supershotgun".into()],
						images: [
							asset_storage.load("stgnum3.patch"),
							asset_storage.load("stysnum3.patch"),
						],
					},
				));

				// Weapon 4
				world.push((
					UiTransform {
						position: Vector2::new(135.0, 172.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(4.0, 6.0),
						stretch: [false; 2],
					},
					UiImage {
						image: asset_storage.load("stysnum0.patch"),
					},
					ArmsStat {
						weapons: vec!["chaingun".into()],
						images: [
							asset_storage.load("stgnum4.patch"),
							asset_storage.load("stysnum4.patch"),
						],
					},
				));

				// Weapon 5
				world.push((
					UiTransform {
						position: Vector2::new(111.0, 182.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(4.0, 6.0),
						stretch: [false; 2],
					},
					UiImage {
						image: asset_storage.load("stysnum0.patch"),
					},
					ArmsStat {
						weapons: vec!["missile".into()],
						images: [
							asset_storage.load("stgnum5.patch"),
							asset_storage.load("stysnum5.patch"),
						],
					},
				));

				// Weapon 6
				world.push((
					UiTransform {
						position: Vector2::new(123.0, 182.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(4.0, 6.0),
						stretch: [false; 2],
					},
					UiImage {
						image: asset_storage.load("stysnum0.patch"),
					},
					ArmsStat {
						weapons: vec!["plasma".into()],
						images: [
							asset_storage.load("stgnum6.patch"),
							asset_storage.load("stysnum6.patch"),
						],
					},
				));

				// Weapon 7
				world.push((
					UiTransform {
						position: Vector2::new(135.0, 182.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(4.0, 6.0),
						stretch: [false; 2],
					},
					UiImage {
						image: asset_storage.load("stgnum7.patch"),
					},
					ArmsStat {
						weapons: vec!["bfg".into()],
						images: [
							asset_storage.load("stgnum7.patch"),
							asset_storage.load("stysnum7.patch"),
						],
					},
				));

				// Face
				world.push((
					UiTransform {
						position: Vector2::new(143.0, 168.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(24.0, 29.0),
						stretch: [false; 2],
					},
					UiImage {
						image: asset_storage.load("stfst00.patch"),
					},
				));

				// Armor
				world.push((
					UiTransform {
						position: Vector2::new(179.0, 171.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(56.0, 20.0),
						stretch: [false; 2],
					},
					UiText {
						text: "  0%".into(),
						font: asset_storage.load("sttnum.font"),
					},
				));

				// Blue key
				world.push((
					UiTransform {
						position: Vector2::new(239.0, 171.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(7.0, 5.0),
						stretch: [false; 2],
					},
					UiImage {
						image: asset_storage.load("stkeys0.patch"),
					},
				));

				// Yellow key
				world.push((
					UiTransform {
						position: Vector2::new(239.0, 181.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(7.0, 5.0),
						stretch: [false; 2],
					},
					UiImage {
						image: asset_storage.load("stkeys1.patch"),
					},
				));

				// Red key
				world.push((
					UiTransform {
						position: Vector2::new(239.0, 191.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(7.0, 5.0),
						stretch: [false; 2],
					},
					UiImage {
						image: asset_storage.load("stkeys2.patch"),
					},
				));

				// Bullets current
				world.push((
					UiTransform {
						position: Vector2::new(276.0, 173.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(12.0, 6.0),
						stretch: [false; 2],
					},
					UiText {
						text: " 50".into(),
						font: asset_storage.load("stysnum.font"),
					},
					AmmoStat {
						ammo_type: Some(asset_storage.load("bullets.ammo")),
						show_max: false,
					},
				));

				// Bullets max
				world.push((
					UiTransform {
						position: Vector2::new(302.0, 173.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(12.0, 6.0),
						stretch: [false; 2],
					},
					UiText {
						text: "200".into(),
						font: asset_storage.load("stysnum.font"),
					},
					AmmoStat {
						ammo_type: Some(asset_storage.load("bullets.ammo")),
						show_max: true,
					},
				));

				// Shells current
				world.push((
					UiTransform {
						position: Vector2::new(276.0, 179.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(12.0, 6.0),
						stretch: [false; 2],
					},
					UiText {
						text: "  0".into(),
						font: asset_storage.load("stysnum.font"),
					},
					AmmoStat {
						ammo_type: Some(asset_storage.load("shells.ammo")),
						show_max: false,
					},
				));

				// Shells max
				world.push((
					UiTransform {
						position: Vector2::new(302.0, 179.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(12.0, 6.0),
						stretch: [false; 2],
					},
					UiText {
						text: " 50".into(),
						font: asset_storage.load("stysnum.font"),
					},
					AmmoStat {
						ammo_type: Some(asset_storage.load("shells.ammo")),
						show_max: true,
					},
				));

				// Rockets current
				world.push((
					UiTransform {
						position: Vector2::new(276.0, 185.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(12.0, 6.0),
						stretch: [false; 2],
					},
					UiText {
						text: "  0".into(),
						font: asset_storage.load("stysnum.font"),
					},
					AmmoStat {
						ammo_type: Some(asset_storage.load("rockets.ammo")),
						show_max: false,
					},
				));

				// Rockets max
				world.push((
					UiTransform {
						position: Vector2::new(302.0, 185.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(12.0, 6.0),
						stretch: [false; 2],
					},
					UiText {
						text: " 50".into(),
						font: asset_storage.load("stysnum.font"),
					},
					AmmoStat {
						ammo_type: Some(asset_storage.load("rockets.ammo")),
						show_max: true,
					},
				));

				// Cells current
				world.push((
					UiTransform {
						position: Vector2::new(276.0, 191.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(12.0, 6.0),
						stretch: [false; 2],
					},
					UiText {
						text: "  0".into(),
						font: asset_storage.load("stysnum.font"),
					},
					AmmoStat {
						ammo_type: Some(asset_storage.load("cells.ammo")),
						show_max: false,
					},
				));

				// Cells max
				world.push((
					UiTransform {
						position: Vector2::new(302.0, 191.0),
						depth: 10.0,
						alignment: [UiAlignment::Middle, UiAlignment::Far],
						size: Vector2::new(12.0, 6.0),
						stretch: [false; 2],
					},
					UiText {
						text: "300".into(),
						font: asset_storage.load("stysnum.font"),
					},
					AmmoStat {
						ammo_type: Some(asset_storage.load("cells.ammo")),
						show_max: true,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		ui
	});
