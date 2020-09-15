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
use legion::{systems::ResourceSet, Resources, Write};
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
	asset_storage.insert(template);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(2)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 2 }),
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(3)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 3 }),
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(4)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 4 }),
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Thing(11)),
		components: EntityComponents::new(),
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		name: Some("player"),
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
				impact_sound: asset_storage.load("dsoof.sound"),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("play.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(24);
					states.insert("play".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states.insert("play_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 0, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("play_run2".to_owned()))),
					});
					states.insert("play_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("play_run3".to_owned()))),
					});
					states.insert("play_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("play_run4".to_owned()))),
					});
					states.insert("play_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("play_run1".to_owned()))),
					});
					states.insert("play_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 4, full_bright: false},
						next: Some((12 * FRAME_TIME, Some("play".to_owned()))),
					});
					states.insert("play_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 6, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("play_pain2".to_owned()))),
					});
					states.insert("play_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 6, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("play".to_owned()))),
					});
					states.insert("play_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 7, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("play_die2".to_owned()))),
					});
					states.insert("play_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 8, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("play_die3".to_owned()))),
					});
					states.insert("play_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 9, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("play_die4".to_owned()))),
					});
					states.insert("play_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 10, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("play_die5".to_owned()))),
					});
					states.insert("play_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 11, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("play_die6".to_owned()))),
					});
					states.insert("play_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 12, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("play_die7".to_owned()))),
					});
					states.insert("play_die7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 13, full_bright: false},
						next: None,
					});
					states.insert("play_xdie1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("play_xdie2".to_owned()))),
					});
					states.insert("play_xdie2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("play_xdie3".to_owned()))),
					});
					states.insert("play_xdie3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("play_xdie4".to_owned()))),
					});
					states.insert("play_xdie4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("play_xdie5".to_owned()))),
					});
					states.insert("play_xdie5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 18, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("play_xdie6".to_owned()))),
					});
					states.insert("play_xdie6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 19, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("play_xdie7".to_owned()))),
					});
					states.insert("play_xdie7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 20, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("play_xdie8".to_owned()))),
					});
					states.insert("play_xdie8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 21, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("play_xdie9".to_owned()))),
					});
					states.insert("play_xdie9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 22, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("play".to_owned()))),
				spawn_state: Some("play".to_owned()),
				see_state: Some("play_run1".to_owned()),
				pain_state: Some("play_pain".to_owned()),
				melee_state: None,
				missile_state: Some("play_atk1".to_owned()),
				death_state: Some("play_die1".to_owned()),
				xdeath_state: Some("play_xdie1".to_owned()),
				raise_state: None,
			})
			.with_component(User {
				error_sound: asset_storage.load("dsnoway.sound"),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("player", template);

	let template = EntityTemplate {
		name: Some("possessed"),
		type_id: Some(EntityTypeId::Thing(3004)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("poss.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(33);
					states.insert("poss_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("poss_stnd2".to_owned()))),
					});
					states.insert("poss_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("poss_stnd".to_owned()))),
					});
					states.insert("poss_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 0, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("poss_run2".to_owned()))),
					});
					states.insert("poss_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 0, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("poss_run3".to_owned()))),
					});
					states.insert("poss_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("poss_run4".to_owned()))),
					});
					states.insert("poss_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("poss_run5".to_owned()))),
					});
					states.insert("poss_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("poss_run6".to_owned()))),
					});
					states.insert("poss_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("poss_run7".to_owned()))),
					});
					states.insert("poss_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("poss_run8".to_owned()))),
					});
					states.insert("poss_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("poss_run1".to_owned()))),
					});
					states.insert("poss_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 4, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("poss_atk2".to_owned()))),
					});
					states.insert("poss_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 5, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("poss_atk3".to_owned()))),
					});
					states.insert("poss_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 4, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("poss_run1".to_owned()))),
					});
					states.insert("poss_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 6, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("poss_pain2".to_owned()))),
					});
					states.insert("poss_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 6, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("poss_run1".to_owned()))),
					});
					states.insert("poss_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 7, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_die2".to_owned()))),
					});
					states.insert("poss_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_die3".to_owned()))),
					});
					states.insert("poss_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_die4".to_owned()))),
					});
					states.insert("poss_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_die5".to_owned()))),
					});
					states.insert("poss_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 11, full_bright: false},
						next: None,
					});
					states.insert("poss_xdie1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_xdie2".to_owned()))),
					});
					states.insert("poss_xdie2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_xdie3".to_owned()))),
					});
					states.insert("poss_xdie3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_xdie4".to_owned()))),
					});
					states.insert("poss_xdie4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_xdie5".to_owned()))),
					});
					states.insert("poss_xdie5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_xdie6".to_owned()))),
					});
					states.insert("poss_xdie6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_xdie7".to_owned()))),
					});
					states.insert("poss_xdie7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 18, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_xdie8".to_owned()))),
					});
					states.insert("poss_xdie8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 19, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_xdie9".to_owned()))),
					});
					states.insert("poss_xdie9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 20, full_bright: false},
						next: None,
					});
					states.insert("poss_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_raise2".to_owned()))),
					});
					states.insert("poss_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_raise3".to_owned()))),
					});
					states.insert("poss_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_raise4".to_owned()))),
					});
					states.insert("poss_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 7, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("poss_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("poss_stnd".to_owned()))),
				spawn_state: Some("poss_stnd".to_owned()),
				see_state: Some("poss_run1".to_owned()),
				pain_state: Some("poss_pain".to_owned()),
				melee_state: None,
				missile_state: Some("poss_atk1".to_owned()),
				death_state: Some("poss_die1".to_owned()),
				xdeath_state: Some("poss_xdie1".to_owned()),
				raise_state: Some("poss_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("possessed", template);

	let template = EntityTemplate {
		name: Some("shotguy"),
		type_id: Some(EntityTypeId::Thing(9)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("spos.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(34);
					states.insert("spos_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spos_stnd2".to_owned()))),
					});
					states.insert("spos_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spos_stnd".to_owned()))),
					});
					states.insert("spos_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spos_run2".to_owned()))),
					});
					states.insert("spos_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spos_run3".to_owned()))),
					});
					states.insert("spos_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spos_run4".to_owned()))),
					});
					states.insert("spos_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spos_run5".to_owned()))),
					});
					states.insert("spos_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spos_run6".to_owned()))),
					});
					states.insert("spos_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spos_run7".to_owned()))),
					});
					states.insert("spos_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spos_run8".to_owned()))),
					});
					states.insert("spos_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spos_run1".to_owned()))),
					});
					states.insert("spos_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 4, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spos_atk2".to_owned()))),
					});
					states.insert("spos_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 5, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("spos_atk3".to_owned()))),
					});
					states.insert("spos_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 4, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spos_run1".to_owned()))),
					});
					states.insert("spos_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 6, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spos_pain2".to_owned()))),
					});
					states.insert("spos_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 6, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spos_run1".to_owned()))),
					});
					states.insert("spos_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 7, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_die2".to_owned()))),
					});
					states.insert("spos_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_die3".to_owned()))),
					});
					states.insert("spos_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_die4".to_owned()))),
					});
					states.insert("spos_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_die5".to_owned()))),
					});
					states.insert("spos_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 11, full_bright: false},
						next: None,
					});
					states.insert("spos_xdie1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_xdie2".to_owned()))),
					});
					states.insert("spos_xdie2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_xdie3".to_owned()))),
					});
					states.insert("spos_xdie3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_xdie4".to_owned()))),
					});
					states.insert("spos_xdie4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_xdie5".to_owned()))),
					});
					states.insert("spos_xdie5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_xdie6".to_owned()))),
					});
					states.insert("spos_xdie6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_xdie7".to_owned()))),
					});
					states.insert("spos_xdie7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 18, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_xdie8".to_owned()))),
					});
					states.insert("spos_xdie8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 19, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_xdie9".to_owned()))),
					});
					states.insert("spos_xdie9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 20, full_bright: false},
						next: None,
					});
					states.insert("spos_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_raise2".to_owned()))),
					});
					states.insert("spos_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_raise3".to_owned()))),
					});
					states.insert("spos_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_raise4".to_owned()))),
					});
					states.insert("spos_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_raise5".to_owned()))),
					});
					states.insert("spos_raise5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 7, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("spos_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("spos_stnd".to_owned()))),
				spawn_state: Some("spos_stnd".to_owned()),
				see_state: Some("spos_run1".to_owned()),
				pain_state: Some("spos_pain".to_owned()),
				melee_state: None,
				missile_state: Some("spos_atk1".to_owned()),
				death_state: Some("spos_die1".to_owned()),
				xdeath_state: Some("spos_xdie1".to_owned()),
				raise_state: Some("spos_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("shotguy", template);

	let template = EntityTemplate {
		name: Some("vile"),
		type_id: Some(EntityTypeId::Thing(64)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("vile.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(37);
					states.insert("vile_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("vile_stnd2".to_owned()))),
					});
					states.insert("vile_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("vile_stnd".to_owned()))),
					});
					states.insert("vile_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("vile_run2".to_owned()))),
					});
					states.insert("vile_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("vile_run3".to_owned()))),
					});
					states.insert("vile_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("vile_run4".to_owned()))),
					});
					states.insert("vile_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("vile_run5".to_owned()))),
					});
					states.insert("vile_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("vile_run6".to_owned()))),
					});
					states.insert("vile_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("vile_run7".to_owned()))),
					});
					states.insert("vile_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("vile_run8".to_owned()))),
					});
					states.insert("vile_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("vile_run9".to_owned()))),
					});
					states.insert("vile_run9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 4, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("vile_run10".to_owned()))),
					});
					states.insert("vile_run10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 4, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("vile_run11".to_owned()))),
					});
					states.insert("vile_run11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 5, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("vile_run12".to_owned()))),
					});
					states.insert("vile_run12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 5, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("vile_run1".to_owned()))),
					});
					states.insert("vile_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 6, full_bright: true},
						next: Some((0 * FRAME_TIME, Some("vile_atk2".to_owned()))),
					});
					states.insert("vile_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 6, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("vile_atk3".to_owned()))),
					});
					states.insert("vile_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 7, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("vile_atk4".to_owned()))),
					});
					states.insert("vile_atk4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 8, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("vile_atk5".to_owned()))),
					});
					states.insert("vile_atk5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 9, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("vile_atk6".to_owned()))),
					});
					states.insert("vile_atk6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 10, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("vile_atk7".to_owned()))),
					});
					states.insert("vile_atk7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 11, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("vile_atk8".to_owned()))),
					});
					states.insert("vile_atk8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 12, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("vile_atk9".to_owned()))),
					});
					states.insert("vile_atk9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 13, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("vile_atk10".to_owned()))),
					});
					states.insert("vile_atk10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 14, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("vile_atk11".to_owned()))),
					});
					states.insert("vile_atk11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 15, full_bright: true},
						next: Some((20 * FRAME_TIME, Some("vile_run1".to_owned()))),
					});
					states.insert("vile_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("vile_pain2".to_owned()))),
					});
					states.insert("vile_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("vile_run1".to_owned()))),
					});
					states.insert("vile_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 16, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("vile_die2".to_owned()))),
					});
					states.insert("vile_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 17, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("vile_die3".to_owned()))),
					});
					states.insert("vile_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 18, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("vile_die4".to_owned()))),
					});
					states.insert("vile_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 19, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("vile_die5".to_owned()))),
					});
					states.insert("vile_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 20, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("vile_die6".to_owned()))),
					});
					states.insert("vile_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 21, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("vile_die7".to_owned()))),
					});
					states.insert("vile_die7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 22, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("vile_die8".to_owned()))),
					});
					states.insert("vile_die8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 23, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("vile_die9".to_owned()))),
					});
					states.insert("vile_die9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 24, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("vile_die10".to_owned()))),
					});
					states.insert("vile_die10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 25, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("vile_stnd".to_owned()))),
				spawn_state: Some("vile_stnd".to_owned()),
				see_state: Some("vile_run1".to_owned()),
				pain_state: Some("vile_pain".to_owned()),
				melee_state: None,
				missile_state: Some("vile_atk1".to_owned()),
				death_state: Some("vile_die1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("vile", template);

	let template = EntityTemplate {
		name: Some("fire"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("fire.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(30);
					states.insert("fire1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 0, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire2".to_owned()))),
					});
					states.insert("fire2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 1, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire3".to_owned()))),
					});
					states.insert("fire3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 0, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire4".to_owned()))),
					});
					states.insert("fire4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 1, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire5".to_owned()))),
					});
					states.insert("fire5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 2, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire6".to_owned()))),
					});
					states.insert("fire6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 1, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire7".to_owned()))),
					});
					states.insert("fire7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 2, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire8".to_owned()))),
					});
					states.insert("fire8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 1, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire9".to_owned()))),
					});
					states.insert("fire9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 2, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire10".to_owned()))),
					});
					states.insert("fire10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 3, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire11".to_owned()))),
					});
					states.insert("fire11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 2, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire12".to_owned()))),
					});
					states.insert("fire12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 3, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire13".to_owned()))),
					});
					states.insert("fire13".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 2, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire14".to_owned()))),
					});
					states.insert("fire14".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 3, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire15".to_owned()))),
					});
					states.insert("fire15".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 4, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire16".to_owned()))),
					});
					states.insert("fire16".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 3, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire17".to_owned()))),
					});
					states.insert("fire17".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 4, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire18".to_owned()))),
					});
					states.insert("fire18".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 3, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire19".to_owned()))),
					});
					states.insert("fire19".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 4, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire20".to_owned()))),
					});
					states.insert("fire20".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 5, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire21".to_owned()))),
					});
					states.insert("fire21".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 4, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire22".to_owned()))),
					});
					states.insert("fire22".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 5, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire23".to_owned()))),
					});
					states.insert("fire23".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 4, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire24".to_owned()))),
					});
					states.insert("fire24".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 5, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire25".to_owned()))),
					});
					states.insert("fire25".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 6, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire26".to_owned()))),
					});
					states.insert("fire26".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 7, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire27".to_owned()))),
					});
					states.insert("fire27".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 6, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire28".to_owned()))),
					});
					states.insert("fire28".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 7, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire29".to_owned()))),
					});
					states.insert("fire29".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 6, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("fire30".to_owned()))),
					});
					states.insert("fire30".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 7, full_bright: true},
						next: Some((2 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("fire1".to_owned()))),
				spawn_state: Some("fire1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("fire", template);

	let template = EntityTemplate {
		name: Some("undead"),
		type_id: Some(EntityTypeId::Thing(66)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("skel.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(36);
					states.insert("skel_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("skel_stnd2".to_owned()))),
					});
					states.insert("skel_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("skel_stnd".to_owned()))),
					});
					states.insert("skel_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("skel_run2".to_owned()))),
					});
					states.insert("skel_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("skel_run3".to_owned()))),
					});
					states.insert("skel_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("skel_run4".to_owned()))),
					});
					states.insert("skel_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("skel_run5".to_owned()))),
					});
					states.insert("skel_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("skel_run6".to_owned()))),
					});
					states.insert("skel_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("skel_run7".to_owned()))),
					});
					states.insert("skel_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("skel_run8".to_owned()))),
					});
					states.insert("skel_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("skel_run9".to_owned()))),
					});
					states.insert("skel_run9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 4, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("skel_run10".to_owned()))),
					});
					states.insert("skel_run10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 4, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("skel_run11".to_owned()))),
					});
					states.insert("skel_run11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 5, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("skel_run12".to_owned()))),
					});
					states.insert("skel_run12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 5, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("skel_run1".to_owned()))),
					});
					states.insert("skel_fist1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 6, full_bright: false},
						next: Some((0 * FRAME_TIME, Some("skel_fist2".to_owned()))),
					});
					states.insert("skel_fist2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 6, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("skel_fist3".to_owned()))),
					});
					states.insert("skel_fist3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 7, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("skel_fist4".to_owned()))),
					});
					states.insert("skel_fist4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 8, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("skel_run1".to_owned()))),
					});
					states.insert("skel_miss1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 9, full_bright: true},
						next: Some((0 * FRAME_TIME, Some("skel_miss2".to_owned()))),
					});
					states.insert("skel_miss2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 9, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("skel_miss3".to_owned()))),
					});
					states.insert("skel_miss3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 10, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("skel_miss4".to_owned()))),
					});
					states.insert("skel_miss4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 10, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("skel_run1".to_owned()))),
					});
					states.insert("skel_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("skel_pain2".to_owned()))),
					});
					states.insert("skel_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("skel_run1".to_owned()))),
					});
					states.insert("skel_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 11, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("skel_die2".to_owned()))),
					});
					states.insert("skel_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 12, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("skel_die3".to_owned()))),
					});
					states.insert("skel_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 13, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("skel_die4".to_owned()))),
					});
					states.insert("skel_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 14, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("skel_die5".to_owned()))),
					});
					states.insert("skel_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 15, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("skel_die6".to_owned()))),
					});
					states.insert("skel_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 16, full_bright: false},
						next: None,
					});
					states.insert("skel_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("skel_raise2".to_owned()))),
					});
					states.insert("skel_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("skel_raise3".to_owned()))),
					});
					states.insert("skel_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("skel_raise4".to_owned()))),
					});
					states.insert("skel_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("skel_raise5".to_owned()))),
					});
					states.insert("skel_raise5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("skel_raise6".to_owned()))),
					});
					states.insert("skel_raise6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("skel_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("skel_stnd".to_owned()))),
				spawn_state: Some("skel_stnd".to_owned()),
				see_state: Some("skel_run1".to_owned()),
				pain_state: Some("skel_pain".to_owned()),
				melee_state: Some("skel_fist1".to_owned()),
				missile_state: Some("skel_miss1".to_owned()),
				death_state: Some("skel_die1".to_owned()),
				xdeath_state: None,
				raise_state: Some("skel_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("undead", template);

	let template = EntityTemplate {
		name: Some("tracer"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("fatb.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(5);
					states.insert("tracer".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatb.sprite"), frame: 0, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("tracer2".to_owned()))),
					});
					states.insert("tracer2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatb.sprite"), frame: 1, full_bright: true},
						next: Some((2 * FRAME_TIME, Some("tracer".to_owned()))),
					});
					states.insert("traceexp1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fbxp.sprite"), frame: 0, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("traceexp2".to_owned()))),
					});
					states.insert("traceexp2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fbxp.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("traceexp3".to_owned()))),
					});
					states.insert("traceexp3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fbxp.sprite"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("tracer".to_owned()))),
				spawn_state: Some("tracer".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("traceexp1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("tracer", template);

	let template = EntityTemplate {
		name: Some("smoke"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("puff.sprite"),
				frame: 1,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(5);
					states.insert("smoke1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("smoke2".to_owned()))),
					});
					states.insert("smoke2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("smoke3".to_owned()))),
					});
					states.insert("smoke3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("smoke4".to_owned()))),
					});
					states.insert("smoke4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("smoke5".to_owned()))),
					});
					states.insert("smoke5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("smoke1".to_owned()))),
				spawn_state: Some("smoke1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("smoke", template);

	let template = EntityTemplate {
		name: Some("fatso"),
		type_id: Some(EntityTypeId::Thing(67)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 48.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("fatt.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(44);
					states.insert("fatt_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 0, full_bright: false},
						next: Some((15 * FRAME_TIME, Some("fatt_stnd2".to_owned()))),
					});
					states.insert("fatt_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 1, full_bright: false},
						next: Some((15 * FRAME_TIME, Some("fatt_stnd".to_owned()))),
					});
					states.insert("fatt_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 0, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("fatt_run2".to_owned()))),
					});
					states.insert("fatt_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 0, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("fatt_run3".to_owned()))),
					});
					states.insert("fatt_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("fatt_run4".to_owned()))),
					});
					states.insert("fatt_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("fatt_run5".to_owned()))),
					});
					states.insert("fatt_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("fatt_run6".to_owned()))),
					});
					states.insert("fatt_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("fatt_run7".to_owned()))),
					});
					states.insert("fatt_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("fatt_run8".to_owned()))),
					});
					states.insert("fatt_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("fatt_run9".to_owned()))),
					});
					states.insert("fatt_run9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 4, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("fatt_run10".to_owned()))),
					});
					states.insert("fatt_run10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 4, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("fatt_run11".to_owned()))),
					});
					states.insert("fatt_run11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 5, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("fatt_run12".to_owned()))),
					});
					states.insert("fatt_run12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 5, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("fatt_run1".to_owned()))),
					});
					states.insert("fatt_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 6, full_bright: false},
						next: Some((20 * FRAME_TIME, Some("fatt_atk2".to_owned()))),
					});
					states.insert("fatt_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 7, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("fatt_atk3".to_owned()))),
					});
					states.insert("fatt_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_atk4".to_owned()))),
					});
					states.insert("fatt_atk4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 6, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_atk5".to_owned()))),
					});
					states.insert("fatt_atk5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 7, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("fatt_atk6".to_owned()))),
					});
					states.insert("fatt_atk6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_atk7".to_owned()))),
					});
					states.insert("fatt_atk7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 6, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_atk8".to_owned()))),
					});
					states.insert("fatt_atk8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 7, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("fatt_atk9".to_owned()))),
					});
					states.insert("fatt_atk9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_atk10".to_owned()))),
					});
					states.insert("fatt_atk10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 6, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_run1".to_owned()))),
					});
					states.insert("fatt_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 9, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("fatt_pain2".to_owned()))),
					});
					states.insert("fatt_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 9, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("fatt_run1".to_owned()))),
					});
					states.insert("fatt_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 10, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("fatt_die2".to_owned()))),
					});
					states.insert("fatt_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 11, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("fatt_die3".to_owned()))),
					});
					states.insert("fatt_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 12, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("fatt_die4".to_owned()))),
					});
					states.insert("fatt_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 13, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("fatt_die5".to_owned()))),
					});
					states.insert("fatt_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 14, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("fatt_die6".to_owned()))),
					});
					states.insert("fatt_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 15, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("fatt_die7".to_owned()))),
					});
					states.insert("fatt_die7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 16, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("fatt_die8".to_owned()))),
					});
					states.insert("fatt_die8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 17, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("fatt_die9".to_owned()))),
					});
					states.insert("fatt_die9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 18, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("fatt_die10".to_owned()))),
					});
					states.insert("fatt_die10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 19, full_bright: false},
						next: None,
					});
					states.insert("fatt_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_raise2".to_owned()))),
					});
					states.insert("fatt_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_raise3".to_owned()))),
					});
					states.insert("fatt_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_raise4".to_owned()))),
					});
					states.insert("fatt_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_raise5".to_owned()))),
					});
					states.insert("fatt_raise5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_raise6".to_owned()))),
					});
					states.insert("fatt_raise6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_raise7".to_owned()))),
					});
					states.insert("fatt_raise7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_raise8".to_owned()))),
					});
					states.insert("fatt_raise8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("fatt_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("fatt_stnd".to_owned()))),
				spawn_state: Some("fatt_stnd".to_owned()),
				see_state: Some("fatt_run1".to_owned()),
				pain_state: Some("fatt_pain".to_owned()),
				melee_state: None,
				missile_state: Some("fatt_atk1".to_owned()),
				death_state: Some("fatt_die1".to_owned()),
				xdeath_state: None,
				raise_state: Some("fatt_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("fatso", template);

	let template = EntityTemplate {
		name: Some("fatshot"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("manf.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(5);
					states.insert("fatshot1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("manf.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("fatshot2".to_owned()))),
					});
					states.insert("fatshot2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("manf.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("fatshot1".to_owned()))),
					});
					states.insert("fatshotx1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 1, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("fatshotx2".to_owned()))),
					});
					states.insert("fatshotx2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("fatshotx3".to_owned()))),
					});
					states.insert("fatshotx3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("fatshot1".to_owned()))),
				spawn_state: Some("fatshot1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("fatshotx1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("fatshot", template);

	let template = EntityTemplate {
		name: Some("chainguy"),
		type_id: Some(EntityTypeId::Thing(65)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("cpos.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(36);
					states.insert("cpos_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cpos_stnd2".to_owned()))),
					});
					states.insert("cpos_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cpos_stnd".to_owned()))),
					});
					states.insert("cpos_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cpos_run2".to_owned()))),
					});
					states.insert("cpos_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cpos_run3".to_owned()))),
					});
					states.insert("cpos_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cpos_run4".to_owned()))),
					});
					states.insert("cpos_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cpos_run5".to_owned()))),
					});
					states.insert("cpos_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cpos_run6".to_owned()))),
					});
					states.insert("cpos_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cpos_run7".to_owned()))),
					});
					states.insert("cpos_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cpos_run8".to_owned()))),
					});
					states.insert("cpos_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cpos_run1".to_owned()))),
					});
					states.insert("cpos_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 4, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cpos_atk2".to_owned()))),
					});
					states.insert("cpos_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 5, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("cpos_atk3".to_owned()))),
					});
					states.insert("cpos_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 4, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("cpos_atk4".to_owned()))),
					});
					states.insert("cpos_atk4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 5, full_bright: false},
						next: Some((1 * FRAME_TIME, Some("cpos_atk2".to_owned()))),
					});
					states.insert("cpos_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 6, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cpos_pain2".to_owned()))),
					});
					states.insert("cpos_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 6, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cpos_run1".to_owned()))),
					});
					states.insert("cpos_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 7, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_die2".to_owned()))),
					});
					states.insert("cpos_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_die3".to_owned()))),
					});
					states.insert("cpos_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_die4".to_owned()))),
					});
					states.insert("cpos_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_die5".to_owned()))),
					});
					states.insert("cpos_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_die6".to_owned()))),
					});
					states.insert("cpos_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_die7".to_owned()))),
					});
					states.insert("cpos_die7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 13, full_bright: false},
						next: None,
					});
					states.insert("cpos_xdie1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_xdie2".to_owned()))),
					});
					states.insert("cpos_xdie2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_xdie3".to_owned()))),
					});
					states.insert("cpos_xdie3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_xdie4".to_owned()))),
					});
					states.insert("cpos_xdie4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_xdie5".to_owned()))),
					});
					states.insert("cpos_xdie5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 18, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_xdie6".to_owned()))),
					});
					states.insert("cpos_xdie6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 19, full_bright: false},
						next: None,
					});
					states.insert("cpos_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_raise2".to_owned()))),
					});
					states.insert("cpos_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_raise3".to_owned()))),
					});
					states.insert("cpos_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_raise4".to_owned()))),
					});
					states.insert("cpos_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_raise5".to_owned()))),
					});
					states.insert("cpos_raise5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_raise6".to_owned()))),
					});
					states.insert("cpos_raise6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_raise7".to_owned()))),
					});
					states.insert("cpos_raise7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 7, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("cpos_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("cpos_stnd".to_owned()))),
				spawn_state: Some("cpos_stnd".to_owned()),
				see_state: Some("cpos_run1".to_owned()),
				pain_state: Some("cpos_pain".to_owned()),
				melee_state: None,
				missile_state: Some("cpos_atk1".to_owned()),
				death_state: Some("cpos_die1".to_owned()),
				xdeath_state: Some("cpos_xdie1".to_owned()),
				raise_state: Some("cpos_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("chainguy", template);

	let template = EntityTemplate {
		name: Some("troop"),
		type_id: Some(EntityTypeId::Thing(3001)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("troo.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(33);
					states.insert("troo_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("troo_stnd2".to_owned()))),
					});
					states.insert("troo_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("troo_stnd".to_owned()))),
					});
					states.insert("troo_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("troo_run2".to_owned()))),
					});
					states.insert("troo_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("troo_run3".to_owned()))),
					});
					states.insert("troo_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("troo_run4".to_owned()))),
					});
					states.insert("troo_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("troo_run5".to_owned()))),
					});
					states.insert("troo_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("troo_run6".to_owned()))),
					});
					states.insert("troo_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("troo_run7".to_owned()))),
					});
					states.insert("troo_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("troo_run8".to_owned()))),
					});
					states.insert("troo_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("troo_run1".to_owned()))),
					});
					states.insert("troo_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 4, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("troo_atk2".to_owned()))),
					});
					states.insert("troo_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 5, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("troo_atk3".to_owned()))),
					});
					states.insert("troo_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 6, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("troo_run1".to_owned()))),
					});
					states.insert("troo_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("troo_pain2".to_owned()))),
					});
					states.insert("troo_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("troo_run1".to_owned()))),
					});
					states.insert("troo_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("troo_die2".to_owned()))),
					});
					states.insert("troo_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("troo_die3".to_owned()))),
					});
					states.insert("troo_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 10, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("troo_die4".to_owned()))),
					});
					states.insert("troo_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 11, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("troo_die5".to_owned()))),
					});
					states.insert("troo_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 12, full_bright: false},
						next: None,
					});
					states.insert("troo_xdie1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("troo_xdie2".to_owned()))),
					});
					states.insert("troo_xdie2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("troo_xdie3".to_owned()))),
					});
					states.insert("troo_xdie3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("troo_xdie4".to_owned()))),
					});
					states.insert("troo_xdie4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("troo_xdie5".to_owned()))),
					});
					states.insert("troo_xdie5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("troo_xdie6".to_owned()))),
					});
					states.insert("troo_xdie6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 18, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("troo_xdie7".to_owned()))),
					});
					states.insert("troo_xdie7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 19, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("troo_xdie8".to_owned()))),
					});
					states.insert("troo_xdie8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 20, full_bright: false},
						next: None,
					});
					states.insert("troo_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("troo_raise2".to_owned()))),
					});
					states.insert("troo_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("troo_raise3".to_owned()))),
					});
					states.insert("troo_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 10, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("troo_raise4".to_owned()))),
					});
					states.insert("troo_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 9, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("troo_raise5".to_owned()))),
					});
					states.insert("troo_raise5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 8, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("troo_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("troo_stnd".to_owned()))),
				spawn_state: Some("troo_stnd".to_owned()),
				see_state: Some("troo_run1".to_owned()),
				pain_state: Some("troo_pain".to_owned()),
				melee_state: Some("troo_atk1".to_owned()),
				missile_state: Some("troo_atk1".to_owned()),
				death_state: Some("troo_die1".to_owned()),
				xdeath_state: Some("troo_xdie1".to_owned()),
				raise_state: Some("troo_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("troop", template);

	let template = EntityTemplate {
		name: Some("sergeant"),
		type_id: Some(EntityTypeId::Thing(3002)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 30.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("sarg.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(27);
					states.insert("sarg_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("sarg_stnd2".to_owned()))),
					});
					states.insert("sarg_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("sarg_stnd".to_owned()))),
					});
					states.insert("sarg_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run2".to_owned()))),
					});
					states.insert("sarg_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run3".to_owned()))),
					});
					states.insert("sarg_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run4".to_owned()))),
					});
					states.insert("sarg_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run5".to_owned()))),
					});
					states.insert("sarg_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run6".to_owned()))),
					});
					states.insert("sarg_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run7".to_owned()))),
					});
					states.insert("sarg_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run8".to_owned()))),
					});
					states.insert("sarg_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run1".to_owned()))),
					});
					states.insert("sarg_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 4, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("sarg_atk2".to_owned()))),
					});
					states.insert("sarg_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 5, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("sarg_atk3".to_owned()))),
					});
					states.insert("sarg_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 6, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("sarg_run1".to_owned()))),
					});
					states.insert("sarg_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_pain2".to_owned()))),
					});
					states.insert("sarg_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run1".to_owned()))),
					});
					states.insert("sarg_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("sarg_die2".to_owned()))),
					});
					states.insert("sarg_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("sarg_die3".to_owned()))),
					});
					states.insert("sarg_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 10, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("sarg_die4".to_owned()))),
					});
					states.insert("sarg_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 11, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("sarg_die5".to_owned()))),
					});
					states.insert("sarg_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 12, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("sarg_die6".to_owned()))),
					});
					states.insert("sarg_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 13, full_bright: false},
						next: None,
					});
					states.insert("sarg_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sarg_raise2".to_owned()))),
					});
					states.insert("sarg_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sarg_raise3".to_owned()))),
					});
					states.insert("sarg_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sarg_raise4".to_owned()))),
					});
					states.insert("sarg_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sarg_raise5".to_owned()))),
					});
					states.insert("sarg_raise5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sarg_raise6".to_owned()))),
					});
					states.insert("sarg_raise6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sarg_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("sarg_stnd".to_owned()))),
				spawn_state: Some("sarg_stnd".to_owned()),
				see_state: Some("sarg_run1".to_owned()),
				pain_state: Some("sarg_pain".to_owned()),
				melee_state: Some("sarg_atk1".to_owned()),
				missile_state: None,
				death_state: Some("sarg_die1".to_owned()),
				xdeath_state: None,
				raise_state: Some("sarg_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("sergeant", template);

	let template = EntityTemplate {
		name: Some("shadows"),
		type_id: Some(EntityTypeId::Thing(58)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 30.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("sarg.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(27);
					states.insert("sarg_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("sarg_stnd2".to_owned()))),
					});
					states.insert("sarg_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("sarg_stnd".to_owned()))),
					});
					states.insert("sarg_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run2".to_owned()))),
					});
					states.insert("sarg_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 0, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run3".to_owned()))),
					});
					states.insert("sarg_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run4".to_owned()))),
					});
					states.insert("sarg_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 1, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run5".to_owned()))),
					});
					states.insert("sarg_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run6".to_owned()))),
					});
					states.insert("sarg_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 2, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run7".to_owned()))),
					});
					states.insert("sarg_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run8".to_owned()))),
					});
					states.insert("sarg_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 3, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run1".to_owned()))),
					});
					states.insert("sarg_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 4, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("sarg_atk2".to_owned()))),
					});
					states.insert("sarg_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 5, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("sarg_atk3".to_owned()))),
					});
					states.insert("sarg_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 6, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("sarg_run1".to_owned()))),
					});
					states.insert("sarg_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_pain2".to_owned()))),
					});
					states.insert("sarg_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("sarg_run1".to_owned()))),
					});
					states.insert("sarg_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("sarg_die2".to_owned()))),
					});
					states.insert("sarg_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("sarg_die3".to_owned()))),
					});
					states.insert("sarg_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 10, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("sarg_die4".to_owned()))),
					});
					states.insert("sarg_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 11, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("sarg_die5".to_owned()))),
					});
					states.insert("sarg_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 12, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("sarg_die6".to_owned()))),
					});
					states.insert("sarg_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 13, full_bright: false},
						next: None,
					});
					states.insert("sarg_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sarg_raise2".to_owned()))),
					});
					states.insert("sarg_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sarg_raise3".to_owned()))),
					});
					states.insert("sarg_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sarg_raise4".to_owned()))),
					});
					states.insert("sarg_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sarg_raise5".to_owned()))),
					});
					states.insert("sarg_raise5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sarg_raise6".to_owned()))),
					});
					states.insert("sarg_raise6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sarg_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("sarg_stnd".to_owned()))),
				spawn_state: Some("sarg_stnd".to_owned()),
				see_state: Some("sarg_run1".to_owned()),
				pain_state: Some("sarg_pain".to_owned()),
				melee_state: Some("sarg_atk1".to_owned()),
				missile_state: None,
				death_state: Some("sarg_die1".to_owned()),
				xdeath_state: None,
				raise_state: Some("sarg_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("shadows", template);

	let template = EntityTemplate {
		name: Some("head"),
		type_id: Some(EntityTypeId::Thing(3005)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 31.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("head.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(20);
					states.insert("head_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("head_stnd".to_owned()))),
					});
					states.insert("head_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("head_run1".to_owned()))),
					});
					states.insert("head_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 1, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("head_atk2".to_owned()))),
					});
					states.insert("head_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 2, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("head_atk3".to_owned()))),
					});
					states.insert("head_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 3, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("head_run1".to_owned()))),
					});
					states.insert("head_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 4, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("head_pain2".to_owned()))),
					});
					states.insert("head_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 4, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("head_pain3".to_owned()))),
					});
					states.insert("head_pain3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 5, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("head_run1".to_owned()))),
					});
					states.insert("head_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 6, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("head_die2".to_owned()))),
					});
					states.insert("head_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 7, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("head_die3".to_owned()))),
					});
					states.insert("head_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("head_die4".to_owned()))),
					});
					states.insert("head_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("head_die5".to_owned()))),
					});
					states.insert("head_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("head_die6".to_owned()))),
					});
					states.insert("head_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 11, full_bright: false},
						next: None,
					});
					states.insert("head_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("head_raise2".to_owned()))),
					});
					states.insert("head_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("head_raise3".to_owned()))),
					});
					states.insert("head_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("head_raise4".to_owned()))),
					});
					states.insert("head_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("head_raise5".to_owned()))),
					});
					states.insert("head_raise5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 7, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("head_raise6".to_owned()))),
					});
					states.insert("head_raise6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 6, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("head_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("head_stnd".to_owned()))),
				spawn_state: Some("head_stnd".to_owned()),
				see_state: Some("head_run1".to_owned()),
				pain_state: Some("head_pain".to_owned()),
				melee_state: None,
				missile_state: Some("head_atk1".to_owned()),
				death_state: Some("head_die1".to_owned()),
				xdeath_state: None,
				raise_state: Some("head_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("head", template);

	let template = EntityTemplate {
		name: Some("bruiser"),
		type_id: Some(EntityTypeId::Thing(3003)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 24.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("boss.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(29);
					states.insert("boss_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("boss_stnd2".to_owned()))),
					});
					states.insert("boss_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("boss_stnd".to_owned()))),
					});
					states.insert("boss_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("boss_run2".to_owned()))),
					});
					states.insert("boss_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("boss_run3".to_owned()))),
					});
					states.insert("boss_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("boss_run4".to_owned()))),
					});
					states.insert("boss_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("boss_run5".to_owned()))),
					});
					states.insert("boss_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("boss_run6".to_owned()))),
					});
					states.insert("boss_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("boss_run7".to_owned()))),
					});
					states.insert("boss_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("boss_run8".to_owned()))),
					});
					states.insert("boss_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("boss_run1".to_owned()))),
					});
					states.insert("boss_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 4, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_atk2".to_owned()))),
					});
					states.insert("boss_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 5, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_atk3".to_owned()))),
					});
					states.insert("boss_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 6, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_run1".to_owned()))),
					});
					states.insert("boss_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("boss_pain2".to_owned()))),
					});
					states.insert("boss_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("boss_run1".to_owned()))),
					});
					states.insert("boss_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_die2".to_owned()))),
					});
					states.insert("boss_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_die3".to_owned()))),
					});
					states.insert("boss_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_die4".to_owned()))),
					});
					states.insert("boss_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_die5".to_owned()))),
					});
					states.insert("boss_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_die6".to_owned()))),
					});
					states.insert("boss_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 13, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_die7".to_owned()))),
					});
					states.insert("boss_die7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 14, full_bright: false},
						next: None,
					});
					states.insert("boss_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 14, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_raise2".to_owned()))),
					});
					states.insert("boss_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 13, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_raise3".to_owned()))),
					});
					states.insert("boss_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_raise4".to_owned()))),
					});
					states.insert("boss_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_raise5".to_owned()))),
					});
					states.insert("boss_raise5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_raise6".to_owned()))),
					});
					states.insert("boss_raise6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_raise7".to_owned()))),
					});
					states.insert("boss_raise7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("boss_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("boss_stnd".to_owned()))),
				spawn_state: Some("boss_stnd".to_owned()),
				see_state: Some("boss_run1".to_owned()),
				pain_state: Some("boss_pain".to_owned()),
				melee_state: Some("boss_atk1".to_owned()),
				missile_state: Some("boss_atk1".to_owned()),
				death_state: Some("boss_die1".to_owned()),
				xdeath_state: None,
				raise_state: Some("boss_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("bruiser", template);

	let template = EntityTemplate {
		name: Some("bruisershot"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("bal7.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(5);
					states.insert("brball1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal7.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("brball2".to_owned()))),
					});
					states.insert("brball2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal7.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("brball1".to_owned()))),
					});
					states.insert("brballx1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal7.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("brballx2".to_owned()))),
					});
					states.insert("brballx2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal7.sprite"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("brballx3".to_owned()))),
					});
					states.insert("brballx3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal7.sprite"), frame: 4, full_bright: true},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("brball1".to_owned()))),
				spawn_state: Some("brball1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("brballx1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("bruisershot", template);

	let template = EntityTemplate {
		name: Some("knight"),
		type_id: Some(EntityTypeId::Thing(69)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 24.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("bos2.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(29);
					states.insert("bos2_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("bos2_stnd2".to_owned()))),
					});
					states.insert("bos2_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("bos2_stnd".to_owned()))),
					});
					states.insert("bos2_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bos2_run2".to_owned()))),
					});
					states.insert("bos2_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bos2_run3".to_owned()))),
					});
					states.insert("bos2_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bos2_run4".to_owned()))),
					});
					states.insert("bos2_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bos2_run5".to_owned()))),
					});
					states.insert("bos2_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bos2_run6".to_owned()))),
					});
					states.insert("bos2_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bos2_run7".to_owned()))),
					});
					states.insert("bos2_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bos2_run8".to_owned()))),
					});
					states.insert("bos2_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bos2_run1".to_owned()))),
					});
					states.insert("bos2_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 4, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_atk2".to_owned()))),
					});
					states.insert("bos2_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 5, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_atk3".to_owned()))),
					});
					states.insert("bos2_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 6, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_run1".to_owned()))),
					});
					states.insert("bos2_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("bos2_pain2".to_owned()))),
					});
					states.insert("bos2_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 7, full_bright: false},
						next: Some((2 * FRAME_TIME, Some("bos2_run1".to_owned()))),
					});
					states.insert("bos2_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_die2".to_owned()))),
					});
					states.insert("bos2_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_die3".to_owned()))),
					});
					states.insert("bos2_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_die4".to_owned()))),
					});
					states.insert("bos2_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_die5".to_owned()))),
					});
					states.insert("bos2_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_die6".to_owned()))),
					});
					states.insert("bos2_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 13, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_die7".to_owned()))),
					});
					states.insert("bos2_die7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 14, full_bright: false},
						next: None,
					});
					states.insert("bos2_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 14, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_raise2".to_owned()))),
					});
					states.insert("bos2_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 13, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_raise3".to_owned()))),
					});
					states.insert("bos2_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_raise4".to_owned()))),
					});
					states.insert("bos2_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_raise5".to_owned()))),
					});
					states.insert("bos2_raise5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_raise6".to_owned()))),
					});
					states.insert("bos2_raise6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_raise7".to_owned()))),
					});
					states.insert("bos2_raise7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bos2_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bos2_stnd".to_owned()))),
				spawn_state: Some("bos2_stnd".to_owned()),
				see_state: Some("bos2_run1".to_owned()),
				pain_state: Some("bos2_pain".to_owned()),
				melee_state: Some("bos2_atk1".to_owned()),
				missile_state: Some("bos2_atk1".to_owned()),
				death_state: Some("bos2_die1".to_owned()),
				xdeath_state: None,
				raise_state: Some("bos2_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("knight", template);

	let template = EntityTemplate {
		name: Some("skull"),
		type_id: Some(EntityTypeId::Thing(3006)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("skul.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(16);
					states.insert("skull_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 0, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("skull_stnd2".to_owned()))),
					});
					states.insert("skull_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("skull_stnd".to_owned()))),
					});
					states.insert("skull_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("skull_run2".to_owned()))),
					});
					states.insert("skull_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("skull_run1".to_owned()))),
					});
					states.insert("skull_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 2, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("skull_atk2".to_owned()))),
					});
					states.insert("skull_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("skull_atk3".to_owned()))),
					});
					states.insert("skull_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("skull_atk4".to_owned()))),
					});
					states.insert("skull_atk4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("skull_atk3".to_owned()))),
					});
					states.insert("skull_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 4, full_bright: true},
						next: Some((3 * FRAME_TIME, Some("skull_pain2".to_owned()))),
					});
					states.insert("skull_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 4, full_bright: true},
						next: Some((3 * FRAME_TIME, Some("skull_run1".to_owned()))),
					});
					states.insert("skull_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 5, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("skull_die2".to_owned()))),
					});
					states.insert("skull_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 6, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("skull_die3".to_owned()))),
					});
					states.insert("skull_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 7, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("skull_die4".to_owned()))),
					});
					states.insert("skull_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 8, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("skull_die5".to_owned()))),
					});
					states.insert("skull_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 9, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("skull_die6".to_owned()))),
					});
					states.insert("skull_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 10, full_bright: false},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("skull_stnd".to_owned()))),
				spawn_state: Some("skull_stnd".to_owned()),
				see_state: Some("skull_run1".to_owned()),
				pain_state: Some("skull_pain".to_owned()),
				melee_state: None,
				missile_state: Some("skull_atk1".to_owned()),
				death_state: Some("skull_die1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("skull", template);

	let template = EntityTemplate {
		name: Some("spider"),
		type_id: Some(EntityTypeId::Thing(7)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 100.0,
				radius: 128.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("spid.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(31);
					states.insert("spid_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spid_stnd2".to_owned()))),
					});
					states.insert("spid_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spid_stnd".to_owned()))),
					});
					states.insert("spid_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_run2".to_owned()))),
					});
					states.insert("spid_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_run3".to_owned()))),
					});
					states.insert("spid_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_run4".to_owned()))),
					});
					states.insert("spid_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_run5".to_owned()))),
					});
					states.insert("spid_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_run6".to_owned()))),
					});
					states.insert("spid_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_run7".to_owned()))),
					});
					states.insert("spid_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_run8".to_owned()))),
					});
					states.insert("spid_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_run9".to_owned()))),
					});
					states.insert("spid_run9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 4, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_run10".to_owned()))),
					});
					states.insert("spid_run10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 4, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_run11".to_owned()))),
					});
					states.insert("spid_run11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 5, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_run12".to_owned()))),
					});
					states.insert("spid_run12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 5, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_run1".to_owned()))),
					});
					states.insert("spid_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 0, full_bright: true},
						next: Some((20 * FRAME_TIME, Some("spid_atk2".to_owned()))),
					});
					states.insert("spid_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 6, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("spid_atk3".to_owned()))),
					});
					states.insert("spid_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 7, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("spid_atk4".to_owned()))),
					});
					states.insert("spid_atk4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 7, full_bright: true},
						next: Some((1 * FRAME_TIME, Some("spid_atk2".to_owned()))),
					});
					states.insert("spid_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 8, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_pain2".to_owned()))),
					});
					states.insert("spid_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 8, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("spid_run1".to_owned()))),
					});
					states.insert("spid_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 9, full_bright: false},
						next: Some((20 * FRAME_TIME, Some("spid_die2".to_owned()))),
					});
					states.insert("spid_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 10, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spid_die3".to_owned()))),
					});
					states.insert("spid_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 11, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spid_die4".to_owned()))),
					});
					states.insert("spid_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 12, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spid_die5".to_owned()))),
					});
					states.insert("spid_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 13, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spid_die6".to_owned()))),
					});
					states.insert("spid_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 14, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spid_die7".to_owned()))),
					});
					states.insert("spid_die7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 15, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spid_die8".to_owned()))),
					});
					states.insert("spid_die8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 16, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spid_die9".to_owned()))),
					});
					states.insert("spid_die9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 17, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("spid_die10".to_owned()))),
					});
					states.insert("spid_die10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 18, full_bright: false},
						next: Some((30 * FRAME_TIME, Some("spid_die11".to_owned()))),
					});
					states.insert("spid_die11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 18, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("spid_stnd".to_owned()))),
				spawn_state: Some("spid_stnd".to_owned()),
				see_state: Some("spid_run1".to_owned()),
				pain_state: Some("spid_pain".to_owned()),
				melee_state: None,
				missile_state: Some("spid_atk1".to_owned()),
				death_state: Some("spid_die1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("spider", template);

	let template = EntityTemplate {
		name: Some("baby"),
		type_id: Some(EntityTypeId::Thing(68)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 64.0,
				radius: 64.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("bspi.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(35);
					states.insert("bspi_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("bspi_stnd2".to_owned()))),
					});
					states.insert("bspi_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("bspi_stnd".to_owned()))),
					});
					states.insert("bspi_sight".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 0, full_bright: false},
						next: Some((20 * FRAME_TIME, Some("bspi_run1".to_owned()))),
					});
					states.insert("bspi_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_run2".to_owned()))),
					});
					states.insert("bspi_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_run3".to_owned()))),
					});
					states.insert("bspi_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_run4".to_owned()))),
					});
					states.insert("bspi_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_run5".to_owned()))),
					});
					states.insert("bspi_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_run6".to_owned()))),
					});
					states.insert("bspi_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_run7".to_owned()))),
					});
					states.insert("bspi_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_run8".to_owned()))),
					});
					states.insert("bspi_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_run9".to_owned()))),
					});
					states.insert("bspi_run9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 4, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_run10".to_owned()))),
					});
					states.insert("bspi_run10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 4, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_run11".to_owned()))),
					});
					states.insert("bspi_run11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 5, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_run12".to_owned()))),
					});
					states.insert("bspi_run12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 5, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_run1".to_owned()))),
					});
					states.insert("bspi_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 0, full_bright: true},
						next: Some((20 * FRAME_TIME, Some("bspi_atk2".to_owned()))),
					});
					states.insert("bspi_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 6, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("bspi_atk3".to_owned()))),
					});
					states.insert("bspi_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 7, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("bspi_atk4".to_owned()))),
					});
					states.insert("bspi_atk4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 7, full_bright: true},
						next: Some((1 * FRAME_TIME, Some("bspi_atk2".to_owned()))),
					});
					states.insert("bspi_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 8, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_pain2".to_owned()))),
					});
					states.insert("bspi_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 8, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("bspi_run1".to_owned()))),
					});
					states.insert("bspi_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 9, full_bright: false},
						next: Some((20 * FRAME_TIME, Some("bspi_die2".to_owned()))),
					});
					states.insert("bspi_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 10, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("bspi_die3".to_owned()))),
					});
					states.insert("bspi_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 11, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("bspi_die4".to_owned()))),
					});
					states.insert("bspi_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 12, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("bspi_die5".to_owned()))),
					});
					states.insert("bspi_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 13, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("bspi_die6".to_owned()))),
					});
					states.insert("bspi_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 14, full_bright: false},
						next: Some((7 * FRAME_TIME, Some("bspi_die7".to_owned()))),
					});
					states.insert("bspi_die7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 15, full_bright: false},
						next: None,
					});
					states.insert("bspi_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("bspi_raise2".to_owned()))),
					});
					states.insert("bspi_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("bspi_raise3".to_owned()))),
					});
					states.insert("bspi_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("bspi_raise4".to_owned()))),
					});
					states.insert("bspi_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("bspi_raise5".to_owned()))),
					});
					states.insert("bspi_raise5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("bspi_raise6".to_owned()))),
					});
					states.insert("bspi_raise6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("bspi_raise7".to_owned()))),
					});
					states.insert("bspi_raise7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("bspi_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bspi_stnd".to_owned()))),
				spawn_state: Some("bspi_stnd".to_owned()),
				see_state: Some("bspi_sight".to_owned()),
				pain_state: Some("bspi_pain".to_owned()),
				melee_state: None,
				missile_state: Some("bspi_atk1".to_owned()),
				death_state: Some("bspi_die1".to_owned()),
				xdeath_state: None,
				raise_state: Some("bspi_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("baby", template);

	let template = EntityTemplate {
		name: Some("cyborg"),
		type_id: Some(EntityTypeId::Thing(16)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 110.0,
				radius: 40.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("cybr.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(27);
					states.insert("cyber_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cyber_stnd2".to_owned()))),
					});
					states.insert("cyber_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cyber_stnd".to_owned()))),
					});
					states.insert("cyber_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cyber_run2".to_owned()))),
					});
					states.insert("cyber_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cyber_run3".to_owned()))),
					});
					states.insert("cyber_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cyber_run4".to_owned()))),
					});
					states.insert("cyber_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cyber_run5".to_owned()))),
					});
					states.insert("cyber_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cyber_run6".to_owned()))),
					});
					states.insert("cyber_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cyber_run7".to_owned()))),
					});
					states.insert("cyber_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cyber_run8".to_owned()))),
					});
					states.insert("cyber_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("cyber_run1".to_owned()))),
					});
					states.insert("cyber_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 4, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("cyber_atk2".to_owned()))),
					});
					states.insert("cyber_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 5, full_bright: false},
						next: Some((12 * FRAME_TIME, Some("cyber_atk3".to_owned()))),
					});
					states.insert("cyber_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 4, full_bright: false},
						next: Some((12 * FRAME_TIME, Some("cyber_atk4".to_owned()))),
					});
					states.insert("cyber_atk4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 5, full_bright: false},
						next: Some((12 * FRAME_TIME, Some("cyber_atk5".to_owned()))),
					});
					states.insert("cyber_atk5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 4, full_bright: false},
						next: Some((12 * FRAME_TIME, Some("cyber_atk6".to_owned()))),
					});
					states.insert("cyber_atk6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 5, full_bright: false},
						next: Some((12 * FRAME_TIME, Some("cyber_run1".to_owned()))),
					});
					states.insert("cyber_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 6, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cyber_run1".to_owned()))),
					});
					states.insert("cyber_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 7, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cyber_die2".to_owned()))),
					});
					states.insert("cyber_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 8, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cyber_die3".to_owned()))),
					});
					states.insert("cyber_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 9, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cyber_die4".to_owned()))),
					});
					states.insert("cyber_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 10, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cyber_die5".to_owned()))),
					});
					states.insert("cyber_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 11, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cyber_die6".to_owned()))),
					});
					states.insert("cyber_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 12, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cyber_die7".to_owned()))),
					});
					states.insert("cyber_die7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 13, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cyber_die8".to_owned()))),
					});
					states.insert("cyber_die8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 14, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("cyber_die9".to_owned()))),
					});
					states.insert("cyber_die9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 15, full_bright: false},
						next: Some((30 * FRAME_TIME, Some("cyber_die10".to_owned()))),
					});
					states.insert("cyber_die10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 15, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("cyber_stnd".to_owned()))),
				spawn_state: Some("cyber_stnd".to_owned()),
				see_state: Some("cyber_run1".to_owned()),
				pain_state: Some("cyber_pain".to_owned()),
				melee_state: None,
				missile_state: Some("cyber_atk1".to_owned()),
				death_state: Some("cyber_die1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("cyborg", template);

	let template = EntityTemplate {
		name: Some("pain"),
		type_id: Some(EntityTypeId::Thing(71)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 31.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("pain.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(25);
					states.insert("pain_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("pain_stnd".to_owned()))),
					});
					states.insert("pain_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("pain_run2".to_owned()))),
					});
					states.insert("pain_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("pain_run3".to_owned()))),
					});
					states.insert("pain_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("pain_run4".to_owned()))),
					});
					states.insert("pain_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("pain_run5".to_owned()))),
					});
					states.insert("pain_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("pain_run6".to_owned()))),
					});
					states.insert("pain_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("pain_run1".to_owned()))),
					});
					states.insert("pain_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 3, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("pain_atk2".to_owned()))),
					});
					states.insert("pain_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 4, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("pain_atk3".to_owned()))),
					});
					states.insert("pain_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 5, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("pain_atk4".to_owned()))),
					});
					states.insert("pain_atk4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 5, full_bright: true},
						next: Some((0 * FRAME_TIME, Some("pain_run1".to_owned()))),
					});
					states.insert("pain_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 6, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("pain_pain2".to_owned()))),
					});
					states.insert("pain_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 6, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("pain_run1".to_owned()))),
					});
					states.insert("pain_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 7, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("pain_die2".to_owned()))),
					});
					states.insert("pain_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 8, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("pain_die3".to_owned()))),
					});
					states.insert("pain_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 9, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("pain_die4".to_owned()))),
					});
					states.insert("pain_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 10, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("pain_die5".to_owned()))),
					});
					states.insert("pain_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 11, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("pain_die6".to_owned()))),
					});
					states.insert("pain_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 12, full_bright: true},
						next: Some((8 * FRAME_TIME, None)),
					});
					states.insert("pain_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("pain_raise2".to_owned()))),
					});
					states.insert("pain_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 11, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("pain_raise3".to_owned()))),
					});
					states.insert("pain_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 10, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("pain_raise4".to_owned()))),
					});
					states.insert("pain_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 9, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("pain_raise5".to_owned()))),
					});
					states.insert("pain_raise5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 8, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("pain_raise6".to_owned()))),
					});
					states.insert("pain_raise6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 7, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("pain_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("pain_stnd".to_owned()))),
				spawn_state: Some("pain_stnd".to_owned()),
				see_state: Some("pain_run1".to_owned()),
				pain_state: Some("pain_pain".to_owned()),
				melee_state: None,
				missile_state: Some("pain_atk1".to_owned()),
				death_state: Some("pain_die1".to_owned()),
				xdeath_state: None,
				raise_state: Some("pain_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("pain", template);

	let template = EntityTemplate {
		name: Some("wolfss"),
		type_id: Some(EntityTypeId::Thing(84)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 20.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("sswv.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(37);
					states.insert("sswv_stnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("sswv_stnd2".to_owned()))),
					});
					states.insert("sswv_stnd2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 1, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("sswv_stnd".to_owned()))),
					});
					states.insert("sswv_run1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("sswv_run2".to_owned()))),
					});
					states.insert("sswv_run2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 0, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("sswv_run3".to_owned()))),
					});
					states.insert("sswv_run3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("sswv_run4".to_owned()))),
					});
					states.insert("sswv_run4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 1, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("sswv_run5".to_owned()))),
					});
					states.insert("sswv_run5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("sswv_run6".to_owned()))),
					});
					states.insert("sswv_run6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 2, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("sswv_run7".to_owned()))),
					});
					states.insert("sswv_run7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("sswv_run8".to_owned()))),
					});
					states.insert("sswv_run8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 3, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("sswv_run1".to_owned()))),
					});
					states.insert("sswv_atk1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 4, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("sswv_atk2".to_owned()))),
					});
					states.insert("sswv_atk2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 5, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("sswv_atk3".to_owned()))),
					});
					states.insert("sswv_atk3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 6, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("sswv_atk4".to_owned()))),
					});
					states.insert("sswv_atk4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 5, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("sswv_atk5".to_owned()))),
					});
					states.insert("sswv_atk5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 6, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("sswv_atk6".to_owned()))),
					});
					states.insert("sswv_atk6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 5, full_bright: false},
						next: Some((1 * FRAME_TIME, Some("sswv_atk2".to_owned()))),
					});
					states.insert("sswv_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 7, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("sswv_pain2".to_owned()))),
					});
					states.insert("sswv_pain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 7, full_bright: false},
						next: Some((3 * FRAME_TIME, Some("sswv_run1".to_owned()))),
					});
					states.insert("sswv_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_die2".to_owned()))),
					});
					states.insert("sswv_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_die3".to_owned()))),
					});
					states.insert("sswv_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_die4".to_owned()))),
					});
					states.insert("sswv_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_die5".to_owned()))),
					});
					states.insert("sswv_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 12, full_bright: false},
						next: None,
					});
					states.insert("sswv_xdie1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 13, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_xdie2".to_owned()))),
					});
					states.insert("sswv_xdie2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 14, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_xdie3".to_owned()))),
					});
					states.insert("sswv_xdie3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 15, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_xdie4".to_owned()))),
					});
					states.insert("sswv_xdie4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 16, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_xdie5".to_owned()))),
					});
					states.insert("sswv_xdie5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 17, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_xdie6".to_owned()))),
					});
					states.insert("sswv_xdie6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 18, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_xdie7".to_owned()))),
					});
					states.insert("sswv_xdie7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 19, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_xdie8".to_owned()))),
					});
					states.insert("sswv_xdie8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 20, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_xdie9".to_owned()))),
					});
					states.insert("sswv_xdie9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 21, full_bright: false},
						next: None,
					});
					states.insert("sswv_raise1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 12, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_raise2".to_owned()))),
					});
					states.insert("sswv_raise2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 11, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_raise3".to_owned()))),
					});
					states.insert("sswv_raise3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 10, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_raise4".to_owned()))),
					});
					states.insert("sswv_raise4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 9, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_raise5".to_owned()))),
					});
					states.insert("sswv_raise5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 8, full_bright: false},
						next: Some((5 * FRAME_TIME, Some("sswv_run1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("sswv_stnd".to_owned()))),
				spawn_state: Some("sswv_stnd".to_owned()),
				see_state: Some("sswv_run1".to_owned()),
				pain_state: Some("sswv_pain".to_owned()),
				melee_state: None,
				missile_state: Some("sswv_atk1".to_owned()),
				death_state: Some("sswv_die1".to_owned()),
				xdeath_state: Some("sswv_xdie1".to_owned()),
				raise_state: Some("sswv_raise1".to_owned()),
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("wolfss", template);

	let template = EntityTemplate {
		name: Some("keen"),
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
				sprite: asset_storage.load("keen.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(15);
					states.insert("keenstnd".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states.insert("commkeen".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("commkeen2".to_owned()))),
					});
					states.insert("commkeen2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("commkeen3".to_owned()))),
					});
					states.insert("commkeen3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 2, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("commkeen4".to_owned()))),
					});
					states.insert("commkeen4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 3, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("commkeen5".to_owned()))),
					});
					states.insert("commkeen5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 4, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("commkeen6".to_owned()))),
					});
					states.insert("commkeen6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 5, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("commkeen7".to_owned()))),
					});
					states.insert("commkeen7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 6, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("commkeen8".to_owned()))),
					});
					states.insert("commkeen8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 7, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("commkeen9".to_owned()))),
					});
					states.insert("commkeen9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 8, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("commkeen10".to_owned()))),
					});
					states.insert("commkeen10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 9, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("commkeen11".to_owned()))),
					});
					states.insert("commkeen11".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 10, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("commkeen12".to_owned()))),
					});
					states.insert("commkeen12".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 11, full_bright: false},
						next: None,
					});
					states.insert("keenpain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 12, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("keenpain2".to_owned()))),
					});
					states.insert("keenpain2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 12, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("keenstnd".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("keenstnd".to_owned()))),
				spawn_state: Some("keenstnd".to_owned()),
				see_state: None,
				pain_state: Some("keenpain".to_owned()),
				melee_state: None,
				missile_state: None,
				death_state: Some("commkeen".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("keen", template);

	let template = EntityTemplate {
		name: Some("bossbrain"),
		type_id: Some(EntityTypeId::Thing(88)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("bbrn.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(6);
					states.insert("brain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bbrn.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states.insert("brain_pain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bbrn.sprite"), frame: 1, full_bright: false},
						next: Some((36 * FRAME_TIME, Some("brain".to_owned()))),
					});
					states.insert("brain_die1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bbrn.sprite"), frame: 0, full_bright: false},
						next: Some((100 * FRAME_TIME, Some("brain_die2".to_owned()))),
					});
					states.insert("brain_die2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bbrn.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("brain_die3".to_owned()))),
					});
					states.insert("brain_die3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bbrn.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("brain_die4".to_owned()))),
					});
					states.insert("brain_die4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bbrn.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("brain".to_owned()))),
				spawn_state: Some("brain".to_owned()),
				see_state: None,
				pain_state: Some("brain_pain".to_owned()),
				melee_state: None,
				missile_state: None,
				death_state: Some("brain_die1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("bossbrain", template);

	let template = EntityTemplate {
		name: Some("bossspit"),
		type_id: Some(EntityTypeId::Thing(89)),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("sswv.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(3);
					states.insert("braineye".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("braineye".to_owned()))),
					});
					states.insert("braineyesee".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 0, full_bright: false},
						next: Some((181 * FRAME_TIME, Some("braineye1".to_owned()))),
					});
					states.insert("braineye1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 0, full_bright: false},
						next: Some((150 * FRAME_TIME, Some("braineye1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("braineye".to_owned()))),
				spawn_state: Some("braineye".to_owned()),
				see_state: Some("braineyesee".to_owned()),
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("bossspit", template);

	let template = EntityTemplate {
		name: Some("bosstarget"),
		type_id: Some(EntityTypeId::Thing(87)),
		components: EntityComponents::new(),
	};
	asset_storage.insert_with_name("bosstarget", template);

	let template = EntityTemplate {
		name: Some("spawnshot"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("bosf.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("spawn1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bosf.sprite"), frame: 0, full_bright: true},
						next: Some((3 * FRAME_TIME, Some("spawn2".to_owned()))),
					});
					states.insert("spawn2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bosf.sprite"), frame: 1, full_bright: true},
						next: Some((3 * FRAME_TIME, Some("spawn3".to_owned()))),
					});
					states.insert("spawn3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bosf.sprite"), frame: 2, full_bright: true},
						next: Some((3 * FRAME_TIME, Some("spawn4".to_owned()))),
					});
					states.insert("spawn4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bosf.sprite"), frame: 3, full_bright: true},
						next: Some((3 * FRAME_TIME, Some("spawn1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("spawn1".to_owned()))),
				spawn_state: Some("spawn1".to_owned()),
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
	asset_storage.insert_with_name("spawnshot", template);

	let template = EntityTemplate {
		name: Some("spawnfire"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("fire.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(8);
					states.insert("spawnfire1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("spawnfire2".to_owned()))),
					});
					states.insert("spawnfire2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("spawnfire3".to_owned()))),
					});
					states.insert("spawnfire3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("spawnfire4".to_owned()))),
					});
					states.insert("spawnfire4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("spawnfire5".to_owned()))),
					});
					states.insert("spawnfire5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 4, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("spawnfire6".to_owned()))),
					});
					states.insert("spawnfire6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 5, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("spawnfire7".to_owned()))),
					});
					states.insert("spawnfire7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 6, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("spawnfire8".to_owned()))),
					});
					states.insert("spawnfire8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 7, full_bright: true},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("spawnfire1".to_owned()))),
				spawn_state: Some("spawnfire1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("spawnfire", template);

	let template = EntityTemplate {
		name: Some("barrel"),
		type_id: Some(EntityTypeId::Thing(2035)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 42.0,
				radius: 10.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("bar1.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(7);
					states.insert("bar1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bar1.sprite"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bar2".to_owned()))),
					});
					states.insert("bar2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bar1.sprite"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bar1".to_owned()))),
					});
					states.insert("bexp".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bexp.sprite"), frame: 0, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("bexp2".to_owned()))),
					});
					states.insert("bexp2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bexp.sprite"), frame: 1, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("bexp3".to_owned()))),
					});
					states.insert("bexp3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bexp.sprite"), frame: 2, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("bexp4".to_owned()))),
					});
					states.insert("bexp4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bexp.sprite"), frame: 3, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("bexp5".to_owned()))),
					});
					states.insert("bexp5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bexp.sprite"), frame: 4, full_bright: true},
						next: Some((10 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bar1".to_owned()))),
				spawn_state: Some("bar1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("bexp".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("barrel", template);

	let template = EntityTemplate {
		name: Some("troopshot"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("bal1.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(5);
					states.insert("tball1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal1.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("tball2".to_owned()))),
					});
					states.insert("tball2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal1.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("tball1".to_owned()))),
					});
					states.insert("tballx1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal1.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("tballx2".to_owned()))),
					});
					states.insert("tballx2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal1.sprite"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("tballx3".to_owned()))),
					});
					states.insert("tballx3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal1.sprite"), frame: 4, full_bright: true},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("tball1".to_owned()))),
				spawn_state: Some("tball1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("tballx1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("troopshot", template);

	let template = EntityTemplate {
		name: Some("headshot"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("bal2.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(5);
					states.insert("rball1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal2.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("rball2".to_owned()))),
					});
					states.insert("rball2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal2.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("rball1".to_owned()))),
					});
					states.insert("rballx1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal2.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("rballx2".to_owned()))),
					});
					states.insert("rballx2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal2.sprite"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("rballx3".to_owned()))),
					});
					states.insert("rballx3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bal2.sprite"), frame: 4, full_bright: true},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("rball1".to_owned()))),
				spawn_state: Some("rball1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("rballx1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("headshot", template);

	let template = EntityTemplate {
		name: Some("rocket"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("misl.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("rocket".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 0, full_bright: true},
						next: Some((1 * FRAME_TIME, Some("rocket".to_owned()))),
					});
					states.insert("explode1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 1, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("explode2".to_owned()))),
					});
					states.insert("explode2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("explode3".to_owned()))),
					});
					states.insert("explode3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("rocket".to_owned()))),
				spawn_state: Some("rocket".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("explode1".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("rocket", template);

	let template = EntityTemplate {
		name: Some("plasma"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("plss.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(7);
					states.insert("plasball".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("plss.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("plasball2".to_owned()))),
					});
					states.insert("plasball2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("plss.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("plasball".to_owned()))),
					});
					states.insert("plasexp".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("plse.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("plasexp2".to_owned()))),
					});
					states.insert("plasexp2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("plse.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("plasexp3".to_owned()))),
					});
					states.insert("plasexp3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("plse.sprite"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("plasexp4".to_owned()))),
					});
					states.insert("plasexp4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("plse.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("plasexp5".to_owned()))),
					});
					states.insert("plasexp5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("plse.sprite"), frame: 4, full_bright: true},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("plasball".to_owned()))),
				spawn_state: Some("plasball".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("plasexp".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("plasma", template);

	let template = EntityTemplate {
		name: Some("bfg"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("bfs1.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(8);
					states.insert("bfgshot".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bfs1.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("bfgshot2".to_owned()))),
					});
					states.insert("bfgshot2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bfs1.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("bfgshot".to_owned()))),
					});
					states.insert("bfgland".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bfe1.sprite"), frame: 0, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("bfgland2".to_owned()))),
					});
					states.insert("bfgland2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bfe1.sprite"), frame: 1, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("bfgland3".to_owned()))),
					});
					states.insert("bfgland3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bfe1.sprite"), frame: 2, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("bfgland4".to_owned()))),
					});
					states.insert("bfgland4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bfe1.sprite"), frame: 3, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("bfgland5".to_owned()))),
					});
					states.insert("bfgland5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bfe1.sprite"), frame: 4, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("bfgland6".to_owned()))),
					});
					states.insert("bfgland6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bfe1.sprite"), frame: 5, full_bright: true},
						next: Some((8 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bfgshot".to_owned()))),
				spawn_state: Some("bfgshot".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("bfgland".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("bfg", template);

	let template = EntityTemplate {
		name: Some("arachplaz"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("apls.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(7);
					states.insert("arach_plaz".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("apls.sprite"), frame: 0, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("arach_plaz2".to_owned()))),
					});
					states.insert("arach_plaz2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("apls.sprite"), frame: 1, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("arach_plaz".to_owned()))),
					});
					states.insert("arach_plex".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("apbx.sprite"), frame: 0, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("arach_plex2".to_owned()))),
					});
					states.insert("arach_plex2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("apbx.sprite"), frame: 1, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("arach_plex3".to_owned()))),
					});
					states.insert("arach_plex3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("apbx.sprite"), frame: 2, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("arach_plex4".to_owned()))),
					});
					states.insert("arach_plex4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("apbx.sprite"), frame: 3, full_bright: true},
						next: Some((5 * FRAME_TIME, Some("arach_plex5".to_owned()))),
					});
					states.insert("arach_plex5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("apbx.sprite"), frame: 4, full_bright: true},
						next: Some((5 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("arach_plaz".to_owned()))),
				spawn_state: Some("arach_plaz".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: Some("arach_plex".to_owned()),
				xdeath_state: None,
				raise_state: None,
			})
			.with_component(Velocity::default()),
	};
	asset_storage.insert_with_name("arachplaz", template);

	let template = EntityTemplate {
		name: Some("puff"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("puff.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("puff1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("puff2".to_owned()))),
					});
					states.insert("puff2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 1, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("puff3".to_owned()))),
					});
					states.insert("puff3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 2, full_bright: false},
						next: Some((4 * FRAME_TIME, Some("puff4".to_owned()))),
					});
					states.insert("puff4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 3, full_bright: false},
						next: Some((4 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("puff1".to_owned()))),
				spawn_state: Some("puff1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("puff", template);

	let template = EntityTemplate {
		name: Some("blood"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("blud.sprite"),
				frame: 2,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(3);
					states.insert("blood1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("blud.sprite"), frame: 2, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("blood2".to_owned()))),
					});
					states.insert("blood2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("blud.sprite"), frame: 1, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("blood3".to_owned()))),
					});
					states.insert("blood3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("blud.sprite"), frame: 0, full_bright: false},
						next: Some((8 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("blood1".to_owned()))),
				spawn_state: Some("blood1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("blood", template);

	let template = EntityTemplate {
		name: Some("tfog"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("tfog.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(12);
					states.insert("tfog".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("tfog01".to_owned()))),
					});
					states.insert("tfog01".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("tfog02".to_owned()))),
					});
					states.insert("tfog02".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("tfog2".to_owned()))),
					});
					states.insert("tfog2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("tfog3".to_owned()))),
					});
					states.insert("tfog3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("tfog4".to_owned()))),
					});
					states.insert("tfog4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("tfog5".to_owned()))),
					});
					states.insert("tfog5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 4, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("tfog6".to_owned()))),
					});
					states.insert("tfog6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 5, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("tfog7".to_owned()))),
					});
					states.insert("tfog7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 6, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("tfog8".to_owned()))),
					});
					states.insert("tfog8".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 7, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("tfog9".to_owned()))),
					});
					states.insert("tfog9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 8, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("tfog10".to_owned()))),
					});
					states.insert("tfog10".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 9, full_bright: true},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("tfog".to_owned()))),
				spawn_state: Some("tfog".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("tfog", template);

	let template = EntityTemplate {
		name: Some("ifog"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("ifog.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(7);
					states.insert("ifog".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("ifog01".to_owned()))),
					});
					states.insert("ifog01".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("ifog02".to_owned()))),
					});
					states.insert("ifog02".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("ifog2".to_owned()))),
					});
					states.insert("ifog2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("ifog3".to_owned()))),
					});
					states.insert("ifog3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("ifog4".to_owned()))),
					});
					states.insert("ifog4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("ifog5".to_owned()))),
					});
					states.insert("ifog5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 4, full_bright: true},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("ifog".to_owned()))),
				spawn_state: Some("ifog".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("ifog", template);

	let template = EntityTemplate {
		name: Some("teleportman"),
		type_id: Some(EntityTypeId::Thing(14)),
		components: EntityComponents::new(),
	};
	asset_storage.insert_with_name("teleportman", template);

	let template = EntityTemplate {
		name: Some("extrabfg"),
		type_id: None,
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("bfe2.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("bfgexp".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bfe2.sprite"), frame: 0, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("bfgexp2".to_owned()))),
					});
					states.insert("bfgexp2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bfe2.sprite"), frame: 1, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("bfgexp3".to_owned()))),
					});
					states.insert("bfgexp3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bfe2.sprite"), frame: 2, full_bright: true},
						next: Some((8 * FRAME_TIME, Some("bfgexp4".to_owned()))),
					});
					states.insert("bfgexp4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bfe2.sprite"), frame: 3, full_bright: true},
						next: Some((8 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bfgexp".to_owned()))),
				spawn_state: Some("bfgexp".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("extrabfg", template);

	let template = EntityTemplate {
		name: Some("misc0"),
		type_id: Some(EntityTypeId::Thing(2018)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("arm1.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("arm1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("arm1.sprite"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("arm1a".to_owned()))),
					});
					states.insert("arm1a".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("arm1.sprite"), frame: 1, full_bright: true},
						next: Some((7 * FRAME_TIME, Some("arm1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("arm1".to_owned()))),
				spawn_state: Some("arm1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc0", template);

	let template = EntityTemplate {
		name: Some("misc1"),
		type_id: Some(EntityTypeId::Thing(2019)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("arm2.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("arm2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("arm2.sprite"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("arm2a".to_owned()))),
					});
					states.insert("arm2a".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("arm2.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("arm2".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("arm2".to_owned()))),
				spawn_state: Some("arm2".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc1", template);

	let template = EntityTemplate {
		name: Some("misc2"),
		type_id: Some(EntityTypeId::Thing(2014)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("bon1.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(6);
					states.insert("bon1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bon1.sprite"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bon1a".to_owned()))),
					});
					states.insert("bon1a".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bon1.sprite"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bon1b".to_owned()))),
					});
					states.insert("bon1b".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bon1.sprite"), frame: 2, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bon1c".to_owned()))),
					});
					states.insert("bon1c".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bon1.sprite"), frame: 3, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bon1d".to_owned()))),
					});
					states.insert("bon1d".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bon1.sprite"), frame: 2, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bon1e".to_owned()))),
					});
					states.insert("bon1e".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bon1.sprite"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bon1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bon1".to_owned()))),
				spawn_state: Some("bon1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc2", template);

	let template = EntityTemplate {
		name: Some("misc3"),
		type_id: Some(EntityTypeId::Thing(2015)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("bon2.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(6);
					states.insert("bon2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bon2.sprite"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bon2a".to_owned()))),
					});
					states.insert("bon2a".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bon2.sprite"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bon2b".to_owned()))),
					});
					states.insert("bon2b".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bon2.sprite"), frame: 2, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bon2c".to_owned()))),
					});
					states.insert("bon2c".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bon2.sprite"), frame: 3, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bon2d".to_owned()))),
					});
					states.insert("bon2d".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bon2.sprite"), frame: 2, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bon2e".to_owned()))),
					});
					states.insert("bon2e".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bon2.sprite"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bon2".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bon2".to_owned()))),
				spawn_state: Some("bon2".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc3", template);

	let template = EntityTemplate {
		name: Some("misc4"),
		type_id: Some(EntityTypeId::Thing(5)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("bkey.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("bkey".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bkey.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("bkey2".to_owned()))),
					});
					states.insert("bkey2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bkey.sprite"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("bkey".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bkey".to_owned()))),
				spawn_state: Some("bkey".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc4", template);

	let template = EntityTemplate {
		name: Some("misc5"),
		type_id: Some(EntityTypeId::Thing(13)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("rkey.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("rkey".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("rkey.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("rkey2".to_owned()))),
					});
					states.insert("rkey2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("rkey.sprite"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("rkey".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("rkey".to_owned()))),
				spawn_state: Some("rkey".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc5", template);

	let template = EntityTemplate {
		name: Some("misc6"),
		type_id: Some(EntityTypeId::Thing(6)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("ykey.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("ykey".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ykey.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("ykey2".to_owned()))),
					});
					states.insert("ykey2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ykey.sprite"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("ykey".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("ykey".to_owned()))),
				spawn_state: Some("ykey".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc6", template);

	let template = EntityTemplate {
		name: Some("misc7"),
		type_id: Some(EntityTypeId::Thing(39)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("ysku.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("yskull".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ysku.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("yskull2".to_owned()))),
					});
					states.insert("yskull2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ysku.sprite"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("yskull".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("yskull".to_owned()))),
				spawn_state: Some("yskull".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc7", template);

	let template = EntityTemplate {
		name: Some("misc8"),
		type_id: Some(EntityTypeId::Thing(38)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("rsku.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("rskull".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("rsku.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("rskull2".to_owned()))),
					});
					states.insert("rskull2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("rsku.sprite"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("rskull".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("rskull".to_owned()))),
				spawn_state: Some("rskull".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc8", template);

	let template = EntityTemplate {
		name: Some("misc9"),
		type_id: Some(EntityTypeId::Thing(40)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("bsku.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("bskull".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bsku.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("bskull2".to_owned()))),
					});
					states.insert("bskull2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bsku.sprite"), frame: 1, full_bright: true},
						next: Some((10 * FRAME_TIME, Some("bskull".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bskull".to_owned()))),
				spawn_state: Some("bskull".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc9", template);

	let template = EntityTemplate {
		name: Some("misc10"),
		type_id: Some(EntityTypeId::Thing(2011)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("stim.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("stim".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("stim.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("stim".to_owned()))),
				spawn_state: Some("stim".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc10", template);

	let template = EntityTemplate {
		name: Some("misc11"),
		type_id: Some(EntityTypeId::Thing(2012)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("medi.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("medi".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("medi.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("medi".to_owned()))),
				spawn_state: Some("medi".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc11", template);

	let template = EntityTemplate {
		name: Some("misc12"),
		type_id: Some(EntityTypeId::Thing(2013)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("soul.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(6);
					states.insert("soul".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("soul.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("soul2".to_owned()))),
					});
					states.insert("soul2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("soul.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("soul3".to_owned()))),
					});
					states.insert("soul3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("soul.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("soul4".to_owned()))),
					});
					states.insert("soul4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("soul.sprite"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("soul5".to_owned()))),
					});
					states.insert("soul5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("soul.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("soul6".to_owned()))),
					});
					states.insert("soul6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("soul.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("soul".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("soul".to_owned()))),
				spawn_state: Some("soul".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc12", template);

	let template = EntityTemplate {
		name: Some("inv"),
		type_id: Some(EntityTypeId::Thing(2022)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("pinv.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("pinv".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pinv.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pinv2".to_owned()))),
					});
					states.insert("pinv2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pinv.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pinv3".to_owned()))),
					});
					states.insert("pinv3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pinv.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pinv4".to_owned()))),
					});
					states.insert("pinv4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pinv.sprite"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pinv".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("pinv".to_owned()))),
				spawn_state: Some("pinv".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("inv", template);

	let template = EntityTemplate {
		name: Some("misc13"),
		type_id: Some(EntityTypeId::Thing(2023)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("pstr.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("pstr".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pstr.sprite"), frame: 0, full_bright: true},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("pstr".to_owned()))),
				spawn_state: Some("pstr".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc13", template);

	let template = EntityTemplate {
		name: Some("ins"),
		type_id: Some(EntityTypeId::Thing(2024)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("pins.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("pins".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pins.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pins2".to_owned()))),
					});
					states.insert("pins2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pins.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pins3".to_owned()))),
					});
					states.insert("pins3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pins.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pins4".to_owned()))),
					});
					states.insert("pins4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pins.sprite"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pins".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("pins".to_owned()))),
				spawn_state: Some("pins".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("ins", template);

	let template = EntityTemplate {
		name: Some("misc14"),
		type_id: Some(EntityTypeId::Thing(2025)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("suit.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("suit".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("suit.sprite"), frame: 0, full_bright: true},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("suit".to_owned()))),
				spawn_state: Some("suit".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc14", template);

	let template = EntityTemplate {
		name: Some("misc15"),
		type_id: Some(EntityTypeId::Thing(2026)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("pmap.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(6);
					states.insert("pmap".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pmap.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pmap2".to_owned()))),
					});
					states.insert("pmap2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pmap.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pmap3".to_owned()))),
					});
					states.insert("pmap3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pmap.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pmap4".to_owned()))),
					});
					states.insert("pmap4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pmap.sprite"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pmap5".to_owned()))),
					});
					states.insert("pmap5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pmap.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pmap6".to_owned()))),
					});
					states.insert("pmap6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pmap.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pmap".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("pmap".to_owned()))),
				spawn_state: Some("pmap".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc15", template);

	let template = EntityTemplate {
		name: Some("misc16"),
		type_id: Some(EntityTypeId::Thing(2045)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("pvis.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("pvis".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pvis.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("pvis2".to_owned()))),
					});
					states.insert("pvis2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pvis.sprite"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("pvis".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("pvis".to_owned()))),
				spawn_state: Some("pvis".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc16", template);

	let template = EntityTemplate {
		name: Some("mega"),
		type_id: Some(EntityTypeId::Thing(83)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("mega.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("mega".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("mega.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("mega2".to_owned()))),
					});
					states.insert("mega2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("mega.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("mega3".to_owned()))),
					});
					states.insert("mega3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("mega.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("mega4".to_owned()))),
					});
					states.insert("mega4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("mega.sprite"), frame: 3, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("mega".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("mega".to_owned()))),
				spawn_state: Some("mega".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("mega", template);

	let template = EntityTemplate {
		name: Some("clip"),
		type_id: Some(EntityTypeId::Thing(2007)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("clip.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("clip".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("clip.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("clip".to_owned()))),
				spawn_state: Some("clip".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("clip", template);

	let template = EntityTemplate {
		name: Some("misc17"),
		type_id: Some(EntityTypeId::Thing(2048)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("ammo.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("ammo".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ammo.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("ammo".to_owned()))),
				spawn_state: Some("ammo".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc17", template);

	let template = EntityTemplate {
		name: Some("misc18"),
		type_id: Some(EntityTypeId::Thing(2010)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("rock.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("rock".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("rock.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("rock".to_owned()))),
				spawn_state: Some("rock".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc18", template);

	let template = EntityTemplate {
		name: Some("misc19"),
		type_id: Some(EntityTypeId::Thing(2046)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("brok.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("brok".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("brok.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("brok".to_owned()))),
				spawn_state: Some("brok".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc19", template);

	let template = EntityTemplate {
		name: Some("misc20"),
		type_id: Some(EntityTypeId::Thing(2047)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("cell.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("cell".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cell.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("cell".to_owned()))),
				spawn_state: Some("cell".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc20", template);

	let template = EntityTemplate {
		name: Some("misc21"),
		type_id: Some(EntityTypeId::Thing(17)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("celp.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("celp".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("celp.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("celp".to_owned()))),
				spawn_state: Some("celp".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc21", template);

	let template = EntityTemplate {
		name: Some("misc22"),
		type_id: Some(EntityTypeId::Thing(2008)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("shel.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("shel".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("shel.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("shel".to_owned()))),
				spawn_state: Some("shel".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc22", template);

	let template = EntityTemplate {
		name: Some("misc23"),
		type_id: Some(EntityTypeId::Thing(2049)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("sbox.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("sbox".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sbox.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("sbox".to_owned()))),
				spawn_state: Some("sbox".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc23", template);

	let template = EntityTemplate {
		name: Some("misc24"),
		type_id: Some(EntityTypeId::Thing(8)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("bpak.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("bpak".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bpak.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bpak".to_owned()))),
				spawn_state: Some("bpak".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc24", template);

	let template = EntityTemplate {
		name: Some("misc25"),
		type_id: Some(EntityTypeId::Thing(2006)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("bfug.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("bfug".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("bfug.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bfug".to_owned()))),
				spawn_state: Some("bfug".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc25", template);

	let template = EntityTemplate {
		name: Some("chaingun"),
		type_id: Some(EntityTypeId::Thing(2002)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("mgun.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("mgun".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("mgun.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("mgun".to_owned()))),
				spawn_state: Some("mgun".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("chaingun", template);

	let template = EntityTemplate {
		name: Some("misc26"),
		type_id: Some(EntityTypeId::Thing(2005)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("csaw.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("csaw".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("csaw.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("csaw".to_owned()))),
				spawn_state: Some("csaw".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc26", template);

	let template = EntityTemplate {
		name: Some("misc27"),
		type_id: Some(EntityTypeId::Thing(2003)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("laun.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("laun".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("laun.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("laun".to_owned()))),
				spawn_state: Some("laun".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc27", template);

	let template = EntityTemplate {
		name: Some("misc28"),
		type_id: Some(EntityTypeId::Thing(2004)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("plas.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("plas".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("plas.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("plas".to_owned()))),
				spawn_state: Some("plas".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc28", template);

	let template = EntityTemplate {
		name: Some("shotgun"),
		type_id: Some(EntityTypeId::Thing(2001)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("shot.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("shot".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("shot.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("shot".to_owned()))),
				spawn_state: Some("shot".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("shotgun", template);

	let template = EntityTemplate {
		name: Some("supershotgun"),
		type_id: Some(EntityTypeId::Thing(82)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("sgn2.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("shot2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sgn2.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("shot2".to_owned()))),
				spawn_state: Some("shot2".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("supershotgun", template);

	let template = EntityTemplate {
		name: Some("misc29"),
		type_id: Some(EntityTypeId::Thing(85)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("tlmp.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("techlamp".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tlmp.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("techlamp2".to_owned()))),
					});
					states.insert("techlamp2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tlmp.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("techlamp3".to_owned()))),
					});
					states.insert("techlamp3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tlmp.sprite"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("techlamp4".to_owned()))),
					});
					states.insert("techlamp4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tlmp.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("techlamp".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("techlamp".to_owned()))),
				spawn_state: Some("techlamp".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc29", template);

	let template = EntityTemplate {
		name: Some("misc30"),
		type_id: Some(EntityTypeId::Thing(86)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("tlp2.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("tech2lamp".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tlp2.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("tech2lamp2".to_owned()))),
					});
					states.insert("tech2lamp2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tlp2.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("tech2lamp3".to_owned()))),
					});
					states.insert("tech2lamp3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tlp2.sprite"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("tech2lamp4".to_owned()))),
					});
					states.insert("tech2lamp4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tlp2.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("tech2lamp".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("tech2lamp".to_owned()))),
				spawn_state: Some("tech2lamp".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc30", template);

	let template = EntityTemplate {
		name: Some("misc31"),
		type_id: Some(EntityTypeId::Thing(2028)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("colu.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("colu".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("colu.sprite"), frame: 0, full_bright: true},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("colu".to_owned()))),
				spawn_state: Some("colu".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc31", template);

	let template = EntityTemplate {
		name: Some("misc32"),
		type_id: Some(EntityTypeId::Thing(30)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("col1.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("tallgrncol".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("col1.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("tallgrncol".to_owned()))),
				spawn_state: Some("tallgrncol".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc32", template);

	let template = EntityTemplate {
		name: Some("misc33"),
		type_id: Some(EntityTypeId::Thing(31)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("col2.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("shrtgrncol".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("col2.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("shrtgrncol".to_owned()))),
				spawn_state: Some("shrtgrncol".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc33", template);

	let template = EntityTemplate {
		name: Some("misc34"),
		type_id: Some(EntityTypeId::Thing(32)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("col3.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("tallredcol".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("col3.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("tallredcol".to_owned()))),
				spawn_state: Some("tallredcol".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc34", template);

	let template = EntityTemplate {
		name: Some("misc35"),
		type_id: Some(EntityTypeId::Thing(33)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("col4.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("shrtredcol".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("col4.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("shrtredcol".to_owned()))),
				spawn_state: Some("shrtredcol".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc35", template);

	let template = EntityTemplate {
		name: Some("misc36"),
		type_id: Some(EntityTypeId::Thing(37)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("col6.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("skullcol".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("col6.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("skullcol".to_owned()))),
				spawn_state: Some("skullcol".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc36", template);

	let template = EntityTemplate {
		name: Some("misc37"),
		type_id: Some(EntityTypeId::Thing(36)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("col5.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("heartcol".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("col5.sprite"), frame: 0, full_bright: false},
						next: Some((14 * FRAME_TIME, Some("heartcol2".to_owned()))),
					});
					states.insert("heartcol2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("col5.sprite"), frame: 1, full_bright: false},
						next: Some((14 * FRAME_TIME, Some("heartcol".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("heartcol".to_owned()))),
				spawn_state: Some("heartcol".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc37", template);

	let template = EntityTemplate {
		name: Some("misc38"),
		type_id: Some(EntityTypeId::Thing(41)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("ceye.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("evileye".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ceye.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("evileye2".to_owned()))),
					});
					states.insert("evileye2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ceye.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("evileye3".to_owned()))),
					});
					states.insert("evileye3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ceye.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("evileye4".to_owned()))),
					});
					states.insert("evileye4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("ceye.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("evileye".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("evileye".to_owned()))),
				spawn_state: Some("evileye".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc38", template);

	let template = EntityTemplate {
		name: Some("misc39"),
		type_id: Some(EntityTypeId::Thing(42)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("fsku.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(3);
					states.insert("floatskull".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fsku.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("floatskull2".to_owned()))),
					});
					states.insert("floatskull2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fsku.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("floatskull3".to_owned()))),
					});
					states.insert("floatskull3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fsku.sprite"), frame: 2, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("floatskull".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("floatskull".to_owned()))),
				spawn_state: Some("floatskull".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc39", template);

	let template = EntityTemplate {
		name: Some("misc40"),
		type_id: Some(EntityTypeId::Thing(43)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("tre1.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("torchtree".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tre1.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("torchtree".to_owned()))),
				spawn_state: Some("torchtree".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc40", template);

	let template = EntityTemplate {
		name: Some("misc41"),
		type_id: Some(EntityTypeId::Thing(44)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("tblu.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("bluetorch".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tblu.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("bluetorch2".to_owned()))),
					});
					states.insert("bluetorch2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tblu.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("bluetorch3".to_owned()))),
					});
					states.insert("bluetorch3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tblu.sprite"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("bluetorch4".to_owned()))),
					});
					states.insert("bluetorch4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tblu.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("bluetorch".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bluetorch".to_owned()))),
				spawn_state: Some("bluetorch".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc41", template);

	let template = EntityTemplate {
		name: Some("misc42"),
		type_id: Some(EntityTypeId::Thing(45)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("tgrn.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("greentorch".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tgrn.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("greentorch2".to_owned()))),
					});
					states.insert("greentorch2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tgrn.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("greentorch3".to_owned()))),
					});
					states.insert("greentorch3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tgrn.sprite"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("greentorch4".to_owned()))),
					});
					states.insert("greentorch4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tgrn.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("greentorch".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("greentorch".to_owned()))),
				spawn_state: Some("greentorch".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc42", template);

	let template = EntityTemplate {
		name: Some("misc43"),
		type_id: Some(EntityTypeId::Thing(46)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("tred.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("redtorch".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tred.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("redtorch2".to_owned()))),
					});
					states.insert("redtorch2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tred.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("redtorch3".to_owned()))),
					});
					states.insert("redtorch3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tred.sprite"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("redtorch4".to_owned()))),
					});
					states.insert("redtorch4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tred.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("redtorch".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("redtorch".to_owned()))),
				spawn_state: Some("redtorch".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc43", template);

	let template = EntityTemplate {
		name: Some("misc44"),
		type_id: Some(EntityTypeId::Thing(55)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("smbt.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("btorchshrt".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("smbt.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("btorchshrt2".to_owned()))),
					});
					states.insert("btorchshrt2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("smbt.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("btorchshrt3".to_owned()))),
					});
					states.insert("btorchshrt3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("smbt.sprite"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("btorchshrt4".to_owned()))),
					});
					states.insert("btorchshrt4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("smbt.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("btorchshrt".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("btorchshrt".to_owned()))),
				spawn_state: Some("btorchshrt".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc44", template);

	let template = EntityTemplate {
		name: Some("misc45"),
		type_id: Some(EntityTypeId::Thing(56)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("smgt.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("gtorchshrt".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("smgt.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("gtorchshrt2".to_owned()))),
					});
					states.insert("gtorchshrt2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("smgt.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("gtorchshrt3".to_owned()))),
					});
					states.insert("gtorchshrt3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("smgt.sprite"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("gtorchshrt4".to_owned()))),
					});
					states.insert("gtorchshrt4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("smgt.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("gtorchshrt".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("gtorchshrt".to_owned()))),
				spawn_state: Some("gtorchshrt".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc45", template);

	let template = EntityTemplate {
		name: Some("misc46"),
		type_id: Some(EntityTypeId::Thing(57)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("smrt.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("rtorchshrt".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("smrt.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("rtorchshrt2".to_owned()))),
					});
					states.insert("rtorchshrt2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("smrt.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("rtorchshrt3".to_owned()))),
					});
					states.insert("rtorchshrt3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("smrt.sprite"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("rtorchshrt4".to_owned()))),
					});
					states.insert("rtorchshrt4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("smrt.sprite"), frame: 3, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("rtorchshrt".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("rtorchshrt".to_owned()))),
				spawn_state: Some("rtorchshrt".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc46", template);

	let template = EntityTemplate {
		name: Some("misc47"),
		type_id: Some(EntityTypeId::Thing(47)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("smit.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("stalagtite".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("smit.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("stalagtite".to_owned()))),
				spawn_state: Some("stalagtite".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc47", template);

	let template = EntityTemplate {
		name: Some("misc48"),
		type_id: Some(EntityTypeId::Thing(48)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("elec.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("techpillar".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("elec.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("techpillar".to_owned()))),
				spawn_state: Some("techpillar".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc48", template);

	let template = EntityTemplate {
		name: Some("misc49"),
		type_id: Some(EntityTypeId::Thing(34)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("cand.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("candlestik".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cand.sprite"), frame: 0, full_bright: true},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("candlestik".to_owned()))),
				spawn_state: Some("candlestik".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc49", template);

	let template = EntityTemplate {
		name: Some("misc50"),
		type_id: Some(EntityTypeId::Thing(35)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("cbra.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("candelabra".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("cbra.sprite"), frame: 0, full_bright: true},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("candelabra".to_owned()))),
				spawn_state: Some("candelabra".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc50", template);

	let template = EntityTemplate {
		name: Some("misc51"),
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
				sprite: asset_storage.load("gor1.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("bloodytwitch".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("bloodytwitch2".to_owned()))),
					});
					states.insert("bloodytwitch2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 1, full_bright: false},
						next: Some((15 * FRAME_TIME, Some("bloodytwitch3".to_owned()))),
					});
					states.insert("bloodytwitch3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 2, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bloodytwitch4".to_owned()))),
					});
					states.insert("bloodytwitch4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bloodytwitch".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bloodytwitch".to_owned()))),
				spawn_state: Some("bloodytwitch".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc51", template);

	let template = EntityTemplate {
		name: Some("misc52"),
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
				sprite: asset_storage.load("gor2.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("meat2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor2.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("meat2".to_owned()))),
				spawn_state: Some("meat2".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc52", template);

	let template = EntityTemplate {
		name: Some("misc53"),
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
				sprite: asset_storage.load("gor3.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("meat3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor3.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("meat3".to_owned()))),
				spawn_state: Some("meat3".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc53", template);

	let template = EntityTemplate {
		name: Some("misc54"),
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
				sprite: asset_storage.load("gor4.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("meat4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor4.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("meat4".to_owned()))),
				spawn_state: Some("meat4".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc54", template);

	let template = EntityTemplate {
		name: Some("misc55"),
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
				sprite: asset_storage.load("gor5.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("meat5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor5.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("meat5".to_owned()))),
				spawn_state: Some("meat5".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc55", template);

	let template = EntityTemplate {
		name: Some("misc56"),
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
				sprite: asset_storage.load("gor2.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("meat2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor2.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("meat2".to_owned()))),
				spawn_state: Some("meat2".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc56", template);

	let template = EntityTemplate {
		name: Some("misc57"),
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
				sprite: asset_storage.load("gor4.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("meat4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor4.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("meat4".to_owned()))),
				spawn_state: Some("meat4".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc57", template);

	let template = EntityTemplate {
		name: Some("misc58"),
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
				sprite: asset_storage.load("gor3.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("meat3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor3.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("meat3".to_owned()))),
				spawn_state: Some("meat3".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc58", template);

	let template = EntityTemplate {
		name: Some("misc59"),
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
				sprite: asset_storage.load("gor5.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("meat5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor5.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("meat5".to_owned()))),
				spawn_state: Some("meat5".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc59", template);

	let template = EntityTemplate {
		name: Some("misc60"),
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
				sprite: asset_storage.load("gor1.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(4);
					states.insert("bloodytwitch".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 0, full_bright: false},
						next: Some((10 * FRAME_TIME, Some("bloodytwitch2".to_owned()))),
					});
					states.insert("bloodytwitch2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 1, full_bright: false},
						next: Some((15 * FRAME_TIME, Some("bloodytwitch3".to_owned()))),
					});
					states.insert("bloodytwitch3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 2, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("bloodytwitch4".to_owned()))),
					});
					states.insert("bloodytwitch4".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 1, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("bloodytwitch".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bloodytwitch".to_owned()))),
				spawn_state: Some("bloodytwitch".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc60", template);

	let template = EntityTemplate {
		name: Some("misc61"),
		type_id: Some(EntityTypeId::Thing(22)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("head.sprite"),
				frame: 11,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("head_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 11, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("head_die6".to_owned()))),
				spawn_state: Some("head_die6".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc61", template);

	let template = EntityTemplate {
		name: Some("misc62"),
		type_id: Some(EntityTypeId::Thing(15)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("play.sprite"),
				frame: 13,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("play_die7".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 13, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("play_die7".to_owned()))),
				spawn_state: Some("play_die7".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc62", template);

	let template = EntityTemplate {
		name: Some("misc63"),
		type_id: Some(EntityTypeId::Thing(18)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("poss.sprite"),
				frame: 11,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("poss_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 11, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("poss_die5".to_owned()))),
				spawn_state: Some("poss_die5".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc63", template);

	let template = EntityTemplate {
		name: Some("misc64"),
		type_id: Some(EntityTypeId::Thing(21)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("sarg.sprite"),
				frame: 13,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("sarg_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 13, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("sarg_die6".to_owned()))),
				spawn_state: Some("sarg_die6".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc64", template);

	let template = EntityTemplate {
		name: Some("misc65"),
		type_id: Some(EntityTypeId::Thing(23)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("skul.sprite"),
				frame: 10,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("skull_die6".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 10, full_bright: false},
						next: Some((6 * FRAME_TIME, None)),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("skull_die6".to_owned()))),
				spawn_state: Some("skull_die6".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc65", template);

	let template = EntityTemplate {
		name: Some("misc66"),
		type_id: Some(EntityTypeId::Thing(20)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("troo.sprite"),
				frame: 12,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("troo_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 12, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("troo_die5".to_owned()))),
				spawn_state: Some("troo_die5".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc66", template);

	let template = EntityTemplate {
		name: Some("misc67"),
		type_id: Some(EntityTypeId::Thing(19)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("spos.sprite"),
				frame: 11,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("spos_die5".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 11, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("spos_die5".to_owned()))),
				spawn_state: Some("spos_die5".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc67", template);

	let template = EntityTemplate {
		name: Some("misc68"),
		type_id: Some(EntityTypeId::Thing(10)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("play.sprite"),
				frame: 22,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("play_xdie9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 22, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("play_xdie9".to_owned()))),
				spawn_state: Some("play_xdie9".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc68", template);

	let template = EntityTemplate {
		name: Some("misc69"),
		type_id: Some(EntityTypeId::Thing(12)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("play.sprite"),
				frame: 22,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("play_xdie9".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 22, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("play_xdie9".to_owned()))),
				spawn_state: Some("play_xdie9".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc69", template);

	let template = EntityTemplate {
		name: Some("misc70"),
		type_id: Some(EntityTypeId::Thing(28)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("pol2.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("headsonstick".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pol2.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("headsonstick".to_owned()))),
				spawn_state: Some("headsonstick".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc70", template);

	let template = EntityTemplate {
		name: Some("misc71"),
		type_id: Some(EntityTypeId::Thing(24)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 20.0,
				solid_mask: SolidMask::empty(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("pol5.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("gibs".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pol5.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("gibs".to_owned()))),
				spawn_state: Some("gibs".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc71", template);

	let template = EntityTemplate {
		name: Some("misc72"),
		type_id: Some(EntityTypeId::Thing(27)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("pol4.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("headonastick".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pol4.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("headonastick".to_owned()))),
				spawn_state: Some("headonastick".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc72", template);

	let template = EntityTemplate {
		name: Some("misc73"),
		type_id: Some(EntityTypeId::Thing(29)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("pol3.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("headcandles".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pol3.sprite"), frame: 0, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("headcandles2".to_owned()))),
					});
					states.insert("headcandles2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pol3.sprite"), frame: 1, full_bright: true},
						next: Some((6 * FRAME_TIME, Some("headcandles".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("headcandles".to_owned()))),
				spawn_state: Some("headcandles".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc73", template);

	let template = EntityTemplate {
		name: Some("misc74"),
		type_id: Some(EntityTypeId::Thing(25)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("pol1.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("deadstick".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pol1.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("deadstick".to_owned()))),
				spawn_state: Some("deadstick".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc74", template);

	let template = EntityTemplate {
		name: Some("misc75"),
		type_id: Some(EntityTypeId::Thing(26)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("pol6.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(2);
					states.insert("livestick".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pol6.sprite"), frame: 0, full_bright: false},
						next: Some((6 * FRAME_TIME, Some("livestick2".to_owned()))),
					});
					states.insert("livestick2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pol6.sprite"), frame: 1, full_bright: false},
						next: Some((8 * FRAME_TIME, Some("livestick".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("livestick".to_owned()))),
				spawn_state: Some("livestick".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc75", template);

	let template = EntityTemplate {
		name: Some("misc76"),
		type_id: Some(EntityTypeId::Thing(54)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 32.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("tre2.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("bigtree".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("tre2.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bigtree".to_owned()))),
				spawn_state: Some("bigtree".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc76", template);

	let template = EntityTemplate {
		name: Some("misc77"),
		type_id: Some(EntityTypeId::Thing(70)),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 16.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("fcan.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(3);
					states.insert("bbar1".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fcan.sprite"), frame: 0, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("bbar2".to_owned()))),
					});
					states.insert("bbar2".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fcan.sprite"), frame: 1, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("bbar3".to_owned()))),
					});
					states.insert("bbar3".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("fcan.sprite"), frame: 2, full_bright: true},
						next: Some((4 * FRAME_TIME, Some("bbar1".to_owned()))),
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("bbar1".to_owned()))),
				spawn_state: Some("bbar1".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc77", template);

	let template = EntityTemplate {
		name: Some("misc78"),
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
				sprite: asset_storage.load("hdb1.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("hangnoguts".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("hdb1.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("hangnoguts".to_owned()))),
				spawn_state: Some("hangnoguts".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc78", template);

	let template = EntityTemplate {
		name: Some("misc79"),
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
				sprite: asset_storage.load("hdb2.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("hangbnobrain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("hdb2.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("hangbnobrain".to_owned()))),
				spawn_state: Some("hangbnobrain".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc79", template);

	let template = EntityTemplate {
		name: Some("misc80"),
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
				sprite: asset_storage.load("hdb3.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("hangtlookdn".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("hdb3.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("hangtlookdn".to_owned()))),
				spawn_state: Some("hangtlookdn".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc80", template);

	let template = EntityTemplate {
		name: Some("misc81"),
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
				sprite: asset_storage.load("hdb4.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("hangtskull".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("hdb4.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("hangtskull".to_owned()))),
				spawn_state: Some("hangtskull".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc81", template);

	let template = EntityTemplate {
		name: Some("misc82"),
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
				sprite: asset_storage.load("hdb5.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("hangtlookup".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("hdb5.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("hangtlookup".to_owned()))),
				spawn_state: Some("hangtlookup".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc82", template);

	let template = EntityTemplate {
		name: Some("misc83"),
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
				sprite: asset_storage.load("hdb6.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("hangtnobrain".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("hdb6.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("hangtnobrain".to_owned()))),
				spawn_state: Some("hangtnobrain".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc83", template);

	let template = EntityTemplate {
		name: Some("misc84"),
		type_id: Some(EntityTypeId::Thing(79)),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("pob1.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("colongibs".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pob1.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("colongibs".to_owned()))),
				spawn_state: Some("colongibs".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc84", template);

	let template = EntityTemplate {
		name: Some("misc85"),
		type_id: Some(EntityTypeId::Thing(80)),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("pob2.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("smallpool".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("pob2.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("smallpool".to_owned()))),
				spawn_state: Some("smallpool".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc85", template);

	let template = EntityTemplate {
		name: Some("misc86"),
		type_id: Some(EntityTypeId::Thing(81)),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("brs1.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				states: Arc::new({
					let mut states = HashMap::with_capacity(1);
					states.insert("brainstem".to_owned(), StateDef {
						sprite: SpriteRender {sprite: asset_storage.load("brs1.sprite"), frame: 0, full_bright: false},
						next: None,
					});
					states
				}),
				next: Some((Timer::new(Duration::default()), Some("brainstem".to_owned()))),
				spawn_state: Some("brainstem".to_owned()),
				see_state: None,
				pain_state: None,
				melee_state: None,
				missile_state: None,
				death_state: None,
				xdeath_state: None,
				raise_state: None,
			}),
	};
	asset_storage.insert_with_name("misc86", template);
}
