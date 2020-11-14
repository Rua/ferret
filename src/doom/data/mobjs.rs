#![allow(unused_variables)]
use crate::{
	common::{assets::AssetStorage, frame::FrameRngDef},
	doom::{
		camera::{Camera, MovementBob},
		client::User,
		components::{RandomTransformDef, SpawnPoint, TransformDef},
		data::{FRAME_RATE, FRAME_TIME},
		draw::{sprite::SpriteRender, wsprite::WeaponSpriteRender},
		health::HealthDef,
		physics::{
			BoxCollider, CollisionResponse, DamageParticle, Physics, PhysicsDef, SolidBits,
			SolidType,
		},
		sound::StartSound,
		state::{
			entity::{
				NextState, NextStateRandomTimeDef, RemoveEntity, SetBlocksTypes, SetEntitySprite,
				StateDef,
			},
			weapon::WeaponStateDef,
			EntityDef, StateName,
		},
		template::{EntityTemplate, EntityTemplateRefDef, EntityTypeId},
		WadMode,
	},
};
use legion::{systems::ResourceSet, Read, Resources, World, Write};
use nalgebra::{Vector2, Vector3};
use std::{collections::HashMap, default::Default};

#[rustfmt::skip]
pub fn load(resources: &mut Resources) {
	let (wad_mode, mut asset_storage) = <(Read<WadMode>, Write<AssetStorage>)>::fetch_mut(resources);
	let wad_mode = *wad_mode;

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Thing(1)),
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
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Thing(2)),
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
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Thing(3)),
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
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Thing(4)),
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
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Thing(11)),
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
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		name: Some("player"),
		type_id: None,
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
				},
				EntityTemplateRefDef,
				FrameRngDef,
				HealthDef {
					max: 100.0,
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
				WeaponStateDef,
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(24);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("play.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("play.sprite"),
							frame: 3,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("play.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsplpain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 12 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("play.sprite"),
							frame: 4,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("play.sprite"),
							frame: 13,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("xdeath").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("play.sprite"),
							frame: 15,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsslop.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("play.sprite"),
							frame: 22,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("player", template);

	let template = EntityTemplate {
		name: Some("possessed"),
		type_id: Some(EntityTypeId::Thing(3004)),
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
				FrameRngDef,
				HealthDef {
					max: 20.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("poss.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("poss.sprite"),
							frame: 3,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("poss.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dspopain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("poss.sprite"),
							frame: 4,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("poss.sprite"),
							frame: 8,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dspodth1.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("poss.sprite"),
							frame: 11,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("xdeath").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("poss.sprite"),
							frame: 13,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsslop.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("poss.sprite"),
							frame: 20,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("poss.sprite"),
							frame: 7,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("possessed", template);

	let template = EntityTemplate {
		name: Some("shotguy"),
		type_id: Some(EntityTypeId::Thing(9)),
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
				FrameRngDef,
				HealthDef {
					max: 30.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spos.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spos.sprite"),
							frame: 3,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spos.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dspopain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spos.sprite"),
							frame: 4,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spos.sprite"),
							frame: 8,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dspodth2.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spos.sprite"),
							frame: 11,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("xdeath").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spos.sprite"),
							frame: 13,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsslop.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spos.sprite"),
							frame: 20,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spos.sprite"),
							frame: 7,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("shotguy", template);

	let template = EntityTemplate {
		name: Some("smoke"),
		type_id: None,
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("smoke", template);

	let template = EntityTemplate {
		name: Some("troop"),
		type_id: Some(EntityTypeId::Thing(3001)),
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
				FrameRngDef,
				HealthDef {
					max: 60.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("troo.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("troo.sprite"),
							frame: 3,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("troo.sprite"),
							frame: 7,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dspopain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("melee").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("melee").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("melee").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("troo.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("troo.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("troo.sprite"),
							frame: 9,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsbgdth1.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("troo.sprite"),
							frame: 12,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("xdeath").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("troo.sprite"),
							frame: 14,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsslop.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("troo.sprite"),
							frame: 20,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("troo.sprite"),
							frame: 8,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("troop", template);

	let template = EntityTemplate {
		name: Some("sergeant"),
		type_id: Some(EntityTypeId::Thing(3002)),
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
				FrameRngDef,
				HealthDef {
					max: 150.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 3,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 7,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsdmpain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("melee").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("melee").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("melee").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 9,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dssgtdth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 13,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 8,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("sergeant", template);

	let template = EntityTemplate {
		name: Some("shadows"),
		type_id: Some(EntityTypeId::Thing(58)),
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
				FrameRngDef,
				HealthDef {
					max: 150.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 3,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 7,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsdmpain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("melee").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("melee").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("melee").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 9,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dssgtdth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 13,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 8,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("shadows", template);

	let template = EntityTemplate {
		name: Some("bruiser"),
		type_id: Some(EntityTypeId::Thing(3003)),
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
				FrameRngDef,
				HealthDef {
					max: 1000.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("boss.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("boss.sprite"),
							frame: 3,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("boss.sprite"),
							frame: 7,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsdmpain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("melee").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("melee").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("melee").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("boss.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("boss.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("boss.sprite"),
							frame: 9,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsbrsdth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("boss.sprite"),
							frame: 14,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("boss.sprite"),
							frame: 8,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("bruiser", template);

	let template = EntityTemplate {
		name: Some("bruisershot"),
		type_id: None,
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
				FrameRngDef,
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
				TransformDef {
					spawn_on_ceiling: false,
				},
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(5);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bal7.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bal7.sprite"),
							frame: 2,
							full_bright: true,
						}),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("bruisershot", template);

	let template = EntityTemplate {
		name: Some("barrel"),
		type_id: Some(EntityTypeId::Thing(2035)),
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
				FrameRngDef,
				HealthDef {
					max: 20.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bar1.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bexp.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsbarexp.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("barrel", template);

	let template = EntityTemplate {
		name: Some("troopshot"),
		type_id: None,
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
				FrameRngDef,
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
				TransformDef {
					spawn_on_ceiling: false,
				},
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(5);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bal1.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bal1.sprite"),
							frame: 2,
							full_bright: true,
						}),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("troopshot", template);

	let template = EntityTemplate {
		name: Some("headshot"),
		type_id: None,
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
				FrameRngDef,
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
				TransformDef {
					spawn_on_ceiling: false,
				},
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(5);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bal2.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bal2.sprite"),
							frame: 2,
							full_bright: true,
						}),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("headshot", template);

	let template = EntityTemplate {
		name: Some("rocket"),
		type_id: None,
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
				FrameRngDef,
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
				TransformDef {
					spawn_on_ceiling: false,
				},
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("misl.sprite"),
							frame: 0,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("misl.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("rocket", template);

	let template = EntityTemplate {
		name: Some("arachplaz"),
		type_id: None,
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
				FrameRngDef,
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
				TransformDef {
					spawn_on_ceiling: false,
				},
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(7);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("apls.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("apbx.sprite"),
							frame: 0,
							full_bright: true,
						}),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("arachplaz", template);

	let template = EntityTemplate {
		name: Some("puff1"),
		type_id: None,
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("puff1", template);

	let template = EntityTemplate {
		name: Some("puff3"),
		type_id: None,
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("puff3", template);

	let template = EntityTemplate {
		name: Some("blood1"),
		type_id: None,
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
				FrameRngDef,
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
				RandomTransformDef([(0.0..=0.0).into(), (0.0..=0.0).into(), (-4.0..=4.0).into()]),
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(3);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextStateRandomTimeDef {
							time: (FRAME_TIME..8 * FRAME_TIME).into(),
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("blood1", template);

	let template = EntityTemplate {
		name: Some("blood2"),
		type_id: None,
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
				FrameRngDef,
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
				RandomTransformDef([(0.0..=0.0).into(), (0.0..=0.0).into(), (-4.0..=4.0).into()]),
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(2);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("blood2", template);

	let template = EntityTemplate {
		name: Some("blood3"),
		type_id: None,
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
				FrameRngDef,
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
				RandomTransformDef([(0.0..=0.0).into(), (0.0..=0.0).into(), (-4.0..=4.0).into()]),
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("blood3", template);

	let template = EntityTemplate {
		name: Some("tfog"),
		type_id: None,
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 10),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 11),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 12),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("tfog", template);

	let template = EntityTemplate {
		name: Some("ifog"),
		type_id: None,
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("ifog", template);

	let template = EntityTemplate {
		name: Some("teleportman"),
		type_id: Some(EntityTypeId::Thing(14)),
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
				FrameRngDef,
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
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("teleportman", template);

	let template = EntityTemplate {
		name: Some("misc0"),
		type_id: Some(EntityTypeId::Thing(2018)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("arm1.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc0", template);

	let template = EntityTemplate {
		name: Some("misc1"),
		type_id: Some(EntityTypeId::Thing(2019)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("arm2.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc1", template);

	let template = EntityTemplate {
		name: Some("misc2"),
		type_id: Some(EntityTypeId::Thing(2014)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bon1.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc2", template);

	let template = EntityTemplate {
		name: Some("misc3"),
		type_id: Some(EntityTypeId::Thing(2015)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bon2.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc3", template);

	let template = EntityTemplate {
		name: Some("misc4"),
		type_id: Some(EntityTypeId::Thing(5)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bkey.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc4", template);

	let template = EntityTemplate {
		name: Some("misc5"),
		type_id: Some(EntityTypeId::Thing(13)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("rkey.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc5", template);

	let template = EntityTemplate {
		name: Some("misc6"),
		type_id: Some(EntityTypeId::Thing(6)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("ykey.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc6", template);

	let template = EntityTemplate {
		name: Some("misc10"),
		type_id: Some(EntityTypeId::Thing(2011)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("stim.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc10", template);

	let template = EntityTemplate {
		name: Some("misc11"),
		type_id: Some(EntityTypeId::Thing(2012)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("medi.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc11", template);

	let template = EntityTemplate {
		name: Some("misc12"),
		type_id: Some(EntityTypeId::Thing(2013)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("soul.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc12", template);

	let template = EntityTemplate {
		name: Some("ins"),
		type_id: Some(EntityTypeId::Thing(2024)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pins.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("ins", template);

	let template = EntityTemplate {
		name: Some("misc14"),
		type_id: Some(EntityTypeId::Thing(2025)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("suit.sprite"),
							frame: 0,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc14", template);

	let template = EntityTemplate {
		name: Some("misc15"),
		type_id: Some(EntityTypeId::Thing(2026)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pmap.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc15", template);

	let template = EntityTemplate {
		name: Some("misc16"),
		type_id: Some(EntityTypeId::Thing(2045)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pvis.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc16", template);

	let template = EntityTemplate {
		name: Some("clip"),
		type_id: Some(EntityTypeId::Thing(2007)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("clip.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("clip", template);

	let template = EntityTemplate {
		name: Some("misc17"),
		type_id: Some(EntityTypeId::Thing(2048)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("ammo.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc17", template);

	let template = EntityTemplate {
		name: Some("misc18"),
		type_id: Some(EntityTypeId::Thing(2010)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("rock.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc18", template);

	let template = EntityTemplate {
		name: Some("misc19"),
		type_id: Some(EntityTypeId::Thing(2046)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("brok.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc19", template);

	let template = EntityTemplate {
		name: Some("misc22"),
		type_id: Some(EntityTypeId::Thing(2008)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("shel.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc22", template);

	let template = EntityTemplate {
		name: Some("misc23"),
		type_id: Some(EntityTypeId::Thing(2049)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sbox.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc23", template);

	let template = EntityTemplate {
		name: Some("misc24"),
		type_id: Some(EntityTypeId::Thing(8)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bpak.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc24", template);

	let template = EntityTemplate {
		name: Some("chaingun"),
		type_id: Some(EntityTypeId::Thing(2002)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("mgun.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("chaingun", template);

	let template = EntityTemplate {
		name: Some("misc26"),
		type_id: Some(EntityTypeId::Thing(2005)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("csaw.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc26", template);

	let template = EntityTemplate {
		name: Some("misc27"),
		type_id: Some(EntityTypeId::Thing(2003)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("laun.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc27", template);

	let template = EntityTemplate {
		name: Some("shotgun"),
		type_id: Some(EntityTypeId::Thing(2001)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("shot.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("shotgun", template);

	let template = EntityTemplate {
		name: Some("misc31"),
		type_id: Some(EntityTypeId::Thing(2028)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("colu.sprite"),
							frame: 0,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc31", template);

	let template = EntityTemplate {
		name: Some("misc43"),
		type_id: Some(EntityTypeId::Thing(46)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("tred.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc43", template);

	let template = EntityTemplate {
		name: Some("misc48"),
		type_id: Some(EntityTypeId::Thing(48)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("elec.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc48", template);

	let template = EntityTemplate {
		name: Some("misc49"),
		type_id: Some(EntityTypeId::Thing(34)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cand.sprite"),
							frame: 0,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc49", template);

	let template = EntityTemplate {
		name: Some("misc50"),
		type_id: Some(EntityTypeId::Thing(35)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cbra.sprite"),
							frame: 0,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc50", template);

	let template = EntityTemplate {
		name: Some("misc62"),
		type_id: Some(EntityTypeId::Thing(15)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("play.sprite"),
							frame: 13,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc62", template);

	let template = EntityTemplate {
		name: Some("misc63"),
		type_id: Some(EntityTypeId::Thing(18)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("poss.sprite"),
							frame: 11,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc63", template);

	let template = EntityTemplate {
		name: Some("misc64"),
		type_id: Some(EntityTypeId::Thing(21)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sarg.sprite"),
							frame: 13,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc64", template);

	let template = EntityTemplate {
		name: Some("misc66"),
		type_id: Some(EntityTypeId::Thing(20)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("troo.sprite"),
							frame: 12,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc66", template);

	let template = EntityTemplate {
		name: Some("misc67"),
		type_id: Some(EntityTypeId::Thing(19)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spos.sprite"),
							frame: 11,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc67", template);

	let template = EntityTemplate {
		name: Some("misc68"),
		type_id: Some(EntityTypeId::Thing(10)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("play.sprite"),
							frame: 22,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc68", template);

	let template = EntityTemplate {
		name: Some("misc69"),
		type_id: Some(EntityTypeId::Thing(12)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("play.sprite"),
							frame: 22,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc69", template);

	let template = EntityTemplate {
		name: Some("misc71"),
		type_id: Some(EntityTypeId::Thing(24)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pol5.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc71", template);

	if wad_mode < WadMode::Doom1 {
		return;
	}

	let template = EntityTemplate {
		name: Some("head"),
		type_id: Some(EntityTypeId::Thing(3005)),
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
				FrameRngDef,
				HealthDef {
					max: 400.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("head.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("head.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("head.sprite"),
							frame: 4,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsdmpain.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("head.sprite"),
							frame: 5,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("head.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("head.sprite"),
							frame: 7,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dscacdth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("head.sprite"),
							frame: 11,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("head.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("head", template);

	let template = EntityTemplate {
		name: Some("skull"),
		type_id: Some(EntityTypeId::Thing(3006)),
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
				FrameRngDef,
				HealthDef {
					max: 100.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("skul.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("skul.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("skul.sprite"),
							frame: 4,
							full_bright: true,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsdmpain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("skul.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("skul.sprite"),
							frame: 6,
							full_bright: true,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsfirxpl.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("skull", template);

	let template = EntityTemplate {
		name: Some("spider"),
		type_id: Some(EntityTypeId::Thing(7)),
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
				FrameRngDef,
				HealthDef {
					max: 3000.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spid.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 10),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 11),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spid.sprite"),
							frame: 5,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spid.sprite"),
							frame: 8,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsdmpain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 20 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spid.sprite"),
							frame: 7,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 20 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spid.sprite"),
							frame: 9,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsspidth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 30 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 10),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("spid.sprite"),
							frame: 18,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("spider", template);

	let template = EntityTemplate {
		name: Some("cyborg"),
		type_id: Some(EntityTypeId::Thing(16)),
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
				FrameRngDef,
				HealthDef {
					max: 4000.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cybr.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cybr.sprite"),
							frame: 3,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cybr.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsdmpain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 12 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 12 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 12 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 12 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 12 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cybr.sprite"),
							frame: 5,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cybr.sprite"),
							frame: 8,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dscybdth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 30 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cybr.sprite"),
							frame: 15,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("cyborg", template);

	let template = EntityTemplate {
		name: Some("plasma"),
		type_id: None,
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
				FrameRngDef,
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
				TransformDef {
					spawn_on_ceiling: false,
				},
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(7);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("plss.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("plse.sprite"),
							frame: 0,
							full_bright: true,
						}),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("plasma", template);

	let template = EntityTemplate {
		name: Some("bfg"),
		type_id: None,
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
				FrameRngDef,
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
				TransformDef {
					spawn_on_ceiling: false,
				},
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(8);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bfs1.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bfe1.sprite"),
							frame: 0,
							full_bright: true,
						}),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bfe1.sprite"),
							frame: 2,
							full_bright: true,
						}),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("bfg", template);

	let template = EntityTemplate {
		name: Some("extrabfg"),
		type_id: None,
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("extrabfg", template);

	let template = EntityTemplate {
		name: Some("misc7"),
		type_id: Some(EntityTypeId::Thing(39)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("ysku.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc7", template);

	let template = EntityTemplate {
		name: Some("misc8"),
		type_id: Some(EntityTypeId::Thing(38)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("rsku.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc8", template);

	let template = EntityTemplate {
		name: Some("misc9"),
		type_id: Some(EntityTypeId::Thing(40)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bsku.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc9", template);

	let template = EntityTemplate {
		name: Some("inv"),
		type_id: Some(EntityTypeId::Thing(2022)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pinv.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("inv", template);

	let template = EntityTemplate {
		name: Some("misc13"),
		type_id: Some(EntityTypeId::Thing(2023)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pstr.sprite"),
							frame: 0,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc13", template);

	let template = EntityTemplate {
		name: Some("misc20"),
		type_id: Some(EntityTypeId::Thing(2047)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cell.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc20", template);

	let template = EntityTemplate {
		name: Some("misc21"),
		type_id: Some(EntityTypeId::Thing(17)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("celp.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc21", template);

	let template = EntityTemplate {
		name: Some("misc25"),
		type_id: Some(EntityTypeId::Thing(2006)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bfug.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc25", template);

	let template = EntityTemplate {
		name: Some("misc28"),
		type_id: Some(EntityTypeId::Thing(2004)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("plas.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc28", template);

	let template = EntityTemplate {
		name: Some("misc32"),
		type_id: Some(EntityTypeId::Thing(30)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("col1.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc32", template);

	let template = EntityTemplate {
		name: Some("misc33"),
		type_id: Some(EntityTypeId::Thing(31)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("col2.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc33", template);

	let template = EntityTemplate {
		name: Some("misc34"),
		type_id: Some(EntityTypeId::Thing(32)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("col3.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc34", template);

	let template = EntityTemplate {
		name: Some("misc35"),
		type_id: Some(EntityTypeId::Thing(33)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("col4.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc35", template);

	let template = EntityTemplate {
		name: Some("misc36"),
		type_id: Some(EntityTypeId::Thing(37)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("col6.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc36", template);

	let template = EntityTemplate {
		name: Some("misc37"),
		type_id: Some(EntityTypeId::Thing(36)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 14 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 14 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("col5.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc37", template);

	let template = EntityTemplate {
		name: Some("misc38"),
		type_id: Some(EntityTypeId::Thing(41)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("ceye.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc38", template);

	let template = EntityTemplate {
		name: Some("misc39"),
		type_id: Some(EntityTypeId::Thing(42)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("fsku.sprite"),
							frame: 2,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc39", template);

	let template = EntityTemplate {
		name: Some("misc40"),
		type_id: Some(EntityTypeId::Thing(43)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("tre1.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc40", template);

	let template = EntityTemplate {
		name: Some("misc41"),
		type_id: Some(EntityTypeId::Thing(44)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("tblu.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc41", template);

	let template = EntityTemplate {
		name: Some("misc42"),
		type_id: Some(EntityTypeId::Thing(45)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("tgrn.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc42", template);

	let template = EntityTemplate {
		name: Some("misc44"),
		type_id: Some(EntityTypeId::Thing(55)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("smbt.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc44", template);

	let template = EntityTemplate {
		name: Some("misc45"),
		type_id: Some(EntityTypeId::Thing(56)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("smgt.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc45", template);

	let template = EntityTemplate {
		name: Some("misc46"),
		type_id: Some(EntityTypeId::Thing(57)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("smrt.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc46", template);

	let template = EntityTemplate {
		name: Some("misc47"),
		type_id: Some(EntityTypeId::Thing(47)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("smit.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc47", template);

	let template = EntityTemplate {
		name: Some("misc51"),
		type_id: Some(EntityTypeId::Thing(49)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 15 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("gor1.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc51", template);

	let template = EntityTemplate {
		name: Some("misc52"),
		type_id: Some(EntityTypeId::Thing(50)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("gor2.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc52", template);

	let template = EntityTemplate {
		name: Some("misc53"),
		type_id: Some(EntityTypeId::Thing(51)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("gor3.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc53", template);

	let template = EntityTemplate {
		name: Some("misc54"),
		type_id: Some(EntityTypeId::Thing(52)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("gor4.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc54", template);

	let template = EntityTemplate {
		name: Some("misc55"),
		type_id: Some(EntityTypeId::Thing(53)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("gor5.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc55", template);

	let template = EntityTemplate {
		name: Some("misc56"),
		type_id: Some(EntityTypeId::Thing(59)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("gor2.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc56", template);

	let template = EntityTemplate {
		name: Some("misc57"),
		type_id: Some(EntityTypeId::Thing(60)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("gor4.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc57", template);

	let template = EntityTemplate {
		name: Some("misc58"),
		type_id: Some(EntityTypeId::Thing(61)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("gor3.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc58", template);

	let template = EntityTemplate {
		name: Some("misc59"),
		type_id: Some(EntityTypeId::Thing(62)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("gor5.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc59", template);

	let template = EntityTemplate {
		name: Some("misc60"),
		type_id: Some(EntityTypeId::Thing(63)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 15 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("gor1.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc60", template);

	let template = EntityTemplate {
		name: Some("misc61"),
		type_id: Some(EntityTypeId::Thing(22)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("head.sprite"),
							frame: 11,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc61", template);

	let template = EntityTemplate {
		name: Some("misc65"),
		type_id: Some(EntityTypeId::Thing(23)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc65", template);

	let template = EntityTemplate {
		name: Some("misc70"),
		type_id: Some(EntityTypeId::Thing(28)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pol2.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc70", template);

	let template = EntityTemplate {
		name: Some("misc72"),
		type_id: Some(EntityTypeId::Thing(27)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pol4.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc72", template);

	let template = EntityTemplate {
		name: Some("misc73"),
		type_id: Some(EntityTypeId::Thing(29)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pol3.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc73", template);

	let template = EntityTemplate {
		name: Some("misc74"),
		type_id: Some(EntityTypeId::Thing(25)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pol1.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc74", template);

	let template = EntityTemplate {
		name: Some("misc75"),
		type_id: Some(EntityTypeId::Thing(26)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pol6.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc75", template);

	let template = EntityTemplate {
		name: Some("misc76"),
		type_id: Some(EntityTypeId::Thing(54)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("tre2.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc76", template);

	let template = EntityTemplate {
		name: Some("misc77"),
		type_id: Some(EntityTypeId::Thing(70)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("fcan.sprite"),
							frame: 2,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc77", template);

	if wad_mode < WadMode::Doom2 {
		return;
	}

	let template = EntityTemplate {
		name: Some("vile"),
		type_id: Some(EntityTypeId::Thing(64)),
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
				FrameRngDef,
				HealthDef {
					max: 700.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("vile.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 10),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 11),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("vile.sprite"),
							frame: 5,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("vile.sprite"),
							frame: 16,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsvipain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 0 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 10),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 20 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("vile.sprite"),
							frame: 15,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("vile.sprite"),
							frame: 17,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsvildth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("vile.sprite"),
							frame: 25,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("vile", template);

	let template = EntityTemplate {
		name: Some("fire"),
		type_id: None,
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 10),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 11),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 12),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 13),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 14),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 15),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 16),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 17),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 18),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 19),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 20),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 21),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 22),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 23),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 24),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 25),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 26),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 27),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 28),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 29),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 30),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("fire", template);

	let template = EntityTemplate {
		name: Some("undead"),
		type_id: Some(EntityTypeId::Thing(66)),
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
				FrameRngDef,
				HealthDef {
					max: 300.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("skel.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 10),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 11),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("skel.sprite"),
							frame: 5,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("skel.sprite"),
							frame: 11,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dspopain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("melee").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 0 * FRAME_TIME,
							state: (StateName::from("melee").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("melee").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("melee").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("skel.sprite"),
							frame: 8,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 0 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("skel.sprite"),
							frame: 10,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("skel.sprite"),
							frame: 13,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsskedth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("skel.sprite"),
							frame: 16,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("skel.sprite"),
							frame: 11,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("undead", template);

	let template = EntityTemplate {
		name: Some("tracer"),
		type_id: None,
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
				FrameRngDef,
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
				TransformDef {
					spawn_on_ceiling: false,
				},
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(5);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("fatb.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("fbxp.sprite"),
							frame: 0,
							full_bright: true,
						}),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("tracer", template);

	let template = EntityTemplate {
		name: Some("fatso"),
		type_id: Some(EntityTypeId::Thing(67)),
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
				FrameRngDef,
				HealthDef {
					max: 600.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 15 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 15 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("fatt.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 10),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 11),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("fatt.sprite"),
							frame: 5,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("fatt.sprite"),
							frame: 9,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsmnpain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 20 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("fatt.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("fatt.sprite"),
							frame: 11,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsmandth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("fatt.sprite"),
							frame: 19,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("fatt.sprite"),
							frame: 10,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("fatso", template);

	let template = EntityTemplate {
		name: Some("fatshot"),
		type_id: None,
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
				FrameRngDef,
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
				TransformDef {
					spawn_on_ceiling: false,
				},
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(5);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("manf.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("misl.sprite"),
							frame: 1,
							full_bright: true,
						}),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("fatshot", template);

	let template = EntityTemplate {
		name: Some("chainguy"),
		type_id: Some(EntityTypeId::Thing(65)),
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
				FrameRngDef,
				HealthDef {
					max: 70.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cpos.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cpos.sprite"),
							frame: 3,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cpos.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dspopain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cpos.sprite"),
							frame: 5,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cpos.sprite"),
							frame: 8,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dspodth2.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cpos.sprite"),
							frame: 13,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("xdeath").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cpos.sprite"),
							frame: 15,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsslop.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cpos.sprite"),
							frame: 19,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("cpos.sprite"),
							frame: 7,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("chainguy", template);

	let template = EntityTemplate {
		name: Some("knight"),
		type_id: Some(EntityTypeId::Thing(69)),
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
				FrameRngDef,
				HealthDef {
					max: 500.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bos2.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bos2.sprite"),
							frame: 3,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 2 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bos2.sprite"),
							frame: 7,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsdmpain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("melee").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("melee").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("melee").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bos2.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bos2.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bos2.sprite"),
							frame: 9,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dskntdth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bos2.sprite"),
							frame: 14,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bos2.sprite"),
							frame: 8,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("knight", template);

	let template = EntityTemplate {
		name: Some("baby"),
		type_id: Some(EntityTypeId::Thing(68)),
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
				FrameRngDef,
				HealthDef {
					max: 500.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bspi.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 20 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 10),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 11),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 12),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bspi.sprite"),
							frame: 5,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bspi.sprite"),
							frame: 8,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsdmpain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 20 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bspi.sprite"),
							frame: 7,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 20 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bspi.sprite"),
							frame: 9,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsbspdth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 7 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bspi.sprite"),
							frame: 15,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bspi.sprite"),
							frame: 9,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("baby", template);

	let template = EntityTemplate {
		name: Some("pain"),
		type_id: Some(EntityTypeId::Thing(71)),
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
				FrameRngDef,
				HealthDef {
					max: 400.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pain.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pain.sprite"),
							frame: 2,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pain.sprite"),
							frame: 6,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dspepain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 0 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pain.sprite"),
							frame: 5,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pain.sprite"),
							frame: 8,
							full_bright: true,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dspedth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
					world.push((
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pain.sprite"),
							frame: 7,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("pain", template);

	let template = EntityTemplate {
		name: Some("wolfss"),
		type_id: Some(EntityTypeId::Thing(84)),
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
				FrameRngDef,
				HealthDef {
					max: 50.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sswv.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sswv.sprite"),
							frame: 3,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sswv.sprite"),
							frame: 7,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dspopain.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("missile").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 1 * FRAME_TIME,
							state: (StateName::from("missile").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sswv.sprite"),
							frame: 5,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sswv.sprite"),
							frame: 9,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsssdth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sswv.sprite"),
							frame: 12,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("xdeath").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sswv.sprite"),
							frame: 14,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsslop.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetBlocksTypes(SolidBits::empty()),
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("xdeath").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sswv.sprite"),
							frame: 21,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("raise").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("raise").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 5 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sswv.sprite"),
							frame: 8,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("wolfss", template);

	let template = EntityTemplate {
		name: Some("keen"),
		type_id: Some(EntityTypeId::Thing(72)),
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
				FrameRngDef,
				HealthDef {
					max: 100.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("keen.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("pain").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 8 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("keen.sprite"),
							frame: 12,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dskeenpn.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("keen.sprite"),
							frame: 2,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dskeendt.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 9),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 10),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 11),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("keen.sprite"),
							frame: 11,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("keen", template);

	let template = EntityTemplate {
		name: Some("bossbrain"),
		type_id: Some(EntityTypeId::Thing(88)),
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
				FrameRngDef,
				HealthDef {
					max: 250.0,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bbrn.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("pain").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 36 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bbrn.sprite"),
							frame: 1,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsbospn.sound")),
					));
					world
				},
			]);
			states.insert(StateName::from("death").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 100 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bbrn.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world.push((
						EntityDef,
						StartSound(asset_storage.load("dsbosdth.sound")),
					));
					world
				},
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("death").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bbrn.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("bossbrain", template);

	let template = EntityTemplate {
		name: Some("bossspit"),
		type_id: Some(EntityTypeId::Thing(89)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 10 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sswv.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states.insert(StateName::from("see").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 181 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 150 * FRAME_TIME,
							state: (StateName::from("see").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sswv.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("bossspit", template);

	let template = EntityTemplate {
		name: Some("bosstarget"),
		type_id: Some(EntityTypeId::Thing(87)),
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
				FrameRngDef,
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
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("bosstarget", template);

	let template = EntityTemplate {
		name: Some("spawnshot"),
		type_id: None,
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
				FrameRngDef,
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
				TransformDef {
					spawn_on_ceiling: false,
				},
			));
			world
		},
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 3 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("bosf.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("spawnshot", template);

	let template = EntityTemplate {
		name: Some("spawnfire"),
		type_id: None,
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 4),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 5),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 6),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 7),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 8),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						RemoveEntity,
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("spawnfire", template);

	let template = EntityTemplate {
		name: Some("mega"),
		type_id: Some(EntityTypeId::Thing(83)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 6 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("mega.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("mega", template);

	let template = EntityTemplate {
		name: Some("supershotgun"),
		type_id: Some(EntityTypeId::Thing(82)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("sgn2.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("supershotgun", template);

	let template = EntityTemplate {
		name: Some("misc29"),
		type_id: Some(EntityTypeId::Thing(85)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("tlmp.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc29", template);

	let template = EntityTemplate {
		name: Some("misc30"),
		type_id: Some(EntityTypeId::Thing(86)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 1),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 2),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 3),
						},
					));
					world.push((
						EntityDef,
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
						EntityDef,
						NextState {
							time: 4 * FRAME_TIME,
							state: (StateName::from("spawn").unwrap(), 0),
						},
					));
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("tlp2.sprite"),
							frame: 3,
							full_bright: true,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc30", template);

	let template = EntityTemplate {
		name: Some("misc78"),
		type_id: Some(EntityTypeId::Thing(73)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("hdb1.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc78", template);

	let template = EntityTemplate {
		name: Some("misc79"),
		type_id: Some(EntityTypeId::Thing(74)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("hdb2.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc79", template);

	let template = EntityTemplate {
		name: Some("misc80"),
		type_id: Some(EntityTypeId::Thing(75)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("hdb3.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc80", template);

	let template = EntityTemplate {
		name: Some("misc81"),
		type_id: Some(EntityTypeId::Thing(76)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("hdb4.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc81", template);

	let template = EntityTemplate {
		name: Some("misc82"),
		type_id: Some(EntityTypeId::Thing(77)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("hdb5.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc82", template);

	let template = EntityTemplate {
		name: Some("misc83"),
		type_id: Some(EntityTypeId::Thing(78)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("hdb6.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc83", template);

	let template = EntityTemplate {
		name: Some("misc84"),
		type_id: Some(EntityTypeId::Thing(79)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pob1.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc84", template);

	let template = EntityTemplate {
		name: Some("misc85"),
		type_id: Some(EntityTypeId::Thing(80)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("pob2.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc85", template);

	let template = EntityTemplate {
		name: Some("misc86"),
		type_id: Some(EntityTypeId::Thing(81)),
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
				FrameRngDef,
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
			states.insert(StateName::from("spawn").unwrap(), vec![
				{
					let mut world = World::default();
					world.push((
						EntityDef,
						SetEntitySprite(SpriteRender {
							sprite: asset_storage.load("brs1.sprite"),
							frame: 0,
							full_bright: false,
						}),
					));
					world
				},
			]);
			states
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc86", template);
}
