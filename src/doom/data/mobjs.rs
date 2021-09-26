#![allow(unused_variables)]
use crate::{
	common::{assets::AssetStorage, geometry::Angle},
	doom::{
		assets::template::{EntityTemplate, EntityTemplateRefDef},
		data::{FRAME_RATE, FRAME_TIME},
		draw::{sprite::SpriteRender, wsprite::WeaponSpriteRender},
		game::{
			camera::{Camera, MovementBob},
			client::{PlayerTouch, User},
			combat::{
				weapon::{AmmoState, WeaponStateDef},
				HealthDef, OwnerDef, ProjectileTouch, RadiusAttack, SprayAttack,
			},
			map::SpawnPoint,
			physics::{
				BoxCollider, CollisionResponse, DamageParticle, Physics, PhysicsDef,
				SetBlocksTypes, SetSolidType, SolidBits, SolidType, TouchEventDef, Touchable,
			},
			state::{
				entity::{
					EntityStateEventDef, NextState, NextStateRandomTimeDef, RemoveEntity, StateDef,
				},
				StateName,
			},
			RandomTransformDef, SetEntitySprite, TransformDef,
		},
		sound::StartSoundEventDef,
	},
};
use legion::World;
use nalgebra::{Vector2, Vector3};
use once_cell::sync::Lazy;
use std::{collections::HashMap, default::Default};

pub static MOBJS: Lazy<HashMap<&'static str, fn(&mut AssetStorage) -> EntityTemplate>> =
	Lazy::new(|| {
		let mut mobjs: HashMap<&'static str, fn(&mut AssetStorage) -> EntityTemplate> =
			HashMap::new();

		mobjs.insert("spawn1.entity", |asset_storage| EntityTemplate {
			name: Some("spawn1"),
			world: {
				let mut world = World::default();
				world.push((
					EntityTemplateRefDef,
					SpawnPoint { player_num: 1 },
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("spawn2.entity", |asset_storage| EntityTemplate {
			name: Some("spawn2"),
			world: {
				let mut world = World::default();
				world.push((
					EntityTemplateRefDef,
					SpawnPoint { player_num: 2 },
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("spawn3.entity", |asset_storage| EntityTemplate {
			name: Some("spawn3"),
			world: {
				let mut world = World::default();
				world.push((
					EntityTemplateRefDef,
					SpawnPoint { player_num: 3 },
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("spawn4.entity", |asset_storage| EntityTemplate {
			name: Some("spawn4"),
			world: {
				let mut world = World::default();
				world.push((
					EntityTemplateRefDef,
					SpawnPoint { player_num: 4 },
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("spawndm.entity", |asset_storage| EntityTemplate {
			name: Some("spawndm"),
			world: {
				let mut world = World::default();
				world.push((
					EntityTemplateRefDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("player.entity", |asset_storage| EntityTemplate {
			name: Some("player"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 56.0,
						radius: 16.0,
						solid_type: SolidType::PLAYER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					Camera {
						base: Vector3::new(0.0, 0.0, 41.0),
						offset: Vector3::zeros(),
						bob_period: 20 * FRAME_TIME,
						weapon_bob_period: 64 * FRAME_TIME,
						deviation_position: 0.0,
						deviation_velocity: 0.0,
						impact_sound: asset_storage.load("dsoof.sound"),
						extra_light: 0.0,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 100,
						pain_chance: 0.99609375,
					},
					MovementBob {
						max: 16.0,
						amplitude: 0.0,
					},
					PhysicsDef {
						collision_response: CollisionResponse::StepSlide,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("play.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					Touchable,
					TransformDef {
						spawn_on_ceiling: false,
					},
					User {
						error_sound: asset_storage.load("dsnoway.sound"),
					},
					WeaponSpriteRender {
						position: Vector2::new(0.0, 96.0),
						slots: [
							Some(SpriteRender {
								sprite: asset_storage.load("pisg.sprite"),
								frame: 0,
								full_bright: false,
							}),
							None,
						],
					},
					WeaponStateDef {
						current: asset_storage.load("pistol.weapon"),
						inventory: [
							asset_storage.load("fist.weapon"),
							asset_storage.load("pistol.weapon"),
						]
						.into(),
						ammo: [
							(
								asset_storage.load("bullets.ammo"),
								AmmoState {
									current: 50,
									max: 200,
								},
							),
							(
								asset_storage.load("shells.ammo"),
								AmmoState {
									current: 0,
									max: 50,
								},
							),
							(
								asset_storage.load("rockets.ammo"),
								AmmoState {
									current: 0,
									max: 50,
								},
							),
							(
								asset_storage.load("cells.ammo"),
								AmmoState {
									current: 0,
									max: 300,
								},
							),
						]
						.into(),
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(24);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("play.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsplpain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							NextState {
								time: 12 * FRAME_TIME,
								state: (StateName::from("spawn").unwrap(), 0),
							},
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("play.sprite"),
								frame: 4,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("xdeath").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsslop.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 3),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 17,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 18,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 19,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 20,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 21,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("play.sprite"),
									frame: 22,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			touch: {
				let mut world = World::default();
				world.push((TouchEventDef, PlayerTouch));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("possessed.entity", |asset_storage| EntityTemplate {
			name: Some("possessed"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 56.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 20,
						pain_chance: 0.78125,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("poss.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(33);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dspopain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dspodth1.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("xdeath").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsslop.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 3),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 17,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 18,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 19,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 20,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("poss.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("shotguy.entity", |asset_storage| EntityTemplate {
			name: Some("shotguy"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 56.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 30,
						pain_chance: 0.6640625,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("spos.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(34);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dspopain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 5,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dspodth2.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("xdeath").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsslop.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 3),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 17,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 18,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 19,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 20,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spos.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("vile.entity", |asset_storage| EntityTemplate {
			name: Some("vile"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 56.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 700,
						pain_chance: 0.0390625,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 500.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("vile.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(37);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 10),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 11),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsvipain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 0 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 6,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 6,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 8,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 9,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 10,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 11,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 12,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 13,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 10),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 14,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 20 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 15,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 17,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsvildth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 18,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 19,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 20,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 21,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 22,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 23,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 24,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("vile.sprite"),
									frame: 25,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("fire.entity", |asset_storage| EntityTemplate {
			name: Some("fire"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("fire.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(30);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 10),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 11),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 12),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 13),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 14),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 15),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 16),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 17),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 18),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 19),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 20),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 5,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 21),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 22),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 5,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 23),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 24),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 5,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 25),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 6,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 26),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 27),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 6,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 28),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 29),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 6,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 30),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("undead.entity", |asset_storage| EntityTemplate {
			name: Some("undead"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 56.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 300,
						pain_chance: 0.390625,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 500.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("skel.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(36);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 10),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 11),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dspopain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("melee").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 0 * FRAME_TIME,
									state: (StateName::from("melee").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("melee").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("melee").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 0 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 9,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 9,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsskedth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skel.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("tracer.entity", |asset_storage| EntityTemplate {
			name: Some("tracer"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 8.0,
						radius: 11.0,
						solid_type: SolidType::PROJECTILE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					OwnerDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 10.0 * FRAME_RATE,
					},
					SpriteRender {
						sprite: asset_storage.load("fatb.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					Touchable,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(6);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 0 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatb.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsskeatk.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatb.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatb.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextStateRandomTimeDef {
									time: (FRAME_TIME..8 * FRAME_TIME).into(),
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fbxp.sprite"),
									frame: 0,
									full_bright: true,
								}),
								SetSolidType(SolidType::PARTICLE),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsbarexp.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fbxp.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fbxp.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			touch: {
				let mut world = World::default();
				world.push((
					TouchEventDef,
					ProjectileTouch {
						damage_range: (1..=8).into(),
						damage_multiplier: 10,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("smoke.entity", |asset_storage| EntityTemplate {
			name: Some("smoke"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("puff.sprite"),
						frame: 1,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(5);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("puff.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("puff.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("puff.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("puff.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("puff.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("fatso.entity", |asset_storage| EntityTemplate {
			name: Some("fatso"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 64.0,
						radius: 48.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 600,
						pain_chance: 0.3125,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 1000.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("fatt.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(44);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 15 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 15 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 10),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 11),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsmnpain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 20 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsmandth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 17,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 18,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 19,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 17,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fatt.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("fatshot.entity", |asset_storage| EntityTemplate {
			name: Some("fatshot"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 8.0,
						radius: 6.0,
						solid_type: SolidType::PROJECTILE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					OwnerDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 20.0 * FRAME_RATE,
					},
					SpriteRender {
						sprite: asset_storage.load("manf.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					Touchable,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(6);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 0 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("manf.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsfirsht.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("manf.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("manf.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextStateRandomTimeDef {
									time: (FRAME_TIME..8 * FRAME_TIME).into(),
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("misl.sprite"),
									frame: 1,
									full_bright: true,
								}),
								SetSolidType(SolidType::PARTICLE),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsfirxpl.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("misl.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("misl.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			touch: {
				let mut world = World::default();
				world.push((
					TouchEventDef,
					ProjectileTouch {
						damage_range: (1..=8).into(),
						damage_multiplier: 8,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("chainguy.entity", |asset_storage| EntityTemplate {
			name: Some("chainguy"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 56.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 70,
						pain_chance: 0.6640625,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("cpos.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(36);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dspopain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 5,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 1 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dspodth2.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("xdeath").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsslop.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 3),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 17,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 18,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 19,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cpos.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("troop.entity", |asset_storage| EntityTemplate {
			name: Some("troop"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 56.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 60,
						pain_chance: 0.78125,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("troo.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(36);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dspopain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("melee").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("melee").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("melee").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsbgdth1.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("xdeath").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsslop.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 4),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 17,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 18,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 19,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 20,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("troo.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("sergeant.entity", |asset_storage| EntityTemplate {
			name: Some("sergeant"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 56.0,
						radius: 30.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 150,
						pain_chance: 0.703125,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 400.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("sarg.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(27);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsdmpain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("melee").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("melee").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("melee").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dssgtdth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("shadows.entity", |asset_storage| EntityTemplate {
			name: Some("shadows"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 56.0,
						radius: 30.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 150,
						pain_chance: 0.703125,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 400.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("sarg.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(27);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsdmpain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("melee").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("melee").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("melee").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dssgtdth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sarg.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("head.entity", |asset_storage| EntityTemplate {
			name: Some("head"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 56.0,
						radius: 31.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 400,
						pain_chance: 0.5,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 400.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("head.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(20);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							NextState {
								time: 10 * FRAME_TIME,
								state: (StateName::from("spawn").unwrap(), 0),
							},
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("head.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							NextState {
								time: 3 * FRAME_TIME,
								state: (StateName::from("see").unwrap(), 0),
							},
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("head.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsdmpain.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dscacdth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("head.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("bruiser.entity", |asset_storage| EntityTemplate {
			name: Some("bruiser"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 64.0,
						radius: 24.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 1000,
						pain_chance: 0.1953125,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 1000.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("boss.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(32);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsdmpain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("melee").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("melee").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("melee").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsbrsdth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("boss.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("bruisershot.entity", |asset_storage| EntityTemplate {
			name: Some("bruisershot"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 8.0,
						radius: 6.0,
						solid_type: SolidType::PROJECTILE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					OwnerDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 15.0 * FRAME_RATE,
					},
					SpriteRender {
						sprite: asset_storage.load("bal7.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					Touchable,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(6);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 0 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal7.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsfirsht.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal7.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal7.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextStateRandomTimeDef {
									time: (FRAME_TIME..6 * FRAME_TIME).into(),
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal7.sprite"),
									frame: 2,
									full_bright: true,
								}),
								SetSolidType(SolidType::PARTICLE),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsfirxpl.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal7.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal7.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			touch: {
				let mut world = World::default();
				world.push((
					TouchEventDef,
					ProjectileTouch {
						damage_range: (1..=8).into(),
						damage_multiplier: 8,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("knight.entity", |asset_storage| EntityTemplate {
			name: Some("knight"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 64.0,
						radius: 24.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 500,
						pain_chance: 0.1953125,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 1000.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("bos2.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(32);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 2 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsdmpain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("melee").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("melee").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("melee").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dskntdth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bos2.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("skull.entity", |asset_storage| EntityTemplate {
			name: Some("skull"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 56.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 100,
						pain_chance: 1.0,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 50.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("skul.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(16);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsdmpain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 5,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 6,
									full_bright: true,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsfirxpl.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 8,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("spider.entity", |asset_storage| EntityTemplate {
			name: Some("spider"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 100.0,
						radius: 128.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 3000,
						pain_chance: 0.15625,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 1000.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("spid.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(31);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 10),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 11),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsdmpain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 20 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 6,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 1 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 20 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsspidth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 17,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 30 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 10),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 18,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("spid.sprite"),
									frame: 18,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("baby.entity", |asset_storage| EntityTemplate {
			name: Some("baby"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 64.0,
						radius: 64.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 500,
						pain_chance: 0.5,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 600.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("bspi.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(35);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 20 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 10),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 11),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 12),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsdmpain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 20 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 6,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 1 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 20 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsbspdth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bspi.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("cyborg.entity", |asset_storage| EntityTemplate {
			name: Some("cyborg"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 110.0,
						radius: 40.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 4000,
						pain_chance: 0.078125,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 1000.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("cybr.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(27);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							NextState {
								time: 10 * FRAME_TIME,
								state: (StateName::from("see").unwrap(), 0),
							},
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("cybr.sprite"),
								frame: 6,
								full_bright: false,
							}),
						));
						world.push((StartSoundEventDef {
							handle: asset_storage.load("dsdmpain.sound"),
						},));
						world
					}],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 12 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 12 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 12 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 12 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 12 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dscybdth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 6),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 30 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("cybr.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("pain.entity", |asset_storage| EntityTemplate {
			name: Some("pain"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 56.0,
						radius: 31.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 400,
						pain_chance: 0.5,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 400.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pain.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(25);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							NextState {
								time: 10 * FRAME_TIME,
								state: (StateName::from("spawn").unwrap(), 0),
							},
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("pain.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dspepain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 5,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 0 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 5,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 8,
									full_bright: true,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dspedth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 9,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 10,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 11,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 12,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pain.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("wolfss.entity", |asset_storage| EntityTemplate {
			name: Some("wolfss"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 56.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 50,
						pain_chance: 0.6640625,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("sswv.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(37);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dspopain.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("missile").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 6,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 6,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 1 * FRAME_TIME,
									state: (StateName::from("missile").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsssdth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("xdeath").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 13,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 14,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsslop.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 3),
								},
								SetBlocksTypes(SolidBits::empty()),
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 15,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 16,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 17,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 18,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 19,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("xdeath").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 20,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 21,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("raise").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("raise").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("keen.entity", |asset_storage| EntityTemplate {
			name: Some("keen"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 72.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 100,
						pain_chance: 1.0,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 10000000.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("keen.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(15);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("keen.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("pain").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 12,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dskeenpn.sound"),
							},));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dskeendt.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 4,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 5,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 6,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 7,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 8,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 10),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 9,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 11),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("keen.sprite"),
									frame: 11,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("bossbrain.entity", |asset_storage| EntityTemplate {
			name: Some("bossbrain"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 250,
						pain_chance: 0.99609375,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 10000000.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("bbrn.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(6);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("bbrn.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states.insert(
					StateName::from("pain").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							NextState {
								time: 36 * FRAME_TIME,
								state: (StateName::from("spawn").unwrap(), 0),
							},
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("bbrn.sprite"),
								frame: 1,
								full_bright: false,
							}),
						));
						world.push((StartSoundEventDef {
							handle: asset_storage.load("dsbospn.sound"),
						},));
						world
					}],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 100 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bbrn.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsbosdth.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bbrn.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bbrn.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bbrn.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("bossspit.entity", |asset_storage| EntityTemplate {
			name: Some("bossspit"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 32.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("sswv.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(3);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							NextState {
								time: 10 * FRAME_TIME,
								state: (StateName::from("spawn").unwrap(), 0),
							},
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("sswv.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states.insert(
					StateName::from("see").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 181 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 150 * FRAME_TIME,
									state: (StateName::from("see").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("sswv.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("bosstarget.entity", |asset_storage| EntityTemplate {
			name: Some("bosstarget"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 32.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("spawnshot.entity", |asset_storage| EntityTemplate {
			name: Some("spawnshot"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 32.0,
						radius: 6.0,
						solid_type: SolidType::PROJECTILE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					OwnerDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 10.0 * FRAME_RATE,
					},
					SpriteRender {
						sprite: asset_storage.load("bosf.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					Touchable,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(5);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 0 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bosf.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsbospit.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bosf.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bosf.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bosf.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 3 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bosf.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			touch: {
				let mut world = World::default();
				world.push((
					TouchEventDef,
					ProjectileTouch {
						damage_range: (1..=8).into(),
						damage_multiplier: 3,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("spawnfire.entity", |asset_storage| EntityTemplate {
			name: Some("spawnfire"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("fire.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(8);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 5,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 6,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fire.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("barrel.entity", |asset_storage| EntityTemplate {
			name: Some("barrel"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 42.0,
						radius: 10.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Puff,
					},
					EntityTemplateRefDef,
					HealthDef {
						max: 20,
						pain_chance: 0.0,
					},
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("bar1.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(7);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bar1.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bar1.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bexp.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bexp.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsbarexp.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bexp.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								RadiusAttack {
									damage: 128,
									radius: 128.0,
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bexp.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bexp.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("troopshot.entity", |asset_storage| EntityTemplate {
			name: Some("troopshot"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 8.0,
						radius: 6.0,
						solid_type: SolidType::PROJECTILE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					OwnerDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 10.0 * FRAME_RATE,
					},
					SpriteRender {
						sprite: asset_storage.load("bal1.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					Touchable,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(6);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 0 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal1.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsfirsht.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal1.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal1.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextStateRandomTimeDef {
									time: (FRAME_TIME..6 * FRAME_TIME).into(),
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal1.sprite"),
									frame: 2,
									full_bright: true,
								}),
								SetSolidType(SolidType::PARTICLE),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsfirxpl.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal1.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal1.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			touch: {
				let mut world = World::default();
				world.push((
					TouchEventDef,
					ProjectileTouch {
						damage_range: (1..=8).into(),
						damage_multiplier: 3,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("headshot.entity", |asset_storage| EntityTemplate {
			name: Some("headshot"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 8.0,
						radius: 6.0,
						solid_type: SolidType::PROJECTILE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					OwnerDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 10.0 * FRAME_RATE,
					},
					SpriteRender {
						sprite: asset_storage.load("bal2.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					Touchable,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(6);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 0 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal2.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsfirsht.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal2.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal2.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextStateRandomTimeDef {
									time: (FRAME_TIME..6 * FRAME_TIME).into(),
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal2.sprite"),
									frame: 2,
									full_bright: true,
								}),
								SetSolidType(SolidType::PARTICLE),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsfirxpl.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal2.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bal2.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			touch: {
				let mut world = World::default();
				world.push((
					TouchEventDef,
					ProjectileTouch {
						damage_range: (1..=8).into(),
						damage_multiplier: 5,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("rocket.entity", |asset_storage| EntityTemplate {
			name: Some("rocket"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 8.0,
						radius: 11.0,
						solid_type: SolidType::PROJECTILE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					OwnerDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 20.0 * FRAME_RATE,
					},
					SpriteRender {
						sprite: asset_storage.load("misl.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					Touchable,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(5);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 0 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("misl.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsrlaunc.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 1 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("misl.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextStateRandomTimeDef {
									time: (FRAME_TIME..8 * FRAME_TIME).into(),
									state: (StateName::from("death").unwrap(), 1),
								},
								RadiusAttack {
									damage: 128,
									radius: 128.0,
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("misl.sprite"),
									frame: 1,
									full_bright: true,
								}),
								SetSolidType(SolidType::PARTICLE),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsbarexp.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("misl.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("misl.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			touch: {
				let mut world = World::default();
				world.push((
					TouchEventDef,
					ProjectileTouch {
						damage_range: (1..=8).into(),
						damage_multiplier: 20,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("plasma.entity", |asset_storage| EntityTemplate {
			name: Some("plasma"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 8.0,
						radius: 13.0,
						solid_type: SolidType::PROJECTILE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					OwnerDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 25.0 * FRAME_RATE,
					},
					SpriteRender {
						sprite: asset_storage.load("plss.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					Touchable,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(8);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 0 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("plss.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsplasma.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("plss.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("plss.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextStateRandomTimeDef {
									time: (FRAME_TIME..4 * FRAME_TIME).into(),
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("plse.sprite"),
									frame: 0,
									full_bright: true,
								}),
								SetSolidType(SolidType::PARTICLE),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsfirxpl.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("plse.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("plse.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("plse.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("plse.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			touch: {
				let mut world = World::default();
				world.push((
					TouchEventDef,
					ProjectileTouch {
						damage_range: (1..=8).into(),
						damage_multiplier: 5,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("bfg.entity", |asset_storage| EntityTemplate {
			name: Some("bfg"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 8.0,
						radius: 13.0,
						solid_type: SolidType::PROJECTILE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					OwnerDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 25.0 * FRAME_RATE,
					},
					SpriteRender {
						sprite: asset_storage.load("bfs1.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					Touchable,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(8);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bfs1.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bfs1.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextStateRandomTimeDef {
									time: (FRAME_TIME..8 * FRAME_TIME).into(),
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bfe1.sprite"),
									frame: 0,
									full_bright: true,
								}),
								SetSolidType(SolidType::PARTICLE),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsrxplod.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bfe1.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bfe1.sprite"),
									frame: 2,
									full_bright: true,
								}),
								SprayAttack {
									count: 40,
									damage_range: (15..=120).into(),
									damage_multiplier: 1,
									distance: 1024.0,
									particle: asset_storage.load("extrabfg.entity"),
									spread: Vector2::new(
										Angle::from_units(1.0 / 8.0),
										Angle::from_units(1.0 / 10.0),
									),
								},
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bfe1.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bfe1.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bfe1.sprite"),
									frame: 5,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			touch: {
				let mut world = World::default();
				world.push((
					TouchEventDef,
					ProjectileTouch {
						damage_range: (1..=8).into(),
						damage_multiplier: 100,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("arachplaz.entity", |asset_storage| EntityTemplate {
			name: Some("arachplaz"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 8.0,
						radius: 13.0,
						solid_type: SolidType::PROJECTILE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					OwnerDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 25.0 * FRAME_RATE,
					},
					SpriteRender {
						sprite: asset_storage.load("apls.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					Touchable,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(8);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 0 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("apls.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsplasma.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("apls.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("apls.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states.insert(
					StateName::from("death").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextStateRandomTimeDef {
									time: (FRAME_TIME..5 * FRAME_TIME).into(),
									state: (StateName::from("death").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("apbx.sprite"),
									frame: 0,
									full_bright: true,
								}),
								SetSolidType(SolidType::PARTICLE),
							));
							world.push((StartSoundEventDef {
								handle: asset_storage.load("dsfirxpl.sound"),
							},));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("apbx.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("apbx.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("apbx.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 5 * FRAME_TIME,
									state: (StateName::from("death").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("apbx.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			touch: {
				let mut world = World::default();
				world.push((
					TouchEventDef,
					ProjectileTouch {
						damage_range: (1..=8).into(),
						damage_multiplier: 5,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("puff1.entity", |asset_storage| EntityTemplate {
			name: Some("puff1"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("puff.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("puff.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("puff.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("puff.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("puff.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("puff3.entity", |asset_storage| EntityTemplate {
			name: Some("puff3"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("puff.sprite"),
						frame: 2,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("puff.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("puff.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("blood1.entity", |asset_storage| EntityTemplate {
			name: Some("blood1"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					Physics {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						velocity: Vector3::new(0.0, 0.0, 2.0 * FRAME_RATE),
					},
					SpriteRender {
						sprite: asset_storage.load("blud.sprite"),
						frame: 2,
						full_bright: false,
					},
					StateDef,
					RandomTransformDef([
						(0.0..=0.0).into(),
						(0.0..=0.0).into(),
						(-4.0..=4.0).into(),
					]),
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(3);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextStateRandomTimeDef {
									time: (FRAME_TIME..8 * FRAME_TIME).into(),
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("blud.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("blud.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("blud.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("blood2.entity", |asset_storage| EntityTemplate {
			name: Some("blood2"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					Physics {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						velocity: Vector3::new(0.0, 0.0, 2.0 * FRAME_RATE),
					},
					SpriteRender {
						sprite: asset_storage.load("blud.sprite"),
						frame: 1,
						full_bright: false,
					},
					StateDef,
					RandomTransformDef([
						(0.0..=0.0).into(),
						(0.0..=0.0).into(),
						(-4.0..=4.0).into(),
					]),
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("blud.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("blud.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("blood3.entity", |asset_storage| EntityTemplate {
			name: Some("blood3"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					Physics {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						velocity: Vector3::new(0.0, 0.0, 2.0 * FRAME_RATE),
					},
					SpriteRender {
						sprite: asset_storage.load("blud.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					RandomTransformDef([
						(0.0..=0.0).into(),
						(0.0..=0.0).into(),
						(-4.0..=4.0).into(),
					]),
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("blud.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("tfog.entity", |asset_storage| EntityTemplate {
			name: Some("tfog"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("tfog.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(12);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tfog.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tfog.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tfog.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tfog.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tfog.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tfog.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tfog.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 8),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tfog.sprite"),
									frame: 5,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 9),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tfog.sprite"),
									frame: 6,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 10),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tfog.sprite"),
									frame: 7,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 11),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tfog.sprite"),
									frame: 8,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 12),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tfog.sprite"),
									frame: 9,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("ifog.entity", |asset_storage| EntityTemplate {
			name: Some("ifog"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("ifog.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(7);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ifog.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ifog.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ifog.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ifog.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ifog.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 6),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ifog.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 7),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ifog.sprite"),
									frame: 4,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("teleportman.entity", |asset_storage| EntityTemplate {
			name: Some("teleportman"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			..EntityTemplate::default()
		});

		mobjs.insert("extrabfg.entity", |asset_storage| EntityTemplate {
			name: Some("extrabfg"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("bfe2.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bfe2.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bfe2.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bfe2.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bfe2.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc0.entity", |asset_storage| EntityTemplate {
			name: Some("misc0"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("arm1.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("arm1.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 7 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("arm1.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc1.entity", |asset_storage| EntityTemplate {
			name: Some("misc1"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("arm2.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("arm2.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("arm2.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc2.entity", |asset_storage| EntityTemplate {
			name: Some("misc2"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("bon1.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(6);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bon1.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bon1.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bon1.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bon1.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bon1.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bon1.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc3.entity", |asset_storage| EntityTemplate {
			name: Some("misc3"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("bon2.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(6);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bon2.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bon2.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bon2.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bon2.sprite"),
									frame: 3,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bon2.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bon2.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc4.entity", |asset_storage| EntityTemplate {
			name: Some("misc4"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("bkey.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bkey.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bkey.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc5.entity", |asset_storage| EntityTemplate {
			name: Some("misc5"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("rkey.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("rkey.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("rkey.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc6.entity", |asset_storage| EntityTemplate {
			name: Some("misc6"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("ykey.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ykey.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ykey.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc7.entity", |asset_storage| EntityTemplate {
			name: Some("misc7"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("ysku.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ysku.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ysku.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc8.entity", |asset_storage| EntityTemplate {
			name: Some("misc8"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("rsku.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("rsku.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("rsku.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc9.entity", |asset_storage| EntityTemplate {
			name: Some("misc9"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("bsku.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bsku.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("bsku.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc10.entity", |asset_storage| EntityTemplate {
			name: Some("misc10"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("stim.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("stim.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc11.entity", |asset_storage| EntityTemplate {
			name: Some("misc11"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("medi.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("medi.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc12.entity", |asset_storage| EntityTemplate {
			name: Some("misc12"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("soul.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(6);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("soul.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("soul.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("soul.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("soul.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("soul.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("soul.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("inv.entity", |asset_storage| EntityTemplate {
			name: Some("inv"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pinv.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pinv.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pinv.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pinv.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pinv.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc13.entity", |asset_storage| EntityTemplate {
			name: Some("misc13"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pstr.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("pstr.sprite"),
								frame: 0,
								full_bright: true,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("ins.entity", |asset_storage| EntityTemplate {
			name: Some("ins"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pins.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pins.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pins.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pins.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pins.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc14.entity", |asset_storage| EntityTemplate {
			name: Some("misc14"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("suit.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("suit.sprite"),
								frame: 0,
								full_bright: true,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc15.entity", |asset_storage| EntityTemplate {
			name: Some("misc15"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pmap.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(6);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pmap.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pmap.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pmap.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 4),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pmap.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 5),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pmap.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pmap.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc16.entity", |asset_storage| EntityTemplate {
			name: Some("misc16"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pvis.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pvis.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pvis.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("mega.entity", |asset_storage| EntityTemplate {
			name: Some("mega"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("mega.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("mega.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("mega.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("mega.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("mega.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("clip.entity", |asset_storage| EntityTemplate {
			name: Some("clip"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("clip.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("clip.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc17.entity", |asset_storage| EntityTemplate {
			name: Some("misc17"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("ammo.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("ammo.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc18.entity", |asset_storage| EntityTemplate {
			name: Some("misc18"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("rock.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("rock.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc19.entity", |asset_storage| EntityTemplate {
			name: Some("misc19"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("brok.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("brok.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc20.entity", |asset_storage| EntityTemplate {
			name: Some("misc20"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("cell.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("cell.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc21.entity", |asset_storage| EntityTemplate {
			name: Some("misc21"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("celp.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("celp.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc22.entity", |asset_storage| EntityTemplate {
			name: Some("misc22"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("shel.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("shel.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc23.entity", |asset_storage| EntityTemplate {
			name: Some("misc23"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("sbox.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("sbox.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc24.entity", |asset_storage| EntityTemplate {
			name: Some("misc24"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("bpak.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("bpak.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc25.entity", |asset_storage| EntityTemplate {
			name: Some("misc25"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("bfug.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("bfug.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("chaingun.entity", |asset_storage| EntityTemplate {
			name: Some("chaingun"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("mgun.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("mgun.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc26.entity", |asset_storage| EntityTemplate {
			name: Some("misc26"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("csaw.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("csaw.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc27.entity", |asset_storage| EntityTemplate {
			name: Some("misc27"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("laun.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("laun.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc28.entity", |asset_storage| EntityTemplate {
			name: Some("misc28"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("plas.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("plas.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("shotgun.entity", |asset_storage| EntityTemplate {
			name: Some("shotgun"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("shot.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("shot.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("supershotgun.entity", |asset_storage| EntityTemplate {
			name: Some("supershotgun"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("sgn2.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("sgn2.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc29.entity", |asset_storage| EntityTemplate {
			name: Some("misc29"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("tlmp.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tlmp.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tlmp.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tlmp.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tlmp.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc30.entity", |asset_storage| EntityTemplate {
			name: Some("misc30"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("tlp2.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tlp2.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tlp2.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tlp2.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tlp2.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc31.entity", |asset_storage| EntityTemplate {
			name: Some("misc31"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("colu.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("colu.sprite"),
								frame: 0,
								full_bright: true,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc32.entity", |asset_storage| EntityTemplate {
			name: Some("misc32"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("col1.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("col1.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc33.entity", |asset_storage| EntityTemplate {
			name: Some("misc33"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("col2.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("col2.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc34.entity", |asset_storage| EntityTemplate {
			name: Some("misc34"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("col3.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("col3.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc35.entity", |asset_storage| EntityTemplate {
			name: Some("misc35"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("col4.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("col4.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc36.entity", |asset_storage| EntityTemplate {
			name: Some("misc36"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("col6.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("col6.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc37.entity", |asset_storage| EntityTemplate {
			name: Some("misc37"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("col5.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 14 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("col5.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 14 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("col5.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc38.entity", |asset_storage| EntityTemplate {
			name: Some("misc38"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("ceye.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ceye.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ceye.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ceye.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("ceye.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc39.entity", |asset_storage| EntityTemplate {
			name: Some("misc39"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("fsku.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(3);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fsku.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fsku.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fsku.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc40.entity", |asset_storage| EntityTemplate {
			name: Some("misc40"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("tre1.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("tre1.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc41.entity", |asset_storage| EntityTemplate {
			name: Some("misc41"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("tblu.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tblu.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tblu.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tblu.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tblu.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc42.entity", |asset_storage| EntityTemplate {
			name: Some("misc42"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("tgrn.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tgrn.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tgrn.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tgrn.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tgrn.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc43.entity", |asset_storage| EntityTemplate {
			name: Some("misc43"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("tred.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tred.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tred.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tred.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("tred.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc44.entity", |asset_storage| EntityTemplate {
			name: Some("misc44"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("smbt.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("smbt.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("smbt.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("smbt.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("smbt.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc45.entity", |asset_storage| EntityTemplate {
			name: Some("misc45"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("smgt.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("smgt.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("smgt.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("smgt.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("smgt.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc46.entity", |asset_storage| EntityTemplate {
			name: Some("misc46"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("smrt.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("smrt.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("smrt.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("smrt.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("smrt.sprite"),
									frame: 3,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc47.entity", |asset_storage| EntityTemplate {
			name: Some("misc47"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("smit.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("smit.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc48.entity", |asset_storage| EntityTemplate {
			name: Some("misc48"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("elec.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("elec.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc49.entity", |asset_storage| EntityTemplate {
			name: Some("misc49"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("cand.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("cand.sprite"),
								frame: 0,
								full_bright: true,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc50.entity", |asset_storage| EntityTemplate {
			name: Some("misc50"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("cbra.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("cbra.sprite"),
								frame: 0,
								full_bright: true,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc51.entity", |asset_storage| EntityTemplate {
			name: Some("misc51"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 68.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("gor1.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("gor1.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 15 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("gor1.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("gor1.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("gor1.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc52.entity", |asset_storage| EntityTemplate {
			name: Some("misc52"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 84.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("gor2.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("gor2.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc53.entity", |asset_storage| EntityTemplate {
			name: Some("misc53"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 84.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("gor3.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("gor3.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc54.entity", |asset_storage| EntityTemplate {
			name: Some("misc54"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 68.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("gor4.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("gor4.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc55.entity", |asset_storage| EntityTemplate {
			name: Some("misc55"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 52.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("gor5.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("gor5.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc56.entity", |asset_storage| EntityTemplate {
			name: Some("misc56"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 84.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("gor2.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("gor2.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc57.entity", |asset_storage| EntityTemplate {
			name: Some("misc57"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 68.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("gor4.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("gor4.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc58.entity", |asset_storage| EntityTemplate {
			name: Some("misc58"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 52.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("gor3.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("gor3.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc59.entity", |asset_storage| EntityTemplate {
			name: Some("misc59"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 52.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("gor5.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("gor5.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc60.entity", |asset_storage| EntityTemplate {
			name: Some("misc60"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 68.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("gor1.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(4);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 10 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("gor1.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 15 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("gor1.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 3),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("gor1.sprite"),
									frame: 2,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("gor1.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc61.entity", |asset_storage| EntityTemplate {
			name: Some("misc61"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("head.sprite"),
						frame: 11,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("head.sprite"),
								frame: 11,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc62.entity", |asset_storage| EntityTemplate {
			name: Some("misc62"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("play.sprite"),
						frame: 13,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("play.sprite"),
								frame: 13,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc63.entity", |asset_storage| EntityTemplate {
			name: Some("misc63"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("poss.sprite"),
						frame: 11,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("poss.sprite"),
								frame: 11,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc64.entity", |asset_storage| EntityTemplate {
			name: Some("misc64"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("sarg.sprite"),
						frame: 13,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("sarg.sprite"),
								frame: 13,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc65.entity", |asset_storage| EntityTemplate {
			name: Some("misc65"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("skul.sprite"),
						frame: 10,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("skul.sprite"),
									frame: 10,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((EntityStateEventDef, RemoveEntity));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc66.entity", |asset_storage| EntityTemplate {
			name: Some("misc66"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("troo.sprite"),
						frame: 12,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("troo.sprite"),
								frame: 12,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc67.entity", |asset_storage| EntityTemplate {
			name: Some("misc67"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("spos.sprite"),
						frame: 11,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("spos.sprite"),
								frame: 11,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc68.entity", |asset_storage| EntityTemplate {
			name: Some("misc68"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("play.sprite"),
						frame: 22,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("play.sprite"),
								frame: 22,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc69.entity", |asset_storage| EntityTemplate {
			name: Some("misc69"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("play.sprite"),
						frame: 22,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("play.sprite"),
								frame: 22,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc70.entity", |asset_storage| EntityTemplate {
			name: Some("misc70"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pol2.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("pol2.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc71.entity", |asset_storage| EntityTemplate {
			name: Some("misc71"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pol5.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("pol5.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc72.entity", |asset_storage| EntityTemplate {
			name: Some("misc72"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pol4.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("pol4.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc73.entity", |asset_storage| EntityTemplate {
			name: Some("misc73"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pol3.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pol3.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pol3.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc74.entity", |asset_storage| EntityTemplate {
			name: Some("misc74"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pol1.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("pol1.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc75.entity", |asset_storage| EntityTemplate {
			name: Some("misc75"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pol6.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(2);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 6 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pol6.sprite"),
									frame: 0,
									full_bright: false,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 8 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("pol6.sprite"),
									frame: 1,
									full_bright: false,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc76.entity", |asset_storage| EntityTemplate {
			name: Some("misc76"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 32.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("tre2.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("tre2.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc77.entity", |asset_storage| EntityTemplate {
			name: Some("misc77"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("fcan.sprite"),
						frame: 0,
						full_bright: true,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(3);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 1),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fcan.sprite"),
									frame: 0,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 2),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fcan.sprite"),
									frame: 1,
									full_bright: true,
								}),
							));
							world
						},
						{
							let mut world = World::default();
							world.push((
								EntityStateEventDef,
								NextState {
									time: 4 * FRAME_TIME,
									state: (StateName::from("spawn").unwrap(), 0),
								},
								SetEntitySprite(SpriteRender {
									sprite: asset_storage.load("fcan.sprite"),
									frame: 2,
									full_bright: true,
								}),
							));
							world
						},
					],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc78.entity", |asset_storage| EntityTemplate {
			name: Some("misc78"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 88.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("hdb1.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("hdb1.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc79.entity", |asset_storage| EntityTemplate {
			name: Some("misc79"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 88.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("hdb2.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("hdb2.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc80.entity", |asset_storage| EntityTemplate {
			name: Some("misc80"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 64.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("hdb3.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("hdb3.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc81.entity", |asset_storage| EntityTemplate {
			name: Some("misc81"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 64.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("hdb4.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("hdb4.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc82.entity", |asset_storage| EntityTemplate {
			name: Some("misc82"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 64.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("hdb5.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("hdb5.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc83.entity", |asset_storage| EntityTemplate {
			name: Some("misc83"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 64.0,
						radius: 16.0,
						solid_type: SolidType::MONSTER,
						blocks_types: SolidBits::all(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: false,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("hdb6.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: true,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("hdb6.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc84.entity", |asset_storage| EntityTemplate {
			name: Some("misc84"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pob1.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("pob1.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc85.entity", |asset_storage| EntityTemplate {
			name: Some("misc85"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("pob2.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("pob2.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs.insert("misc86.entity", |asset_storage| EntityTemplate {
			name: Some("misc86"),
			world: {
				let mut world = World::default();
				world.push((
					BoxCollider {
						height: 16.0,
						radius: 20.0,
						solid_type: SolidType::PARTICLE,
						blocks_types: SolidBits::empty(),
						damage_particle: DamageParticle::Blood,
					},
					EntityTemplateRefDef,
					PhysicsDef {
						collision_response: CollisionResponse::Stop,
						gravity: true,
						mass: 100.0,
						speed: 0.0,
					},
					SpriteRender {
						sprite: asset_storage.load("brs1.sprite"),
						frame: 0,
						full_bright: false,
					},
					StateDef,
					TransformDef {
						spawn_on_ceiling: false,
					},
				));
				world
			},
			states: {
				let mut states = HashMap::with_capacity(1);
				states.insert(
					StateName::from("spawn").unwrap(),
					vec![{
						let mut world = World::default();
						world.push((
							EntityStateEventDef,
							SetEntitySprite(SpriteRender {
								sprite: asset_storage.load("brs1.sprite"),
								frame: 0,
								full_bright: false,
							}),
						));
						world
					}],
				);
				states
			},
			..EntityTemplate::default()
		});

		mobjs
	});

pub static DOOMEDNUMS: Lazy<HashMap<u16, &'static str>> = Lazy::new(|| {
	let mut doomednums: HashMap<u16, &'static str> = HashMap::new();
	doomednums.insert(1, "spawn1.entity");
	doomednums.insert(2, "spawn2.entity");
	doomednums.insert(3, "spawn3.entity");
	doomednums.insert(4, "spawn4.entity");
	doomednums.insert(11, "spawndm.entity");
	doomednums.insert(3004, "possessed.entity");
	doomednums.insert(9, "shotguy.entity");
	doomednums.insert(64, "vile.entity");
	doomednums.insert(66, "undead.entity");
	doomednums.insert(67, "fatso.entity");
	doomednums.insert(65, "chainguy.entity");
	doomednums.insert(3001, "troop.entity");
	doomednums.insert(3002, "sergeant.entity");
	doomednums.insert(58, "shadows.entity");
	doomednums.insert(3005, "head.entity");
	doomednums.insert(3003, "bruiser.entity");
	doomednums.insert(69, "knight.entity");
	doomednums.insert(3006, "skull.entity");
	doomednums.insert(7, "spider.entity");
	doomednums.insert(68, "baby.entity");
	doomednums.insert(16, "cyborg.entity");
	doomednums.insert(71, "pain.entity");
	doomednums.insert(84, "wolfss.entity");
	doomednums.insert(72, "keen.entity");
	doomednums.insert(88, "bossbrain.entity");
	doomednums.insert(89, "bossspit.entity");
	doomednums.insert(87, "bosstarget.entity");
	doomednums.insert(2035, "barrel.entity");
	doomednums.insert(14, "teleportman.entity");
	doomednums.insert(2018, "misc0.entity");
	doomednums.insert(2019, "misc1.entity");
	doomednums.insert(2014, "misc2.entity");
	doomednums.insert(2015, "misc3.entity");
	doomednums.insert(5, "misc4.entity");
	doomednums.insert(13, "misc5.entity");
	doomednums.insert(6, "misc6.entity");
	doomednums.insert(39, "misc7.entity");
	doomednums.insert(38, "misc8.entity");
	doomednums.insert(40, "misc9.entity");
	doomednums.insert(2011, "misc10.entity");
	doomednums.insert(2012, "misc11.entity");
	doomednums.insert(2013, "misc12.entity");
	doomednums.insert(2022, "inv.entity");
	doomednums.insert(2023, "misc13.entity");
	doomednums.insert(2024, "ins.entity");
	doomednums.insert(2025, "misc14.entity");
	doomednums.insert(2026, "misc15.entity");
	doomednums.insert(2045, "misc16.entity");
	doomednums.insert(83, "mega.entity");
	doomednums.insert(2007, "clip.entity");
	doomednums.insert(2048, "misc17.entity");
	doomednums.insert(2010, "misc18.entity");
	doomednums.insert(2046, "misc19.entity");
	doomednums.insert(2047, "misc20.entity");
	doomednums.insert(17, "misc21.entity");
	doomednums.insert(2008, "misc22.entity");
	doomednums.insert(2049, "misc23.entity");
	doomednums.insert(8, "misc24.entity");
	doomednums.insert(2006, "misc25.entity");
	doomednums.insert(2002, "chaingun.entity");
	doomednums.insert(2005, "misc26.entity");
	doomednums.insert(2003, "misc27.entity");
	doomednums.insert(2004, "misc28.entity");
	doomednums.insert(2001, "shotgun.entity");
	doomednums.insert(82, "supershotgun.entity");
	doomednums.insert(85, "misc29.entity");
	doomednums.insert(86, "misc30.entity");
	doomednums.insert(2028, "misc31.entity");
	doomednums.insert(30, "misc32.entity");
	doomednums.insert(31, "misc33.entity");
	doomednums.insert(32, "misc34.entity");
	doomednums.insert(33, "misc35.entity");
	doomednums.insert(37, "misc36.entity");
	doomednums.insert(36, "misc37.entity");
	doomednums.insert(41, "misc38.entity");
	doomednums.insert(42, "misc39.entity");
	doomednums.insert(43, "misc40.entity");
	doomednums.insert(44, "misc41.entity");
	doomednums.insert(45, "misc42.entity");
	doomednums.insert(46, "misc43.entity");
	doomednums.insert(55, "misc44.entity");
	doomednums.insert(56, "misc45.entity");
	doomednums.insert(57, "misc46.entity");
	doomednums.insert(47, "misc47.entity");
	doomednums.insert(48, "misc48.entity");
	doomednums.insert(34, "misc49.entity");
	doomednums.insert(35, "misc50.entity");
	doomednums.insert(49, "misc51.entity");
	doomednums.insert(50, "misc52.entity");
	doomednums.insert(51, "misc53.entity");
	doomednums.insert(52, "misc54.entity");
	doomednums.insert(53, "misc55.entity");
	doomednums.insert(59, "misc56.entity");
	doomednums.insert(60, "misc57.entity");
	doomednums.insert(61, "misc58.entity");
	doomednums.insert(62, "misc59.entity");
	doomednums.insert(63, "misc60.entity");
	doomednums.insert(22, "misc61.entity");
	doomednums.insert(15, "misc62.entity");
	doomednums.insert(18, "misc63.entity");
	doomednums.insert(21, "misc64.entity");
	doomednums.insert(23, "misc65.entity");
	doomednums.insert(20, "misc66.entity");
	doomednums.insert(19, "misc67.entity");
	doomednums.insert(10, "misc68.entity");
	doomednums.insert(12, "misc69.entity");
	doomednums.insert(28, "misc70.entity");
	doomednums.insert(24, "misc71.entity");
	doomednums.insert(27, "misc72.entity");
	doomednums.insert(29, "misc73.entity");
	doomednums.insert(25, "misc74.entity");
	doomednums.insert(26, "misc75.entity");
	doomednums.insert(54, "misc76.entity");
	doomednums.insert(70, "misc77.entity");
	doomednums.insert(73, "misc78.entity");
	doomednums.insert(74, "misc79.entity");
	doomednums.insert(75, "misc80.entity");
	doomednums.insert(76, "misc81.entity");
	doomednums.insert(77, "misc82.entity");
	doomednums.insert(78, "misc83.entity");
	doomednums.insert(79, "misc84.entity");
	doomednums.insert(80, "misc85.entity");
	doomednums.insert(81, "misc86.entity");
	doomednums
});
