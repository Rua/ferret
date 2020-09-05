#![allow(unused_variables)]
use crate::{
	common::{assets::AssetStorage, component::EntityComponents, geometry::Angle, timer::Timer},
	doom::{
		camera::Camera,
		client::User,
		components::{SpawnOnCeiling, SpawnPoint, Velocity},
		data::FRAME_TIME,
		entitytemplate::{EntityTemplate, EntityTypeId},
		physics::{BoxCollider, SolidMask},
		render::sprite::SpriteRender,
		state::{State, StateDef},
	},
};
use legion::prelude::{ResourceSet, Resources, Write};
use nalgebra::Vector3;
use std::{collections::HashMap, default::Default, sync::Arc, time::Duration};

#[rustfmt::skip]
pub fn load(resources: &mut Resources) {
	let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(1)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 1 }),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(2)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 2 }),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(3)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 3 }),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(4)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 4 }),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(11)),
		components: EntityComponents::new(),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: Some("PLAYER"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(Camera {
				base: Vector3::new(0.0, 0.0, 41.0),
				offset: Vector3::zeros(),
				bob_angle: Angle::default(),
				bob_max: 16.0,
				bob_period: 20 * FRAME_TIME,
				deviation_position: 0.0,
				deviation_velocity: 0.0,
				impact_sound: asset_storage.load("DSOOF"),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PLAY"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(24);
					states.insert("PLAY".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 0, full_bright: false},
						next: None,
					});
					states.insert("PLAY_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 0, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("PLAY_RUN2".to_owned()))),
					});
					states.insert("PLAY_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("PLAY_RUN3".to_owned()))),
					});
					states.insert("PLAY_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("PLAY_RUN4".to_owned()))),
					});
					states.insert("PLAY_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("PLAY_RUN1".to_owned()))),
					});
					states.insert("PLAY_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 4, full_bright: false},
						next: Some((12 * FRAME_TIME, Some("PLAY".to_owned()))),
					});
					states.insert("PLAY_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 6, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("PLAY_PAIN2".to_owned()))),
					});
					states.insert("PLAY_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 6, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("PLAY".to_owned()))),
					});
					states.insert("PLAY_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 7, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("PLAY_DIE2".to_owned()))),
					});
					states.insert("PLAY_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 8, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("PLAY_DIE3".to_owned()))),
					});
					states.insert("PLAY_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 9, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("PLAY_DIE4".to_owned()))),
					});
					states.insert("PLAY_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 10, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("PLAY_DIE5".to_owned()))),
					});
					states.insert("PLAY_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 11, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("PLAY_DIE6".to_owned()))),
					});
					states.insert("PLAY_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 12, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("PLAY_DIE7".to_owned()))),
					});
					states.insert("PLAY_DIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 13, full_bright: false},
						next: None,
					});
					states.insert("PLAY_XDIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("PLAY_XDIE2".to_owned()))),
					});
					states.insert("PLAY_XDIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("PLAY_XDIE3".to_owned()))),
					});
					states.insert("PLAY_XDIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("PLAY_XDIE4".to_owned()))),
					});
					states.insert("PLAY_XDIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("PLAY_XDIE5".to_owned()))),
					});
					states.insert("PLAY_XDIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 18, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("PLAY_XDIE6".to_owned()))),
					});
					states.insert("PLAY_XDIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 19, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("PLAY_XDIE7".to_owned()))),
					});
					states.insert("PLAY_XDIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 20, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("PLAY_XDIE8".to_owned()))),
					});
					states.insert("PLAY_XDIE8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 21, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("PLAY_XDIE9".to_owned()))),
					});
					states.insert("PLAY_XDIE9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 22, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("PLAY".to_owned()))),
				spawn_state: Some("PLAY".to_owned()),
				see_state: Some("PLAY_RUN1".to_owned()),
				pain_state: Some("PLAY_PAIN".to_owned()),
				melee_state: None,
				missile_state: Some("PLAY_ATK1".to_owned()),
				death_state: Some("PLAY_DIE1".to_owned()),
				xdeath_state: Some("PLAY_XDIE1".to_owned()),
				raise_state: None,
			})
			.with_component(User {
				error_sound: asset_storage.load("DSNOWAY"),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("PLAYER", template);

	let template = EntityTemplate {
		name: Some("POSSESSED"),
		type_id: Some(EntityTypeId::Thing(3004)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POSS"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(33);
					states.insert("POSS_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("POSS_STND2".to_owned()))),
					});
					states.insert("POSS_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("POSS_STND".to_owned()))),
					});
					states.insert("POSS_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 0, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("POSS_RUN2".to_owned()))),
					});
					states.insert("POSS_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 0, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("POSS_RUN3".to_owned()))),
					});
					states.insert("POSS_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("POSS_RUN4".to_owned()))),
					});
					states.insert("POSS_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("POSS_RUN5".to_owned()))),
					});
					states.insert("POSS_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("POSS_RUN6".to_owned()))),
					});
					states.insert("POSS_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("POSS_RUN7".to_owned()))),
					});
					states.insert("POSS_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("POSS_RUN8".to_owned()))),
					});
					states.insert("POSS_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("POSS_RUN1".to_owned()))),
					});
					states.insert("POSS_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 4, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("POSS_ATK2".to_owned()))),
					});
					states.insert("POSS_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 5, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("POSS_ATK3".to_owned()))),
					});
					states.insert("POSS_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 4, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("POSS_RUN1".to_owned()))),
					});
					states.insert("POSS_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 6, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("POSS_PAIN2".to_owned()))),
					});
					states.insert("POSS_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 6, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("POSS_RUN1".to_owned()))),
					});
					states.insert("POSS_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 7, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_DIE2".to_owned()))),
					});
					states.insert("POSS_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_DIE3".to_owned()))),
					});
					states.insert("POSS_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_DIE4".to_owned()))),
					});
					states.insert("POSS_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_DIE5".to_owned()))),
					});
					states.insert("POSS_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 11, full_bright: false},
						next: None,
					});
					states.insert("POSS_XDIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_XDIE2".to_owned()))),
					});
					states.insert("POSS_XDIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_XDIE3".to_owned()))),
					});
					states.insert("POSS_XDIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_XDIE4".to_owned()))),
					});
					states.insert("POSS_XDIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_XDIE5".to_owned()))),
					});
					states.insert("POSS_XDIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_XDIE6".to_owned()))),
					});
					states.insert("POSS_XDIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_XDIE7".to_owned()))),
					});
					states.insert("POSS_XDIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 18, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_XDIE8".to_owned()))),
					});
					states.insert("POSS_XDIE8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 19, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_XDIE9".to_owned()))),
					});
					states.insert("POSS_XDIE9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 20, full_bright: false},
						next: None,
					});
					states.insert("POSS_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_RAISE2".to_owned()))),
					});
					states.insert("POSS_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_RAISE3".to_owned()))),
					});
					states.insert("POSS_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_RAISE4".to_owned()))),
					});
					states.insert("POSS_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 7, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("POSS_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("POSS_STND".to_owned()))),
				spawn_state: Some("POSS_STND".to_owned()),
				see_state: Some("POSS_RUN1".to_owned()),
				pain_state: Some("POSS_PAIN".to_owned()),
				melee_state: None,
				missile_state: Some("POSS_ATK1".to_owned()),
				death_state: Some("POSS_DIE1".to_owned()),
				xdeath_state: Some("POSS_XDIE1".to_owned()),
				raise_state: Some("POSS_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("POSSESSED", template);

	let template = EntityTemplate {
		name: Some("SHOTGUY"),
		type_id: Some(EntityTypeId::Thing(9)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SPOS"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(34);
					states.insert("SPOS_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPOS_STND2".to_owned()))),
					});
					states.insert("SPOS_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPOS_STND".to_owned()))),
					});
					states.insert("SPOS_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPOS_RUN2".to_owned()))),
					});
					states.insert("SPOS_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPOS_RUN3".to_owned()))),
					});
					states.insert("SPOS_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPOS_RUN4".to_owned()))),
					});
					states.insert("SPOS_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPOS_RUN5".to_owned()))),
					});
					states.insert("SPOS_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPOS_RUN6".to_owned()))),
					});
					states.insert("SPOS_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPOS_RUN7".to_owned()))),
					});
					states.insert("SPOS_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPOS_RUN8".to_owned()))),
					});
					states.insert("SPOS_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPOS_RUN1".to_owned()))),
					});
					states.insert("SPOS_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 4, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPOS_ATK2".to_owned()))),
					});
					states.insert("SPOS_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 5, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("SPOS_ATK3".to_owned()))),
					});
					states.insert("SPOS_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 4, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPOS_RUN1".to_owned()))),
					});
					states.insert("SPOS_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 6, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPOS_PAIN2".to_owned()))),
					});
					states.insert("SPOS_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 6, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPOS_RUN1".to_owned()))),
					});
					states.insert("SPOS_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 7, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_DIE2".to_owned()))),
					});
					states.insert("SPOS_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_DIE3".to_owned()))),
					});
					states.insert("SPOS_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_DIE4".to_owned()))),
					});
					states.insert("SPOS_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_DIE5".to_owned()))),
					});
					states.insert("SPOS_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 11, full_bright: false},
						next: None,
					});
					states.insert("SPOS_XDIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_XDIE2".to_owned()))),
					});
					states.insert("SPOS_XDIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_XDIE3".to_owned()))),
					});
					states.insert("SPOS_XDIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_XDIE4".to_owned()))),
					});
					states.insert("SPOS_XDIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_XDIE5".to_owned()))),
					});
					states.insert("SPOS_XDIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_XDIE6".to_owned()))),
					});
					states.insert("SPOS_XDIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_XDIE7".to_owned()))),
					});
					states.insert("SPOS_XDIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 18, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_XDIE8".to_owned()))),
					});
					states.insert("SPOS_XDIE8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 19, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_XDIE9".to_owned()))),
					});
					states.insert("SPOS_XDIE9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 20, full_bright: false},
						next: None,
					});
					states.insert("SPOS_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_RAISE2".to_owned()))),
					});
					states.insert("SPOS_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_RAISE3".to_owned()))),
					});
					states.insert("SPOS_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_RAISE4".to_owned()))),
					});
					states.insert("SPOS_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_RAISE5".to_owned()))),
					});
					states.insert("SPOS_RAISE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 7, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SPOS_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SPOS_STND".to_owned()))),
				spawn_state: Some("SPOS_STND".to_owned()),
				see_state: Some("SPOS_RUN1".to_owned()),
				pain_state: Some("SPOS_PAIN".to_owned()),
				melee_state: None,
				missile_state: Some("SPOS_ATK1".to_owned()),
				death_state: Some("SPOS_DIE1".to_owned()),
				xdeath_state: Some("SPOS_XDIE1".to_owned()),
				raise_state: Some("SPOS_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SHOTGUY", template);

	let template = EntityTemplate {
		name: Some("VILE"),
		type_id: Some(EntityTypeId::Thing(64)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("VILE"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(37);
					states.insert("VILE_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("VILE_STND2".to_owned()))),
					});
					states.insert("VILE_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("VILE_STND".to_owned()))),
					});
					states.insert("VILE_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("VILE_RUN2".to_owned()))),
					});
					states.insert("VILE_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("VILE_RUN3".to_owned()))),
					});
					states.insert("VILE_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("VILE_RUN4".to_owned()))),
					});
					states.insert("VILE_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("VILE_RUN5".to_owned()))),
					});
					states.insert("VILE_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("VILE_RUN6".to_owned()))),
					});
					states.insert("VILE_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("VILE_RUN7".to_owned()))),
					});
					states.insert("VILE_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("VILE_RUN8".to_owned()))),
					});
					states.insert("VILE_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("VILE_RUN9".to_owned()))),
					});
					states.insert("VILE_RUN9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 4, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("VILE_RUN10".to_owned()))),
					});
					states.insert("VILE_RUN10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 4, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("VILE_RUN11".to_owned()))),
					});
					states.insert("VILE_RUN11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 5, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("VILE_RUN12".to_owned()))),
					});
					states.insert("VILE_RUN12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 5, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("VILE_RUN1".to_owned()))),
					});
					states.insert("VILE_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 6, full_bright: true},
						next: Some((0 * FRAME_TIME, Some("VILE_ATK2".to_owned()))),
					});
					states.insert("VILE_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 6, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("VILE_ATK3".to_owned()))),
					});
					states.insert("VILE_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 7, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("VILE_ATK4".to_owned()))),
					});
					states.insert("VILE_ATK4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 8, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("VILE_ATK5".to_owned()))),
					});
					states.insert("VILE_ATK5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 9, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("VILE_ATK6".to_owned()))),
					});
					states.insert("VILE_ATK6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 10, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("VILE_ATK7".to_owned()))),
					});
					states.insert("VILE_ATK7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 11, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("VILE_ATK8".to_owned()))),
					});
					states.insert("VILE_ATK8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 12, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("VILE_ATK9".to_owned()))),
					});
					states.insert("VILE_ATK9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 13, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("VILE_ATK10".to_owned()))),
					});
					states.insert("VILE_ATK10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 14, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("VILE_ATK11".to_owned()))),
					});
					states.insert("VILE_ATK11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 15, full_bright: true},
						next: Some((20 * FRAME_TIME, Some("VILE_RUN1".to_owned()))),
					});
					states.insert("VILE_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("VILE_PAIN2".to_owned()))),
					});
					states.insert("VILE_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("VILE_RUN1".to_owned()))),
					});
					states.insert("VILE_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 16, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("VILE_DIE2".to_owned()))),
					});
					states.insert("VILE_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 17, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("VILE_DIE3".to_owned()))),
					});
					states.insert("VILE_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 18, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("VILE_DIE4".to_owned()))),
					});
					states.insert("VILE_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 19, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("VILE_DIE5".to_owned()))),
					});
					states.insert("VILE_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 20, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("VILE_DIE6".to_owned()))),
					});
					states.insert("VILE_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 21, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("VILE_DIE7".to_owned()))),
					});
					states.insert("VILE_DIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 22, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("VILE_DIE8".to_owned()))),
					});
					states.insert("VILE_DIE8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 23, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("VILE_DIE9".to_owned()))),
					});
					states.insert("VILE_DIE9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 24, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("VILE_DIE10".to_owned()))),
					});
					states.insert("VILE_DIE10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("VILE"), frame: 25, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("VILE_STND".to_owned()))),
				spawn_state: Some("VILE_STND".to_owned()),
				see_state: Some("VILE_RUN1".to_owned()),
				pain_state: Some("VILE_PAIN".to_owned()),
				melee_state: None,
				missile_state: Some("VILE_ATK1".to_owned()),
				death_state: Some("VILE_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("VILE", template);

	let template = EntityTemplate {
		name: Some("FIRE"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("FIRE"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(30);
					states.insert("FIRE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 0, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE2".to_owned()))),
					});
					states.insert("FIRE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 1, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE3".to_owned()))),
					});
					states.insert("FIRE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 0, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE4".to_owned()))),
					});
					states.insert("FIRE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 1, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE5".to_owned()))),
					});
					states.insert("FIRE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 2, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE6".to_owned()))),
					});
					states.insert("FIRE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 1, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE7".to_owned()))),
					});
					states.insert("FIRE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 2, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE8".to_owned()))),
					});
					states.insert("FIRE8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 1, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE9".to_owned()))),
					});
					states.insert("FIRE9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 2, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE10".to_owned()))),
					});
					states.insert("FIRE10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 3, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE11".to_owned()))),
					});
					states.insert("FIRE11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 2, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE12".to_owned()))),
					});
					states.insert("FIRE12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 3, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE13".to_owned()))),
					});
					states.insert("FIRE13".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 2, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE14".to_owned()))),
					});
					states.insert("FIRE14".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 3, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE15".to_owned()))),
					});
					states.insert("FIRE15".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 4, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE16".to_owned()))),
					});
					states.insert("FIRE16".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 3, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE17".to_owned()))),
					});
					states.insert("FIRE17".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 4, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE18".to_owned()))),
					});
					states.insert("FIRE18".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 3, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE19".to_owned()))),
					});
					states.insert("FIRE19".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 4, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE20".to_owned()))),
					});
					states.insert("FIRE20".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 5, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE21".to_owned()))),
					});
					states.insert("FIRE21".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 4, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE22".to_owned()))),
					});
					states.insert("FIRE22".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 5, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE23".to_owned()))),
					});
					states.insert("FIRE23".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 4, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE24".to_owned()))),
					});
					states.insert("FIRE24".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 5, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE25".to_owned()))),
					});
					states.insert("FIRE25".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 6, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE26".to_owned()))),
					});
					states.insert("FIRE26".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 7, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE27".to_owned()))),
					});
					states.insert("FIRE27".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 6, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE28".to_owned()))),
					});
					states.insert("FIRE28".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 7, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE29".to_owned()))),
					});
					states.insert("FIRE29".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 6, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("FIRE30".to_owned()))),
					});
					states.insert("FIRE30".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 7, full_bright: true},
						next: Some((2 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("FIRE1".to_owned()))),
				spawn_state: Some("FIRE1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("FIRE", template);

	let template = EntityTemplate {
		name: Some("UNDEAD"),
		type_id: Some(EntityTypeId::Thing(66)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SKEL"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(36);
					states.insert("SKEL_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SKEL_STND2".to_owned()))),
					});
					states.insert("SKEL_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SKEL_STND".to_owned()))),
					});
					states.insert("SKEL_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SKEL_RUN2".to_owned()))),
					});
					states.insert("SKEL_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SKEL_RUN3".to_owned()))),
					});
					states.insert("SKEL_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SKEL_RUN4".to_owned()))),
					});
					states.insert("SKEL_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SKEL_RUN5".to_owned()))),
					});
					states.insert("SKEL_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SKEL_RUN6".to_owned()))),
					});
					states.insert("SKEL_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SKEL_RUN7".to_owned()))),
					});
					states.insert("SKEL_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SKEL_RUN8".to_owned()))),
					});
					states.insert("SKEL_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SKEL_RUN9".to_owned()))),
					});
					states.insert("SKEL_RUN9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 4, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SKEL_RUN10".to_owned()))),
					});
					states.insert("SKEL_RUN10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 4, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SKEL_RUN11".to_owned()))),
					});
					states.insert("SKEL_RUN11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 5, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SKEL_RUN12".to_owned()))),
					});
					states.insert("SKEL_RUN12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 5, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SKEL_RUN1".to_owned()))),
					});
					states.insert("SKEL_FIST1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 6, full_bright: false},
						next: Some((0 * FRAME_TIME, Some("SKEL_FIST2".to_owned()))),
					});
					states.insert("SKEL_FIST2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 6, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("SKEL_FIST3".to_owned()))),
					});
					states.insert("SKEL_FIST3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 7, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("SKEL_FIST4".to_owned()))),
					});
					states.insert("SKEL_FIST4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 8, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("SKEL_RUN1".to_owned()))),
					});
					states.insert("SKEL_MISS1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 9, full_bright: true},
						next: Some((0 * FRAME_TIME, Some("SKEL_MISS2".to_owned()))),
					});
					states.insert("SKEL_MISS2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 9, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("SKEL_MISS3".to_owned()))),
					});
					states.insert("SKEL_MISS3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 10, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SKEL_MISS4".to_owned()))),
					});
					states.insert("SKEL_MISS4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 10, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SKEL_RUN1".to_owned()))),
					});
					states.insert("SKEL_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SKEL_PAIN2".to_owned()))),
					});
					states.insert("SKEL_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SKEL_RUN1".to_owned()))),
					});
					states.insert("SKEL_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 11, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("SKEL_DIE2".to_owned()))),
					});
					states.insert("SKEL_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 12, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("SKEL_DIE3".to_owned()))),
					});
					states.insert("SKEL_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 13, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("SKEL_DIE4".to_owned()))),
					});
					states.insert("SKEL_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 14, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("SKEL_DIE5".to_owned()))),
					});
					states.insert("SKEL_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 15, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("SKEL_DIE6".to_owned()))),
					});
					states.insert("SKEL_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 16, full_bright: false},
						next: None,
					});
					states.insert("SKEL_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SKEL_RAISE2".to_owned()))),
					});
					states.insert("SKEL_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SKEL_RAISE3".to_owned()))),
					});
					states.insert("SKEL_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SKEL_RAISE4".to_owned()))),
					});
					states.insert("SKEL_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SKEL_RAISE5".to_owned()))),
					});
					states.insert("SKEL_RAISE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SKEL_RAISE6".to_owned()))),
					});
					states.insert("SKEL_RAISE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKEL"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SKEL_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SKEL_STND".to_owned()))),
				spawn_state: Some("SKEL_STND".to_owned()),
				see_state: Some("SKEL_RUN1".to_owned()),
				pain_state: Some("SKEL_PAIN".to_owned()),
				melee_state: Some("SKEL_FIST1".to_owned()),
				missile_state: Some("SKEL_MISS1".to_owned()),
				death_state: Some("SKEL_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: Some("SKEL_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("UNDEAD", template);

	let template = EntityTemplate {
		name: Some("TRACER"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("FATB"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(5);
					states.insert("TRACER".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATB"), frame: 0, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("TRACER2".to_owned()))),
					});
					states.insert("TRACER2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATB"), frame: 1, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("TRACER".to_owned()))),
					});
					states.insert("TRACEEXP1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FBXP"), frame: 0, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("TRACEEXP2".to_owned()))),
					});
					states.insert("TRACEEXP2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FBXP"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TRACEEXP3".to_owned()))),
					});
					states.insert("TRACEEXP3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FBXP"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("TRACER".to_owned()))),
				spawn_state: Some("TRACER".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("TRACEEXP1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("TRACER", template);

	let template = EntityTemplate {
		name: Some("SMOKE"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("PUFF"),
				frame: 1,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(5);
					states.insert("SMOKE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PUFF"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("SMOKE2".to_owned()))),
					});
					states.insert("SMOKE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PUFF"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("SMOKE3".to_owned()))),
					});
					states.insert("SMOKE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PUFF"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("SMOKE4".to_owned()))),
					});
					states.insert("SMOKE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PUFF"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("SMOKE5".to_owned()))),
					});
					states.insert("SMOKE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PUFF"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SMOKE1".to_owned()))),
				spawn_state: Some("SMOKE1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SMOKE", template);

	let template = EntityTemplate {
		name: Some("FATSO"),
		type_id: Some(EntityTypeId::Thing(67)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 48.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("FATT"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(44);
					states.insert("FATT_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 0, full_bright: false},
						next: Some((15 * FRAME_TIME, Some("FATT_STND2".to_owned()))),
					});
					states.insert("FATT_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 1, full_bright: false},
						next: Some((15 * FRAME_TIME, Some("FATT_STND".to_owned()))),
					});
					states.insert("FATT_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 0, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("FATT_RUN2".to_owned()))),
					});
					states.insert("FATT_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 0, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("FATT_RUN3".to_owned()))),
					});
					states.insert("FATT_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("FATT_RUN4".to_owned()))),
					});
					states.insert("FATT_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("FATT_RUN5".to_owned()))),
					});
					states.insert("FATT_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("FATT_RUN6".to_owned()))),
					});
					states.insert("FATT_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("FATT_RUN7".to_owned()))),
					});
					states.insert("FATT_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("FATT_RUN8".to_owned()))),
					});
					states.insert("FATT_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("FATT_RUN9".to_owned()))),
					});
					states.insert("FATT_RUN9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 4, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("FATT_RUN10".to_owned()))),
					});
					states.insert("FATT_RUN10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 4, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("FATT_RUN11".to_owned()))),
					});
					states.insert("FATT_RUN11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 5, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("FATT_RUN12".to_owned()))),
					});
					states.insert("FATT_RUN12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 5, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("FATT_RUN1".to_owned()))),
					});
					states.insert("FATT_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 6, full_bright: false},
						next: Some((20 * FRAME_TIME, Some("FATT_ATK2".to_owned()))),
					});
					states.insert("FATT_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 7, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("FATT_ATK3".to_owned()))),
					});
					states.insert("FATT_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_ATK4".to_owned()))),
					});
					states.insert("FATT_ATK4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 6, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_ATK5".to_owned()))),
					});
					states.insert("FATT_ATK5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 7, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("FATT_ATK6".to_owned()))),
					});
					states.insert("FATT_ATK6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_ATK7".to_owned()))),
					});
					states.insert("FATT_ATK7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 6, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_ATK8".to_owned()))),
					});
					states.insert("FATT_ATK8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 7, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("FATT_ATK9".to_owned()))),
					});
					states.insert("FATT_ATK9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_ATK10".to_owned()))),
					});
					states.insert("FATT_ATK10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 6, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_RUN1".to_owned()))),
					});
					states.insert("FATT_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 9, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("FATT_PAIN2".to_owned()))),
					});
					states.insert("FATT_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 9, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("FATT_RUN1".to_owned()))),
					});
					states.insert("FATT_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 10, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("FATT_DIE2".to_owned()))),
					});
					states.insert("FATT_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 11, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("FATT_DIE3".to_owned()))),
					});
					states.insert("FATT_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 12, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("FATT_DIE4".to_owned()))),
					});
					states.insert("FATT_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 13, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("FATT_DIE5".to_owned()))),
					});
					states.insert("FATT_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 14, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("FATT_DIE6".to_owned()))),
					});
					states.insert("FATT_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 15, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("FATT_DIE7".to_owned()))),
					});
					states.insert("FATT_DIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 16, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("FATT_DIE8".to_owned()))),
					});
					states.insert("FATT_DIE8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 17, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("FATT_DIE9".to_owned()))),
					});
					states.insert("FATT_DIE9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 18, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("FATT_DIE10".to_owned()))),
					});
					states.insert("FATT_DIE10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 19, full_bright: false},
						next: None,
					});
					states.insert("FATT_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_RAISE2".to_owned()))),
					});
					states.insert("FATT_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_RAISE3".to_owned()))),
					});
					states.insert("FATT_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_RAISE4".to_owned()))),
					});
					states.insert("FATT_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_RAISE5".to_owned()))),
					});
					states.insert("FATT_RAISE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_RAISE6".to_owned()))),
					});
					states.insert("FATT_RAISE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_RAISE7".to_owned()))),
					});
					states.insert("FATT_RAISE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_RAISE8".to_owned()))),
					});
					states.insert("FATT_RAISE8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FATT"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("FATT_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("FATT_STND".to_owned()))),
				spawn_state: Some("FATT_STND".to_owned()),
				see_state: Some("FATT_RUN1".to_owned()),
				pain_state: Some("FATT_PAIN".to_owned()),
				melee_state: None,
				missile_state: Some("FATT_ATK1".to_owned()),
				death_state: Some("FATT_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: Some("FATT_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("FATSO", template);

	let template = EntityTemplate {
		name: Some("FATSHOT"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("MANF"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(5);
					states.insert("FATSHOT1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MANF"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("FATSHOT2".to_owned()))),
					});
					states.insert("FATSHOT2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MANF"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("FATSHOT1".to_owned()))),
					});
					states.insert("FATSHOTX1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MISL"), frame: 1, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("FATSHOTX2".to_owned()))),
					});
					states.insert("FATSHOTX2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MISL"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("FATSHOTX3".to_owned()))),
					});
					states.insert("FATSHOTX3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MISL"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("FATSHOT1".to_owned()))),
				spawn_state: Some("FATSHOT1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("FATSHOTX1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("FATSHOT", template);

	let template = EntityTemplate {
		name: Some("CHAINGUY"),
		type_id: Some(EntityTypeId::Thing(65)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CPOS"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(36);
					states.insert("CPOS_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CPOS_STND2".to_owned()))),
					});
					states.insert("CPOS_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CPOS_STND".to_owned()))),
					});
					states.insert("CPOS_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CPOS_RUN2".to_owned()))),
					});
					states.insert("CPOS_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CPOS_RUN3".to_owned()))),
					});
					states.insert("CPOS_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CPOS_RUN4".to_owned()))),
					});
					states.insert("CPOS_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CPOS_RUN5".to_owned()))),
					});
					states.insert("CPOS_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CPOS_RUN6".to_owned()))),
					});
					states.insert("CPOS_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CPOS_RUN7".to_owned()))),
					});
					states.insert("CPOS_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CPOS_RUN8".to_owned()))),
					});
					states.insert("CPOS_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CPOS_RUN1".to_owned()))),
					});
					states.insert("CPOS_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 4, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CPOS_ATK2".to_owned()))),
					});
					states.insert("CPOS_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 5, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("CPOS_ATK3".to_owned()))),
					});
					states.insert("CPOS_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 4, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("CPOS_ATK4".to_owned()))),
					});
					states.insert("CPOS_ATK4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 5, full_bright: false},
						next: Some((1 * FRAME_TIME, Some("CPOS_ATK2".to_owned()))),
					});
					states.insert("CPOS_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 6, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CPOS_PAIN2".to_owned()))),
					});
					states.insert("CPOS_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 6, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CPOS_RUN1".to_owned()))),
					});
					states.insert("CPOS_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 7, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_DIE2".to_owned()))),
					});
					states.insert("CPOS_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_DIE3".to_owned()))),
					});
					states.insert("CPOS_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_DIE4".to_owned()))),
					});
					states.insert("CPOS_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_DIE5".to_owned()))),
					});
					states.insert("CPOS_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_DIE6".to_owned()))),
					});
					states.insert("CPOS_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_DIE7".to_owned()))),
					});
					states.insert("CPOS_DIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 13, full_bright: false},
						next: None,
					});
					states.insert("CPOS_XDIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_XDIE2".to_owned()))),
					});
					states.insert("CPOS_XDIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_XDIE3".to_owned()))),
					});
					states.insert("CPOS_XDIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_XDIE4".to_owned()))),
					});
					states.insert("CPOS_XDIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_XDIE5".to_owned()))),
					});
					states.insert("CPOS_XDIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 18, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_XDIE6".to_owned()))),
					});
					states.insert("CPOS_XDIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 19, full_bright: false},
						next: None,
					});
					states.insert("CPOS_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_RAISE2".to_owned()))),
					});
					states.insert("CPOS_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_RAISE3".to_owned()))),
					});
					states.insert("CPOS_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_RAISE4".to_owned()))),
					});
					states.insert("CPOS_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_RAISE5".to_owned()))),
					});
					states.insert("CPOS_RAISE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_RAISE6".to_owned()))),
					});
					states.insert("CPOS_RAISE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_RAISE7".to_owned()))),
					});
					states.insert("CPOS_RAISE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CPOS"), frame: 7, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("CPOS_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("CPOS_STND".to_owned()))),
				spawn_state: Some("CPOS_STND".to_owned()),
				see_state: Some("CPOS_RUN1".to_owned()),
				pain_state: Some("CPOS_PAIN".to_owned()),
				melee_state: None,
				missile_state: Some("CPOS_ATK1".to_owned()),
				death_state: Some("CPOS_DIE1".to_owned()),
				xdeath_state: Some("CPOS_XDIE1".to_owned()),
				raise_state: Some("CPOS_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("CHAINGUY", template);

	let template = EntityTemplate {
		name: Some("TROOP"),
		type_id: Some(EntityTypeId::Thing(3001)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TROO"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(33);
					states.insert("TROO_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("TROO_STND2".to_owned()))),
					});
					states.insert("TROO_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("TROO_STND".to_owned()))),
					});
					states.insert("TROO_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("TROO_RUN2".to_owned()))),
					});
					states.insert("TROO_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("TROO_RUN3".to_owned()))),
					});
					states.insert("TROO_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("TROO_RUN4".to_owned()))),
					});
					states.insert("TROO_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("TROO_RUN5".to_owned()))),
					});
					states.insert("TROO_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("TROO_RUN6".to_owned()))),
					});
					states.insert("TROO_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("TROO_RUN7".to_owned()))),
					});
					states.insert("TROO_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("TROO_RUN8".to_owned()))),
					});
					states.insert("TROO_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("TROO_RUN1".to_owned()))),
					});
					states.insert("TROO_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 4, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("TROO_ATK2".to_owned()))),
					});
					states.insert("TROO_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 5, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("TROO_ATK3".to_owned()))),
					});
					states.insert("TROO_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 6, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("TROO_RUN1".to_owned()))),
					});
					states.insert("TROO_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("TROO_PAIN2".to_owned()))),
					});
					states.insert("TROO_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("TROO_RUN1".to_owned()))),
					});
					states.insert("TROO_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("TROO_DIE2".to_owned()))),
					});
					states.insert("TROO_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("TROO_DIE3".to_owned()))),
					});
					states.insert("TROO_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 10, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("TROO_DIE4".to_owned()))),
					});
					states.insert("TROO_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 11, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("TROO_DIE5".to_owned()))),
					});
					states.insert("TROO_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 12, full_bright: false},
						next: None,
					});
					states.insert("TROO_XDIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("TROO_XDIE2".to_owned()))),
					});
					states.insert("TROO_XDIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("TROO_XDIE3".to_owned()))),
					});
					states.insert("TROO_XDIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("TROO_XDIE4".to_owned()))),
					});
					states.insert("TROO_XDIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("TROO_XDIE5".to_owned()))),
					});
					states.insert("TROO_XDIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("TROO_XDIE6".to_owned()))),
					});
					states.insert("TROO_XDIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 18, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("TROO_XDIE7".to_owned()))),
					});
					states.insert("TROO_XDIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 19, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("TROO_XDIE8".to_owned()))),
					});
					states.insert("TROO_XDIE8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 20, full_bright: false},
						next: None,
					});
					states.insert("TROO_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("TROO_RAISE2".to_owned()))),
					});
					states.insert("TROO_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("TROO_RAISE3".to_owned()))),
					});
					states.insert("TROO_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 10, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("TROO_RAISE4".to_owned()))),
					});
					states.insert("TROO_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 9, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("TROO_RAISE5".to_owned()))),
					});
					states.insert("TROO_RAISE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 8, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("TROO_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("TROO_STND".to_owned()))),
				spawn_state: Some("TROO_STND".to_owned()),
				see_state: Some("TROO_RUN1".to_owned()),
				pain_state: Some("TROO_PAIN".to_owned()),
				melee_state: Some("TROO_ATK1".to_owned()),
				missile_state: Some("TROO_ATK1".to_owned()),
				death_state: Some("TROO_DIE1".to_owned()),
				xdeath_state: Some("TROO_XDIE1".to_owned()),
				raise_state: Some("TROO_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("TROOP", template);

	let template = EntityTemplate {
		name: Some("SERGEANT"),
		type_id: Some(EntityTypeId::Thing(3002)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 30.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SARG"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(27);
					states.insert("SARG_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SARG_STND2".to_owned()))),
					});
					states.insert("SARG_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SARG_STND".to_owned()))),
					});
					states.insert("SARG_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN2".to_owned()))),
					});
					states.insert("SARG_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN3".to_owned()))),
					});
					states.insert("SARG_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN4".to_owned()))),
					});
					states.insert("SARG_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN5".to_owned()))),
					});
					states.insert("SARG_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN6".to_owned()))),
					});
					states.insert("SARG_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN7".to_owned()))),
					});
					states.insert("SARG_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN8".to_owned()))),
					});
					states.insert("SARG_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN1".to_owned()))),
					});
					states.insert("SARG_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 4, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("SARG_ATK2".to_owned()))),
					});
					states.insert("SARG_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 5, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("SARG_ATK3".to_owned()))),
					});
					states.insert("SARG_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 6, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("SARG_RUN1".to_owned()))),
					});
					states.insert("SARG_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_PAIN2".to_owned()))),
					});
					states.insert("SARG_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN1".to_owned()))),
					});
					states.insert("SARG_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("SARG_DIE2".to_owned()))),
					});
					states.insert("SARG_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("SARG_DIE3".to_owned()))),
					});
					states.insert("SARG_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 10, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("SARG_DIE4".to_owned()))),
					});
					states.insert("SARG_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 11, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("SARG_DIE5".to_owned()))),
					});
					states.insert("SARG_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 12, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("SARG_DIE6".to_owned()))),
					});
					states.insert("SARG_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 13, full_bright: false},
						next: None,
					});
					states.insert("SARG_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SARG_RAISE2".to_owned()))),
					});
					states.insert("SARG_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SARG_RAISE3".to_owned()))),
					});
					states.insert("SARG_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SARG_RAISE4".to_owned()))),
					});
					states.insert("SARG_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SARG_RAISE5".to_owned()))),
					});
					states.insert("SARG_RAISE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SARG_RAISE6".to_owned()))),
					});
					states.insert("SARG_RAISE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SARG_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SARG_STND".to_owned()))),
				spawn_state: Some("SARG_STND".to_owned()),
				see_state: Some("SARG_RUN1".to_owned()),
				pain_state: Some("SARG_PAIN".to_owned()),
				melee_state: Some("SARG_ATK1".to_owned()),
				missile_state: None,
				death_state: Some("SARG_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: Some("SARG_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SERGEANT", template);

	let template = EntityTemplate {
		name: Some("SHADOWS"),
		type_id: Some(EntityTypeId::Thing(58)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 30.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SARG"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(27);
					states.insert("SARG_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SARG_STND2".to_owned()))),
					});
					states.insert("SARG_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SARG_STND".to_owned()))),
					});
					states.insert("SARG_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN2".to_owned()))),
					});
					states.insert("SARG_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN3".to_owned()))),
					});
					states.insert("SARG_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN4".to_owned()))),
					});
					states.insert("SARG_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN5".to_owned()))),
					});
					states.insert("SARG_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN6".to_owned()))),
					});
					states.insert("SARG_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN7".to_owned()))),
					});
					states.insert("SARG_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN8".to_owned()))),
					});
					states.insert("SARG_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN1".to_owned()))),
					});
					states.insert("SARG_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 4, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("SARG_ATK2".to_owned()))),
					});
					states.insert("SARG_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 5, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("SARG_ATK3".to_owned()))),
					});
					states.insert("SARG_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 6, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("SARG_RUN1".to_owned()))),
					});
					states.insert("SARG_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_PAIN2".to_owned()))),
					});
					states.insert("SARG_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("SARG_RUN1".to_owned()))),
					});
					states.insert("SARG_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("SARG_DIE2".to_owned()))),
					});
					states.insert("SARG_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("SARG_DIE3".to_owned()))),
					});
					states.insert("SARG_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 10, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("SARG_DIE4".to_owned()))),
					});
					states.insert("SARG_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 11, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("SARG_DIE5".to_owned()))),
					});
					states.insert("SARG_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 12, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("SARG_DIE6".to_owned()))),
					});
					states.insert("SARG_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 13, full_bright: false},
						next: None,
					});
					states.insert("SARG_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SARG_RAISE2".to_owned()))),
					});
					states.insert("SARG_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SARG_RAISE3".to_owned()))),
					});
					states.insert("SARG_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SARG_RAISE4".to_owned()))),
					});
					states.insert("SARG_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SARG_RAISE5".to_owned()))),
					});
					states.insert("SARG_RAISE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SARG_RAISE6".to_owned()))),
					});
					states.insert("SARG_RAISE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SARG_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SARG_STND".to_owned()))),
				spawn_state: Some("SARG_STND".to_owned()),
				see_state: Some("SARG_RUN1".to_owned()),
				pain_state: Some("SARG_PAIN".to_owned()),
				melee_state: Some("SARG_ATK1".to_owned()),
				missile_state: None,
				death_state: Some("SARG_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: Some("SARG_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SHADOWS", template);

	let template = EntityTemplate {
		name: Some("HEAD"),
		type_id: Some(EntityTypeId::Thing(3005)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 31.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HEAD"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(20);
					states.insert("HEAD_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("HEAD_STND".to_owned()))),
					});
					states.insert("HEAD_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("HEAD_RUN1".to_owned()))),
					});
					states.insert("HEAD_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 1, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("HEAD_ATK2".to_owned()))),
					});
					states.insert("HEAD_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 2, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("HEAD_ATK3".to_owned()))),
					});
					states.insert("HEAD_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 3, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("HEAD_RUN1".to_owned()))),
					});
					states.insert("HEAD_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 4, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("HEAD_PAIN2".to_owned()))),
					});
					states.insert("HEAD_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 4, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("HEAD_PAIN3".to_owned()))),
					});
					states.insert("HEAD_PAIN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 5, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("HEAD_RUN1".to_owned()))),
					});
					states.insert("HEAD_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 6, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("HEAD_DIE2".to_owned()))),
					});
					states.insert("HEAD_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 7, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("HEAD_DIE3".to_owned()))),
					});
					states.insert("HEAD_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("HEAD_DIE4".to_owned()))),
					});
					states.insert("HEAD_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("HEAD_DIE5".to_owned()))),
					});
					states.insert("HEAD_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("HEAD_DIE6".to_owned()))),
					});
					states.insert("HEAD_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 11, full_bright: false},
						next: None,
					});
					states.insert("HEAD_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("HEAD_RAISE2".to_owned()))),
					});
					states.insert("HEAD_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("HEAD_RAISE3".to_owned()))),
					});
					states.insert("HEAD_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("HEAD_RAISE4".to_owned()))),
					});
					states.insert("HEAD_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("HEAD_RAISE5".to_owned()))),
					});
					states.insert("HEAD_RAISE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 7, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("HEAD_RAISE6".to_owned()))),
					});
					states.insert("HEAD_RAISE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 6, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("HEAD_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("HEAD_STND".to_owned()))),
				spawn_state: Some("HEAD_STND".to_owned()),
				see_state: Some("HEAD_RUN1".to_owned()),
				pain_state: Some("HEAD_PAIN".to_owned()),
				melee_state: None,
				missile_state: Some("HEAD_ATK1".to_owned()),
				death_state: Some("HEAD_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: Some("HEAD_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("HEAD", template);

	let template = EntityTemplate {
		name: Some("BRUISER"),
		type_id: Some(EntityTypeId::Thing(3003)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 24.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BOSS"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(29);
					states.insert("BOSS_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("BOSS_STND2".to_owned()))),
					});
					states.insert("BOSS_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("BOSS_STND".to_owned()))),
					});
					states.insert("BOSS_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOSS_RUN2".to_owned()))),
					});
					states.insert("BOSS_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOSS_RUN3".to_owned()))),
					});
					states.insert("BOSS_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOSS_RUN4".to_owned()))),
					});
					states.insert("BOSS_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOSS_RUN5".to_owned()))),
					});
					states.insert("BOSS_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOSS_RUN6".to_owned()))),
					});
					states.insert("BOSS_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOSS_RUN7".to_owned()))),
					});
					states.insert("BOSS_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOSS_RUN8".to_owned()))),
					});
					states.insert("BOSS_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOSS_RUN1".to_owned()))),
					});
					states.insert("BOSS_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 4, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_ATK2".to_owned()))),
					});
					states.insert("BOSS_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 5, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_ATK3".to_owned()))),
					});
					states.insert("BOSS_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 6, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_RUN1".to_owned()))),
					});
					states.insert("BOSS_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("BOSS_PAIN2".to_owned()))),
					});
					states.insert("BOSS_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("BOSS_RUN1".to_owned()))),
					});
					states.insert("BOSS_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_DIE2".to_owned()))),
					});
					states.insert("BOSS_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_DIE3".to_owned()))),
					});
					states.insert("BOSS_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_DIE4".to_owned()))),
					});
					states.insert("BOSS_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_DIE5".to_owned()))),
					});
					states.insert("BOSS_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_DIE6".to_owned()))),
					});
					states.insert("BOSS_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 13, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_DIE7".to_owned()))),
					});
					states.insert("BOSS_DIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 14, full_bright: false},
						next: None,
					});
					states.insert("BOSS_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 14, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_RAISE2".to_owned()))),
					});
					states.insert("BOSS_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 13, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_RAISE3".to_owned()))),
					});
					states.insert("BOSS_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_RAISE4".to_owned()))),
					});
					states.insert("BOSS_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_RAISE5".to_owned()))),
					});
					states.insert("BOSS_RAISE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_RAISE6".to_owned()))),
					});
					states.insert("BOSS_RAISE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_RAISE7".to_owned()))),
					});
					states.insert("BOSS_RAISE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSS"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOSS_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BOSS_STND".to_owned()))),
				spawn_state: Some("BOSS_STND".to_owned()),
				see_state: Some("BOSS_RUN1".to_owned()),
				pain_state: Some("BOSS_PAIN".to_owned()),
				melee_state: Some("BOSS_ATK1".to_owned()),
				missile_state: Some("BOSS_ATK1".to_owned()),
				death_state: Some("BOSS_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: Some("BOSS_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BRUISER", template);

	let template = EntityTemplate {
		name: Some("BRUISERSHOT"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BAL7"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(5);
					states.insert("BRBALL1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL7"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BRBALL2".to_owned()))),
					});
					states.insert("BRBALL2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL7"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BRBALL1".to_owned()))),
					});
					states.insert("BRBALLX1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL7"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("BRBALLX2".to_owned()))),
					});
					states.insert("BRBALLX2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL7"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("BRBALLX3".to_owned()))),
					});
					states.insert("BRBALLX3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL7"), frame: 4, full_bright: true},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BRBALL1".to_owned()))),
				spawn_state: Some("BRBALL1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("BRBALLX1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BRUISERSHOT", template);

	let template = EntityTemplate {
		name: Some("KNIGHT"),
		type_id: Some(EntityTypeId::Thing(69)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 24.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BOS2"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(29);
					states.insert("BOS2_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("BOS2_STND2".to_owned()))),
					});
					states.insert("BOS2_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("BOS2_STND".to_owned()))),
					});
					states.insert("BOS2_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOS2_RUN2".to_owned()))),
					});
					states.insert("BOS2_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOS2_RUN3".to_owned()))),
					});
					states.insert("BOS2_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOS2_RUN4".to_owned()))),
					});
					states.insert("BOS2_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOS2_RUN5".to_owned()))),
					});
					states.insert("BOS2_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOS2_RUN6".to_owned()))),
					});
					states.insert("BOS2_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOS2_RUN7".to_owned()))),
					});
					states.insert("BOS2_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOS2_RUN8".to_owned()))),
					});
					states.insert("BOS2_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BOS2_RUN1".to_owned()))),
					});
					states.insert("BOS2_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 4, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_ATK2".to_owned()))),
					});
					states.insert("BOS2_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 5, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_ATK3".to_owned()))),
					});
					states.insert("BOS2_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 6, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_RUN1".to_owned()))),
					});
					states.insert("BOS2_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("BOS2_PAIN2".to_owned()))),
					});
					states.insert("BOS2_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("BOS2_RUN1".to_owned()))),
					});
					states.insert("BOS2_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_DIE2".to_owned()))),
					});
					states.insert("BOS2_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_DIE3".to_owned()))),
					});
					states.insert("BOS2_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_DIE4".to_owned()))),
					});
					states.insert("BOS2_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_DIE5".to_owned()))),
					});
					states.insert("BOS2_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_DIE6".to_owned()))),
					});
					states.insert("BOS2_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 13, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_DIE7".to_owned()))),
					});
					states.insert("BOS2_DIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 14, full_bright: false},
						next: None,
					});
					states.insert("BOS2_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 14, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_RAISE2".to_owned()))),
					});
					states.insert("BOS2_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 13, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_RAISE3".to_owned()))),
					});
					states.insert("BOS2_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_RAISE4".to_owned()))),
					});
					states.insert("BOS2_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_RAISE5".to_owned()))),
					});
					states.insert("BOS2_RAISE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_RAISE6".to_owned()))),
					});
					states.insert("BOS2_RAISE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_RAISE7".to_owned()))),
					});
					states.insert("BOS2_RAISE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOS2"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BOS2_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BOS2_STND".to_owned()))),
				spawn_state: Some("BOS2_STND".to_owned()),
				see_state: Some("BOS2_RUN1".to_owned()),
				pain_state: Some("BOS2_PAIN".to_owned()),
				melee_state: Some("BOS2_ATK1".to_owned()),
				missile_state: Some("BOS2_ATK1".to_owned()),
				death_state: Some("BOS2_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: Some("BOS2_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("KNIGHT", template);

	let template = EntityTemplate {
		name: Some("SKULL"),
		type_id: Some(EntityTypeId::Thing(3006)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SKUL"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(16);
					states.insert("SKULL_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 0, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("SKULL_STND2".to_owned()))),
					});
					states.insert("SKULL_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("SKULL_STND".to_owned()))),
					});
					states.insert("SKULL_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("SKULL_RUN2".to_owned()))),
					});
					states.insert("SKULL_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("SKULL_RUN1".to_owned()))),
					});
					states.insert("SKULL_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 2, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("SKULL_ATK2".to_owned()))),
					});
					states.insert("SKULL_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SKULL_ATK3".to_owned()))),
					});
					states.insert("SKULL_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SKULL_ATK4".to_owned()))),
					});
					states.insert("SKULL_ATK4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SKULL_ATK3".to_owned()))),
					});
					states.insert("SKULL_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 4, full_bright: true},
						next: Some((3 * FRAME_TIME, Some("SKULL_PAIN2".to_owned()))),
					});
					states.insert("SKULL_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 4, full_bright: true},
						next: Some((3 * FRAME_TIME, Some("SKULL_RUN1".to_owned()))),
					});
					states.insert("SKULL_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 5, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("SKULL_DIE2".to_owned()))),
					});
					states.insert("SKULL_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 6, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("SKULL_DIE3".to_owned()))),
					});
					states.insert("SKULL_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 7, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("SKULL_DIE4".to_owned()))),
					});
					states.insert("SKULL_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 8, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("SKULL_DIE5".to_owned()))),
					});
					states.insert("SKULL_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 9, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("SKULL_DIE6".to_owned()))),
					});
					states.insert("SKULL_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 10, full_bright: false},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SKULL_STND".to_owned()))),
				spawn_state: Some("SKULL_STND".to_owned()),
				see_state: Some("SKULL_RUN1".to_owned()),
				pain_state: Some("SKULL_PAIN".to_owned()),
				melee_state: None,
				missile_state: Some("SKULL_ATK1".to_owned()),
				death_state: Some("SKULL_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SKULL", template);

	let template = EntityTemplate {
		name: Some("SPIDER"),
		type_id: Some(EntityTypeId::Thing(7)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 100.0,
				radius: 128.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SPID"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(31);
					states.insert("SPID_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPID_STND2".to_owned()))),
					});
					states.insert("SPID_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPID_STND".to_owned()))),
					});
					states.insert("SPID_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_RUN2".to_owned()))),
					});
					states.insert("SPID_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_RUN3".to_owned()))),
					});
					states.insert("SPID_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_RUN4".to_owned()))),
					});
					states.insert("SPID_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_RUN5".to_owned()))),
					});
					states.insert("SPID_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_RUN6".to_owned()))),
					});
					states.insert("SPID_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_RUN7".to_owned()))),
					});
					states.insert("SPID_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_RUN8".to_owned()))),
					});
					states.insert("SPID_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_RUN9".to_owned()))),
					});
					states.insert("SPID_RUN9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 4, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_RUN10".to_owned()))),
					});
					states.insert("SPID_RUN10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 4, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_RUN11".to_owned()))),
					});
					states.insert("SPID_RUN11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 5, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_RUN12".to_owned()))),
					});
					states.insert("SPID_RUN12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 5, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_RUN1".to_owned()))),
					});
					states.insert("SPID_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 0, full_bright: true},
						next: Some((20 * FRAME_TIME, Some("SPID_ATK2".to_owned()))),
					});
					states.insert("SPID_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 6, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SPID_ATK3".to_owned()))),
					});
					states.insert("SPID_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 7, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SPID_ATK4".to_owned()))),
					});
					states.insert("SPID_ATK4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 7, full_bright: true},
						next: Some((1 * FRAME_TIME, Some("SPID_ATK2".to_owned()))),
					});
					states.insert("SPID_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 8, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_PAIN2".to_owned()))),
					});
					states.insert("SPID_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 8, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SPID_RUN1".to_owned()))),
					});
					states.insert("SPID_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 9, full_bright: false},
						next: Some((20 * FRAME_TIME, Some("SPID_DIE2".to_owned()))),
					});
					states.insert("SPID_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 10, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPID_DIE3".to_owned()))),
					});
					states.insert("SPID_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 11, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPID_DIE4".to_owned()))),
					});
					states.insert("SPID_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 12, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPID_DIE5".to_owned()))),
					});
					states.insert("SPID_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 13, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPID_DIE6".to_owned()))),
					});
					states.insert("SPID_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 14, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPID_DIE7".to_owned()))),
					});
					states.insert("SPID_DIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 15, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPID_DIE8".to_owned()))),
					});
					states.insert("SPID_DIE8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 16, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPID_DIE9".to_owned()))),
					});
					states.insert("SPID_DIE9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 17, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SPID_DIE10".to_owned()))),
					});
					states.insert("SPID_DIE10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 18, full_bright: false},
						next: Some((30 * FRAME_TIME, Some("SPID_DIE11".to_owned()))),
					});
					states.insert("SPID_DIE11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPID"), frame: 18, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SPID_STND".to_owned()))),
				spawn_state: Some("SPID_STND".to_owned()),
				see_state: Some("SPID_RUN1".to_owned()),
				pain_state: Some("SPID_PAIN".to_owned()),
				melee_state: None,
				missile_state: Some("SPID_ATK1".to_owned()),
				death_state: Some("SPID_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SPIDER", template);

	let template = EntityTemplate {
		name: Some("BABY"),
		type_id: Some(EntityTypeId::Thing(68)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 64.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BSPI"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(35);
					states.insert("BSPI_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("BSPI_STND2".to_owned()))),
					});
					states.insert("BSPI_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("BSPI_STND".to_owned()))),
					});
					states.insert("BSPI_SIGHT".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 0, full_bright: false},
						next: Some((20 * FRAME_TIME, Some("BSPI_RUN1".to_owned()))),
					});
					states.insert("BSPI_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_RUN2".to_owned()))),
					});
					states.insert("BSPI_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_RUN3".to_owned()))),
					});
					states.insert("BSPI_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_RUN4".to_owned()))),
					});
					states.insert("BSPI_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_RUN5".to_owned()))),
					});
					states.insert("BSPI_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_RUN6".to_owned()))),
					});
					states.insert("BSPI_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_RUN7".to_owned()))),
					});
					states.insert("BSPI_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_RUN8".to_owned()))),
					});
					states.insert("BSPI_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_RUN9".to_owned()))),
					});
					states.insert("BSPI_RUN9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 4, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_RUN10".to_owned()))),
					});
					states.insert("BSPI_RUN10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 4, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_RUN11".to_owned()))),
					});
					states.insert("BSPI_RUN11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 5, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_RUN12".to_owned()))),
					});
					states.insert("BSPI_RUN12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 5, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_RUN1".to_owned()))),
					});
					states.insert("BSPI_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 0, full_bright: true},
						next: Some((20 * FRAME_TIME, Some("BSPI_ATK2".to_owned()))),
					});
					states.insert("BSPI_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 6, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BSPI_ATK3".to_owned()))),
					});
					states.insert("BSPI_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 7, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BSPI_ATK4".to_owned()))),
					});
					states.insert("BSPI_ATK4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 7, full_bright: true},
						next: Some((1 * FRAME_TIME, Some("BSPI_ATK2".to_owned()))),
					});
					states.insert("BSPI_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 8, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_PAIN2".to_owned()))),
					});
					states.insert("BSPI_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 8, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("BSPI_RUN1".to_owned()))),
					});
					states.insert("BSPI_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 9, full_bright: false},
						next: Some((20 * FRAME_TIME, Some("BSPI_DIE2".to_owned()))),
					});
					states.insert("BSPI_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 10, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("BSPI_DIE3".to_owned()))),
					});
					states.insert("BSPI_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 11, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("BSPI_DIE4".to_owned()))),
					});
					states.insert("BSPI_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 12, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("BSPI_DIE5".to_owned()))),
					});
					states.insert("BSPI_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 13, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("BSPI_DIE6".to_owned()))),
					});
					states.insert("BSPI_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 14, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("BSPI_DIE7".to_owned()))),
					});
					states.insert("BSPI_DIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 15, full_bright: false},
						next: None,
					});
					states.insert("BSPI_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("BSPI_RAISE2".to_owned()))),
					});
					states.insert("BSPI_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("BSPI_RAISE3".to_owned()))),
					});
					states.insert("BSPI_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("BSPI_RAISE4".to_owned()))),
					});
					states.insert("BSPI_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("BSPI_RAISE5".to_owned()))),
					});
					states.insert("BSPI_RAISE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("BSPI_RAISE6".to_owned()))),
					});
					states.insert("BSPI_RAISE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("BSPI_RAISE7".to_owned()))),
					});
					states.insert("BSPI_RAISE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSPI"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("BSPI_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BSPI_STND".to_owned()))),
				spawn_state: Some("BSPI_STND".to_owned()),
				see_state: Some("BSPI_SIGHT".to_owned()),
				pain_state: Some("BSPI_PAIN".to_owned()),
				melee_state: None,
				missile_state: Some("BSPI_ATK1".to_owned()),
				death_state: Some("BSPI_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: Some("BSPI_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BABY", template);

	let template = EntityTemplate {
		name: Some("CYBORG"),
		type_id: Some(EntityTypeId::Thing(16)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 110.0,
				radius: 40.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CYBR"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(27);
					states.insert("CYBER_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CYBER_STND2".to_owned()))),
					});
					states.insert("CYBER_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CYBER_STND".to_owned()))),
					});
					states.insert("CYBER_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CYBER_RUN2".to_owned()))),
					});
					states.insert("CYBER_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CYBER_RUN3".to_owned()))),
					});
					states.insert("CYBER_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CYBER_RUN4".to_owned()))),
					});
					states.insert("CYBER_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CYBER_RUN5".to_owned()))),
					});
					states.insert("CYBER_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CYBER_RUN6".to_owned()))),
					});
					states.insert("CYBER_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CYBER_RUN7".to_owned()))),
					});
					states.insert("CYBER_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CYBER_RUN8".to_owned()))),
					});
					states.insert("CYBER_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("CYBER_RUN1".to_owned()))),
					});
					states.insert("CYBER_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 4, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("CYBER_ATK2".to_owned()))),
					});
					states.insert("CYBER_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 5, full_bright: false},
						next: Some((12 * FRAME_TIME, Some("CYBER_ATK3".to_owned()))),
					});
					states.insert("CYBER_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 4, full_bright: false},
						next: Some((12 * FRAME_TIME, Some("CYBER_ATK4".to_owned()))),
					});
					states.insert("CYBER_ATK4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 5, full_bright: false},
						next: Some((12 * FRAME_TIME, Some("CYBER_ATK5".to_owned()))),
					});
					states.insert("CYBER_ATK5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 4, full_bright: false},
						next: Some((12 * FRAME_TIME, Some("CYBER_ATK6".to_owned()))),
					});
					states.insert("CYBER_ATK6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 5, full_bright: false},
						next: Some((12 * FRAME_TIME, Some("CYBER_RUN1".to_owned()))),
					});
					states.insert("CYBER_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 6, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CYBER_RUN1".to_owned()))),
					});
					states.insert("CYBER_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 7, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CYBER_DIE2".to_owned()))),
					});
					states.insert("CYBER_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 8, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CYBER_DIE3".to_owned()))),
					});
					states.insert("CYBER_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 9, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CYBER_DIE4".to_owned()))),
					});
					states.insert("CYBER_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 10, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CYBER_DIE5".to_owned()))),
					});
					states.insert("CYBER_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 11, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CYBER_DIE6".to_owned()))),
					});
					states.insert("CYBER_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 12, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CYBER_DIE7".to_owned()))),
					});
					states.insert("CYBER_DIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 13, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CYBER_DIE8".to_owned()))),
					});
					states.insert("CYBER_DIE8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 14, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("CYBER_DIE9".to_owned()))),
					});
					states.insert("CYBER_DIE9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 15, full_bright: false},
						next: Some((30 * FRAME_TIME, Some("CYBER_DIE10".to_owned()))),
					});
					states.insert("CYBER_DIE10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CYBR"), frame: 15, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("CYBER_STND".to_owned()))),
				spawn_state: Some("CYBER_STND".to_owned()),
				see_state: Some("CYBER_RUN1".to_owned()),
				pain_state: Some("CYBER_PAIN".to_owned()),
				melee_state: None,
				missile_state: Some("CYBER_ATK1".to_owned()),
				death_state: Some("CYBER_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("CYBORG", template);

	let template = EntityTemplate {
		name: Some("PAIN"),
		type_id: Some(EntityTypeId::Thing(71)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 31.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PAIN"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(25);
					states.insert("PAIN_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("PAIN_STND".to_owned()))),
					});
					states.insert("PAIN_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("PAIN_RUN2".to_owned()))),
					});
					states.insert("PAIN_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("PAIN_RUN3".to_owned()))),
					});
					states.insert("PAIN_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("PAIN_RUN4".to_owned()))),
					});
					states.insert("PAIN_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("PAIN_RUN5".to_owned()))),
					});
					states.insert("PAIN_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("PAIN_RUN6".to_owned()))),
					});
					states.insert("PAIN_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("PAIN_RUN1".to_owned()))),
					});
					states.insert("PAIN_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 3, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("PAIN_ATK2".to_owned()))),
					});
					states.insert("PAIN_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 4, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("PAIN_ATK3".to_owned()))),
					});
					states.insert("PAIN_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 5, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("PAIN_ATK4".to_owned()))),
					});
					states.insert("PAIN_ATK4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 5, full_bright: true},
						next: Some((0 * FRAME_TIME, Some("PAIN_RUN1".to_owned()))),
					});
					states.insert("PAIN_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 6, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("PAIN_PAIN2".to_owned()))),
					});
					states.insert("PAIN_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 6, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("PAIN_RUN1".to_owned()))),
					});
					states.insert("PAIN_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 7, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("PAIN_DIE2".to_owned()))),
					});
					states.insert("PAIN_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 8, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("PAIN_DIE3".to_owned()))),
					});
					states.insert("PAIN_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 9, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("PAIN_DIE4".to_owned()))),
					});
					states.insert("PAIN_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 10, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("PAIN_DIE5".to_owned()))),
					});
					states.insert("PAIN_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 11, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("PAIN_DIE6".to_owned()))),
					});
					states.insert("PAIN_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 12, full_bright: true},
						next: Some((8 * FRAME_TIME, None)),
					});
					states.insert("PAIN_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("PAIN_RAISE2".to_owned()))),
					});
					states.insert("PAIN_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("PAIN_RAISE3".to_owned()))),
					});
					states.insert("PAIN_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("PAIN_RAISE4".to_owned()))),
					});
					states.insert("PAIN_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("PAIN_RAISE5".to_owned()))),
					});
					states.insert("PAIN_RAISE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("PAIN_RAISE6".to_owned()))),
					});
					states.insert("PAIN_RAISE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PAIN"), frame: 7, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("PAIN_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("PAIN_STND".to_owned()))),
				spawn_state: Some("PAIN_STND".to_owned()),
				see_state: Some("PAIN_RUN1".to_owned()),
				pain_state: Some("PAIN_PAIN".to_owned()),
				melee_state: None,
				missile_state: Some("PAIN_ATK1".to_owned()),
				death_state: Some("PAIN_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: Some("PAIN_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("PAIN", template);

	let template = EntityTemplate {
		name: Some("WOLFSS"),
		type_id: Some(EntityTypeId::Thing(84)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SSWV"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(37);
					states.insert("SSWV_STND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SSWV_STND2".to_owned()))),
					});
					states.insert("SSWV_STND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SSWV_STND".to_owned()))),
					});
					states.insert("SSWV_RUN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SSWV_RUN2".to_owned()))),
					});
					states.insert("SSWV_RUN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SSWV_RUN3".to_owned()))),
					});
					states.insert("SSWV_RUN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SSWV_RUN4".to_owned()))),
					});
					states.insert("SSWV_RUN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SSWV_RUN5".to_owned()))),
					});
					states.insert("SSWV_RUN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SSWV_RUN6".to_owned()))),
					});
					states.insert("SSWV_RUN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SSWV_RUN7".to_owned()))),
					});
					states.insert("SSWV_RUN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SSWV_RUN8".to_owned()))),
					});
					states.insert("SSWV_RUN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SSWV_RUN1".to_owned()))),
					});
					states.insert("SSWV_ATK1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 4, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SSWV_ATK2".to_owned()))),
					});
					states.insert("SSWV_ATK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 5, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("SSWV_ATK3".to_owned()))),
					});
					states.insert("SSWV_ATK3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 6, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SSWV_ATK4".to_owned()))),
					});
					states.insert("SSWV_ATK4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 5, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("SSWV_ATK5".to_owned()))),
					});
					states.insert("SSWV_ATK5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 6, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SSWV_ATK6".to_owned()))),
					});
					states.insert("SSWV_ATK6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 5, full_bright: false},
						next: Some((1 * FRAME_TIME, Some("SSWV_ATK2".to_owned()))),
					});
					states.insert("SSWV_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 7, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SSWV_PAIN2".to_owned()))),
					});
					states.insert("SSWV_PAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 7, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("SSWV_RUN1".to_owned()))),
					});
					states.insert("SSWV_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_DIE2".to_owned()))),
					});
					states.insert("SSWV_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_DIE3".to_owned()))),
					});
					states.insert("SSWV_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_DIE4".to_owned()))),
					});
					states.insert("SSWV_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_DIE5".to_owned()))),
					});
					states.insert("SSWV_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 12, full_bright: false},
						next: None,
					});
					states.insert("SSWV_XDIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_XDIE2".to_owned()))),
					});
					states.insert("SSWV_XDIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_XDIE3".to_owned()))),
					});
					states.insert("SSWV_XDIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_XDIE4".to_owned()))),
					});
					states.insert("SSWV_XDIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_XDIE5".to_owned()))),
					});
					states.insert("SSWV_XDIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_XDIE6".to_owned()))),
					});
					states.insert("SSWV_XDIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 18, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_XDIE7".to_owned()))),
					});
					states.insert("SSWV_XDIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 19, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_XDIE8".to_owned()))),
					});
					states.insert("SSWV_XDIE8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 20, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_XDIE9".to_owned()))),
					});
					states.insert("SSWV_XDIE9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 21, full_bright: false},
						next: None,
					});
					states.insert("SSWV_RAISE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_RAISE2".to_owned()))),
					});
					states.insert("SSWV_RAISE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_RAISE3".to_owned()))),
					});
					states.insert("SSWV_RAISE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_RAISE4".to_owned()))),
					});
					states.insert("SSWV_RAISE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_RAISE5".to_owned()))),
					});
					states.insert("SSWV_RAISE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("SSWV_RUN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SSWV_STND".to_owned()))),
				spawn_state: Some("SSWV_STND".to_owned()),
				see_state: Some("SSWV_RUN1".to_owned()),
				pain_state: Some("SSWV_PAIN".to_owned()),
				melee_state: None,
				missile_state: Some("SSWV_ATK1".to_owned()),
				death_state: Some("SSWV_DIE1".to_owned()),
				xdeath_state: Some("SSWV_XDIE1".to_owned()),
				raise_state: Some("SSWV_RAISE1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("WOLFSS", template);

	let template = EntityTemplate {
		name: Some("KEEN"),
		type_id: Some(EntityTypeId::Thing(72)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 72.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 72.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("KEEN"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(15);
					states.insert("KEENSTND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 0, full_bright: false},
						next: None,
					});
					states.insert("COMMKEEN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("COMMKEEN2".to_owned()))),
					});
					states.insert("COMMKEEN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("COMMKEEN3".to_owned()))),
					});
					states.insert("COMMKEEN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 2, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("COMMKEEN4".to_owned()))),
					});
					states.insert("COMMKEEN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 3, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("COMMKEEN5".to_owned()))),
					});
					states.insert("COMMKEEN5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 4, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("COMMKEEN6".to_owned()))),
					});
					states.insert("COMMKEEN6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 5, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("COMMKEEN7".to_owned()))),
					});
					states.insert("COMMKEEN7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 6, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("COMMKEEN8".to_owned()))),
					});
					states.insert("COMMKEEN8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 7, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("COMMKEEN9".to_owned()))),
					});
					states.insert("COMMKEEN9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 8, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("COMMKEEN10".to_owned()))),
					});
					states.insert("COMMKEEN10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 9, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("COMMKEEN11".to_owned()))),
					});
					states.insert("COMMKEEN11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 10, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("COMMKEEN12".to_owned()))),
					});
					states.insert("COMMKEEN12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 11, full_bright: false},
						next: None,
					});
					states.insert("KEENPAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 12, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("KEENPAIN2".to_owned()))),
					});
					states.insert("KEENPAIN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("KEEN"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("KEENSTND".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("KEENSTND".to_owned()))),
				spawn_state: Some("KEENSTND".to_owned()),
				see_state: None,
				pain_state: Some("KEENPAIN".to_owned()),
				melee_state: None,
				missile_state: None,
				death_state: Some("COMMKEEN".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("KEEN", template);

	let template = EntityTemplate {
		name: Some("BOSSBRAIN"),
		type_id: Some(EntityTypeId::Thing(88)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BBRN"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(6);
					states.insert("BRAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BBRN"), frame: 0, full_bright: false},
						next: None,
					});
					states.insert("BRAIN_PAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BBRN"), frame: 1, full_bright: false},
						next: Some((36 * FRAME_TIME, Some("BRAIN".to_owned()))),
					});
					states.insert("BRAIN_DIE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BBRN"), frame: 0, full_bright: false},
						next: Some((100 * FRAME_TIME, Some("BRAIN_DIE2".to_owned()))),
					});
					states.insert("BRAIN_DIE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BBRN"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("BRAIN_DIE3".to_owned()))),
					});
					states.insert("BRAIN_DIE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BBRN"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("BRAIN_DIE4".to_owned()))),
					});
					states.insert("BRAIN_DIE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BBRN"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BRAIN".to_owned()))),
				spawn_state: Some("BRAIN".to_owned()),
				see_state: None,
				pain_state: Some("BRAIN_PAIN".to_owned()),
				melee_state: None,
				missile_state: None,
				death_state: Some("BRAIN_DIE1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BOSSBRAIN", template);

	let template = EntityTemplate {
		name: Some("BOSSSPIT"),
		type_id: Some(EntityTypeId::Thing(89)),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("SSWV"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(3);
					states.insert("BRAINEYE".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("BRAINEYE".to_owned()))),
					});
					states.insert("BRAINEYESEE".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 0, full_bright: false},
						next: Some((181 * FRAME_TIME, Some("BRAINEYE1".to_owned()))),
					});
					states.insert("BRAINEYE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SSWV"), frame: 0, full_bright: false},
						next: Some((150 * FRAME_TIME, Some("BRAINEYE1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BRAINEYE".to_owned()))),
				spawn_state: Some("BRAINEYE".to_owned()),
				see_state: Some("BRAINEYESEE".to_owned()),
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BOSSSPIT", template);

	let template = EntityTemplate {
		name: Some("BOSSTARGET"),
		type_id: Some(EntityTypeId::Thing(87)),
		components: EntityComponents::new(),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BOSSTARGET", template);

	let template = EntityTemplate {
		name: Some("SPAWNSHOT"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BOSF"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("SPAWN1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSF"), frame: 0, full_bright: true},
						next: Some((3 * FRAME_TIME, Some("SPAWN2".to_owned()))),
					});
					states.insert("SPAWN2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSF"), frame: 1, full_bright: true},
						next: Some((3 * FRAME_TIME, Some("SPAWN3".to_owned()))),
					});
					states.insert("SPAWN3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSF"), frame: 2, full_bright: true},
						next: Some((3 * FRAME_TIME, Some("SPAWN4".to_owned()))),
					});
					states.insert("SPAWN4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BOSF"), frame: 3, full_bright: true},
						next: Some((3 * FRAME_TIME, Some("SPAWN1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SPAWN1".to_owned()))),
				spawn_state: Some("SPAWN1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SPAWNSHOT", template);

	let template = EntityTemplate {
		name: Some("SPAWNFIRE"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("FIRE"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(8);
					states.insert("SPAWNFIRE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SPAWNFIRE2".to_owned()))),
					});
					states.insert("SPAWNFIRE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SPAWNFIRE3".to_owned()))),
					});
					states.insert("SPAWNFIRE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SPAWNFIRE4".to_owned()))),
					});
					states.insert("SPAWNFIRE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SPAWNFIRE5".to_owned()))),
					});
					states.insert("SPAWNFIRE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 4, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SPAWNFIRE6".to_owned()))),
					});
					states.insert("SPAWNFIRE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 5, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SPAWNFIRE7".to_owned()))),
					});
					states.insert("SPAWNFIRE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 6, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("SPAWNFIRE8".to_owned()))),
					});
					states.insert("SPAWNFIRE8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FIRE"), frame: 7, full_bright: true},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SPAWNFIRE1".to_owned()))),
				spawn_state: Some("SPAWNFIRE1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SPAWNFIRE", template);

	let template = EntityTemplate {
		name: Some("BARREL"),
		type_id: Some(EntityTypeId::Thing(2035)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 42.0,
				radius: 10.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BAR1"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(7);
					states.insert("BAR1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAR1"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BAR2".to_owned()))),
					});
					states.insert("BAR2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAR1"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BAR1".to_owned()))),
					});
					states.insert("BEXP".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BEXP"), frame: 0, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("BEXP2".to_owned()))),
					});
					states.insert("BEXP2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BEXP"), frame: 1, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("BEXP3".to_owned()))),
					});
					states.insert("BEXP3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BEXP"), frame: 2, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("BEXP4".to_owned()))),
					});
					states.insert("BEXP4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BEXP"), frame: 3, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("BEXP5".to_owned()))),
					});
					states.insert("BEXP5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BEXP"), frame: 4, full_bright: true},
						next: Some((10 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BAR1".to_owned()))),
				spawn_state: Some("BAR1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("BEXP".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BARREL", template);

	let template = EntityTemplate {
		name: Some("TROOPSHOT"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BAL1"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(5);
					states.insert("TBALL1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL1"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("TBALL2".to_owned()))),
					});
					states.insert("TBALL2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL1"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("TBALL1".to_owned()))),
					});
					states.insert("TBALLX1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL1"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TBALLX2".to_owned()))),
					});
					states.insert("TBALLX2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL1"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TBALLX3".to_owned()))),
					});
					states.insert("TBALLX3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL1"), frame: 4, full_bright: true},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("TBALL1".to_owned()))),
				spawn_state: Some("TBALL1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("TBALLX1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("TROOPSHOT", template);

	let template = EntityTemplate {
		name: Some("HEADSHOT"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BAL2"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(5);
					states.insert("RBALL1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL2"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("RBALL2".to_owned()))),
					});
					states.insert("RBALL2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL2"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("RBALL1".to_owned()))),
					});
					states.insert("RBALLX1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL2"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("RBALLX2".to_owned()))),
					});
					states.insert("RBALLX2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL2"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("RBALLX3".to_owned()))),
					});
					states.insert("RBALLX3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BAL2"), frame: 4, full_bright: true},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("RBALL1".to_owned()))),
				spawn_state: Some("RBALL1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("RBALLX1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("HEADSHOT", template);

	let template = EntityTemplate {
		name: Some("ROCKET"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("MISL"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("ROCKET".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MISL"), frame: 0, full_bright: true},
						next: Some((1 * FRAME_TIME, Some("ROCKET".to_owned()))),
					});
					states.insert("EXPLODE1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MISL"), frame: 1, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("EXPLODE2".to_owned()))),
					});
					states.insert("EXPLODE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MISL"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("EXPLODE3".to_owned()))),
					});
					states.insert("EXPLODE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MISL"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("ROCKET".to_owned()))),
				spawn_state: Some("ROCKET".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("EXPLODE1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("ROCKET", template);

	let template = EntityTemplate {
		name: Some("PLASMA"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("PLSS"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(7);
					states.insert("PLASBALL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLSS"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PLASBALL2".to_owned()))),
					});
					states.insert("PLASBALL2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLSS"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PLASBALL".to_owned()))),
					});
					states.insert("PLASEXP".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLSE"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("PLASEXP2".to_owned()))),
					});
					states.insert("PLASEXP2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLSE"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("PLASEXP3".to_owned()))),
					});
					states.insert("PLASEXP3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLSE"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("PLASEXP4".to_owned()))),
					});
					states.insert("PLASEXP4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLSE"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("PLASEXP5".to_owned()))),
					});
					states.insert("PLASEXP5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLSE"), frame: 4, full_bright: true},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("PLASBALL".to_owned()))),
				spawn_state: Some("PLASBALL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("PLASEXP".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("PLASMA", template);

	let template = EntityTemplate {
		name: Some("BFG"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BFS1"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(8);
					states.insert("BFGSHOT".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BFS1"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BFGSHOT2".to_owned()))),
					});
					states.insert("BFGSHOT2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BFS1"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BFGSHOT".to_owned()))),
					});
					states.insert("BFGLAND".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BFE1"), frame: 0, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("BFGLAND2".to_owned()))),
					});
					states.insert("BFGLAND2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BFE1"), frame: 1, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("BFGLAND3".to_owned()))),
					});
					states.insert("BFGLAND3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BFE1"), frame: 2, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("BFGLAND4".to_owned()))),
					});
					states.insert("BFGLAND4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BFE1"), frame: 3, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("BFGLAND5".to_owned()))),
					});
					states.insert("BFGLAND5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BFE1"), frame: 4, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("BFGLAND6".to_owned()))),
					});
					states.insert("BFGLAND6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BFE1"), frame: 5, full_bright: true},
						next: Some((8 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BFGSHOT".to_owned()))),
				spawn_state: Some("BFGSHOT".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("BFGLAND".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BFG", template);

	let template = EntityTemplate {
		name: Some("ARACHPLAZ"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("APLS"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(7);
					states.insert("ARACH_PLAZ".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("APLS"), frame: 0, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("ARACH_PLAZ2".to_owned()))),
					});
					states.insert("ARACH_PLAZ2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("APLS"), frame: 1, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("ARACH_PLAZ".to_owned()))),
					});
					states.insert("ARACH_PLEX".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("APBX"), frame: 0, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("ARACH_PLEX2".to_owned()))),
					});
					states.insert("ARACH_PLEX2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("APBX"), frame: 1, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("ARACH_PLEX3".to_owned()))),
					});
					states.insert("ARACH_PLEX3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("APBX"), frame: 2, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("ARACH_PLEX4".to_owned()))),
					});
					states.insert("ARACH_PLEX4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("APBX"), frame: 3, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("ARACH_PLEX5".to_owned()))),
					});
					states.insert("ARACH_PLEX5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("APBX"), frame: 4, full_bright: true},
						next: Some((5 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("ARACH_PLAZ".to_owned()))),
				spawn_state: Some("ARACH_PLAZ".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("ARACH_PLEX".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name::<EntityTemplate>("ARACHPLAZ", template);

	let template = EntityTemplate {
		name: Some("PUFF"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("PUFF"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("PUFF1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PUFF"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("PUFF2".to_owned()))),
					});
					states.insert("PUFF2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PUFF"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("PUFF3".to_owned()))),
					});
					states.insert("PUFF3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PUFF"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("PUFF4".to_owned()))),
					});
					states.insert("PUFF4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PUFF"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("PUFF1".to_owned()))),
				spawn_state: Some("PUFF1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("PUFF", template);

	let template = EntityTemplate {
		name: Some("BLOOD"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BLUD"),
				frame: 2,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(3);
					states.insert("BLOOD1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BLUD"), frame: 2, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BLOOD2".to_owned()))),
					});
					states.insert("BLOOD2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BLUD"), frame: 1, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BLOOD3".to_owned()))),
					});
					states.insert("BLOOD3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BLUD"), frame: 0, full_bright: false},
						next: Some((8 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BLOOD1".to_owned()))),
				spawn_state: Some("BLOOD1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("BLOOD", template);

	let template = EntityTemplate {
		name: Some("TFOG"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("TFOG"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(12);
					states.insert("TFOG".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TFOG"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TFOG01".to_owned()))),
					});
					states.insert("TFOG01".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TFOG"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TFOG02".to_owned()))),
					});
					states.insert("TFOG02".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TFOG"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TFOG2".to_owned()))),
					});
					states.insert("TFOG2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TFOG"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TFOG3".to_owned()))),
					});
					states.insert("TFOG3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TFOG"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TFOG4".to_owned()))),
					});
					states.insert("TFOG4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TFOG"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TFOG5".to_owned()))),
					});
					states.insert("TFOG5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TFOG"), frame: 4, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TFOG6".to_owned()))),
					});
					states.insert("TFOG6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TFOG"), frame: 5, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TFOG7".to_owned()))),
					});
					states.insert("TFOG7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TFOG"), frame: 6, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TFOG8".to_owned()))),
					});
					states.insert("TFOG8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TFOG"), frame: 7, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TFOG9".to_owned()))),
					});
					states.insert("TFOG9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TFOG"), frame: 8, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("TFOG10".to_owned()))),
					});
					states.insert("TFOG10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TFOG"), frame: 9, full_bright: true},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("TFOG".to_owned()))),
				spawn_state: Some("TFOG".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("TFOG", template);

	let template = EntityTemplate {
		name: Some("IFOG"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("IFOG"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(7);
					states.insert("IFOG".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("IFOG"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("IFOG01".to_owned()))),
					});
					states.insert("IFOG01".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("IFOG"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("IFOG02".to_owned()))),
					});
					states.insert("IFOG02".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("IFOG"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("IFOG2".to_owned()))),
					});
					states.insert("IFOG2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("IFOG"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("IFOG3".to_owned()))),
					});
					states.insert("IFOG3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("IFOG"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("IFOG4".to_owned()))),
					});
					states.insert("IFOG4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("IFOG"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("IFOG5".to_owned()))),
					});
					states.insert("IFOG5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("IFOG"), frame: 4, full_bright: true},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("IFOG".to_owned()))),
				spawn_state: Some("IFOG".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("IFOG", template);

	let template = EntityTemplate {
		name: Some("TELEPORTMAN"),
		type_id: Some(EntityTypeId::Thing(14)),
		components: EntityComponents::new(),
	};
	asset_storage.insert_with_name::<EntityTemplate>("TELEPORTMAN", template);

	let template = EntityTemplate {
		name: Some("EXTRABFG"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BFE2"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("BFGEXP".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BFE2"), frame: 0, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("BFGEXP2".to_owned()))),
					});
					states.insert("BFGEXP2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BFE2"), frame: 1, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("BFGEXP3".to_owned()))),
					});
					states.insert("BFGEXP3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BFE2"), frame: 2, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("BFGEXP4".to_owned()))),
					});
					states.insert("BFGEXP4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BFE2"), frame: 3, full_bright: true},
						next: Some((8 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BFGEXP".to_owned()))),
				spawn_state: Some("BFGEXP".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("EXTRABFG", template);

	let template = EntityTemplate {
		name: Some("MISC0"),
		type_id: Some(EntityTypeId::Thing(2018)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("ARM1"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("ARM1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ARM1"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("ARM1A".to_owned()))),
					});
					states.insert("ARM1A".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ARM1"), frame: 1, full_bright: true},
						next: Some((7 * FRAME_TIME, Some("ARM1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("ARM1".to_owned()))),
				spawn_state: Some("ARM1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC0", template);

	let template = EntityTemplate {
		name: Some("MISC1"),
		type_id: Some(EntityTypeId::Thing(2019)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("ARM2"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("ARM2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ARM2"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("ARM2A".to_owned()))),
					});
					states.insert("ARM2A".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ARM2"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("ARM2".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("ARM2".to_owned()))),
				spawn_state: Some("ARM2".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC1", template);

	let template = EntityTemplate {
		name: Some("MISC2"),
		type_id: Some(EntityTypeId::Thing(2014)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BON1"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(6);
					states.insert("BON1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BON1"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BON1A".to_owned()))),
					});
					states.insert("BON1A".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BON1"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BON1B".to_owned()))),
					});
					states.insert("BON1B".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BON1"), frame: 2, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BON1C".to_owned()))),
					});
					states.insert("BON1C".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BON1"), frame: 3, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BON1D".to_owned()))),
					});
					states.insert("BON1D".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BON1"), frame: 2, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BON1E".to_owned()))),
					});
					states.insert("BON1E".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BON1"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BON1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BON1".to_owned()))),
				spawn_state: Some("BON1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC2", template);

	let template = EntityTemplate {
		name: Some("MISC3"),
		type_id: Some(EntityTypeId::Thing(2015)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BON2"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(6);
					states.insert("BON2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BON2"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BON2A".to_owned()))),
					});
					states.insert("BON2A".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BON2"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BON2B".to_owned()))),
					});
					states.insert("BON2B".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BON2"), frame: 2, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BON2C".to_owned()))),
					});
					states.insert("BON2C".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BON2"), frame: 3, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BON2D".to_owned()))),
					});
					states.insert("BON2D".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BON2"), frame: 2, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BON2E".to_owned()))),
					});
					states.insert("BON2E".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BON2"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BON2".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BON2".to_owned()))),
				spawn_state: Some("BON2".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC3", template);

	let template = EntityTemplate {
		name: Some("MISC4"),
		type_id: Some(EntityTypeId::Thing(5)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BKEY"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("BKEY".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BKEY"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("BKEY2".to_owned()))),
					});
					states.insert("BKEY2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BKEY"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("BKEY".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BKEY".to_owned()))),
				spawn_state: Some("BKEY".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC4", template);

	let template = EntityTemplate {
		name: Some("MISC5"),
		type_id: Some(EntityTypeId::Thing(13)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("RKEY"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("RKEY".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("RKEY"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("RKEY2".to_owned()))),
					});
					states.insert("RKEY2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("RKEY"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("RKEY".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("RKEY".to_owned()))),
				spawn_state: Some("RKEY".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC5", template);

	let template = EntityTemplate {
		name: Some("MISC6"),
		type_id: Some(EntityTypeId::Thing(6)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("YKEY"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("YKEY".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("YKEY"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("YKEY2".to_owned()))),
					});
					states.insert("YKEY2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("YKEY"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("YKEY".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("YKEY".to_owned()))),
				spawn_state: Some("YKEY".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC6", template);

	let template = EntityTemplate {
		name: Some("MISC7"),
		type_id: Some(EntityTypeId::Thing(39)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("YSKU"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("YSKULL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("YSKU"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("YSKULL2".to_owned()))),
					});
					states.insert("YSKULL2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("YSKU"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("YSKULL".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("YSKULL".to_owned()))),
				spawn_state: Some("YSKULL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC7", template);

	let template = EntityTemplate {
		name: Some("MISC8"),
		type_id: Some(EntityTypeId::Thing(38)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("RSKU"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("RSKULL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("RSKU"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("RSKULL2".to_owned()))),
					});
					states.insert("RSKULL2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("RSKU"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("RSKULL".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("RSKULL".to_owned()))),
				spawn_state: Some("RSKULL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC8", template);

	let template = EntityTemplate {
		name: Some("MISC9"),
		type_id: Some(EntityTypeId::Thing(40)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BSKU"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("BSKULL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSKU"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("BSKULL2".to_owned()))),
					});
					states.insert("BSKULL2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BSKU"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("BSKULL".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BSKULL".to_owned()))),
				spawn_state: Some("BSKULL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC9", template);

	let template = EntityTemplate {
		name: Some("MISC10"),
		type_id: Some(EntityTypeId::Thing(2011)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("STIM"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("STIM".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("STIM"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("STIM".to_owned()))),
				spawn_state: Some("STIM".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC10", template);

	let template = EntityTemplate {
		name: Some("MISC11"),
		type_id: Some(EntityTypeId::Thing(2012)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("MEDI"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("MEDI".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MEDI"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("MEDI".to_owned()))),
				spawn_state: Some("MEDI".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC11", template);

	let template = EntityTemplate {
		name: Some("MISC12"),
		type_id: Some(EntityTypeId::Thing(2013)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SOUL"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(6);
					states.insert("SOUL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SOUL"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("SOUL2".to_owned()))),
					});
					states.insert("SOUL2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SOUL"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("SOUL3".to_owned()))),
					});
					states.insert("SOUL3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SOUL"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("SOUL4".to_owned()))),
					});
					states.insert("SOUL4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SOUL"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("SOUL5".to_owned()))),
					});
					states.insert("SOUL5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SOUL"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("SOUL6".to_owned()))),
					});
					states.insert("SOUL6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SOUL"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("SOUL".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SOUL".to_owned()))),
				spawn_state: Some("SOUL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC12", template);

	let template = EntityTemplate {
		name: Some("INV"),
		type_id: Some(EntityTypeId::Thing(2022)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PINV"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("PINV".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PINV"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PINV2".to_owned()))),
					});
					states.insert("PINV2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PINV"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PINV3".to_owned()))),
					});
					states.insert("PINV3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PINV"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PINV4".to_owned()))),
					});
					states.insert("PINV4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PINV"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PINV".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("PINV".to_owned()))),
				spawn_state: Some("PINV".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("INV", template);

	let template = EntityTemplate {
		name: Some("MISC13"),
		type_id: Some(EntityTypeId::Thing(2023)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PSTR"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("PSTR".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PSTR"), frame: 0, full_bright: true},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("PSTR".to_owned()))),
				spawn_state: Some("PSTR".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC13", template);

	let template = EntityTemplate {
		name: Some("INS"),
		type_id: Some(EntityTypeId::Thing(2024)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PINS"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("PINS".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PINS"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PINS2".to_owned()))),
					});
					states.insert("PINS2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PINS"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PINS3".to_owned()))),
					});
					states.insert("PINS3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PINS"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PINS4".to_owned()))),
					});
					states.insert("PINS4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PINS"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PINS".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("PINS".to_owned()))),
				spawn_state: Some("PINS".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("INS", template);

	let template = EntityTemplate {
		name: Some("MISC14"),
		type_id: Some(EntityTypeId::Thing(2025)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SUIT"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("SUIT".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SUIT"), frame: 0, full_bright: true},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SUIT".to_owned()))),
				spawn_state: Some("SUIT".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC14", template);

	let template = EntityTemplate {
		name: Some("MISC15"),
		type_id: Some(EntityTypeId::Thing(2026)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PMAP"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(6);
					states.insert("PMAP".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PMAP"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PMAP2".to_owned()))),
					});
					states.insert("PMAP2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PMAP"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PMAP3".to_owned()))),
					});
					states.insert("PMAP3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PMAP"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PMAP4".to_owned()))),
					});
					states.insert("PMAP4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PMAP"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PMAP5".to_owned()))),
					});
					states.insert("PMAP5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PMAP"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PMAP6".to_owned()))),
					});
					states.insert("PMAP6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PMAP"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PMAP".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("PMAP".to_owned()))),
				spawn_state: Some("PMAP".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC15", template);

	let template = EntityTemplate {
		name: Some("MISC16"),
		type_id: Some(EntityTypeId::Thing(2045)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PVIS"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("PVIS".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PVIS"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("PVIS2".to_owned()))),
					});
					states.insert("PVIS2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PVIS"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("PVIS".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("PVIS".to_owned()))),
				spawn_state: Some("PVIS".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC16", template);

	let template = EntityTemplate {
		name: Some("MEGA"),
		type_id: Some(EntityTypeId::Thing(83)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("MEGA"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("MEGA".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MEGA"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("MEGA2".to_owned()))),
					});
					states.insert("MEGA2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MEGA"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("MEGA3".to_owned()))),
					});
					states.insert("MEGA3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MEGA"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("MEGA4".to_owned()))),
					});
					states.insert("MEGA4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MEGA"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("MEGA".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("MEGA".to_owned()))),
				spawn_state: Some("MEGA".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MEGA", template);

	let template = EntityTemplate {
		name: Some("CLIP"),
		type_id: Some(EntityTypeId::Thing(2007)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CLIP"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("CLIP".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CLIP"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("CLIP".to_owned()))),
				spawn_state: Some("CLIP".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("CLIP", template);

	let template = EntityTemplate {
		name: Some("MISC17"),
		type_id: Some(EntityTypeId::Thing(2048)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("AMMO"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("AMMO".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("AMMO"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("AMMO".to_owned()))),
				spawn_state: Some("AMMO".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC17", template);

	let template = EntityTemplate {
		name: Some("MISC18"),
		type_id: Some(EntityTypeId::Thing(2010)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("ROCK"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("ROCK".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ROCK"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("ROCK".to_owned()))),
				spawn_state: Some("ROCK".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC18", template);

	let template = EntityTemplate {
		name: Some("MISC19"),
		type_id: Some(EntityTypeId::Thing(2046)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BROK"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("BROK".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BROK"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BROK".to_owned()))),
				spawn_state: Some("BROK".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC19", template);

	let template = EntityTemplate {
		name: Some("MISC20"),
		type_id: Some(EntityTypeId::Thing(2047)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CELL"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("CELL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CELL"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("CELL".to_owned()))),
				spawn_state: Some("CELL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC20", template);

	let template = EntityTemplate {
		name: Some("MISC21"),
		type_id: Some(EntityTypeId::Thing(17)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CELP"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("CELP".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CELP"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("CELP".to_owned()))),
				spawn_state: Some("CELP".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC21", template);

	let template = EntityTemplate {
		name: Some("MISC22"),
		type_id: Some(EntityTypeId::Thing(2008)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SHEL"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("SHEL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SHEL"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SHEL".to_owned()))),
				spawn_state: Some("SHEL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC22", template);

	let template = EntityTemplate {
		name: Some("MISC23"),
		type_id: Some(EntityTypeId::Thing(2049)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SBOX"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("SBOX".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SBOX"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SBOX".to_owned()))),
				spawn_state: Some("SBOX".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC23", template);

	let template = EntityTemplate {
		name: Some("MISC24"),
		type_id: Some(EntityTypeId::Thing(8)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BPAK"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("BPAK".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BPAK"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BPAK".to_owned()))),
				spawn_state: Some("BPAK".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC24", template);

	let template = EntityTemplate {
		name: Some("MISC25"),
		type_id: Some(EntityTypeId::Thing(2006)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("BFUG"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("BFUG".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BFUG"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BFUG".to_owned()))),
				spawn_state: Some("BFUG".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC25", template);

	let template = EntityTemplate {
		name: Some("CHAINGUN"),
		type_id: Some(EntityTypeId::Thing(2002)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("MGUN"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("MGUN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("MGUN"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("MGUN".to_owned()))),
				spawn_state: Some("MGUN".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("CHAINGUN", template);

	let template = EntityTemplate {
		name: Some("MISC26"),
		type_id: Some(EntityTypeId::Thing(2005)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CSAW"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("CSAW".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CSAW"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("CSAW".to_owned()))),
				spawn_state: Some("CSAW".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC26", template);

	let template = EntityTemplate {
		name: Some("MISC27"),
		type_id: Some(EntityTypeId::Thing(2003)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("LAUN"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("LAUN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("LAUN"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("LAUN".to_owned()))),
				spawn_state: Some("LAUN".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC27", template);

	let template = EntityTemplate {
		name: Some("MISC28"),
		type_id: Some(EntityTypeId::Thing(2004)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PLAS"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("PLAS".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAS"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("PLAS".to_owned()))),
				spawn_state: Some("PLAS".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC28", template);

	let template = EntityTemplate {
		name: Some("SHOTGUN"),
		type_id: Some(EntityTypeId::Thing(2001)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SHOT"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("SHOT".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SHOT"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SHOT".to_owned()))),
				spawn_state: Some("SHOT".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SHOTGUN", template);

	let template = EntityTemplate {
		name: Some("SUPERSHOTGUN"),
		type_id: Some(EntityTypeId::Thing(82)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SGN2"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("SHOT2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SGN2"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SHOT2".to_owned()))),
				spawn_state: Some("SHOT2".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("SUPERSHOTGUN", template);

	let template = EntityTemplate {
		name: Some("MISC29"),
		type_id: Some(EntityTypeId::Thing(85)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TLMP"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("TECHLAMP".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TLMP"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("TECHLAMP2".to_owned()))),
					});
					states.insert("TECHLAMP2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TLMP"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("TECHLAMP3".to_owned()))),
					});
					states.insert("TECHLAMP3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TLMP"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("TECHLAMP4".to_owned()))),
					});
					states.insert("TECHLAMP4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TLMP"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("TECHLAMP".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("TECHLAMP".to_owned()))),
				spawn_state: Some("TECHLAMP".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC29", template);

	let template = EntityTemplate {
		name: Some("MISC30"),
		type_id: Some(EntityTypeId::Thing(86)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TLP2"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("TECH2LAMP".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TLP2"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("TECH2LAMP2".to_owned()))),
					});
					states.insert("TECH2LAMP2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TLP2"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("TECH2LAMP3".to_owned()))),
					});
					states.insert("TECH2LAMP3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TLP2"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("TECH2LAMP4".to_owned()))),
					});
					states.insert("TECH2LAMP4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TLP2"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("TECH2LAMP".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("TECH2LAMP".to_owned()))),
				spawn_state: Some("TECH2LAMP".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC30", template);

	let template = EntityTemplate {
		name: Some("MISC31"),
		type_id: Some(EntityTypeId::Thing(2028)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COLU"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("COLU".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("COLU"), frame: 0, full_bright: true},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("COLU".to_owned()))),
				spawn_state: Some("COLU".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC31", template);

	let template = EntityTemplate {
		name: Some("MISC32"),
		type_id: Some(EntityTypeId::Thing(30)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COL1"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("TALLGRNCOL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("COL1"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("TALLGRNCOL".to_owned()))),
				spawn_state: Some("TALLGRNCOL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC32", template);

	let template = EntityTemplate {
		name: Some("MISC33"),
		type_id: Some(EntityTypeId::Thing(31)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COL2"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("SHRTGRNCOL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("COL2"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SHRTGRNCOL".to_owned()))),
				spawn_state: Some("SHRTGRNCOL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC33", template);

	let template = EntityTemplate {
		name: Some("MISC34"),
		type_id: Some(EntityTypeId::Thing(32)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COL3"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("TALLREDCOL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("COL3"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("TALLREDCOL".to_owned()))),
				spawn_state: Some("TALLREDCOL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC34", template);

	let template = EntityTemplate {
		name: Some("MISC35"),
		type_id: Some(EntityTypeId::Thing(33)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COL4"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("SHRTREDCOL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("COL4"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SHRTREDCOL".to_owned()))),
				spawn_state: Some("SHRTREDCOL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC35", template);

	let template = EntityTemplate {
		name: Some("MISC36"),
		type_id: Some(EntityTypeId::Thing(37)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COL6"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("SKULLCOL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("COL6"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SKULLCOL".to_owned()))),
				spawn_state: Some("SKULLCOL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC36", template);

	let template = EntityTemplate {
		name: Some("MISC37"),
		type_id: Some(EntityTypeId::Thing(36)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("COL5"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("HEARTCOL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("COL5"), frame: 0, full_bright: false},
						next: Some((14 * FRAME_TIME, Some("HEARTCOL2".to_owned()))),
					});
					states.insert("HEARTCOL2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("COL5"), frame: 1, full_bright: false},
						next: Some((14 * FRAME_TIME, Some("HEARTCOL".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("HEARTCOL".to_owned()))),
				spawn_state: Some("HEARTCOL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC37", template);

	let template = EntityTemplate {
		name: Some("MISC38"),
		type_id: Some(EntityTypeId::Thing(41)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CEYE"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("EVILEYE".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CEYE"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("EVILEYE2".to_owned()))),
					});
					states.insert("EVILEYE2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CEYE"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("EVILEYE3".to_owned()))),
					});
					states.insert("EVILEYE3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CEYE"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("EVILEYE4".to_owned()))),
					});
					states.insert("EVILEYE4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CEYE"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("EVILEYE".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("EVILEYE".to_owned()))),
				spawn_state: Some("EVILEYE".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC38", template);

	let template = EntityTemplate {
		name: Some("MISC39"),
		type_id: Some(EntityTypeId::Thing(42)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("FSKU"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(3);
					states.insert("FLOATSKULL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FSKU"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("FLOATSKULL2".to_owned()))),
					});
					states.insert("FLOATSKULL2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FSKU"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("FLOATSKULL3".to_owned()))),
					});
					states.insert("FLOATSKULL3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FSKU"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("FLOATSKULL".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("FLOATSKULL".to_owned()))),
				spawn_state: Some("FLOATSKULL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC39", template);

	let template = EntityTemplate {
		name: Some("MISC40"),
		type_id: Some(EntityTypeId::Thing(43)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TRE1"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("TORCHTREE".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TRE1"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("TORCHTREE".to_owned()))),
				spawn_state: Some("TORCHTREE".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC40", template);

	let template = EntityTemplate {
		name: Some("MISC41"),
		type_id: Some(EntityTypeId::Thing(44)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TBLU"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("BLUETORCH".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TBLU"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BLUETORCH2".to_owned()))),
					});
					states.insert("BLUETORCH2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TBLU"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BLUETORCH3".to_owned()))),
					});
					states.insert("BLUETORCH3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TBLU"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BLUETORCH4".to_owned()))),
					});
					states.insert("BLUETORCH4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TBLU"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BLUETORCH".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BLUETORCH".to_owned()))),
				spawn_state: Some("BLUETORCH".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC41", template);

	let template = EntityTemplate {
		name: Some("MISC42"),
		type_id: Some(EntityTypeId::Thing(45)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TGRN"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("GREENTORCH".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TGRN"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("GREENTORCH2".to_owned()))),
					});
					states.insert("GREENTORCH2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TGRN"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("GREENTORCH3".to_owned()))),
					});
					states.insert("GREENTORCH3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TGRN"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("GREENTORCH4".to_owned()))),
					});
					states.insert("GREENTORCH4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TGRN"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("GREENTORCH".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("GREENTORCH".to_owned()))),
				spawn_state: Some("GREENTORCH".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC42", template);

	let template = EntityTemplate {
		name: Some("MISC43"),
		type_id: Some(EntityTypeId::Thing(46)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TRED"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("REDTORCH".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TRED"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("REDTORCH2".to_owned()))),
					});
					states.insert("REDTORCH2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TRED"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("REDTORCH3".to_owned()))),
					});
					states.insert("REDTORCH3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TRED"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("REDTORCH4".to_owned()))),
					});
					states.insert("REDTORCH4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TRED"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("REDTORCH".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("REDTORCH".to_owned()))),
				spawn_state: Some("REDTORCH".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC43", template);

	let template = EntityTemplate {
		name: Some("MISC44"),
		type_id: Some(EntityTypeId::Thing(55)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SMBT"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("BTORCHSHRT".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SMBT"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BTORCHSHRT2".to_owned()))),
					});
					states.insert("BTORCHSHRT2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SMBT"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BTORCHSHRT3".to_owned()))),
					});
					states.insert("BTORCHSHRT3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SMBT"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BTORCHSHRT4".to_owned()))),
					});
					states.insert("BTORCHSHRT4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SMBT"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BTORCHSHRT".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BTORCHSHRT".to_owned()))),
				spawn_state: Some("BTORCHSHRT".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC44", template);

	let template = EntityTemplate {
		name: Some("MISC45"),
		type_id: Some(EntityTypeId::Thing(56)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SMGT"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("GTORCHSHRT".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SMGT"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("GTORCHSHRT2".to_owned()))),
					});
					states.insert("GTORCHSHRT2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SMGT"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("GTORCHSHRT3".to_owned()))),
					});
					states.insert("GTORCHSHRT3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SMGT"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("GTORCHSHRT4".to_owned()))),
					});
					states.insert("GTORCHSHRT4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SMGT"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("GTORCHSHRT".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("GTORCHSHRT".to_owned()))),
				spawn_state: Some("GTORCHSHRT".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC45", template);

	let template = EntityTemplate {
		name: Some("MISC46"),
		type_id: Some(EntityTypeId::Thing(57)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SMRT"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("RTORCHSHRT".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SMRT"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("RTORCHSHRT2".to_owned()))),
					});
					states.insert("RTORCHSHRT2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SMRT"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("RTORCHSHRT3".to_owned()))),
					});
					states.insert("RTORCHSHRT3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SMRT"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("RTORCHSHRT4".to_owned()))),
					});
					states.insert("RTORCHSHRT4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SMRT"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("RTORCHSHRT".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("RTORCHSHRT".to_owned()))),
				spawn_state: Some("RTORCHSHRT".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC46", template);

	let template = EntityTemplate {
		name: Some("MISC47"),
		type_id: Some(EntityTypeId::Thing(47)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SMIT"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("STALAGTITE".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SMIT"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("STALAGTITE".to_owned()))),
				spawn_state: Some("STALAGTITE".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC47", template);

	let template = EntityTemplate {
		name: Some("MISC48"),
		type_id: Some(EntityTypeId::Thing(48)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("ELEC"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("TECHPILLAR".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ELEC"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("TECHPILLAR".to_owned()))),
				spawn_state: Some("TECHPILLAR".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC48", template);

	let template = EntityTemplate {
		name: Some("MISC49"),
		type_id: Some(EntityTypeId::Thing(34)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CAND"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("CANDLESTIK".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CAND"), frame: 0, full_bright: true},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("CANDLESTIK".to_owned()))),
				spawn_state: Some("CANDLESTIK".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC49", template);

	let template = EntityTemplate {
		name: Some("MISC50"),
		type_id: Some(EntityTypeId::Thing(35)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("CBRA"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("CANDELABRA".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("CBRA"), frame: 0, full_bright: true},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("CANDELABRA".to_owned()))),
				spawn_state: Some("CANDELABRA".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC50", template);

	let template = EntityTemplate {
		name: Some("MISC51"),
		type_id: Some(EntityTypeId::Thing(49)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 68.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 68.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR1"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("BLOODYTWITCH".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR1"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("BLOODYTWITCH2".to_owned()))),
					});
					states.insert("BLOODYTWITCH2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR1"), frame: 1, full_bright: false},
						next: Some((15 * FRAME_TIME, Some("BLOODYTWITCH3".to_owned()))),
					});
					states.insert("BLOODYTWITCH3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR1"), frame: 2, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BLOODYTWITCH4".to_owned()))),
					});
					states.insert("BLOODYTWITCH4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR1"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BLOODYTWITCH".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BLOODYTWITCH".to_owned()))),
				spawn_state: Some("BLOODYTWITCH".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC51", template);

	let template = EntityTemplate {
		name: Some("MISC52"),
		type_id: Some(EntityTypeId::Thing(50)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 84.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 84.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR2"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("MEAT2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR2"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("MEAT2".to_owned()))),
				spawn_state: Some("MEAT2".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC52", template);

	let template = EntityTemplate {
		name: Some("MISC53"),
		type_id: Some(EntityTypeId::Thing(51)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 84.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 84.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR3"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("MEAT3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR3"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("MEAT3".to_owned()))),
				spawn_state: Some("MEAT3".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC53", template);

	let template = EntityTemplate {
		name: Some("MISC54"),
		type_id: Some(EntityTypeId::Thing(52)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 68.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 68.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR4"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("MEAT4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR4"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("MEAT4".to_owned()))),
				spawn_state: Some("MEAT4".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC54", template);

	let template = EntityTemplate {
		name: Some("MISC55"),
		type_id: Some(EntityTypeId::Thing(53)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 52.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 52.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR5"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("MEAT5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR5"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("MEAT5".to_owned()))),
				spawn_state: Some("MEAT5".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC55", template);

	let template = EntityTemplate {
		name: Some("MISC56"),
		type_id: Some(EntityTypeId::Thing(59)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 84.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpawnOnCeiling {
				offset: 84.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR2"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("MEAT2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR2"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("MEAT2".to_owned()))),
				spawn_state: Some("MEAT2".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC56", template);

	let template = EntityTemplate {
		name: Some("MISC57"),
		type_id: Some(EntityTypeId::Thing(60)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 68.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpawnOnCeiling {
				offset: 68.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR4"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("MEAT4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR4"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("MEAT4".to_owned()))),
				spawn_state: Some("MEAT4".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC57", template);

	let template = EntityTemplate {
		name: Some("MISC58"),
		type_id: Some(EntityTypeId::Thing(61)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 52.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpawnOnCeiling {
				offset: 52.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR3"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("MEAT3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR3"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("MEAT3".to_owned()))),
				spawn_state: Some("MEAT3".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC58", template);

	let template = EntityTemplate {
		name: Some("MISC59"),
		type_id: Some(EntityTypeId::Thing(62)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 52.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpawnOnCeiling {
				offset: 52.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR5"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("MEAT5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR5"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("MEAT5".to_owned()))),
				spawn_state: Some("MEAT5".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC59", template);

	let template = EntityTemplate {
		name: Some("MISC60"),
		type_id: Some(EntityTypeId::Thing(63)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 68.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpawnOnCeiling {
				offset: 68.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("GOR1"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("BLOODYTWITCH".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR1"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("BLOODYTWITCH2".to_owned()))),
					});
					states.insert("BLOODYTWITCH2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR1"), frame: 1, full_bright: false},
						next: Some((15 * FRAME_TIME, Some("BLOODYTWITCH3".to_owned()))),
					});
					states.insert("BLOODYTWITCH3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR1"), frame: 2, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("BLOODYTWITCH4".to_owned()))),
					});
					states.insert("BLOODYTWITCH4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("GOR1"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("BLOODYTWITCH".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BLOODYTWITCH".to_owned()))),
				spawn_state: Some("BLOODYTWITCH".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC60", template);

	let template = EntityTemplate {
		name: Some("MISC61"),
		type_id: Some(EntityTypeId::Thing(22)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HEAD"),
				frame: 11,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("HEAD_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HEAD"), frame: 11, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("HEAD_DIE6".to_owned()))),
				spawn_state: Some("HEAD_DIE6".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC61", template);

	let template = EntityTemplate {
		name: Some("MISC62"),
		type_id: Some(EntityTypeId::Thing(15)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PLAY"),
				frame: 13,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("PLAY_DIE7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 13, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("PLAY_DIE7".to_owned()))),
				spawn_state: Some("PLAY_DIE7".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC62", template);

	let template = EntityTemplate {
		name: Some("MISC63"),
		type_id: Some(EntityTypeId::Thing(18)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POSS"),
				frame: 11,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("POSS_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POSS"), frame: 11, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("POSS_DIE5".to_owned()))),
				spawn_state: Some("POSS_DIE5".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC63", template);

	let template = EntityTemplate {
		name: Some("MISC64"),
		type_id: Some(EntityTypeId::Thing(21)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SARG"),
				frame: 13,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("SARG_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SARG"), frame: 13, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SARG_DIE6".to_owned()))),
				spawn_state: Some("SARG_DIE6".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC64", template);

	let template = EntityTemplate {
		name: Some("MISC65"),
		type_id: Some(EntityTypeId::Thing(23)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SKUL"),
				frame: 10,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("SKULL_DIE6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SKUL"), frame: 10, full_bright: false},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SKULL_DIE6".to_owned()))),
				spawn_state: Some("SKULL_DIE6".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC65", template);

	let template = EntityTemplate {
		name: Some("MISC66"),
		type_id: Some(EntityTypeId::Thing(20)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TROO"),
				frame: 12,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("TROO_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TROO"), frame: 12, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("TROO_DIE5".to_owned()))),
				spawn_state: Some("TROO_DIE5".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC66", template);

	let template = EntityTemplate {
		name: Some("MISC67"),
		type_id: Some(EntityTypeId::Thing(19)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("SPOS"),
				frame: 11,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("SPOS_DIE5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("SPOS"), frame: 11, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SPOS_DIE5".to_owned()))),
				spawn_state: Some("SPOS_DIE5".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC67", template);

	let template = EntityTemplate {
		name: Some("MISC68"),
		type_id: Some(EntityTypeId::Thing(10)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PLAY"),
				frame: 22,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("PLAY_XDIE9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 22, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("PLAY_XDIE9".to_owned()))),
				spawn_state: Some("PLAY_XDIE9".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC68", template);

	let template = EntityTemplate {
		name: Some("MISC69"),
		type_id: Some(EntityTypeId::Thing(12)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("PLAY"),
				frame: 22,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("PLAY_XDIE9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("PLAY"), frame: 22, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("PLAY_XDIE9".to_owned()))),
				spawn_state: Some("PLAY_XDIE9".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC69", template);

	let template = EntityTemplate {
		name: Some("MISC70"),
		type_id: Some(EntityTypeId::Thing(28)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POL2"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("HEADSONSTICK".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POL2"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("HEADSONSTICK".to_owned()))),
				spawn_state: Some("HEADSONSTICK".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC70", template);

	let template = EntityTemplate {
		name: Some("MISC71"),
		type_id: Some(EntityTypeId::Thing(24)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POL5"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("GIBS".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POL5"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("GIBS".to_owned()))),
				spawn_state: Some("GIBS".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC71", template);

	let template = EntityTemplate {
		name: Some("MISC72"),
		type_id: Some(EntityTypeId::Thing(27)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POL4"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("HEADONASTICK".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POL4"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("HEADONASTICK".to_owned()))),
				spawn_state: Some("HEADONASTICK".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC72", template);

	let template = EntityTemplate {
		name: Some("MISC73"),
		type_id: Some(EntityTypeId::Thing(29)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POL3"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("HEADCANDLES".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POL3"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("HEADCANDLES2".to_owned()))),
					});
					states.insert("HEADCANDLES2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POL3"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("HEADCANDLES".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("HEADCANDLES".to_owned()))),
				spawn_state: Some("HEADCANDLES".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC73", template);

	let template = EntityTemplate {
		name: Some("MISC74"),
		type_id: Some(EntityTypeId::Thing(25)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POL1"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("DEADSTICK".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POL1"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("DEADSTICK".to_owned()))),
				spawn_state: Some("DEADSTICK".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC74", template);

	let template = EntityTemplate {
		name: Some("MISC75"),
		type_id: Some(EntityTypeId::Thing(26)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("POL6"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("LIVESTICK".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POL6"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("LIVESTICK2".to_owned()))),
					});
					states.insert("LIVESTICK2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POL6"), frame: 1, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("LIVESTICK".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("LIVESTICK".to_owned()))),
				spawn_state: Some("LIVESTICK".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC75", template);

	let template = EntityTemplate {
		name: Some("MISC76"),
		type_id: Some(EntityTypeId::Thing(54)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 32.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("TRE2"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("BIGTREE".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("TRE2"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BIGTREE".to_owned()))),
				spawn_state: Some("BIGTREE".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC76", template);

	let template = EntityTemplate {
		name: Some("MISC77"),
		type_id: Some(EntityTypeId::Thing(70)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("FCAN"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(3);
					states.insert("BBAR1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FCAN"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BBAR2".to_owned()))),
					});
					states.insert("BBAR2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FCAN"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BBAR3".to_owned()))),
					});
					states.insert("BBAR3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("FCAN"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("BBAR1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BBAR1".to_owned()))),
				spawn_state: Some("BBAR1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC77", template);

	let template = EntityTemplate {
		name: Some("MISC78"),
		type_id: Some(EntityTypeId::Thing(73)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 88.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 88.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HDB1"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("HANGNOGUTS".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HDB1"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("HANGNOGUTS".to_owned()))),
				spawn_state: Some("HANGNOGUTS".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC78", template);

	let template = EntityTemplate {
		name: Some("MISC79"),
		type_id: Some(EntityTypeId::Thing(74)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 88.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 88.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HDB2"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("HANGBNOBRAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HDB2"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("HANGBNOBRAIN".to_owned()))),
				spawn_state: Some("HANGBNOBRAIN".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC79", template);

	let template = EntityTemplate {
		name: Some("MISC80"),
		type_id: Some(EntityTypeId::Thing(75)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 64.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HDB3"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("HANGTLOOKDN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HDB3"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("HANGTLOOKDN".to_owned()))),
				spawn_state: Some("HANGTLOOKDN".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC80", template);

	let template = EntityTemplate {
		name: Some("MISC81"),
		type_id: Some(EntityTypeId::Thing(76)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 64.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HDB4"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("HANGTSKULL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HDB4"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("HANGTSKULL".to_owned()))),
				spawn_state: Some("HANGTSKULL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC81", template);

	let template = EntityTemplate {
		name: Some("MISC82"),
		type_id: Some(EntityTypeId::Thing(77)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 64.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HDB5"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("HANGTLOOKUP".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HDB5"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("HANGTLOOKUP".to_owned()))),
				spawn_state: Some("HANGTLOOKUP".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC82", template);

	let template = EntityTemplate {
		name: Some("MISC83"),
		type_id: Some(EntityTypeId::Thing(78)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpawnOnCeiling {
				offset: 64.0,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("HDB6"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("HANGTNOBRAIN".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("HDB6"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("HANGTNOBRAIN".to_owned()))),
				spawn_state: Some("HANGTNOBRAIN".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC83", template);

	let template = EntityTemplate {
		name: Some("MISC84"),
		type_id: Some(EntityTypeId::Thing(79)),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("POB1"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("COLONGIBS".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POB1"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("COLONGIBS".to_owned()))),
				spawn_state: Some("COLONGIBS".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC84", template);

	let template = EntityTemplate {
		name: Some("MISC85"),
		type_id: Some(EntityTypeId::Thing(80)),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("POB2"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("SMALLPOOL".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("POB2"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("SMALLPOOL".to_owned()))),
				spawn_state: Some("SMALLPOOL".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC85", template);

	let template = EntityTemplate {
		name: Some("MISC86"),
		type_id: Some(EntityTypeId::Thing(81)),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("BRS1"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("BRAINSTEM".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("BRS1"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("BRAINSTEM".to_owned()))),
				spawn_state: Some("BRAINSTEM".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name::<EntityTemplate>("MISC86", template);
}
