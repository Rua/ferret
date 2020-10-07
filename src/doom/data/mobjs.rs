#![allow(unused_variables)]
use crate::{
	common::{assets::AssetStorage, component::EntityComponents, time::Timer},
	doom::{
		camera::Camera,
		client::User,
		components::{SpawnOnCeiling, SpawnPoint, Velocity},
		data::FRAME_TIME,
		entitytemplate::{EntityTemplate, EntityTypeId},
		physics::{BoxCollider, SolidMask},
		render::{psprite::PlayerSpriteRender, sprite::SpriteRender},
		state::{State, StateDef, StateName},
	},
};
use legion::{systems::ResourceSet, Resources, Write};
use nalgebra::{Vector2, Vector3};
use std::{collections::HashMap, default::Default, time::Duration};

#[rustfmt::skip]
pub fn load(resources: &mut Resources) {
	let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Thing(1)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 1 }),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Thing(2)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 2 }),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Thing(3)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 3 }),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Thing(4)),
		components: EntityComponents::new()
			.with_component(SpawnPoint { player_num: 4 }),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Thing(11)),
		components: EntityComponents::new(),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		name: Some("player"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(24);
			states.insert(StateName::from("play").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("play_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 0, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("play_run2").unwrap()))),
			});
			states.insert(StateName::from("play_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 1, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("play_run3").unwrap()))),
			});
			states.insert(StateName::from("play_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 2, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("play_run4").unwrap()))),
			});
			states.insert(StateName::from("play_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 3, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("play_run1").unwrap()))),
			});
			states.insert(StateName::from("play_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 4, full_bright: false},
				next: Some((12 * FRAME_TIME, Some(StateName::from("play").unwrap()))),
			});
			states.insert(StateName::from("play_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 6, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("play_pain2").unwrap()))),
			});
			states.insert(StateName::from("play_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 6, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("play").unwrap()))),
			});
			states.insert(StateName::from("play_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 7, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("play_die2").unwrap()))),
			});
			states.insert(StateName::from("play_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 8, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("play_die3").unwrap()))),
			});
			states.insert(StateName::from("play_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 9, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("play_die4").unwrap()))),
			});
			states.insert(StateName::from("play_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 10, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("play_die5").unwrap()))),
			});
			states.insert(StateName::from("play_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 11, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("play_die6").unwrap()))),
			});
			states.insert(StateName::from("play_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 12, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("play_die7").unwrap()))),
			});
			states.insert(StateName::from("play_die7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 13, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("play_xdie1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 14, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("play_xdie2").unwrap()))),
			});
			states.insert(StateName::from("play_xdie2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 15, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("play_xdie3").unwrap()))),
			});
			states.insert(StateName::from("play_xdie3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 16, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("play_xdie4").unwrap()))),
			});
			states.insert(StateName::from("play_xdie4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 17, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("play_xdie5").unwrap()))),
			});
			states.insert(StateName::from("play_xdie5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 18, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("play_xdie6").unwrap()))),
			});
			states.insert(StateName::from("play_xdie6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 19, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("play_xdie7").unwrap()))),
			});
			states.insert(StateName::from("play_xdie7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 20, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("play_xdie8").unwrap()))),
			});
			states.insert(StateName::from("play_xdie8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 21, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("play_xdie9").unwrap()))),
			});
			states.insert(StateName::from("play_xdie9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 22, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("play").unwrap()),
		see_state: Some(StateName::from("play_run1").unwrap()),
		pain_state: Some(StateName::from("play_pain").unwrap()),
		missile_state: Some(StateName::from("play_atk1").unwrap()),
		death_state: Some(StateName::from("play_die1").unwrap()),
		xdeath_state: Some(StateName::from("play_xdie1").unwrap()),
		components: EntityComponents::new()
			.with_component(BoxCollider {
				height: 56.0,
				radius: 16.0,
				solid_mask: SolidMask::all(),
			})
			.with_component(Camera {
				base: Vector3::new(0.0, 0.0, 41.0),
				offset: Vector3::zeros(),
				bob_max: 16.0,
				view_bob_period: 20 * FRAME_TIME,
				weapon_bob_period: 64 * FRAME_TIME,
				deviation_position: 0.0,
				deviation_velocity: 0.0,
				impact_sound: asset_storage.load("dsoof.sound"),
			})
			.with_component(PlayerSpriteRender {
				position: Vector2::new(0.0, 0.0),
				weapon: SpriteRender {
					sprite: asset_storage.load("pisg.sprite"),
					frame: 0,
					full_bright: false,
				},
				flash: None,
			})
			.with_component(SpriteRender {
				sprite: asset_storage.load("play.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("play").unwrap()))),
			})
			.with_component(User {
				error_sound: asset_storage.load("dsnoway.sound"),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("player", template);

	let template = EntityTemplate {
		name: Some("possessed"),
		type_id: Some(EntityTypeId::Thing(3004)),
		states: {
			let mut states = HashMap::with_capacity(33);
			states.insert(StateName::from("poss_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("poss_stnd2").unwrap()))),
			});
			states.insert(StateName::from("poss_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("poss_stnd").unwrap()))),
			});
			states.insert(StateName::from("poss_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 0, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("poss_run2").unwrap()))),
			});
			states.insert(StateName::from("poss_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 0, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("poss_run3").unwrap()))),
			});
			states.insert(StateName::from("poss_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 1, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("poss_run4").unwrap()))),
			});
			states.insert(StateName::from("poss_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 1, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("poss_run5").unwrap()))),
			});
			states.insert(StateName::from("poss_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 2, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("poss_run6").unwrap()))),
			});
			states.insert(StateName::from("poss_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 2, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("poss_run7").unwrap()))),
			});
			states.insert(StateName::from("poss_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 3, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("poss_run8").unwrap()))),
			});
			states.insert(StateName::from("poss_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 3, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("poss_run1").unwrap()))),
			});
			states.insert(StateName::from("poss_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 4, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("poss_atk2").unwrap()))),
			});
			states.insert(StateName::from("poss_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 5, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("poss_atk3").unwrap()))),
			});
			states.insert(StateName::from("poss_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 4, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("poss_run1").unwrap()))),
			});
			states.insert(StateName::from("poss_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 6, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("poss_pain2").unwrap()))),
			});
			states.insert(StateName::from("poss_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 6, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("poss_run1").unwrap()))),
			});
			states.insert(StateName::from("poss_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 7, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_die2").unwrap()))),
			});
			states.insert(StateName::from("poss_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 8, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_die3").unwrap()))),
			});
			states.insert(StateName::from("poss_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 9, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_die4").unwrap()))),
			});
			states.insert(StateName::from("poss_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 10, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_die5").unwrap()))),
			});
			states.insert(StateName::from("poss_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 11, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("poss_xdie1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 12, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_xdie2").unwrap()))),
			});
			states.insert(StateName::from("poss_xdie2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 13, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_xdie3").unwrap()))),
			});
			states.insert(StateName::from("poss_xdie3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 14, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_xdie4").unwrap()))),
			});
			states.insert(StateName::from("poss_xdie4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 15, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_xdie5").unwrap()))),
			});
			states.insert(StateName::from("poss_xdie5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 16, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_xdie6").unwrap()))),
			});
			states.insert(StateName::from("poss_xdie6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 17, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_xdie7").unwrap()))),
			});
			states.insert(StateName::from("poss_xdie7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 18, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_xdie8").unwrap()))),
			});
			states.insert(StateName::from("poss_xdie8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 19, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_xdie9").unwrap()))),
			});
			states.insert(StateName::from("poss_xdie9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 20, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("poss_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 10, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_raise2").unwrap()))),
			});
			states.insert(StateName::from("poss_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 9, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_raise3").unwrap()))),
			});
			states.insert(StateName::from("poss_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 8, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_raise4").unwrap()))),
			});
			states.insert(StateName::from("poss_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 7, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("poss_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("poss_stnd").unwrap()),
		see_state: Some(StateName::from("poss_run1").unwrap()),
		pain_state: Some(StateName::from("poss_pain").unwrap()),
		missile_state: Some(StateName::from("poss_atk1").unwrap()),
		death_state: Some(StateName::from("poss_die1").unwrap()),
		xdeath_state: Some(StateName::from("poss_xdie1").unwrap()),
		raise_state: Some(StateName::from("poss_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("poss_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("possessed", template);

	let template = EntityTemplate {
		name: Some("shotguy"),
		type_id: Some(EntityTypeId::Thing(9)),
		states: {
			let mut states = HashMap::with_capacity(34);
			states.insert(StateName::from("spos_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spos_stnd2").unwrap()))),
			});
			states.insert(StateName::from("spos_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spos_stnd").unwrap()))),
			});
			states.insert(StateName::from("spos_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spos_run2").unwrap()))),
			});
			states.insert(StateName::from("spos_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spos_run3").unwrap()))),
			});
			states.insert(StateName::from("spos_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spos_run4").unwrap()))),
			});
			states.insert(StateName::from("spos_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spos_run5").unwrap()))),
			});
			states.insert(StateName::from("spos_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spos_run6").unwrap()))),
			});
			states.insert(StateName::from("spos_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spos_run7").unwrap()))),
			});
			states.insert(StateName::from("spos_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spos_run8").unwrap()))),
			});
			states.insert(StateName::from("spos_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spos_run1").unwrap()))),
			});
			states.insert(StateName::from("spos_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 4, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spos_atk2").unwrap()))),
			});
			states.insert(StateName::from("spos_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 5, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spos_atk3").unwrap()))),
			});
			states.insert(StateName::from("spos_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 4, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spos_run1").unwrap()))),
			});
			states.insert(StateName::from("spos_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 6, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spos_pain2").unwrap()))),
			});
			states.insert(StateName::from("spos_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 6, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spos_run1").unwrap()))),
			});
			states.insert(StateName::from("spos_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 7, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_die2").unwrap()))),
			});
			states.insert(StateName::from("spos_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 8, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_die3").unwrap()))),
			});
			states.insert(StateName::from("spos_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 9, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_die4").unwrap()))),
			});
			states.insert(StateName::from("spos_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 10, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_die5").unwrap()))),
			});
			states.insert(StateName::from("spos_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 11, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("spos_xdie1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 12, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_xdie2").unwrap()))),
			});
			states.insert(StateName::from("spos_xdie2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 13, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_xdie3").unwrap()))),
			});
			states.insert(StateName::from("spos_xdie3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 14, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_xdie4").unwrap()))),
			});
			states.insert(StateName::from("spos_xdie4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 15, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_xdie5").unwrap()))),
			});
			states.insert(StateName::from("spos_xdie5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 16, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_xdie6").unwrap()))),
			});
			states.insert(StateName::from("spos_xdie6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 17, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_xdie7").unwrap()))),
			});
			states.insert(StateName::from("spos_xdie7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 18, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_xdie8").unwrap()))),
			});
			states.insert(StateName::from("spos_xdie8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 19, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_xdie9").unwrap()))),
			});
			states.insert(StateName::from("spos_xdie9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 20, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("spos_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 11, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_raise2").unwrap()))),
			});
			states.insert(StateName::from("spos_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 10, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_raise3").unwrap()))),
			});
			states.insert(StateName::from("spos_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 9, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_raise4").unwrap()))),
			});
			states.insert(StateName::from("spos_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 8, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_raise5").unwrap()))),
			});
			states.insert(StateName::from("spos_raise5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 7, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("spos_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("spos_stnd").unwrap()),
		see_state: Some(StateName::from("spos_run1").unwrap()),
		pain_state: Some(StateName::from("spos_pain").unwrap()),
		missile_state: Some(StateName::from("spos_atk1").unwrap()),
		death_state: Some(StateName::from("spos_die1").unwrap()),
		xdeath_state: Some(StateName::from("spos_xdie1").unwrap()),
		raise_state: Some(StateName::from("spos_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("spos_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("shotguy", template);

	let template = EntityTemplate {
		name: Some("vile"),
		type_id: Some(EntityTypeId::Thing(64)),
		states: {
			let mut states = HashMap::with_capacity(37);
			states.insert(StateName::from("vile_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("vile_stnd2").unwrap()))),
			});
			states.insert(StateName::from("vile_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("vile_stnd").unwrap()))),
			});
			states.insert(StateName::from("vile_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 0, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("vile_run2").unwrap()))),
			});
			states.insert(StateName::from("vile_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 0, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("vile_run3").unwrap()))),
			});
			states.insert(StateName::from("vile_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 1, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("vile_run4").unwrap()))),
			});
			states.insert(StateName::from("vile_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 1, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("vile_run5").unwrap()))),
			});
			states.insert(StateName::from("vile_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 2, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("vile_run6").unwrap()))),
			});
			states.insert(StateName::from("vile_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 2, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("vile_run7").unwrap()))),
			});
			states.insert(StateName::from("vile_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 3, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("vile_run8").unwrap()))),
			});
			states.insert(StateName::from("vile_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 3, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("vile_run9").unwrap()))),
			});
			states.insert(StateName::from("vile_run9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 4, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("vile_run10").unwrap()))),
			});
			states.insert(StateName::from("vile_run10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 4, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("vile_run11").unwrap()))),
			});
			states.insert(StateName::from("vile_run11").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 5, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("vile_run12").unwrap()))),
			});
			states.insert(StateName::from("vile_run12").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 5, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("vile_run1").unwrap()))),
			});
			states.insert(StateName::from("vile_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 6, full_bright: true},
				next: Some((0 * FRAME_TIME, Some(StateName::from("vile_atk2").unwrap()))),
			});
			states.insert(StateName::from("vile_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 6, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("vile_atk3").unwrap()))),
			});
			states.insert(StateName::from("vile_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 7, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("vile_atk4").unwrap()))),
			});
			states.insert(StateName::from("vile_atk4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 8, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("vile_atk5").unwrap()))),
			});
			states.insert(StateName::from("vile_atk5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 9, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("vile_atk6").unwrap()))),
			});
			states.insert(StateName::from("vile_atk6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 10, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("vile_atk7").unwrap()))),
			});
			states.insert(StateName::from("vile_atk7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 11, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("vile_atk8").unwrap()))),
			});
			states.insert(StateName::from("vile_atk8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 12, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("vile_atk9").unwrap()))),
			});
			states.insert(StateName::from("vile_atk9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 13, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("vile_atk10").unwrap()))),
			});
			states.insert(StateName::from("vile_atk10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 14, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("vile_atk11").unwrap()))),
			});
			states.insert(StateName::from("vile_atk11").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 15, full_bright: true},
				next: Some((20 * FRAME_TIME, Some(StateName::from("vile_run1").unwrap()))),
			});
			states.insert(StateName::from("vile_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 16, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("vile_pain2").unwrap()))),
			});
			states.insert(StateName::from("vile_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 16, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("vile_run1").unwrap()))),
			});
			states.insert(StateName::from("vile_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 16, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("vile_die2").unwrap()))),
			});
			states.insert(StateName::from("vile_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 17, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("vile_die3").unwrap()))),
			});
			states.insert(StateName::from("vile_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 18, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("vile_die4").unwrap()))),
			});
			states.insert(StateName::from("vile_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 19, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("vile_die5").unwrap()))),
			});
			states.insert(StateName::from("vile_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 20, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("vile_die6").unwrap()))),
			});
			states.insert(StateName::from("vile_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 21, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("vile_die7").unwrap()))),
			});
			states.insert(StateName::from("vile_die7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 22, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("vile_die8").unwrap()))),
			});
			states.insert(StateName::from("vile_die8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 23, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("vile_die9").unwrap()))),
			});
			states.insert(StateName::from("vile_die9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 24, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("vile_die10").unwrap()))),
			});
			states.insert(StateName::from("vile_die10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("vile.sprite"), frame: 25, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("vile_stnd").unwrap()),
		see_state: Some(StateName::from("vile_run1").unwrap()),
		pain_state: Some(StateName::from("vile_pain").unwrap()),
		missile_state: Some(StateName::from("vile_atk1").unwrap()),
		death_state: Some(StateName::from("vile_die1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("vile_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("vile", template);

	let template = EntityTemplate {
		name: Some("fire"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(30);
			states.insert(StateName::from("fire1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 0, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire2").unwrap()))),
			});
			states.insert(StateName::from("fire2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 1, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire3").unwrap()))),
			});
			states.insert(StateName::from("fire3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 0, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire4").unwrap()))),
			});
			states.insert(StateName::from("fire4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 1, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire5").unwrap()))),
			});
			states.insert(StateName::from("fire5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 2, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire6").unwrap()))),
			});
			states.insert(StateName::from("fire6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 1, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire7").unwrap()))),
			});
			states.insert(StateName::from("fire7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 2, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire8").unwrap()))),
			});
			states.insert(StateName::from("fire8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 1, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire9").unwrap()))),
			});
			states.insert(StateName::from("fire9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 2, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire10").unwrap()))),
			});
			states.insert(StateName::from("fire10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 3, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire11").unwrap()))),
			});
			states.insert(StateName::from("fire11").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 2, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire12").unwrap()))),
			});
			states.insert(StateName::from("fire12").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 3, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire13").unwrap()))),
			});
			states.insert(StateName::from("fire13").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 2, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire14").unwrap()))),
			});
			states.insert(StateName::from("fire14").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 3, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire15").unwrap()))),
			});
			states.insert(StateName::from("fire15").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 4, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire16").unwrap()))),
			});
			states.insert(StateName::from("fire16").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 3, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire17").unwrap()))),
			});
			states.insert(StateName::from("fire17").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 4, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire18").unwrap()))),
			});
			states.insert(StateName::from("fire18").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 3, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire19").unwrap()))),
			});
			states.insert(StateName::from("fire19").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 4, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire20").unwrap()))),
			});
			states.insert(StateName::from("fire20").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 5, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire21").unwrap()))),
			});
			states.insert(StateName::from("fire21").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 4, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire22").unwrap()))),
			});
			states.insert(StateName::from("fire22").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 5, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire23").unwrap()))),
			});
			states.insert(StateName::from("fire23").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 4, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire24").unwrap()))),
			});
			states.insert(StateName::from("fire24").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 5, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire25").unwrap()))),
			});
			states.insert(StateName::from("fire25").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 6, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire26").unwrap()))),
			});
			states.insert(StateName::from("fire26").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 7, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire27").unwrap()))),
			});
			states.insert(StateName::from("fire27").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 6, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire28").unwrap()))),
			});
			states.insert(StateName::from("fire28").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 7, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire29").unwrap()))),
			});
			states.insert(StateName::from("fire29").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 6, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("fire30").unwrap()))),
			});
			states.insert(StateName::from("fire30").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 7, full_bright: true},
				next: Some((2 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("fire1").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("fire.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("fire1").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("fire", template);

	let template = EntityTemplate {
		name: Some("undead"),
		type_id: Some(EntityTypeId::Thing(66)),
		states: {
			let mut states = HashMap::with_capacity(36);
			states.insert(StateName::from("skel_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("skel_stnd2").unwrap()))),
			});
			states.insert(StateName::from("skel_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("skel_stnd").unwrap()))),
			});
			states.insert(StateName::from("skel_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 0, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("skel_run2").unwrap()))),
			});
			states.insert(StateName::from("skel_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 0, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("skel_run3").unwrap()))),
			});
			states.insert(StateName::from("skel_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 1, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("skel_run4").unwrap()))),
			});
			states.insert(StateName::from("skel_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 1, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("skel_run5").unwrap()))),
			});
			states.insert(StateName::from("skel_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 2, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("skel_run6").unwrap()))),
			});
			states.insert(StateName::from("skel_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 2, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("skel_run7").unwrap()))),
			});
			states.insert(StateName::from("skel_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 3, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("skel_run8").unwrap()))),
			});
			states.insert(StateName::from("skel_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 3, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("skel_run9").unwrap()))),
			});
			states.insert(StateName::from("skel_run9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 4, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("skel_run10").unwrap()))),
			});
			states.insert(StateName::from("skel_run10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 4, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("skel_run11").unwrap()))),
			});
			states.insert(StateName::from("skel_run11").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 5, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("skel_run12").unwrap()))),
			});
			states.insert(StateName::from("skel_run12").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 5, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("skel_run1").unwrap()))),
			});
			states.insert(StateName::from("skel_fist1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 6, full_bright: false},
				next: Some((0 * FRAME_TIME, Some(StateName::from("skel_fist2").unwrap()))),
			});
			states.insert(StateName::from("skel_fist2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 6, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("skel_fist3").unwrap()))),
			});
			states.insert(StateName::from("skel_fist3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 7, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("skel_fist4").unwrap()))),
			});
			states.insert(StateName::from("skel_fist4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 8, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("skel_run1").unwrap()))),
			});
			states.insert(StateName::from("skel_miss1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 9, full_bright: true},
				next: Some((0 * FRAME_TIME, Some(StateName::from("skel_miss2").unwrap()))),
			});
			states.insert(StateName::from("skel_miss2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 9, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("skel_miss3").unwrap()))),
			});
			states.insert(StateName::from("skel_miss3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 10, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("skel_miss4").unwrap()))),
			});
			states.insert(StateName::from("skel_miss4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 10, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("skel_run1").unwrap()))),
			});
			states.insert(StateName::from("skel_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 11, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("skel_pain2").unwrap()))),
			});
			states.insert(StateName::from("skel_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 11, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("skel_run1").unwrap()))),
			});
			states.insert(StateName::from("skel_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 11, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("skel_die2").unwrap()))),
			});
			states.insert(StateName::from("skel_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 12, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("skel_die3").unwrap()))),
			});
			states.insert(StateName::from("skel_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 13, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("skel_die4").unwrap()))),
			});
			states.insert(StateName::from("skel_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 14, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("skel_die5").unwrap()))),
			});
			states.insert(StateName::from("skel_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 15, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("skel_die6").unwrap()))),
			});
			states.insert(StateName::from("skel_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 16, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("skel_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 16, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("skel_raise2").unwrap()))),
			});
			states.insert(StateName::from("skel_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 15, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("skel_raise3").unwrap()))),
			});
			states.insert(StateName::from("skel_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 14, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("skel_raise4").unwrap()))),
			});
			states.insert(StateName::from("skel_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 13, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("skel_raise5").unwrap()))),
			});
			states.insert(StateName::from("skel_raise5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 12, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("skel_raise6").unwrap()))),
			});
			states.insert(StateName::from("skel_raise6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skel.sprite"), frame: 11, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("skel_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("skel_stnd").unwrap()),
		see_state: Some(StateName::from("skel_run1").unwrap()),
		pain_state: Some(StateName::from("skel_pain").unwrap()),
		melee_state: Some(StateName::from("skel_fist1").unwrap()),
		missile_state: Some(StateName::from("skel_miss1").unwrap()),
		death_state: Some(StateName::from("skel_die1").unwrap()),
		raise_state: Some(StateName::from("skel_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("skel_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("undead", template);

	let template = EntityTemplate {
		name: Some("tracer"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(5);
			states.insert(StateName::from("tracer").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatb.sprite"), frame: 0, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("tracer2").unwrap()))),
			});
			states.insert(StateName::from("tracer2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatb.sprite"), frame: 1, full_bright: true},
				next: Some((2 * FRAME_TIME, Some(StateName::from("tracer").unwrap()))),
			});
			states.insert(StateName::from("traceexp1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fbxp.sprite"), frame: 0, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("traceexp2").unwrap()))),
			});
			states.insert(StateName::from("traceexp2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fbxp.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("traceexp3").unwrap()))),
			});
			states.insert(StateName::from("traceexp3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fbxp.sprite"), frame: 2, full_bright: true},
				next: Some((4 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("tracer").unwrap()),
		death_state: Some(StateName::from("traceexp1").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("fatb.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("tracer").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("tracer", template);

	let template = EntityTemplate {
		name: Some("smoke"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(5);
			states.insert(StateName::from("smoke1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 1, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("smoke2").unwrap()))),
			});
			states.insert(StateName::from("smoke2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 2, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("smoke3").unwrap()))),
			});
			states.insert(StateName::from("smoke3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 1, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("smoke4").unwrap()))),
			});
			states.insert(StateName::from("smoke4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 2, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("smoke5").unwrap()))),
			});
			states.insert(StateName::from("smoke5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 3, full_bright: false},
				next: Some((4 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("smoke1").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("puff.sprite"),
				frame: 1,
				full_bright: false,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("smoke1").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("smoke", template);

	let template = EntityTemplate {
		name: Some("fatso"),
		type_id: Some(EntityTypeId::Thing(67)),
		states: {
			let mut states = HashMap::with_capacity(44);
			states.insert(StateName::from("fatt_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 0, full_bright: false},
				next: Some((15 * FRAME_TIME, Some(StateName::from("fatt_stnd2").unwrap()))),
			});
			states.insert(StateName::from("fatt_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 1, full_bright: false},
				next: Some((15 * FRAME_TIME, Some(StateName::from("fatt_stnd").unwrap()))),
			});
			states.insert(StateName::from("fatt_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 0, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatt_run2").unwrap()))),
			});
			states.insert(StateName::from("fatt_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 0, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatt_run3").unwrap()))),
			});
			states.insert(StateName::from("fatt_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 1, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatt_run4").unwrap()))),
			});
			states.insert(StateName::from("fatt_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 1, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatt_run5").unwrap()))),
			});
			states.insert(StateName::from("fatt_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 2, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatt_run6").unwrap()))),
			});
			states.insert(StateName::from("fatt_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 2, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatt_run7").unwrap()))),
			});
			states.insert(StateName::from("fatt_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 3, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatt_run8").unwrap()))),
			});
			states.insert(StateName::from("fatt_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 3, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatt_run9").unwrap()))),
			});
			states.insert(StateName::from("fatt_run9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 4, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatt_run10").unwrap()))),
			});
			states.insert(StateName::from("fatt_run10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 4, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatt_run11").unwrap()))),
			});
			states.insert(StateName::from("fatt_run11").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 5, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatt_run12").unwrap()))),
			});
			states.insert(StateName::from("fatt_run12").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 5, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatt_run1").unwrap()))),
			});
			states.insert(StateName::from("fatt_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 6, full_bright: false},
				next: Some((20 * FRAME_TIME, Some(StateName::from("fatt_atk2").unwrap()))),
			});
			states.insert(StateName::from("fatt_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 7, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("fatt_atk3").unwrap()))),
			});
			states.insert(StateName::from("fatt_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 8, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_atk4").unwrap()))),
			});
			states.insert(StateName::from("fatt_atk4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 6, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_atk5").unwrap()))),
			});
			states.insert(StateName::from("fatt_atk5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 7, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("fatt_atk6").unwrap()))),
			});
			states.insert(StateName::from("fatt_atk6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 8, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_atk7").unwrap()))),
			});
			states.insert(StateName::from("fatt_atk7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 6, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_atk8").unwrap()))),
			});
			states.insert(StateName::from("fatt_atk8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 7, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("fatt_atk9").unwrap()))),
			});
			states.insert(StateName::from("fatt_atk9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 8, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_atk10").unwrap()))),
			});
			states.insert(StateName::from("fatt_atk10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 6, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_run1").unwrap()))),
			});
			states.insert(StateName::from("fatt_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 9, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("fatt_pain2").unwrap()))),
			});
			states.insert(StateName::from("fatt_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 9, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("fatt_run1").unwrap()))),
			});
			states.insert(StateName::from("fatt_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 10, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("fatt_die2").unwrap()))),
			});
			states.insert(StateName::from("fatt_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 11, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("fatt_die3").unwrap()))),
			});
			states.insert(StateName::from("fatt_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 12, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("fatt_die4").unwrap()))),
			});
			states.insert(StateName::from("fatt_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 13, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("fatt_die5").unwrap()))),
			});
			states.insert(StateName::from("fatt_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 14, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("fatt_die6").unwrap()))),
			});
			states.insert(StateName::from("fatt_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 15, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("fatt_die7").unwrap()))),
			});
			states.insert(StateName::from("fatt_die7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 16, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("fatt_die8").unwrap()))),
			});
			states.insert(StateName::from("fatt_die8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 17, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("fatt_die9").unwrap()))),
			});
			states.insert(StateName::from("fatt_die9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 18, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("fatt_die10").unwrap()))),
			});
			states.insert(StateName::from("fatt_die10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 19, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("fatt_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 17, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_raise2").unwrap()))),
			});
			states.insert(StateName::from("fatt_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 16, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_raise3").unwrap()))),
			});
			states.insert(StateName::from("fatt_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 15, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_raise4").unwrap()))),
			});
			states.insert(StateName::from("fatt_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 14, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_raise5").unwrap()))),
			});
			states.insert(StateName::from("fatt_raise5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 13, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_raise6").unwrap()))),
			});
			states.insert(StateName::from("fatt_raise6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 12, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_raise7").unwrap()))),
			});
			states.insert(StateName::from("fatt_raise7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 11, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_raise8").unwrap()))),
			});
			states.insert(StateName::from("fatt_raise8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fatt.sprite"), frame: 10, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("fatt_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("fatt_stnd").unwrap()),
		see_state: Some(StateName::from("fatt_run1").unwrap()),
		pain_state: Some(StateName::from("fatt_pain").unwrap()),
		missile_state: Some(StateName::from("fatt_atk1").unwrap()),
		death_state: Some(StateName::from("fatt_die1").unwrap()),
		raise_state: Some(StateName::from("fatt_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("fatt_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("fatso", template);

	let template = EntityTemplate {
		name: Some("fatshot"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(5);
			states.insert(StateName::from("fatshot1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("manf.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatshot2").unwrap()))),
			});
			states.insert(StateName::from("fatshot2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("manf.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("fatshot1").unwrap()))),
			});
			states.insert(StateName::from("fatshotx1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 1, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("fatshotx2").unwrap()))),
			});
			states.insert(StateName::from("fatshotx2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("fatshotx3").unwrap()))),
			});
			states.insert(StateName::from("fatshotx3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("fatshot1").unwrap()),
		death_state: Some(StateName::from("fatshotx1").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("manf.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("fatshot1").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("fatshot", template);

	let template = EntityTemplate {
		name: Some("chainguy"),
		type_id: Some(EntityTypeId::Thing(65)),
		states: {
			let mut states = HashMap::with_capacity(36);
			states.insert(StateName::from("cpos_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cpos_stnd2").unwrap()))),
			});
			states.insert(StateName::from("cpos_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cpos_stnd").unwrap()))),
			});
			states.insert(StateName::from("cpos_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cpos_run2").unwrap()))),
			});
			states.insert(StateName::from("cpos_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cpos_run3").unwrap()))),
			});
			states.insert(StateName::from("cpos_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cpos_run4").unwrap()))),
			});
			states.insert(StateName::from("cpos_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cpos_run5").unwrap()))),
			});
			states.insert(StateName::from("cpos_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cpos_run6").unwrap()))),
			});
			states.insert(StateName::from("cpos_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cpos_run7").unwrap()))),
			});
			states.insert(StateName::from("cpos_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cpos_run8").unwrap()))),
			});
			states.insert(StateName::from("cpos_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cpos_run1").unwrap()))),
			});
			states.insert(StateName::from("cpos_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 4, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cpos_atk2").unwrap()))),
			});
			states.insert(StateName::from("cpos_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 5, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("cpos_atk3").unwrap()))),
			});
			states.insert(StateName::from("cpos_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 4, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("cpos_atk4").unwrap()))),
			});
			states.insert(StateName::from("cpos_atk4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 5, full_bright: false},
				next: Some((1 * FRAME_TIME, Some(StateName::from("cpos_atk2").unwrap()))),
			});
			states.insert(StateName::from("cpos_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 6, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cpos_pain2").unwrap()))),
			});
			states.insert(StateName::from("cpos_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 6, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cpos_run1").unwrap()))),
			});
			states.insert(StateName::from("cpos_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 7, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_die2").unwrap()))),
			});
			states.insert(StateName::from("cpos_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 8, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_die3").unwrap()))),
			});
			states.insert(StateName::from("cpos_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 9, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_die4").unwrap()))),
			});
			states.insert(StateName::from("cpos_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 10, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_die5").unwrap()))),
			});
			states.insert(StateName::from("cpos_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 11, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_die6").unwrap()))),
			});
			states.insert(StateName::from("cpos_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 12, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_die7").unwrap()))),
			});
			states.insert(StateName::from("cpos_die7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 13, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("cpos_xdie1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 14, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_xdie2").unwrap()))),
			});
			states.insert(StateName::from("cpos_xdie2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 15, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_xdie3").unwrap()))),
			});
			states.insert(StateName::from("cpos_xdie3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 16, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_xdie4").unwrap()))),
			});
			states.insert(StateName::from("cpos_xdie4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 17, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_xdie5").unwrap()))),
			});
			states.insert(StateName::from("cpos_xdie5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 18, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_xdie6").unwrap()))),
			});
			states.insert(StateName::from("cpos_xdie6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 19, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("cpos_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 13, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_raise2").unwrap()))),
			});
			states.insert(StateName::from("cpos_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 12, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_raise3").unwrap()))),
			});
			states.insert(StateName::from("cpos_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 11, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_raise4").unwrap()))),
			});
			states.insert(StateName::from("cpos_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 10, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_raise5").unwrap()))),
			});
			states.insert(StateName::from("cpos_raise5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 9, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_raise6").unwrap()))),
			});
			states.insert(StateName::from("cpos_raise6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 8, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_raise7").unwrap()))),
			});
			states.insert(StateName::from("cpos_raise7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cpos.sprite"), frame: 7, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("cpos_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("cpos_stnd").unwrap()),
		see_state: Some(StateName::from("cpos_run1").unwrap()),
		pain_state: Some(StateName::from("cpos_pain").unwrap()),
		missile_state: Some(StateName::from("cpos_atk1").unwrap()),
		death_state: Some(StateName::from("cpos_die1").unwrap()),
		xdeath_state: Some(StateName::from("cpos_xdie1").unwrap()),
		raise_state: Some(StateName::from("cpos_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("cpos_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("chainguy", template);

	let template = EntityTemplate {
		name: Some("troop"),
		type_id: Some(EntityTypeId::Thing(3001)),
		states: {
			let mut states = HashMap::with_capacity(33);
			states.insert(StateName::from("troo_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("troo_stnd2").unwrap()))),
			});
			states.insert(StateName::from("troo_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("troo_stnd").unwrap()))),
			});
			states.insert(StateName::from("troo_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("troo_run2").unwrap()))),
			});
			states.insert(StateName::from("troo_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("troo_run3").unwrap()))),
			});
			states.insert(StateName::from("troo_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("troo_run4").unwrap()))),
			});
			states.insert(StateName::from("troo_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("troo_run5").unwrap()))),
			});
			states.insert(StateName::from("troo_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("troo_run6").unwrap()))),
			});
			states.insert(StateName::from("troo_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("troo_run7").unwrap()))),
			});
			states.insert(StateName::from("troo_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("troo_run8").unwrap()))),
			});
			states.insert(StateName::from("troo_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("troo_run1").unwrap()))),
			});
			states.insert(StateName::from("troo_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 4, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("troo_atk2").unwrap()))),
			});
			states.insert(StateName::from("troo_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 5, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("troo_atk3").unwrap()))),
			});
			states.insert(StateName::from("troo_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 6, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("troo_run1").unwrap()))),
			});
			states.insert(StateName::from("troo_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 7, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("troo_pain2").unwrap()))),
			});
			states.insert(StateName::from("troo_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 7, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("troo_run1").unwrap()))),
			});
			states.insert(StateName::from("troo_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 8, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("troo_die2").unwrap()))),
			});
			states.insert(StateName::from("troo_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 9, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("troo_die3").unwrap()))),
			});
			states.insert(StateName::from("troo_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 10, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("troo_die4").unwrap()))),
			});
			states.insert(StateName::from("troo_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 11, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("troo_die5").unwrap()))),
			});
			states.insert(StateName::from("troo_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 12, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("troo_xdie1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 13, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("troo_xdie2").unwrap()))),
			});
			states.insert(StateName::from("troo_xdie2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 14, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("troo_xdie3").unwrap()))),
			});
			states.insert(StateName::from("troo_xdie3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 15, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("troo_xdie4").unwrap()))),
			});
			states.insert(StateName::from("troo_xdie4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 16, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("troo_xdie5").unwrap()))),
			});
			states.insert(StateName::from("troo_xdie5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 17, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("troo_xdie6").unwrap()))),
			});
			states.insert(StateName::from("troo_xdie6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 18, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("troo_xdie7").unwrap()))),
			});
			states.insert(StateName::from("troo_xdie7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 19, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("troo_xdie8").unwrap()))),
			});
			states.insert(StateName::from("troo_xdie8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 20, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("troo_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 12, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("troo_raise2").unwrap()))),
			});
			states.insert(StateName::from("troo_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 11, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("troo_raise3").unwrap()))),
			});
			states.insert(StateName::from("troo_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 10, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("troo_raise4").unwrap()))),
			});
			states.insert(StateName::from("troo_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 9, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("troo_raise5").unwrap()))),
			});
			states.insert(StateName::from("troo_raise5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 8, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("troo_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("troo_stnd").unwrap()),
		see_state: Some(StateName::from("troo_run1").unwrap()),
		pain_state: Some(StateName::from("troo_pain").unwrap()),
		melee_state: Some(StateName::from("troo_atk1").unwrap()),
		missile_state: Some(StateName::from("troo_atk1").unwrap()),
		death_state: Some(StateName::from("troo_die1").unwrap()),
		xdeath_state: Some(StateName::from("troo_xdie1").unwrap()),
		raise_state: Some(StateName::from("troo_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("troo_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("troop", template);

	let template = EntityTemplate {
		name: Some("sergeant"),
		type_id: Some(EntityTypeId::Thing(3002)),
		states: {
			let mut states = HashMap::with_capacity(27);
			states.insert(StateName::from("sarg_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("sarg_stnd2").unwrap()))),
			});
			states.insert(StateName::from("sarg_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("sarg_stnd").unwrap()))),
			});
			states.insert(StateName::from("sarg_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 0, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run2").unwrap()))),
			});
			states.insert(StateName::from("sarg_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 0, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run3").unwrap()))),
			});
			states.insert(StateName::from("sarg_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 1, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run4").unwrap()))),
			});
			states.insert(StateName::from("sarg_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 1, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run5").unwrap()))),
			});
			states.insert(StateName::from("sarg_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 2, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run6").unwrap()))),
			});
			states.insert(StateName::from("sarg_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 2, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run7").unwrap()))),
			});
			states.insert(StateName::from("sarg_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 3, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run8").unwrap()))),
			});
			states.insert(StateName::from("sarg_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 3, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run1").unwrap()))),
			});
			states.insert(StateName::from("sarg_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 4, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("sarg_atk2").unwrap()))),
			});
			states.insert(StateName::from("sarg_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 5, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("sarg_atk3").unwrap()))),
			});
			states.insert(StateName::from("sarg_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 6, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("sarg_run1").unwrap()))),
			});
			states.insert(StateName::from("sarg_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 7, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_pain2").unwrap()))),
			});
			states.insert(StateName::from("sarg_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 7, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run1").unwrap()))),
			});
			states.insert(StateName::from("sarg_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 8, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("sarg_die2").unwrap()))),
			});
			states.insert(StateName::from("sarg_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 9, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("sarg_die3").unwrap()))),
			});
			states.insert(StateName::from("sarg_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 10, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("sarg_die4").unwrap()))),
			});
			states.insert(StateName::from("sarg_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 11, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("sarg_die5").unwrap()))),
			});
			states.insert(StateName::from("sarg_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 12, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("sarg_die6").unwrap()))),
			});
			states.insert(StateName::from("sarg_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 13, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("sarg_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 13, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sarg_raise2").unwrap()))),
			});
			states.insert(StateName::from("sarg_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 12, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sarg_raise3").unwrap()))),
			});
			states.insert(StateName::from("sarg_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 11, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sarg_raise4").unwrap()))),
			});
			states.insert(StateName::from("sarg_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 10, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sarg_raise5").unwrap()))),
			});
			states.insert(StateName::from("sarg_raise5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 9, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sarg_raise6").unwrap()))),
			});
			states.insert(StateName::from("sarg_raise6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 8, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sarg_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("sarg_stnd").unwrap()),
		see_state: Some(StateName::from("sarg_run1").unwrap()),
		pain_state: Some(StateName::from("sarg_pain").unwrap()),
		melee_state: Some(StateName::from("sarg_atk1").unwrap()),
		death_state: Some(StateName::from("sarg_die1").unwrap()),
		raise_state: Some(StateName::from("sarg_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("sarg_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("sergeant", template);

	let template = EntityTemplate {
		name: Some("shadows"),
		type_id: Some(EntityTypeId::Thing(58)),
		states: {
			let mut states = HashMap::with_capacity(27);
			states.insert(StateName::from("sarg_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("sarg_stnd2").unwrap()))),
			});
			states.insert(StateName::from("sarg_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("sarg_stnd").unwrap()))),
			});
			states.insert(StateName::from("sarg_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 0, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run2").unwrap()))),
			});
			states.insert(StateName::from("sarg_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 0, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run3").unwrap()))),
			});
			states.insert(StateName::from("sarg_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 1, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run4").unwrap()))),
			});
			states.insert(StateName::from("sarg_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 1, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run5").unwrap()))),
			});
			states.insert(StateName::from("sarg_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 2, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run6").unwrap()))),
			});
			states.insert(StateName::from("sarg_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 2, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run7").unwrap()))),
			});
			states.insert(StateName::from("sarg_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 3, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run8").unwrap()))),
			});
			states.insert(StateName::from("sarg_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 3, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run1").unwrap()))),
			});
			states.insert(StateName::from("sarg_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 4, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("sarg_atk2").unwrap()))),
			});
			states.insert(StateName::from("sarg_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 5, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("sarg_atk3").unwrap()))),
			});
			states.insert(StateName::from("sarg_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 6, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("sarg_run1").unwrap()))),
			});
			states.insert(StateName::from("sarg_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 7, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_pain2").unwrap()))),
			});
			states.insert(StateName::from("sarg_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 7, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("sarg_run1").unwrap()))),
			});
			states.insert(StateName::from("sarg_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 8, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("sarg_die2").unwrap()))),
			});
			states.insert(StateName::from("sarg_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 9, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("sarg_die3").unwrap()))),
			});
			states.insert(StateName::from("sarg_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 10, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("sarg_die4").unwrap()))),
			});
			states.insert(StateName::from("sarg_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 11, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("sarg_die5").unwrap()))),
			});
			states.insert(StateName::from("sarg_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 12, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("sarg_die6").unwrap()))),
			});
			states.insert(StateName::from("sarg_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 13, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("sarg_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 13, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sarg_raise2").unwrap()))),
			});
			states.insert(StateName::from("sarg_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 12, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sarg_raise3").unwrap()))),
			});
			states.insert(StateName::from("sarg_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 11, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sarg_raise4").unwrap()))),
			});
			states.insert(StateName::from("sarg_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 10, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sarg_raise5").unwrap()))),
			});
			states.insert(StateName::from("sarg_raise5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 9, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sarg_raise6").unwrap()))),
			});
			states.insert(StateName::from("sarg_raise6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 8, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sarg_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("sarg_stnd").unwrap()),
		see_state: Some(StateName::from("sarg_run1").unwrap()),
		pain_state: Some(StateName::from("sarg_pain").unwrap()),
		melee_state: Some(StateName::from("sarg_atk1").unwrap()),
		death_state: Some(StateName::from("sarg_die1").unwrap()),
		raise_state: Some(StateName::from("sarg_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("sarg_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("shadows", template);

	let template = EntityTemplate {
		name: Some("head"),
		type_id: Some(EntityTypeId::Thing(3005)),
		states: {
			let mut states = HashMap::with_capacity(20);
			states.insert(StateName::from("head_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("head_stnd").unwrap()))),
			});
			states.insert(StateName::from("head_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("head_run1").unwrap()))),
			});
			states.insert(StateName::from("head_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 1, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("head_atk2").unwrap()))),
			});
			states.insert(StateName::from("head_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 2, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("head_atk3").unwrap()))),
			});
			states.insert(StateName::from("head_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 3, full_bright: true},
				next: Some((5 * FRAME_TIME, Some(StateName::from("head_run1").unwrap()))),
			});
			states.insert(StateName::from("head_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 4, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("head_pain2").unwrap()))),
			});
			states.insert(StateName::from("head_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 4, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("head_pain3").unwrap()))),
			});
			states.insert(StateName::from("head_pain3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 5, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("head_run1").unwrap()))),
			});
			states.insert(StateName::from("head_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 6, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("head_die2").unwrap()))),
			});
			states.insert(StateName::from("head_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 7, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("head_die3").unwrap()))),
			});
			states.insert(StateName::from("head_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 8, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("head_die4").unwrap()))),
			});
			states.insert(StateName::from("head_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 9, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("head_die5").unwrap()))),
			});
			states.insert(StateName::from("head_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 10, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("head_die6").unwrap()))),
			});
			states.insert(StateName::from("head_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 11, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("head_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 11, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("head_raise2").unwrap()))),
			});
			states.insert(StateName::from("head_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 10, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("head_raise3").unwrap()))),
			});
			states.insert(StateName::from("head_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 9, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("head_raise4").unwrap()))),
			});
			states.insert(StateName::from("head_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 8, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("head_raise5").unwrap()))),
			});
			states.insert(StateName::from("head_raise5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 7, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("head_raise6").unwrap()))),
			});
			states.insert(StateName::from("head_raise6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 6, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("head_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("head_stnd").unwrap()),
		see_state: Some(StateName::from("head_run1").unwrap()),
		pain_state: Some(StateName::from("head_pain").unwrap()),
		missile_state: Some(StateName::from("head_atk1").unwrap()),
		death_state: Some(StateName::from("head_die1").unwrap()),
		raise_state: Some(StateName::from("head_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("head_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("head", template);

	let template = EntityTemplate {
		name: Some("bruiser"),
		type_id: Some(EntityTypeId::Thing(3003)),
		states: {
			let mut states = HashMap::with_capacity(29);
			states.insert(StateName::from("boss_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("boss_stnd2").unwrap()))),
			});
			states.insert(StateName::from("boss_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("boss_stnd").unwrap()))),
			});
			states.insert(StateName::from("boss_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("boss_run2").unwrap()))),
			});
			states.insert(StateName::from("boss_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("boss_run3").unwrap()))),
			});
			states.insert(StateName::from("boss_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("boss_run4").unwrap()))),
			});
			states.insert(StateName::from("boss_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("boss_run5").unwrap()))),
			});
			states.insert(StateName::from("boss_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("boss_run6").unwrap()))),
			});
			states.insert(StateName::from("boss_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("boss_run7").unwrap()))),
			});
			states.insert(StateName::from("boss_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("boss_run8").unwrap()))),
			});
			states.insert(StateName::from("boss_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("boss_run1").unwrap()))),
			});
			states.insert(StateName::from("boss_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 4, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_atk2").unwrap()))),
			});
			states.insert(StateName::from("boss_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 5, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_atk3").unwrap()))),
			});
			states.insert(StateName::from("boss_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 6, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_run1").unwrap()))),
			});
			states.insert(StateName::from("boss_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 7, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("boss_pain2").unwrap()))),
			});
			states.insert(StateName::from("boss_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 7, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("boss_run1").unwrap()))),
			});
			states.insert(StateName::from("boss_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 8, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_die2").unwrap()))),
			});
			states.insert(StateName::from("boss_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 9, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_die3").unwrap()))),
			});
			states.insert(StateName::from("boss_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 10, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_die4").unwrap()))),
			});
			states.insert(StateName::from("boss_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 11, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_die5").unwrap()))),
			});
			states.insert(StateName::from("boss_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 12, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_die6").unwrap()))),
			});
			states.insert(StateName::from("boss_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 13, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_die7").unwrap()))),
			});
			states.insert(StateName::from("boss_die7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 14, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("boss_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 14, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_raise2").unwrap()))),
			});
			states.insert(StateName::from("boss_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 13, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_raise3").unwrap()))),
			});
			states.insert(StateName::from("boss_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 12, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_raise4").unwrap()))),
			});
			states.insert(StateName::from("boss_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 11, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_raise5").unwrap()))),
			});
			states.insert(StateName::from("boss_raise5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 10, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_raise6").unwrap()))),
			});
			states.insert(StateName::from("boss_raise6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 9, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_raise7").unwrap()))),
			});
			states.insert(StateName::from("boss_raise7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("boss.sprite"), frame: 8, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("boss_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("boss_stnd").unwrap()),
		see_state: Some(StateName::from("boss_run1").unwrap()),
		pain_state: Some(StateName::from("boss_pain").unwrap()),
		melee_state: Some(StateName::from("boss_atk1").unwrap()),
		missile_state: Some(StateName::from("boss_atk1").unwrap()),
		death_state: Some(StateName::from("boss_die1").unwrap()),
		raise_state: Some(StateName::from("boss_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("boss_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("bruiser", template);

	let template = EntityTemplate {
		name: Some("bruisershot"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(5);
			states.insert(StateName::from("brball1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal7.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("brball2").unwrap()))),
			});
			states.insert(StateName::from("brball2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal7.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("brball1").unwrap()))),
			});
			states.insert(StateName::from("brballx1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal7.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("brballx2").unwrap()))),
			});
			states.insert(StateName::from("brballx2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal7.sprite"), frame: 3, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("brballx3").unwrap()))),
			});
			states.insert(StateName::from("brballx3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal7.sprite"), frame: 4, full_bright: true},
				next: Some((6 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("brball1").unwrap()),
		death_state: Some(StateName::from("brballx1").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("bal7.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("brball1").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("bruisershot", template);

	let template = EntityTemplate {
		name: Some("knight"),
		type_id: Some(EntityTypeId::Thing(69)),
		states: {
			let mut states = HashMap::with_capacity(29);
			states.insert(StateName::from("bos2_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("bos2_stnd2").unwrap()))),
			});
			states.insert(StateName::from("bos2_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("bos2_stnd").unwrap()))),
			});
			states.insert(StateName::from("bos2_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bos2_run2").unwrap()))),
			});
			states.insert(StateName::from("bos2_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bos2_run3").unwrap()))),
			});
			states.insert(StateName::from("bos2_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bos2_run4").unwrap()))),
			});
			states.insert(StateName::from("bos2_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bos2_run5").unwrap()))),
			});
			states.insert(StateName::from("bos2_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bos2_run6").unwrap()))),
			});
			states.insert(StateName::from("bos2_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bos2_run7").unwrap()))),
			});
			states.insert(StateName::from("bos2_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bos2_run8").unwrap()))),
			});
			states.insert(StateName::from("bos2_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bos2_run1").unwrap()))),
			});
			states.insert(StateName::from("bos2_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 4, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_atk2").unwrap()))),
			});
			states.insert(StateName::from("bos2_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 5, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_atk3").unwrap()))),
			});
			states.insert(StateName::from("bos2_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 6, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_run1").unwrap()))),
			});
			states.insert(StateName::from("bos2_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 7, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("bos2_pain2").unwrap()))),
			});
			states.insert(StateName::from("bos2_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 7, full_bright: false},
				next: Some((2 * FRAME_TIME, Some(StateName::from("bos2_run1").unwrap()))),
			});
			states.insert(StateName::from("bos2_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 8, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_die2").unwrap()))),
			});
			states.insert(StateName::from("bos2_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 9, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_die3").unwrap()))),
			});
			states.insert(StateName::from("bos2_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 10, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_die4").unwrap()))),
			});
			states.insert(StateName::from("bos2_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 11, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_die5").unwrap()))),
			});
			states.insert(StateName::from("bos2_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 12, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_die6").unwrap()))),
			});
			states.insert(StateName::from("bos2_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 13, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_die7").unwrap()))),
			});
			states.insert(StateName::from("bos2_die7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 14, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("bos2_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 14, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_raise2").unwrap()))),
			});
			states.insert(StateName::from("bos2_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 13, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_raise3").unwrap()))),
			});
			states.insert(StateName::from("bos2_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 12, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_raise4").unwrap()))),
			});
			states.insert(StateName::from("bos2_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 11, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_raise5").unwrap()))),
			});
			states.insert(StateName::from("bos2_raise5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 10, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_raise6").unwrap()))),
			});
			states.insert(StateName::from("bos2_raise6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 9, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_raise7").unwrap()))),
			});
			states.insert(StateName::from("bos2_raise7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bos2.sprite"), frame: 8, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bos2_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("bos2_stnd").unwrap()),
		see_state: Some(StateName::from("bos2_run1").unwrap()),
		pain_state: Some(StateName::from("bos2_pain").unwrap()),
		melee_state: Some(StateName::from("bos2_atk1").unwrap()),
		missile_state: Some(StateName::from("bos2_atk1").unwrap()),
		death_state: Some(StateName::from("bos2_die1").unwrap()),
		raise_state: Some(StateName::from("bos2_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bos2_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("knight", template);

	let template = EntityTemplate {
		name: Some("skull"),
		type_id: Some(EntityTypeId::Thing(3006)),
		states: {
			let mut states = HashMap::with_capacity(16);
			states.insert(StateName::from("skull_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 0, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("skull_stnd2").unwrap()))),
			});
			states.insert(StateName::from("skull_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 1, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("skull_stnd").unwrap()))),
			});
			states.insert(StateName::from("skull_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("skull_run2").unwrap()))),
			});
			states.insert(StateName::from("skull_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("skull_run1").unwrap()))),
			});
			states.insert(StateName::from("skull_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 2, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("skull_atk2").unwrap()))),
			});
			states.insert(StateName::from("skull_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("skull_atk3").unwrap()))),
			});
			states.insert(StateName::from("skull_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 2, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("skull_atk4").unwrap()))),
			});
			states.insert(StateName::from("skull_atk4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("skull_atk3").unwrap()))),
			});
			states.insert(StateName::from("skull_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 4, full_bright: true},
				next: Some((3 * FRAME_TIME, Some(StateName::from("skull_pain2").unwrap()))),
			});
			states.insert(StateName::from("skull_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 4, full_bright: true},
				next: Some((3 * FRAME_TIME, Some(StateName::from("skull_run1").unwrap()))),
			});
			states.insert(StateName::from("skull_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 5, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("skull_die2").unwrap()))),
			});
			states.insert(StateName::from("skull_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 6, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("skull_die3").unwrap()))),
			});
			states.insert(StateName::from("skull_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 7, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("skull_die4").unwrap()))),
			});
			states.insert(StateName::from("skull_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 8, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("skull_die5").unwrap()))),
			});
			states.insert(StateName::from("skull_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 9, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("skull_die6").unwrap()))),
			});
			states.insert(StateName::from("skull_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 10, full_bright: false},
				next: Some((6 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("skull_stnd").unwrap()),
		see_state: Some(StateName::from("skull_run1").unwrap()),
		pain_state: Some(StateName::from("skull_pain").unwrap()),
		missile_state: Some(StateName::from("skull_atk1").unwrap()),
		death_state: Some(StateName::from("skull_die1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("skull_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("skull", template);

	let template = EntityTemplate {
		name: Some("spider"),
		type_id: Some(EntityTypeId::Thing(7)),
		states: {
			let mut states = HashMap::with_capacity(31);
			states.insert(StateName::from("spid_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spid_stnd2").unwrap()))),
			});
			states.insert(StateName::from("spid_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spid_stnd").unwrap()))),
			});
			states.insert(StateName::from("spid_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_run2").unwrap()))),
			});
			states.insert(StateName::from("spid_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_run3").unwrap()))),
			});
			states.insert(StateName::from("spid_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_run4").unwrap()))),
			});
			states.insert(StateName::from("spid_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_run5").unwrap()))),
			});
			states.insert(StateName::from("spid_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_run6").unwrap()))),
			});
			states.insert(StateName::from("spid_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_run7").unwrap()))),
			});
			states.insert(StateName::from("spid_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_run8").unwrap()))),
			});
			states.insert(StateName::from("spid_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_run9").unwrap()))),
			});
			states.insert(StateName::from("spid_run9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 4, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_run10").unwrap()))),
			});
			states.insert(StateName::from("spid_run10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 4, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_run11").unwrap()))),
			});
			states.insert(StateName::from("spid_run11").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 5, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_run12").unwrap()))),
			});
			states.insert(StateName::from("spid_run12").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 5, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_run1").unwrap()))),
			});
			states.insert(StateName::from("spid_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 0, full_bright: true},
				next: Some((20 * FRAME_TIME, Some(StateName::from("spid_atk2").unwrap()))),
			});
			states.insert(StateName::from("spid_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 6, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("spid_atk3").unwrap()))),
			});
			states.insert(StateName::from("spid_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 7, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("spid_atk4").unwrap()))),
			});
			states.insert(StateName::from("spid_atk4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 7, full_bright: true},
				next: Some((1 * FRAME_TIME, Some(StateName::from("spid_atk2").unwrap()))),
			});
			states.insert(StateName::from("spid_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 8, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_pain2").unwrap()))),
			});
			states.insert(StateName::from("spid_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 8, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spid_run1").unwrap()))),
			});
			states.insert(StateName::from("spid_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 9, full_bright: false},
				next: Some((20 * FRAME_TIME, Some(StateName::from("spid_die2").unwrap()))),
			});
			states.insert(StateName::from("spid_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 10, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spid_die3").unwrap()))),
			});
			states.insert(StateName::from("spid_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 11, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spid_die4").unwrap()))),
			});
			states.insert(StateName::from("spid_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 12, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spid_die5").unwrap()))),
			});
			states.insert(StateName::from("spid_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 13, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spid_die6").unwrap()))),
			});
			states.insert(StateName::from("spid_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 14, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spid_die7").unwrap()))),
			});
			states.insert(StateName::from("spid_die7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 15, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spid_die8").unwrap()))),
			});
			states.insert(StateName::from("spid_die8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 16, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spid_die9").unwrap()))),
			});
			states.insert(StateName::from("spid_die9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 17, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("spid_die10").unwrap()))),
			});
			states.insert(StateName::from("spid_die10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 18, full_bright: false},
				next: Some((30 * FRAME_TIME, Some(StateName::from("spid_die11").unwrap()))),
			});
			states.insert(StateName::from("spid_die11").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spid.sprite"), frame: 18, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("spid_stnd").unwrap()),
		see_state: Some(StateName::from("spid_run1").unwrap()),
		pain_state: Some(StateName::from("spid_pain").unwrap()),
		missile_state: Some(StateName::from("spid_atk1").unwrap()),
		death_state: Some(StateName::from("spid_die1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("spid_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("spider", template);

	let template = EntityTemplate {
		name: Some("baby"),
		type_id: Some(EntityTypeId::Thing(68)),
		states: {
			let mut states = HashMap::with_capacity(35);
			states.insert(StateName::from("bspi_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("bspi_stnd2").unwrap()))),
			});
			states.insert(StateName::from("bspi_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("bspi_stnd").unwrap()))),
			});
			states.insert(StateName::from("bspi_sight").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 0, full_bright: false},
				next: Some((20 * FRAME_TIME, Some(StateName::from("bspi_run1").unwrap()))),
			});
			states.insert(StateName::from("bspi_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_run2").unwrap()))),
			});
			states.insert(StateName::from("bspi_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_run3").unwrap()))),
			});
			states.insert(StateName::from("bspi_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_run4").unwrap()))),
			});
			states.insert(StateName::from("bspi_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_run5").unwrap()))),
			});
			states.insert(StateName::from("bspi_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_run6").unwrap()))),
			});
			states.insert(StateName::from("bspi_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_run7").unwrap()))),
			});
			states.insert(StateName::from("bspi_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_run8").unwrap()))),
			});
			states.insert(StateName::from("bspi_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_run9").unwrap()))),
			});
			states.insert(StateName::from("bspi_run9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 4, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_run10").unwrap()))),
			});
			states.insert(StateName::from("bspi_run10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 4, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_run11").unwrap()))),
			});
			states.insert(StateName::from("bspi_run11").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 5, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_run12").unwrap()))),
			});
			states.insert(StateName::from("bspi_run12").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 5, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_run1").unwrap()))),
			});
			states.insert(StateName::from("bspi_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 0, full_bright: true},
				next: Some((20 * FRAME_TIME, Some(StateName::from("bspi_atk2").unwrap()))),
			});
			states.insert(StateName::from("bspi_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 6, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("bspi_atk3").unwrap()))),
			});
			states.insert(StateName::from("bspi_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 7, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("bspi_atk4").unwrap()))),
			});
			states.insert(StateName::from("bspi_atk4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 7, full_bright: true},
				next: Some((1 * FRAME_TIME, Some(StateName::from("bspi_atk2").unwrap()))),
			});
			states.insert(StateName::from("bspi_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 8, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_pain2").unwrap()))),
			});
			states.insert(StateName::from("bspi_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 8, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("bspi_run1").unwrap()))),
			});
			states.insert(StateName::from("bspi_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 9, full_bright: false},
				next: Some((20 * FRAME_TIME, Some(StateName::from("bspi_die2").unwrap()))),
			});
			states.insert(StateName::from("bspi_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 10, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("bspi_die3").unwrap()))),
			});
			states.insert(StateName::from("bspi_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 11, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("bspi_die4").unwrap()))),
			});
			states.insert(StateName::from("bspi_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 12, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("bspi_die5").unwrap()))),
			});
			states.insert(StateName::from("bspi_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 13, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("bspi_die6").unwrap()))),
			});
			states.insert(StateName::from("bspi_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 14, full_bright: false},
				next: Some((7 * FRAME_TIME, Some(StateName::from("bspi_die7").unwrap()))),
			});
			states.insert(StateName::from("bspi_die7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 15, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("bspi_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 15, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("bspi_raise2").unwrap()))),
			});
			states.insert(StateName::from("bspi_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 14, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("bspi_raise3").unwrap()))),
			});
			states.insert(StateName::from("bspi_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 13, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("bspi_raise4").unwrap()))),
			});
			states.insert(StateName::from("bspi_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 12, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("bspi_raise5").unwrap()))),
			});
			states.insert(StateName::from("bspi_raise5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 11, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("bspi_raise6").unwrap()))),
			});
			states.insert(StateName::from("bspi_raise6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 10, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("bspi_raise7").unwrap()))),
			});
			states.insert(StateName::from("bspi_raise7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bspi.sprite"), frame: 9, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("bspi_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("bspi_stnd").unwrap()),
		see_state: Some(StateName::from("bspi_sight").unwrap()),
		pain_state: Some(StateName::from("bspi_pain").unwrap()),
		missile_state: Some(StateName::from("bspi_atk1").unwrap()),
		death_state: Some(StateName::from("bspi_die1").unwrap()),
		raise_state: Some(StateName::from("bspi_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bspi_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("baby", template);

	let template = EntityTemplate {
		name: Some("cyborg"),
		type_id: Some(EntityTypeId::Thing(16)),
		states: {
			let mut states = HashMap::with_capacity(27);
			states.insert(StateName::from("cyber_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cyber_stnd2").unwrap()))),
			});
			states.insert(StateName::from("cyber_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cyber_stnd").unwrap()))),
			});
			states.insert(StateName::from("cyber_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cyber_run2").unwrap()))),
			});
			states.insert(StateName::from("cyber_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cyber_run3").unwrap()))),
			});
			states.insert(StateName::from("cyber_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cyber_run4").unwrap()))),
			});
			states.insert(StateName::from("cyber_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cyber_run5").unwrap()))),
			});
			states.insert(StateName::from("cyber_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cyber_run6").unwrap()))),
			});
			states.insert(StateName::from("cyber_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cyber_run7").unwrap()))),
			});
			states.insert(StateName::from("cyber_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cyber_run8").unwrap()))),
			});
			states.insert(StateName::from("cyber_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("cyber_run1").unwrap()))),
			});
			states.insert(StateName::from("cyber_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 4, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("cyber_atk2").unwrap()))),
			});
			states.insert(StateName::from("cyber_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 5, full_bright: false},
				next: Some((12 * FRAME_TIME, Some(StateName::from("cyber_atk3").unwrap()))),
			});
			states.insert(StateName::from("cyber_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 4, full_bright: false},
				next: Some((12 * FRAME_TIME, Some(StateName::from("cyber_atk4").unwrap()))),
			});
			states.insert(StateName::from("cyber_atk4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 5, full_bright: false},
				next: Some((12 * FRAME_TIME, Some(StateName::from("cyber_atk5").unwrap()))),
			});
			states.insert(StateName::from("cyber_atk5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 4, full_bright: false},
				next: Some((12 * FRAME_TIME, Some(StateName::from("cyber_atk6").unwrap()))),
			});
			states.insert(StateName::from("cyber_atk6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 5, full_bright: false},
				next: Some((12 * FRAME_TIME, Some(StateName::from("cyber_run1").unwrap()))),
			});
			states.insert(StateName::from("cyber_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 6, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cyber_run1").unwrap()))),
			});
			states.insert(StateName::from("cyber_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 7, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cyber_die2").unwrap()))),
			});
			states.insert(StateName::from("cyber_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 8, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cyber_die3").unwrap()))),
			});
			states.insert(StateName::from("cyber_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 9, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cyber_die4").unwrap()))),
			});
			states.insert(StateName::from("cyber_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 10, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cyber_die5").unwrap()))),
			});
			states.insert(StateName::from("cyber_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 11, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cyber_die6").unwrap()))),
			});
			states.insert(StateName::from("cyber_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 12, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cyber_die7").unwrap()))),
			});
			states.insert(StateName::from("cyber_die7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 13, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cyber_die8").unwrap()))),
			});
			states.insert(StateName::from("cyber_die8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 14, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("cyber_die9").unwrap()))),
			});
			states.insert(StateName::from("cyber_die9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 15, full_bright: false},
				next: Some((30 * FRAME_TIME, Some(StateName::from("cyber_die10").unwrap()))),
			});
			states.insert(StateName::from("cyber_die10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cybr.sprite"), frame: 15, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("cyber_stnd").unwrap()),
		see_state: Some(StateName::from("cyber_run1").unwrap()),
		pain_state: Some(StateName::from("cyber_pain").unwrap()),
		missile_state: Some(StateName::from("cyber_atk1").unwrap()),
		death_state: Some(StateName::from("cyber_die1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("cyber_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("cyborg", template);

	let template = EntityTemplate {
		name: Some("pain"),
		type_id: Some(EntityTypeId::Thing(71)),
		states: {
			let mut states = HashMap::with_capacity(25);
			states.insert(StateName::from("pain_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("pain_stnd").unwrap()))),
			});
			states.insert(StateName::from("pain_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("pain_run2").unwrap()))),
			});
			states.insert(StateName::from("pain_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("pain_run3").unwrap()))),
			});
			states.insert(StateName::from("pain_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("pain_run4").unwrap()))),
			});
			states.insert(StateName::from("pain_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("pain_run5").unwrap()))),
			});
			states.insert(StateName::from("pain_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("pain_run6").unwrap()))),
			});
			states.insert(StateName::from("pain_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("pain_run1").unwrap()))),
			});
			states.insert(StateName::from("pain_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 3, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("pain_atk2").unwrap()))),
			});
			states.insert(StateName::from("pain_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 4, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("pain_atk3").unwrap()))),
			});
			states.insert(StateName::from("pain_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 5, full_bright: true},
				next: Some((5 * FRAME_TIME, Some(StateName::from("pain_atk4").unwrap()))),
			});
			states.insert(StateName::from("pain_atk4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 5, full_bright: true},
				next: Some((0 * FRAME_TIME, Some(StateName::from("pain_run1").unwrap()))),
			});
			states.insert(StateName::from("pain_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 6, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pain_pain2").unwrap()))),
			});
			states.insert(StateName::from("pain_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 6, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pain_run1").unwrap()))),
			});
			states.insert(StateName::from("pain_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 7, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("pain_die2").unwrap()))),
			});
			states.insert(StateName::from("pain_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 8, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("pain_die3").unwrap()))),
			});
			states.insert(StateName::from("pain_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 9, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("pain_die4").unwrap()))),
			});
			states.insert(StateName::from("pain_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 10, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("pain_die5").unwrap()))),
			});
			states.insert(StateName::from("pain_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 11, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("pain_die6").unwrap()))),
			});
			states.insert(StateName::from("pain_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 12, full_bright: true},
				next: Some((8 * FRAME_TIME, None)),
			});
			states.insert(StateName::from("pain_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 12, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("pain_raise2").unwrap()))),
			});
			states.insert(StateName::from("pain_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 11, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("pain_raise3").unwrap()))),
			});
			states.insert(StateName::from("pain_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 10, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("pain_raise4").unwrap()))),
			});
			states.insert(StateName::from("pain_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 9, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("pain_raise5").unwrap()))),
			});
			states.insert(StateName::from("pain_raise5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 8, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("pain_raise6").unwrap()))),
			});
			states.insert(StateName::from("pain_raise6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pain.sprite"), frame: 7, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("pain_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("pain_stnd").unwrap()),
		see_state: Some(StateName::from("pain_run1").unwrap()),
		pain_state: Some(StateName::from("pain_pain").unwrap()),
		missile_state: Some(StateName::from("pain_atk1").unwrap()),
		death_state: Some(StateName::from("pain_die1").unwrap()),
		raise_state: Some(StateName::from("pain_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("pain_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("pain", template);

	let template = EntityTemplate {
		name: Some("wolfss"),
		type_id: Some(EntityTypeId::Thing(84)),
		states: {
			let mut states = HashMap::with_capacity(37);
			states.insert(StateName::from("sswv_stnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("sswv_stnd2").unwrap()))),
			});
			states.insert(StateName::from("sswv_stnd2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 1, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("sswv_stnd").unwrap()))),
			});
			states.insert(StateName::from("sswv_run1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("sswv_run2").unwrap()))),
			});
			states.insert(StateName::from("sswv_run2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 0, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("sswv_run3").unwrap()))),
			});
			states.insert(StateName::from("sswv_run3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("sswv_run4").unwrap()))),
			});
			states.insert(StateName::from("sswv_run4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 1, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("sswv_run5").unwrap()))),
			});
			states.insert(StateName::from("sswv_run5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("sswv_run6").unwrap()))),
			});
			states.insert(StateName::from("sswv_run6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 2, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("sswv_run7").unwrap()))),
			});
			states.insert(StateName::from("sswv_run7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("sswv_run8").unwrap()))),
			});
			states.insert(StateName::from("sswv_run8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 3, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("sswv_run1").unwrap()))),
			});
			states.insert(StateName::from("sswv_atk1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 4, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("sswv_atk2").unwrap()))),
			});
			states.insert(StateName::from("sswv_atk2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 5, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("sswv_atk3").unwrap()))),
			});
			states.insert(StateName::from("sswv_atk3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 6, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("sswv_atk4").unwrap()))),
			});
			states.insert(StateName::from("sswv_atk4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 5, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("sswv_atk5").unwrap()))),
			});
			states.insert(StateName::from("sswv_atk5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 6, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("sswv_atk6").unwrap()))),
			});
			states.insert(StateName::from("sswv_atk6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 5, full_bright: false},
				next: Some((1 * FRAME_TIME, Some(StateName::from("sswv_atk2").unwrap()))),
			});
			states.insert(StateName::from("sswv_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 7, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("sswv_pain2").unwrap()))),
			});
			states.insert(StateName::from("sswv_pain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 7, full_bright: false},
				next: Some((3 * FRAME_TIME, Some(StateName::from("sswv_run1").unwrap()))),
			});
			states.insert(StateName::from("sswv_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 8, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_die2").unwrap()))),
			});
			states.insert(StateName::from("sswv_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 9, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_die3").unwrap()))),
			});
			states.insert(StateName::from("sswv_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 10, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_die4").unwrap()))),
			});
			states.insert(StateName::from("sswv_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 11, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_die5").unwrap()))),
			});
			states.insert(StateName::from("sswv_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 12, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("sswv_xdie1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 13, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_xdie2").unwrap()))),
			});
			states.insert(StateName::from("sswv_xdie2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 14, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_xdie3").unwrap()))),
			});
			states.insert(StateName::from("sswv_xdie3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 15, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_xdie4").unwrap()))),
			});
			states.insert(StateName::from("sswv_xdie4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 16, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_xdie5").unwrap()))),
			});
			states.insert(StateName::from("sswv_xdie5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 17, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_xdie6").unwrap()))),
			});
			states.insert(StateName::from("sswv_xdie6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 18, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_xdie7").unwrap()))),
			});
			states.insert(StateName::from("sswv_xdie7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 19, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_xdie8").unwrap()))),
			});
			states.insert(StateName::from("sswv_xdie8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 20, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_xdie9").unwrap()))),
			});
			states.insert(StateName::from("sswv_xdie9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 21, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("sswv_raise1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 12, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_raise2").unwrap()))),
			});
			states.insert(StateName::from("sswv_raise2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 11, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_raise3").unwrap()))),
			});
			states.insert(StateName::from("sswv_raise3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 10, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_raise4").unwrap()))),
			});
			states.insert(StateName::from("sswv_raise4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 9, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_raise5").unwrap()))),
			});
			states.insert(StateName::from("sswv_raise5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 8, full_bright: false},
				next: Some((5 * FRAME_TIME, Some(StateName::from("sswv_run1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("sswv_stnd").unwrap()),
		see_state: Some(StateName::from("sswv_run1").unwrap()),
		pain_state: Some(StateName::from("sswv_pain").unwrap()),
		missile_state: Some(StateName::from("sswv_atk1").unwrap()),
		death_state: Some(StateName::from("sswv_die1").unwrap()),
		xdeath_state: Some(StateName::from("sswv_xdie1").unwrap()),
		raise_state: Some(StateName::from("sswv_raise1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("sswv_stnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("wolfss", template);

	let template = EntityTemplate {
		name: Some("keen"),
		type_id: Some(EntityTypeId::Thing(72)),
		states: {
			let mut states = HashMap::with_capacity(15);
			states.insert(StateName::from("keenstnd").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("commkeen").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 0, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("commkeen2").unwrap()))),
			});
			states.insert(StateName::from("commkeen2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 1, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("commkeen3").unwrap()))),
			});
			states.insert(StateName::from("commkeen3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 2, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("commkeen4").unwrap()))),
			});
			states.insert(StateName::from("commkeen4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 3, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("commkeen5").unwrap()))),
			});
			states.insert(StateName::from("commkeen5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 4, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("commkeen6").unwrap()))),
			});
			states.insert(StateName::from("commkeen6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 5, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("commkeen7").unwrap()))),
			});
			states.insert(StateName::from("commkeen7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 6, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("commkeen8").unwrap()))),
			});
			states.insert(StateName::from("commkeen8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 7, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("commkeen9").unwrap()))),
			});
			states.insert(StateName::from("commkeen9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 8, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("commkeen10").unwrap()))),
			});
			states.insert(StateName::from("commkeen10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 9, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("commkeen11").unwrap()))),
			});
			states.insert(StateName::from("commkeen11").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 10, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("commkeen12").unwrap()))),
			});
			states.insert(StateName::from("commkeen12").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 11, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("keenpain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 12, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("keenpain2").unwrap()))),
			});
			states.insert(StateName::from("keenpain2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("keen.sprite"), frame: 12, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("keenstnd").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("keenstnd").unwrap()),
		pain_state: Some(StateName::from("keenpain").unwrap()),
		death_state: Some(StateName::from("commkeen").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("keenstnd").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("keen", template);

	let template = EntityTemplate {
		name: Some("bossbrain"),
		type_id: Some(EntityTypeId::Thing(88)),
		states: {
			let mut states = HashMap::with_capacity(6);
			states.insert(StateName::from("brain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bbrn.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states.insert(StateName::from("brain_pain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bbrn.sprite"), frame: 1, full_bright: false},
				next: Some((36 * FRAME_TIME, Some(StateName::from("brain").unwrap()))),
			});
			states.insert(StateName::from("brain_die1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bbrn.sprite"), frame: 0, full_bright: false},
				next: Some((100 * FRAME_TIME, Some(StateName::from("brain_die2").unwrap()))),
			});
			states.insert(StateName::from("brain_die2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bbrn.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("brain_die3").unwrap()))),
			});
			states.insert(StateName::from("brain_die3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bbrn.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("brain_die4").unwrap()))),
			});
			states.insert(StateName::from("brain_die4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bbrn.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("brain").unwrap()),
		pain_state: Some(StateName::from("brain_pain").unwrap()),
		death_state: Some(StateName::from("brain_die1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("brain").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("bossbrain", template);

	let template = EntityTemplate {
		name: Some("bossspit"),
		type_id: Some(EntityTypeId::Thing(89)),
		states: {
			let mut states = HashMap::with_capacity(3);
			states.insert(StateName::from("braineye").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("braineye").unwrap()))),
			});
			states.insert(StateName::from("braineyesee").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 0, full_bright: false},
				next: Some((181 * FRAME_TIME, Some(StateName::from("braineye1").unwrap()))),
			});
			states.insert(StateName::from("braineye1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sswv.sprite"), frame: 0, full_bright: false},
				next: Some((150 * FRAME_TIME, Some(StateName::from("braineye1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("braineye").unwrap()),
		see_state: Some(StateName::from("braineyesee").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("sswv.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("braineye").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("bossspit", template);

	let template = EntityTemplate {
		name: Some("bosstarget"),
		type_id: Some(EntityTypeId::Thing(87)),
		components: EntityComponents::new(),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("bosstarget", template);

	let template = EntityTemplate {
		name: Some("spawnshot"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("spawn1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bosf.sprite"), frame: 0, full_bright: true},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spawn2").unwrap()))),
			});
			states.insert(StateName::from("spawn2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bosf.sprite"), frame: 1, full_bright: true},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spawn3").unwrap()))),
			});
			states.insert(StateName::from("spawn3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bosf.sprite"), frame: 2, full_bright: true},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spawn4").unwrap()))),
			});
			states.insert(StateName::from("spawn4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bosf.sprite"), frame: 3, full_bright: true},
				next: Some((3 * FRAME_TIME, Some(StateName::from("spawn1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("spawn1").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("bosf.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("spawn1").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("spawnshot", template);

	let template = EntityTemplate {
		name: Some("spawnfire"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(8);
			states.insert(StateName::from("spawnfire1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("spawnfire2").unwrap()))),
			});
			states.insert(StateName::from("spawnfire2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("spawnfire3").unwrap()))),
			});
			states.insert(StateName::from("spawnfire3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 2, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("spawnfire4").unwrap()))),
			});
			states.insert(StateName::from("spawnfire4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("spawnfire5").unwrap()))),
			});
			states.insert(StateName::from("spawnfire5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 4, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("spawnfire6").unwrap()))),
			});
			states.insert(StateName::from("spawnfire6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 5, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("spawnfire7").unwrap()))),
			});
			states.insert(StateName::from("spawnfire7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 6, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("spawnfire8").unwrap()))),
			});
			states.insert(StateName::from("spawnfire8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fire.sprite"), frame: 7, full_bright: true},
				next: Some((4 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("spawnfire1").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("fire.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("spawnfire1").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("spawnfire", template);

	let template = EntityTemplate {
		name: Some("barrel"),
		type_id: Some(EntityTypeId::Thing(2035)),
		states: {
			let mut states = HashMap::with_capacity(7);
			states.insert(StateName::from("bar1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bar1.sprite"), frame: 0, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bar2").unwrap()))),
			});
			states.insert(StateName::from("bar2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bar1.sprite"), frame: 1, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bar1").unwrap()))),
			});
			states.insert(StateName::from("bexp").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bexp.sprite"), frame: 0, full_bright: true},
				next: Some((5 * FRAME_TIME, Some(StateName::from("bexp2").unwrap()))),
			});
			states.insert(StateName::from("bexp2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bexp.sprite"), frame: 1, full_bright: true},
				next: Some((5 * FRAME_TIME, Some(StateName::from("bexp3").unwrap()))),
			});
			states.insert(StateName::from("bexp3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bexp.sprite"), frame: 2, full_bright: true},
				next: Some((5 * FRAME_TIME, Some(StateName::from("bexp4").unwrap()))),
			});
			states.insert(StateName::from("bexp4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bexp.sprite"), frame: 3, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("bexp5").unwrap()))),
			});
			states.insert(StateName::from("bexp5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bexp.sprite"), frame: 4, full_bright: true},
				next: Some((10 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("bar1").unwrap()),
		death_state: Some(StateName::from("bexp").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bar1").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("barrel", template);

	let template = EntityTemplate {
		name: Some("troopshot"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(5);
			states.insert(StateName::from("tball1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal1.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("tball2").unwrap()))),
			});
			states.insert(StateName::from("tball2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal1.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("tball1").unwrap()))),
			});
			states.insert(StateName::from("tballx1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal1.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("tballx2").unwrap()))),
			});
			states.insert(StateName::from("tballx2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal1.sprite"), frame: 3, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("tballx3").unwrap()))),
			});
			states.insert(StateName::from("tballx3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal1.sprite"), frame: 4, full_bright: true},
				next: Some((6 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("tball1").unwrap()),
		death_state: Some(StateName::from("tballx1").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("bal1.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("tball1").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("troopshot", template);

	let template = EntityTemplate {
		name: Some("headshot"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(5);
			states.insert(StateName::from("rball1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal2.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("rball2").unwrap()))),
			});
			states.insert(StateName::from("rball2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal2.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("rball1").unwrap()))),
			});
			states.insert(StateName::from("rballx1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal2.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("rballx2").unwrap()))),
			});
			states.insert(StateName::from("rballx2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal2.sprite"), frame: 3, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("rballx3").unwrap()))),
			});
			states.insert(StateName::from("rballx3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bal2.sprite"), frame: 4, full_bright: true},
				next: Some((6 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("rball1").unwrap()),
		death_state: Some(StateName::from("rballx1").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("bal2.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("rball1").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("headshot", template);

	let template = EntityTemplate {
		name: Some("rocket"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("rocket").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 0, full_bright: true},
				next: Some((1 * FRAME_TIME, Some(StateName::from("rocket").unwrap()))),
			});
			states.insert(StateName::from("explode1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 1, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("explode2").unwrap()))),
			});
			states.insert(StateName::from("explode2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("explode3").unwrap()))),
			});
			states.insert(StateName::from("explode3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("misl.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("rocket").unwrap()),
		death_state: Some(StateName::from("explode1").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("misl.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("rocket").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("rocket", template);

	let template = EntityTemplate {
		name: Some("plasma"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(7);
			states.insert(StateName::from("plasball").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("plss.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("plasball2").unwrap()))),
			});
			states.insert(StateName::from("plasball2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("plss.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("plasball").unwrap()))),
			});
			states.insert(StateName::from("plasexp").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("plse.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("plasexp2").unwrap()))),
			});
			states.insert(StateName::from("plasexp2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("plse.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("plasexp3").unwrap()))),
			});
			states.insert(StateName::from("plasexp3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("plse.sprite"), frame: 2, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("plasexp4").unwrap()))),
			});
			states.insert(StateName::from("plasexp4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("plse.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("plasexp5").unwrap()))),
			});
			states.insert(StateName::from("plasexp5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("plse.sprite"), frame: 4, full_bright: true},
				next: Some((4 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("plasball").unwrap()),
		death_state: Some(StateName::from("plasexp").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("plss.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("plasball").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("plasma", template);

	let template = EntityTemplate {
		name: Some("bfg"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(8);
			states.insert(StateName::from("bfgshot").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bfs1.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("bfgshot2").unwrap()))),
			});
			states.insert(StateName::from("bfgshot2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bfs1.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("bfgshot").unwrap()))),
			});
			states.insert(StateName::from("bfgland").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bfe1.sprite"), frame: 0, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bfgland2").unwrap()))),
			});
			states.insert(StateName::from("bfgland2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bfe1.sprite"), frame: 1, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bfgland3").unwrap()))),
			});
			states.insert(StateName::from("bfgland3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bfe1.sprite"), frame: 2, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bfgland4").unwrap()))),
			});
			states.insert(StateName::from("bfgland4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bfe1.sprite"), frame: 3, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bfgland5").unwrap()))),
			});
			states.insert(StateName::from("bfgland5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bfe1.sprite"), frame: 4, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bfgland6").unwrap()))),
			});
			states.insert(StateName::from("bfgland6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bfe1.sprite"), frame: 5, full_bright: true},
				next: Some((8 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("bfgshot").unwrap()),
		death_state: Some(StateName::from("bfgland").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("bfs1.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bfgshot").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("bfg", template);

	let template = EntityTemplate {
		name: Some("arachplaz"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(7);
			states.insert(StateName::from("arach_plaz").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("apls.sprite"), frame: 0, full_bright: true},
				next: Some((5 * FRAME_TIME, Some(StateName::from("arach_plaz2").unwrap()))),
			});
			states.insert(StateName::from("arach_plaz2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("apls.sprite"), frame: 1, full_bright: true},
				next: Some((5 * FRAME_TIME, Some(StateName::from("arach_plaz").unwrap()))),
			});
			states.insert(StateName::from("arach_plex").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("apbx.sprite"), frame: 0, full_bright: true},
				next: Some((5 * FRAME_TIME, Some(StateName::from("arach_plex2").unwrap()))),
			});
			states.insert(StateName::from("arach_plex2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("apbx.sprite"), frame: 1, full_bright: true},
				next: Some((5 * FRAME_TIME, Some(StateName::from("arach_plex3").unwrap()))),
			});
			states.insert(StateName::from("arach_plex3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("apbx.sprite"), frame: 2, full_bright: true},
				next: Some((5 * FRAME_TIME, Some(StateName::from("arach_plex4").unwrap()))),
			});
			states.insert(StateName::from("arach_plex4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("apbx.sprite"), frame: 3, full_bright: true},
				next: Some((5 * FRAME_TIME, Some(StateName::from("arach_plex5").unwrap()))),
			});
			states.insert(StateName::from("arach_plex5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("apbx.sprite"), frame: 4, full_bright: true},
				next: Some((5 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("arach_plaz").unwrap()),
		death_state: Some(StateName::from("arach_plex").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("apls.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("arach_plaz").unwrap()))),
			})
			.with_component(Velocity::default()),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("arachplaz", template);

	let template = EntityTemplate {
		name: Some("puff"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("puff1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("puff2").unwrap()))),
			});
			states.insert(StateName::from("puff2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 1, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("puff3").unwrap()))),
			});
			states.insert(StateName::from("puff3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 2, full_bright: false},
				next: Some((4 * FRAME_TIME, Some(StateName::from("puff4").unwrap()))),
			});
			states.insert(StateName::from("puff4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("puff.sprite"), frame: 3, full_bright: false},
				next: Some((4 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("puff1").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("puff.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("puff1").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("puff", template);

	let template = EntityTemplate {
		name: Some("blood"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(3);
			states.insert(StateName::from("blood1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("blud.sprite"), frame: 2, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("blood2").unwrap()))),
			});
			states.insert(StateName::from("blood2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("blud.sprite"), frame: 1, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("blood3").unwrap()))),
			});
			states.insert(StateName::from("blood3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("blud.sprite"), frame: 0, full_bright: false},
				next: Some((8 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("blood1").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("blud.sprite"),
				frame: 2,
				full_bright: false,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("blood1").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("blood", template);

	let template = EntityTemplate {
		name: Some("tfog"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(12);
			states.insert(StateName::from("tfog").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("tfog01").unwrap()))),
			});
			states.insert(StateName::from("tfog01").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("tfog02").unwrap()))),
			});
			states.insert(StateName::from("tfog02").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("tfog2").unwrap()))),
			});
			states.insert(StateName::from("tfog2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("tfog3").unwrap()))),
			});
			states.insert(StateName::from("tfog3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("tfog4").unwrap()))),
			});
			states.insert(StateName::from("tfog4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 3, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("tfog5").unwrap()))),
			});
			states.insert(StateName::from("tfog5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 4, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("tfog6").unwrap()))),
			});
			states.insert(StateName::from("tfog6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 5, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("tfog7").unwrap()))),
			});
			states.insert(StateName::from("tfog7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 6, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("tfog8").unwrap()))),
			});
			states.insert(StateName::from("tfog8").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 7, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("tfog9").unwrap()))),
			});
			states.insert(StateName::from("tfog9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 8, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("tfog10").unwrap()))),
			});
			states.insert(StateName::from("tfog10").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tfog.sprite"), frame: 9, full_bright: true},
				next: Some((6 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("tfog").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("tfog.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("tfog").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("tfog", template);

	let template = EntityTemplate {
		name: Some("ifog"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(7);
			states.insert(StateName::from("ifog").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("ifog01").unwrap()))),
			});
			states.insert(StateName::from("ifog01").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("ifog02").unwrap()))),
			});
			states.insert(StateName::from("ifog02").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("ifog2").unwrap()))),
			});
			states.insert(StateName::from("ifog2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("ifog3").unwrap()))),
			});
			states.insert(StateName::from("ifog3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("ifog4").unwrap()))),
			});
			states.insert(StateName::from("ifog4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 3, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("ifog5").unwrap()))),
			});
			states.insert(StateName::from("ifog5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ifog.sprite"), frame: 4, full_bright: true},
				next: Some((6 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("ifog").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("ifog.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("ifog").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("ifog", template);

	let template = EntityTemplate {
		name: Some("teleportman"),
		type_id: Some(EntityTypeId::Thing(14)),
		components: EntityComponents::new(),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("teleportman", template);

	let template = EntityTemplate {
		name: Some("extrabfg"),
		type_id: None,
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("bfgexp").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bfe2.sprite"), frame: 0, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bfgexp2").unwrap()))),
			});
			states.insert(StateName::from("bfgexp2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bfe2.sprite"), frame: 1, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bfgexp3").unwrap()))),
			});
			states.insert(StateName::from("bfgexp3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bfe2.sprite"), frame: 2, full_bright: true},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bfgexp4").unwrap()))),
			});
			states.insert(StateName::from("bfgexp4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bfe2.sprite"), frame: 3, full_bright: true},
				next: Some((8 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("bfgexp").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("bfe2.sprite"),
				frame: 0,
				full_bright: true,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bfgexp").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("extrabfg", template);

	let template = EntityTemplate {
		name: Some("misc0"),
		type_id: Some(EntityTypeId::Thing(2018)),
		states: {
			let mut states = HashMap::with_capacity(2);
			states.insert(StateName::from("arm1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("arm1.sprite"), frame: 0, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("arm1a").unwrap()))),
			});
			states.insert(StateName::from("arm1a").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("arm1.sprite"), frame: 1, full_bright: true},
				next: Some((7 * FRAME_TIME, Some(StateName::from("arm1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("arm1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("arm1").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc0", template);

	let template = EntityTemplate {
		name: Some("misc1"),
		type_id: Some(EntityTypeId::Thing(2019)),
		states: {
			let mut states = HashMap::with_capacity(2);
			states.insert(StateName::from("arm2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("arm2.sprite"), frame: 0, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("arm2a").unwrap()))),
			});
			states.insert(StateName::from("arm2a").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("arm2.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("arm2").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("arm2").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("arm2").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc1", template);

	let template = EntityTemplate {
		name: Some("misc2"),
		type_id: Some(EntityTypeId::Thing(2014)),
		states: {
			let mut states = HashMap::with_capacity(6);
			states.insert(StateName::from("bon1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bon1.sprite"), frame: 0, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bon1a").unwrap()))),
			});
			states.insert(StateName::from("bon1a").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bon1.sprite"), frame: 1, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bon1b").unwrap()))),
			});
			states.insert(StateName::from("bon1b").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bon1.sprite"), frame: 2, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bon1c").unwrap()))),
			});
			states.insert(StateName::from("bon1c").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bon1.sprite"), frame: 3, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bon1d").unwrap()))),
			});
			states.insert(StateName::from("bon1d").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bon1.sprite"), frame: 2, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bon1e").unwrap()))),
			});
			states.insert(StateName::from("bon1e").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bon1.sprite"), frame: 1, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bon1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("bon1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bon1").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc2", template);

	let template = EntityTemplate {
		name: Some("misc3"),
		type_id: Some(EntityTypeId::Thing(2015)),
		states: {
			let mut states = HashMap::with_capacity(6);
			states.insert(StateName::from("bon2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bon2.sprite"), frame: 0, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bon2a").unwrap()))),
			});
			states.insert(StateName::from("bon2a").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bon2.sprite"), frame: 1, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bon2b").unwrap()))),
			});
			states.insert(StateName::from("bon2b").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bon2.sprite"), frame: 2, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bon2c").unwrap()))),
			});
			states.insert(StateName::from("bon2c").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bon2.sprite"), frame: 3, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bon2d").unwrap()))),
			});
			states.insert(StateName::from("bon2d").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bon2.sprite"), frame: 2, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bon2e").unwrap()))),
			});
			states.insert(StateName::from("bon2e").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bon2.sprite"), frame: 1, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bon2").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("bon2").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bon2").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc3", template);

	let template = EntityTemplate {
		name: Some("misc4"),
		type_id: Some(EntityTypeId::Thing(5)),
		states: {
			let mut states = HashMap::with_capacity(2);
			states.insert(StateName::from("bkey").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bkey.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("bkey2").unwrap()))),
			});
			states.insert(StateName::from("bkey2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bkey.sprite"), frame: 1, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("bkey").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("bkey").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bkey").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc4", template);

	let template = EntityTemplate {
		name: Some("misc5"),
		type_id: Some(EntityTypeId::Thing(13)),
		states: {
			let mut states = HashMap::with_capacity(2);
			states.insert(StateName::from("rkey").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("rkey.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("rkey2").unwrap()))),
			});
			states.insert(StateName::from("rkey2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("rkey.sprite"), frame: 1, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("rkey").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("rkey").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("rkey").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc5", template);

	let template = EntityTemplate {
		name: Some("misc6"),
		type_id: Some(EntityTypeId::Thing(6)),
		states: {
			let mut states = HashMap::with_capacity(2);
			states.insert(StateName::from("ykey").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ykey.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("ykey2").unwrap()))),
			});
			states.insert(StateName::from("ykey2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ykey.sprite"), frame: 1, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("ykey").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("ykey").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("ykey").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc6", template);

	let template = EntityTemplate {
		name: Some("misc7"),
		type_id: Some(EntityTypeId::Thing(39)),
		states: {
			let mut states = HashMap::with_capacity(2);
			states.insert(StateName::from("yskull").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ysku.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("yskull2").unwrap()))),
			});
			states.insert(StateName::from("yskull2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ysku.sprite"), frame: 1, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("yskull").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("yskull").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("yskull").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc7", template);

	let template = EntityTemplate {
		name: Some("misc8"),
		type_id: Some(EntityTypeId::Thing(38)),
		states: {
			let mut states = HashMap::with_capacity(2);
			states.insert(StateName::from("rskull").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("rsku.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("rskull2").unwrap()))),
			});
			states.insert(StateName::from("rskull2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("rsku.sprite"), frame: 1, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("rskull").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("rskull").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("rskull").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc8", template);

	let template = EntityTemplate {
		name: Some("misc9"),
		type_id: Some(EntityTypeId::Thing(40)),
		states: {
			let mut states = HashMap::with_capacity(2);
			states.insert(StateName::from("bskull").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bsku.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("bskull2").unwrap()))),
			});
			states.insert(StateName::from("bskull2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bsku.sprite"), frame: 1, full_bright: true},
				next: Some((10 * FRAME_TIME, Some(StateName::from("bskull").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("bskull").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bskull").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc9", template);

	let template = EntityTemplate {
		name: Some("misc10"),
		type_id: Some(EntityTypeId::Thing(2011)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("stim").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("stim.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("stim").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("stim").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc10", template);

	let template = EntityTemplate {
		name: Some("misc11"),
		type_id: Some(EntityTypeId::Thing(2012)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("medi").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("medi.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("medi").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("medi").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc11", template);

	let template = EntityTemplate {
		name: Some("misc12"),
		type_id: Some(EntityTypeId::Thing(2013)),
		states: {
			let mut states = HashMap::with_capacity(6);
			states.insert(StateName::from("soul").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("soul.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("soul2").unwrap()))),
			});
			states.insert(StateName::from("soul2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("soul.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("soul3").unwrap()))),
			});
			states.insert(StateName::from("soul3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("soul.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("soul4").unwrap()))),
			});
			states.insert(StateName::from("soul4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("soul.sprite"), frame: 3, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("soul5").unwrap()))),
			});
			states.insert(StateName::from("soul5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("soul.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("soul6").unwrap()))),
			});
			states.insert(StateName::from("soul6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("soul.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("soul").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("soul").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("soul").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc12", template);

	let template = EntityTemplate {
		name: Some("inv"),
		type_id: Some(EntityTypeId::Thing(2022)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("pinv").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pinv.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pinv2").unwrap()))),
			});
			states.insert(StateName::from("pinv2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pinv.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pinv3").unwrap()))),
			});
			states.insert(StateName::from("pinv3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pinv.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pinv4").unwrap()))),
			});
			states.insert(StateName::from("pinv4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pinv.sprite"), frame: 3, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pinv").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("pinv").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("pinv").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("inv", template);

	let template = EntityTemplate {
		name: Some("misc13"),
		type_id: Some(EntityTypeId::Thing(2023)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("pstr").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pstr.sprite"), frame: 0, full_bright: true},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("pstr").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("pstr").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc13", template);

	let template = EntityTemplate {
		name: Some("ins"),
		type_id: Some(EntityTypeId::Thing(2024)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("pins").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pins.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pins2").unwrap()))),
			});
			states.insert(StateName::from("pins2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pins.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pins3").unwrap()))),
			});
			states.insert(StateName::from("pins3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pins.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pins4").unwrap()))),
			});
			states.insert(StateName::from("pins4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pins.sprite"), frame: 3, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pins").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("pins").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("pins").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("ins", template);

	let template = EntityTemplate {
		name: Some("misc14"),
		type_id: Some(EntityTypeId::Thing(2025)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("suit").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("suit.sprite"), frame: 0, full_bright: true},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("suit").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("suit").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc14", template);

	let template = EntityTemplate {
		name: Some("misc15"),
		type_id: Some(EntityTypeId::Thing(2026)),
		states: {
			let mut states = HashMap::with_capacity(6);
			states.insert(StateName::from("pmap").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pmap.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pmap2").unwrap()))),
			});
			states.insert(StateName::from("pmap2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pmap.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pmap3").unwrap()))),
			});
			states.insert(StateName::from("pmap3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pmap.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pmap4").unwrap()))),
			});
			states.insert(StateName::from("pmap4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pmap.sprite"), frame: 3, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pmap5").unwrap()))),
			});
			states.insert(StateName::from("pmap5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pmap.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pmap6").unwrap()))),
			});
			states.insert(StateName::from("pmap6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pmap.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pmap").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("pmap").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("pmap").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc15", template);

	let template = EntityTemplate {
		name: Some("misc16"),
		type_id: Some(EntityTypeId::Thing(2045)),
		states: {
			let mut states = HashMap::with_capacity(2);
			states.insert(StateName::from("pvis").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pvis.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pvis2").unwrap()))),
			});
			states.insert(StateName::from("pvis2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pvis.sprite"), frame: 1, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("pvis").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("pvis").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("pvis").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc16", template);

	let template = EntityTemplate {
		name: Some("mega"),
		type_id: Some(EntityTypeId::Thing(83)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("mega").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("mega.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("mega2").unwrap()))),
			});
			states.insert(StateName::from("mega2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("mega.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("mega3").unwrap()))),
			});
			states.insert(StateName::from("mega3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("mega.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("mega4").unwrap()))),
			});
			states.insert(StateName::from("mega4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("mega.sprite"), frame: 3, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("mega").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("mega").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("mega").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("mega", template);

	let template = EntityTemplate {
		name: Some("clip"),
		type_id: Some(EntityTypeId::Thing(2007)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("clip").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("clip.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("clip").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("clip").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("clip", template);

	let template = EntityTemplate {
		name: Some("misc17"),
		type_id: Some(EntityTypeId::Thing(2048)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("ammo").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ammo.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("ammo").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("ammo").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc17", template);

	let template = EntityTemplate {
		name: Some("misc18"),
		type_id: Some(EntityTypeId::Thing(2010)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("rock").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("rock.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("rock").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("rock").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc18", template);

	let template = EntityTemplate {
		name: Some("misc19"),
		type_id: Some(EntityTypeId::Thing(2046)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("brok").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("brok.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("brok").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("brok").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc19", template);

	let template = EntityTemplate {
		name: Some("misc20"),
		type_id: Some(EntityTypeId::Thing(2047)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("cell").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cell.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("cell").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("cell").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc20", template);

	let template = EntityTemplate {
		name: Some("misc21"),
		type_id: Some(EntityTypeId::Thing(17)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("celp").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("celp.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("celp").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("celp").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc21", template);

	let template = EntityTemplate {
		name: Some("misc22"),
		type_id: Some(EntityTypeId::Thing(2008)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("shel").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("shel.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("shel").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("shel").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc22", template);

	let template = EntityTemplate {
		name: Some("misc23"),
		type_id: Some(EntityTypeId::Thing(2049)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("sbox").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sbox.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("sbox").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("sbox").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc23", template);

	let template = EntityTemplate {
		name: Some("misc24"),
		type_id: Some(EntityTypeId::Thing(8)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("bpak").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bpak.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("bpak").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bpak").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc24", template);

	let template = EntityTemplate {
		name: Some("misc25"),
		type_id: Some(EntityTypeId::Thing(2006)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("bfug").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("bfug.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("bfug").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bfug").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc25", template);

	let template = EntityTemplate {
		name: Some("chaingun"),
		type_id: Some(EntityTypeId::Thing(2002)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("mgun").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("mgun.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("mgun").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("mgun").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("chaingun", template);

	let template = EntityTemplate {
		name: Some("misc26"),
		type_id: Some(EntityTypeId::Thing(2005)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("csaw").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("csaw.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("csaw").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("csaw").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc26", template);

	let template = EntityTemplate {
		name: Some("misc27"),
		type_id: Some(EntityTypeId::Thing(2003)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("laun").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("laun.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("laun").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("laun").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc27", template);

	let template = EntityTemplate {
		name: Some("misc28"),
		type_id: Some(EntityTypeId::Thing(2004)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("plas").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("plas.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("plas").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("plas").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc28", template);

	let template = EntityTemplate {
		name: Some("shotgun"),
		type_id: Some(EntityTypeId::Thing(2001)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("shot").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("shot.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("shot").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("shot").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("shotgun", template);

	let template = EntityTemplate {
		name: Some("supershotgun"),
		type_id: Some(EntityTypeId::Thing(82)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("shot2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sgn2.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("shot2").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("shot2").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("supershotgun", template);

	let template = EntityTemplate {
		name: Some("misc29"),
		type_id: Some(EntityTypeId::Thing(85)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("techlamp").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tlmp.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("techlamp2").unwrap()))),
			});
			states.insert(StateName::from("techlamp2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tlmp.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("techlamp3").unwrap()))),
			});
			states.insert(StateName::from("techlamp3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tlmp.sprite"), frame: 2, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("techlamp4").unwrap()))),
			});
			states.insert(StateName::from("techlamp4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tlmp.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("techlamp").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("techlamp").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("techlamp").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc29", template);

	let template = EntityTemplate {
		name: Some("misc30"),
		type_id: Some(EntityTypeId::Thing(86)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("tech2lamp").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tlp2.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("tech2lamp2").unwrap()))),
			});
			states.insert(StateName::from("tech2lamp2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tlp2.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("tech2lamp3").unwrap()))),
			});
			states.insert(StateName::from("tech2lamp3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tlp2.sprite"), frame: 2, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("tech2lamp4").unwrap()))),
			});
			states.insert(StateName::from("tech2lamp4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tlp2.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("tech2lamp").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("tech2lamp").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("tech2lamp").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc30", template);

	let template = EntityTemplate {
		name: Some("misc31"),
		type_id: Some(EntityTypeId::Thing(2028)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("colu").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("colu.sprite"), frame: 0, full_bright: true},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("colu").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("colu").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc31", template);

	let template = EntityTemplate {
		name: Some("misc32"),
		type_id: Some(EntityTypeId::Thing(30)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("tallgrncol").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("col1.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("tallgrncol").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("tallgrncol").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc32", template);

	let template = EntityTemplate {
		name: Some("misc33"),
		type_id: Some(EntityTypeId::Thing(31)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("shrtgrncol").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("col2.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("shrtgrncol").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("shrtgrncol").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc33", template);

	let template = EntityTemplate {
		name: Some("misc34"),
		type_id: Some(EntityTypeId::Thing(32)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("tallredcol").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("col3.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("tallredcol").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("tallredcol").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc34", template);

	let template = EntityTemplate {
		name: Some("misc35"),
		type_id: Some(EntityTypeId::Thing(33)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("shrtredcol").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("col4.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("shrtredcol").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("shrtredcol").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc35", template);

	let template = EntityTemplate {
		name: Some("misc36"),
		type_id: Some(EntityTypeId::Thing(37)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("skullcol").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("col6.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("skullcol").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("skullcol").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc36", template);

	let template = EntityTemplate {
		name: Some("misc37"),
		type_id: Some(EntityTypeId::Thing(36)),
		states: {
			let mut states = HashMap::with_capacity(2);
			states.insert(StateName::from("heartcol").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("col5.sprite"), frame: 0, full_bright: false},
				next: Some((14 * FRAME_TIME, Some(StateName::from("heartcol2").unwrap()))),
			});
			states.insert(StateName::from("heartcol2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("col5.sprite"), frame: 1, full_bright: false},
				next: Some((14 * FRAME_TIME, Some(StateName::from("heartcol").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("heartcol").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("heartcol").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc37", template);

	let template = EntityTemplate {
		name: Some("misc38"),
		type_id: Some(EntityTypeId::Thing(41)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("evileye").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ceye.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("evileye2").unwrap()))),
			});
			states.insert(StateName::from("evileye2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ceye.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("evileye3").unwrap()))),
			});
			states.insert(StateName::from("evileye3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ceye.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("evileye4").unwrap()))),
			});
			states.insert(StateName::from("evileye4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("ceye.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("evileye").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("evileye").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("evileye").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc38", template);

	let template = EntityTemplate {
		name: Some("misc39"),
		type_id: Some(EntityTypeId::Thing(42)),
		states: {
			let mut states = HashMap::with_capacity(3);
			states.insert(StateName::from("floatskull").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fsku.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("floatskull2").unwrap()))),
			});
			states.insert(StateName::from("floatskull2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fsku.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("floatskull3").unwrap()))),
			});
			states.insert(StateName::from("floatskull3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fsku.sprite"), frame: 2, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("floatskull").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("floatskull").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("floatskull").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc39", template);

	let template = EntityTemplate {
		name: Some("misc40"),
		type_id: Some(EntityTypeId::Thing(43)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("torchtree").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tre1.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("torchtree").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("torchtree").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc40", template);

	let template = EntityTemplate {
		name: Some("misc41"),
		type_id: Some(EntityTypeId::Thing(44)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("bluetorch").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tblu.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("bluetorch2").unwrap()))),
			});
			states.insert(StateName::from("bluetorch2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tblu.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("bluetorch3").unwrap()))),
			});
			states.insert(StateName::from("bluetorch3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tblu.sprite"), frame: 2, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("bluetorch4").unwrap()))),
			});
			states.insert(StateName::from("bluetorch4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tblu.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("bluetorch").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("bluetorch").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bluetorch").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc41", template);

	let template = EntityTemplate {
		name: Some("misc42"),
		type_id: Some(EntityTypeId::Thing(45)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("greentorch").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tgrn.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("greentorch2").unwrap()))),
			});
			states.insert(StateName::from("greentorch2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tgrn.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("greentorch3").unwrap()))),
			});
			states.insert(StateName::from("greentorch3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tgrn.sprite"), frame: 2, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("greentorch4").unwrap()))),
			});
			states.insert(StateName::from("greentorch4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tgrn.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("greentorch").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("greentorch").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("greentorch").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc42", template);

	let template = EntityTemplate {
		name: Some("misc43"),
		type_id: Some(EntityTypeId::Thing(46)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("redtorch").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tred.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("redtorch2").unwrap()))),
			});
			states.insert(StateName::from("redtorch2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tred.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("redtorch3").unwrap()))),
			});
			states.insert(StateName::from("redtorch3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tred.sprite"), frame: 2, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("redtorch4").unwrap()))),
			});
			states.insert(StateName::from("redtorch4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tred.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("redtorch").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("redtorch").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("redtorch").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc43", template);

	let template = EntityTemplate {
		name: Some("misc44"),
		type_id: Some(EntityTypeId::Thing(55)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("btorchshrt").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("smbt.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("btorchshrt2").unwrap()))),
			});
			states.insert(StateName::from("btorchshrt2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("smbt.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("btorchshrt3").unwrap()))),
			});
			states.insert(StateName::from("btorchshrt3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("smbt.sprite"), frame: 2, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("btorchshrt4").unwrap()))),
			});
			states.insert(StateName::from("btorchshrt4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("smbt.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("btorchshrt").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("btorchshrt").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("btorchshrt").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc44", template);

	let template = EntityTemplate {
		name: Some("misc45"),
		type_id: Some(EntityTypeId::Thing(56)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("gtorchshrt").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("smgt.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("gtorchshrt2").unwrap()))),
			});
			states.insert(StateName::from("gtorchshrt2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("smgt.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("gtorchshrt3").unwrap()))),
			});
			states.insert(StateName::from("gtorchshrt3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("smgt.sprite"), frame: 2, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("gtorchshrt4").unwrap()))),
			});
			states.insert(StateName::from("gtorchshrt4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("smgt.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("gtorchshrt").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("gtorchshrt").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("gtorchshrt").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc45", template);

	let template = EntityTemplate {
		name: Some("misc46"),
		type_id: Some(EntityTypeId::Thing(57)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("rtorchshrt").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("smrt.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("rtorchshrt2").unwrap()))),
			});
			states.insert(StateName::from("rtorchshrt2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("smrt.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("rtorchshrt3").unwrap()))),
			});
			states.insert(StateName::from("rtorchshrt3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("smrt.sprite"), frame: 2, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("rtorchshrt4").unwrap()))),
			});
			states.insert(StateName::from("rtorchshrt4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("smrt.sprite"), frame: 3, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("rtorchshrt").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("rtorchshrt").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("rtorchshrt").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc46", template);

	let template = EntityTemplate {
		name: Some("misc47"),
		type_id: Some(EntityTypeId::Thing(47)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("stalagtite").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("smit.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("stalagtite").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("stalagtite").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc47", template);

	let template = EntityTemplate {
		name: Some("misc48"),
		type_id: Some(EntityTypeId::Thing(48)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("techpillar").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("elec.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("techpillar").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("techpillar").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc48", template);

	let template = EntityTemplate {
		name: Some("misc49"),
		type_id: Some(EntityTypeId::Thing(34)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("candlestik").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cand.sprite"), frame: 0, full_bright: true},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("candlestik").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("candlestik").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc49", template);

	let template = EntityTemplate {
		name: Some("misc50"),
		type_id: Some(EntityTypeId::Thing(35)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("candelabra").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("cbra.sprite"), frame: 0, full_bright: true},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("candelabra").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("candelabra").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc50", template);

	let template = EntityTemplate {
		name: Some("misc51"),
		type_id: Some(EntityTypeId::Thing(49)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("bloodytwitch").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("bloodytwitch2").unwrap()))),
			});
			states.insert(StateName::from("bloodytwitch2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 1, full_bright: false},
				next: Some((15 * FRAME_TIME, Some(StateName::from("bloodytwitch3").unwrap()))),
			});
			states.insert(StateName::from("bloodytwitch3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 2, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bloodytwitch4").unwrap()))),
			});
			states.insert(StateName::from("bloodytwitch4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 1, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bloodytwitch").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("bloodytwitch").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bloodytwitch").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc51", template);

	let template = EntityTemplate {
		name: Some("misc52"),
		type_id: Some(EntityTypeId::Thing(50)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("meat2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor2.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("meat2").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("meat2").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc52", template);

	let template = EntityTemplate {
		name: Some("misc53"),
		type_id: Some(EntityTypeId::Thing(51)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("meat3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor3.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("meat3").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("meat3").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc53", template);

	let template = EntityTemplate {
		name: Some("misc54"),
		type_id: Some(EntityTypeId::Thing(52)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("meat4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor4.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("meat4").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("meat4").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc54", template);

	let template = EntityTemplate {
		name: Some("misc55"),
		type_id: Some(EntityTypeId::Thing(53)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("meat5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor5.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("meat5").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("meat5").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc55", template);

	let template = EntityTemplate {
		name: Some("misc56"),
		type_id: Some(EntityTypeId::Thing(59)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("meat2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor2.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("meat2").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("meat2").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc56", template);

	let template = EntityTemplate {
		name: Some("misc57"),
		type_id: Some(EntityTypeId::Thing(60)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("meat4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor4.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("meat4").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("meat4").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc57", template);

	let template = EntityTemplate {
		name: Some("misc58"),
		type_id: Some(EntityTypeId::Thing(61)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("meat3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor3.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("meat3").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("meat3").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc58", template);

	let template = EntityTemplate {
		name: Some("misc59"),
		type_id: Some(EntityTypeId::Thing(62)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("meat5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor5.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("meat5").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("meat5").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc59", template);

	let template = EntityTemplate {
		name: Some("misc60"),
		type_id: Some(EntityTypeId::Thing(63)),
		states: {
			let mut states = HashMap::with_capacity(4);
			states.insert(StateName::from("bloodytwitch").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 0, full_bright: false},
				next: Some((10 * FRAME_TIME, Some(StateName::from("bloodytwitch2").unwrap()))),
			});
			states.insert(StateName::from("bloodytwitch2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 1, full_bright: false},
				next: Some((15 * FRAME_TIME, Some(StateName::from("bloodytwitch3").unwrap()))),
			});
			states.insert(StateName::from("bloodytwitch3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 2, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("bloodytwitch4").unwrap()))),
			});
			states.insert(StateName::from("bloodytwitch4").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("gor1.sprite"), frame: 1, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("bloodytwitch").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("bloodytwitch").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bloodytwitch").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc60", template);

	let template = EntityTemplate {
		name: Some("misc61"),
		type_id: Some(EntityTypeId::Thing(22)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("head_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("head.sprite"), frame: 11, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("head_die6").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("head_die6").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc61", template);

	let template = EntityTemplate {
		name: Some("misc62"),
		type_id: Some(EntityTypeId::Thing(15)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("play_die7").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 13, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("play_die7").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("play_die7").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc62", template);

	let template = EntityTemplate {
		name: Some("misc63"),
		type_id: Some(EntityTypeId::Thing(18)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("poss_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("poss.sprite"), frame: 11, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("poss_die5").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("poss_die5").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc63", template);

	let template = EntityTemplate {
		name: Some("misc64"),
		type_id: Some(EntityTypeId::Thing(21)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("sarg_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("sarg.sprite"), frame: 13, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("sarg_die6").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("sarg_die6").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc64", template);

	let template = EntityTemplate {
		name: Some("misc65"),
		type_id: Some(EntityTypeId::Thing(23)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("skull_die6").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("skul.sprite"), frame: 10, full_bright: false},
				next: Some((6 * FRAME_TIME, None)),
			});
			states
		},
		spawn_state: Some(StateName::from("skull_die6").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("skull_die6").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc65", template);

	let template = EntityTemplate {
		name: Some("misc66"),
		type_id: Some(EntityTypeId::Thing(20)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("troo_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("troo.sprite"), frame: 12, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("troo_die5").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("troo_die5").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc66", template);

	let template = EntityTemplate {
		name: Some("misc67"),
		type_id: Some(EntityTypeId::Thing(19)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("spos_die5").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("spos.sprite"), frame: 11, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("spos_die5").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("spos_die5").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc67", template);

	let template = EntityTemplate {
		name: Some("misc68"),
		type_id: Some(EntityTypeId::Thing(10)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("play_xdie9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 22, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("play_xdie9").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("play_xdie9").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc68", template);

	let template = EntityTemplate {
		name: Some("misc69"),
		type_id: Some(EntityTypeId::Thing(12)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("play_xdie9").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("play.sprite"), frame: 22, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("play_xdie9").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("play_xdie9").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc69", template);

	let template = EntityTemplate {
		name: Some("misc70"),
		type_id: Some(EntityTypeId::Thing(28)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("headsonstick").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pol2.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("headsonstick").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("headsonstick").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc70", template);

	let template = EntityTemplate {
		name: Some("misc71"),
		type_id: Some(EntityTypeId::Thing(24)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("gibs").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pol5.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("gibs").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("gibs").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc71", template);

	let template = EntityTemplate {
		name: Some("misc72"),
		type_id: Some(EntityTypeId::Thing(27)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("headonastick").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pol4.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("headonastick").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("headonastick").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc72", template);

	let template = EntityTemplate {
		name: Some("misc73"),
		type_id: Some(EntityTypeId::Thing(29)),
		states: {
			let mut states = HashMap::with_capacity(2);
			states.insert(StateName::from("headcandles").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pol3.sprite"), frame: 0, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("headcandles2").unwrap()))),
			});
			states.insert(StateName::from("headcandles2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pol3.sprite"), frame: 1, full_bright: true},
				next: Some((6 * FRAME_TIME, Some(StateName::from("headcandles").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("headcandles").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("headcandles").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc73", template);

	let template = EntityTemplate {
		name: Some("misc74"),
		type_id: Some(EntityTypeId::Thing(25)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("deadstick").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pol1.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("deadstick").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("deadstick").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc74", template);

	let template = EntityTemplate {
		name: Some("misc75"),
		type_id: Some(EntityTypeId::Thing(26)),
		states: {
			let mut states = HashMap::with_capacity(2);
			states.insert(StateName::from("livestick").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pol6.sprite"), frame: 0, full_bright: false},
				next: Some((6 * FRAME_TIME, Some(StateName::from("livestick2").unwrap()))),
			});
			states.insert(StateName::from("livestick2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pol6.sprite"), frame: 1, full_bright: false},
				next: Some((8 * FRAME_TIME, Some(StateName::from("livestick").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("livestick").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("livestick").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc75", template);

	let template = EntityTemplate {
		name: Some("misc76"),
		type_id: Some(EntityTypeId::Thing(54)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("bigtree").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("tre2.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("bigtree").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bigtree").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc76", template);

	let template = EntityTemplate {
		name: Some("misc77"),
		type_id: Some(EntityTypeId::Thing(70)),
		states: {
			let mut states = HashMap::with_capacity(3);
			states.insert(StateName::from("bbar1").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fcan.sprite"), frame: 0, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("bbar2").unwrap()))),
			});
			states.insert(StateName::from("bbar2").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fcan.sprite"), frame: 1, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("bbar3").unwrap()))),
			});
			states.insert(StateName::from("bbar3").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("fcan.sprite"), frame: 2, full_bright: true},
				next: Some((4 * FRAME_TIME, Some(StateName::from("bbar1").unwrap()))),
			});
			states
		},
		spawn_state: Some(StateName::from("bbar1").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("bbar1").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc77", template);

	let template = EntityTemplate {
		name: Some("misc78"),
		type_id: Some(EntityTypeId::Thing(73)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("hangnoguts").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("hdb1.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("hangnoguts").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("hangnoguts").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc78", template);

	let template = EntityTemplate {
		name: Some("misc79"),
		type_id: Some(EntityTypeId::Thing(74)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("hangbnobrain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("hdb2.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("hangbnobrain").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("hangbnobrain").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc79", template);

	let template = EntityTemplate {
		name: Some("misc80"),
		type_id: Some(EntityTypeId::Thing(75)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("hangtlookdn").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("hdb3.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("hangtlookdn").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("hangtlookdn").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc80", template);

	let template = EntityTemplate {
		name: Some("misc81"),
		type_id: Some(EntityTypeId::Thing(76)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("hangtskull").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("hdb4.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("hangtskull").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("hangtskull").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc81", template);

	let template = EntityTemplate {
		name: Some("misc82"),
		type_id: Some(EntityTypeId::Thing(77)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("hangtlookup").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("hdb5.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("hangtlookup").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("hangtlookup").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc82", template);

	let template = EntityTemplate {
		name: Some("misc83"),
		type_id: Some(EntityTypeId::Thing(78)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("hangtnobrain").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("hdb6.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("hangtnobrain").unwrap()),
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
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("hangtnobrain").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc83", template);

	let template = EntityTemplate {
		name: Some("misc84"),
		type_id: Some(EntityTypeId::Thing(79)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("colongibs").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pob1.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("colongibs").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("pob1.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("colongibs").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc84", template);

	let template = EntityTemplate {
		name: Some("misc85"),
		type_id: Some(EntityTypeId::Thing(80)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("smallpool").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("pob2.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("smallpool").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("pob2.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("smallpool").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc85", template);

	let template = EntityTemplate {
		name: Some("misc86"),
		type_id: Some(EntityTypeId::Thing(81)),
		states: {
			let mut states = HashMap::with_capacity(1);
			states.insert(StateName::from("brainstem").unwrap(), StateDef {
				sprite: SpriteRender {sprite: asset_storage.load("brs1.sprite"), frame: 0, full_bright: false},
				next: None,
			});
			states
		},
		spawn_state: Some(StateName::from("brainstem").unwrap()),
		components: EntityComponents::new()
			.with_component(SpriteRender {
				sprite: asset_storage.load("brs1.sprite"),
				frame: 0,
				full_bright: false,
			})
			.with_component(State {
				current: None,
				next: Some((Timer::new(Duration::default()), Some(StateName::from("brainstem").unwrap()))),
			}),
		.. EntityTemplate::default()
	};
	asset_storage.insert_with_name("misc86", template);
}
