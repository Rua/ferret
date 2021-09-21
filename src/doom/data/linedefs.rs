use crate::{
	common::assets::AssetStorage,
	doom::{
		assets::template::{EntityTemplate, EntityTemplateRefDef},
		data::{FRAME_RATE, FRAME_TIME},
		game::{
			client::{Usable, UseEventDef},
			map::{
				anim::TextureScroll,
				door::{DoorLinedefTouch, DoorParams, DoorState, DoorSwitchUse, DoorUse},
				exit::{ExitMapDef, ExitSwitchUse},
				floor::{FloorLinedefTouch, FloorParams, FloorSwitchUse, FloorTargetHeight},
				plat::{PlatLinedefTouch, PlatParams, PlatSwitchUse, PlatTargetHeight},
				switch::SwitchParams,
				LinedefRefDef,
			},
			physics::{TouchEventDef, Touchable},
		},
	},
};
use legion::World;
use nalgebra::Vector2;
use once_cell::sync::Lazy;
use std::{collections::HashMap, time::Duration};

#[allow(unused_variables)]
#[rustfmt::skip]
pub static LINEDEFS: Lazy<HashMap<&'static str, fn(&mut AssetStorage) -> EntityTemplate>> = Lazy::new(|| {
	let mut linedefs: HashMap<&'static str, fn(&mut AssetStorage) -> EntityTemplate> = HashMap::new();

	// The default, boring, do-nothing linedef
	linedefs.insert("linedef0.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Push doors, open-close
	*/

	// Retrigger, slow
	linedefs.insert("linedef1.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorUse {
					retrigger: true,
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Closed,
						speed: 2.0 * FRAME_RATE,
						wait_time: 150 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					}
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, slow
	// TODO blue key
	linedefs.insert("linedef26.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Closed,
						speed: 2.0 * FRAME_RATE,
						wait_time: 150 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, slow
	// TODO red key
	linedefs.insert("linedef28.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Closed,
						speed: 2.0 * FRAME_RATE,
						wait_time: 150 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, slow
	// TODO yellow key
	linedefs.insert("linedef27.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Closed,
						speed: 2.0 * FRAME_RATE,
						wait_time: 150 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast
	linedefs.insert("linedef117.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				DoorUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Closed,
						speed: 8.0 * FRAME_RATE,
						wait_time: 150 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Push doors, open only
	*/

	// No retrigger, slow
	linedefs.insert("linedef31.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 2.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow
	// TODO blue key
	linedefs.insert("linedef32.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 2.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow
	// TODO red key
	linedefs.insert("linedef33.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 2.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow
	// TODO yellow key
	linedefs.insert("linedef34.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 2.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast
	linedefs.insert("linedef118.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Switch doors, open-close
	*/

	// Retrigger, slow
	linedefs.insert("linedef63.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Closed,
						speed: 2.0 * FRAME_RATE,
						wait_time: 150 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast
	linedefs.insert("linedef114.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Closed,
						speed: 8.0 * FRAME_RATE,
						wait_time: 150 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow
	linedefs.insert("linedef29.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Closed,
						speed: 2.0 * FRAME_RATE,
						wait_time: 150 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast
	linedefs.insert("linedef111.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Closed,
						speed: 8.0 * FRAME_RATE,
						wait_time: 150 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Switch doors, open only
	*/

	// Retrigger, slow
	linedefs.insert("linedef61.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 2.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast
	linedefs.insert("linedef115.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast
	// TODO blue key
	linedefs.insert("linedef99.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast
	// TODO red key
	linedefs.insert("linedef134.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast
	// TODO yellow key
	linedefs.insert("linedef136.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow
	linedefs.insert("linedef103.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 2.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast
	linedefs.insert("linedef112.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast
	// TODO blue key
	linedefs.insert("linedef133.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast
	// TODO red key
	linedefs.insert("linedef135.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast
	// TODO yellow key
	linedefs.insert("linedef137.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Switch doors, close only
	*/

	// Retrigger, slow
	linedefs.insert("linedef42.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Open,
						end_state: DoorState::Closed,
						speed: 2.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast
	linedefs.insert("linedef116.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Open,
						end_state: DoorState::Closed,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow
	linedefs.insert("linedef113.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Open,
						end_state: DoorState::Closed,
						speed: 2.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast
	linedefs.insert("linedef50.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				DoorSwitchUse {
					params: DoorParams {
						start_state: DoorState::Open,
						end_state: DoorState::Closed,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Linedef touch doors, open-close
	*/

	// Retrigger, slow
	linedefs.insert("linedef90.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Closed,
						speed: 2.0 * FRAME_RATE,
						wait_time: 150 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast
	linedefs.insert("linedef105.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Closed,
						speed: 8.0 * FRAME_RATE,
						wait_time: 150 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow
	linedefs.insert("linedef4.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Closed,
						speed: 2.0 * FRAME_RATE,
						wait_time: 150 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast
	linedefs.insert("linedef108.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Closed,
						speed: 8.0 * FRAME_RATE,
						wait_time: 150 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Linedef touch doors, open only
	*/

	// Retrigger, slow
	linedefs.insert("linedef86.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 2.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast
	linedefs.insert("linedef106.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow
	linedefs.insert("linedef2.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 2.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast
	linedefs.insert("linedef109.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Closed,
						end_state: DoorState::Open,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Linedef touch doors, close only
	*/

	// Retrigger, slow
	linedefs.insert("linedef75.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Open,
						end_state: DoorState::Closed,
						speed: 2.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast
	linedefs.insert("linedef107.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Open,
						end_state: DoorState::Closed,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow
	linedefs.insert("linedef3.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Open,
						end_state: DoorState::Closed,
						speed: 2.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast
	linedefs.insert("linedef110.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Open,
						end_state: DoorState::Closed,
						speed: 8.0 * FRAME_RATE,
						wait_time: Duration::ZERO,
						can_reverse: false,

						open_sound: Some(asset_storage.load("dsbdopn.sound")),
						close_sound: Some(asset_storage.load("dsbdcls.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Linedef touch doors, close-open
	*/

	// Retrigger, slow
	linedefs.insert("linedef76.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Open,
						end_state: DoorState::Open,
						speed: 2.0 * FRAME_RATE,
						wait_time: 30 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow
	linedefs.insert("linedef16.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				DoorLinedefTouch {
					params: DoorParams {
						start_state: DoorState::Open,
						end_state: DoorState::Open,
						speed: 2.0 * FRAME_RATE,
						wait_time: 30 * FRAME_TIME,
						can_reverse: true,

						open_sound: Some(asset_storage.load("dsdoropn.sound")),
						close_sound: Some(asset_storage.load("dsdorcls.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Switch floors, current height
	*/

	// No retrigger, slow, offset 512
	linedefs.insert("linedef140.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::Current,
						target_height_offset: 512.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Switch floors, lowest neighbour floor
	*/

	// Retrigger, slow, offset 0
	linedefs.insert("linedef60.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow, offset 0
	linedefs.insert("linedef23.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Switch floors, lowest neighbour floor above
	*/

	// Retrigger, slow, offset 0
	linedefs.insert("linedef69.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow, offset 0
	linedefs.insert("linedef18.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast, offset 0
	linedefs.insert("linedef132.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 4.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast, offset 0
	linedefs.insert("linedef131.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 4.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Switch floors, lowest neighbour ceiling
	*/

	// Retrigger, slow, offset 0
	linedefs.insert("linedef64.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow, offset 0
	linedefs.insert("linedef101.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, slow, offset -8
	// TODO crush
	linedefs.insert("linedef65.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
						target_height_offset: -8.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow, offset -8
	// TODO crush
	linedefs.insert("linedef55.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
						target_height_offset: -8.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Switch floors, highest neighbour floor
	*/

	// Retrigger, slow, offset 0
	linedefs.insert("linedef45.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::HighestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow, offset 0
	linedefs.insert("linedef102.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::HighestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast, offset +8
	linedefs.insert("linedef70.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 4.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::HighestNeighbourFloor,
						target_height_offset: 8.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast, offset +8
	linedefs.insert("linedef71.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				FloorSwitchUse {
					params: FloorParams {
						speed: 4.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::HighestNeighbourFloor,
						target_height_offset: 8.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Linedef touch floors, current height
	*/

	// Retrigger, slow, offset 24
	linedefs.insert("linedef92.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::Current,
						target_height_offset: 24.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow, offset 24
	linedefs.insert("linedef58.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::Current,
						target_height_offset: 24.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, slow, offset 24
	// TODO change type
	linedefs.insert("linedef93.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::Current,
						target_height_offset: 24.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow, offset 24
	// TODO change type
	linedefs.insert("linedef59.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::Current,
						target_height_offset: 24.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Linedef touch floors, lowest neighbour floor
	*/

	// Retrigger, slow, offset 0
	linedefs.insert("linedef82.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow, offset 0
	linedefs.insert("linedef38.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, slow, offset 0
	// TODO type change
	linedefs.insert("linedef84.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow, offset 0
	// TODO type change
	linedefs.insert("linedef37.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Linedef touch floors, lowest neighbour floor above
	*/

	// Retrigger, slow, offset 0
	linedefs.insert("linedef128.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow, offset 0
	linedefs.insert("linedef119.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast, offset 0
	linedefs.insert("linedef129.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 4.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast, offset 0
	linedefs.insert("linedef130.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 4.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Linedef touch floors, lowest neighbour ceiling
	*/

	// Retrigger, slow, offset 0
	linedefs.insert("linedef91.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow, offset 0
	linedefs.insert("linedef5.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, slow, offset -8
	// TODO crush
	linedefs.insert("linedef94.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
						target_height_offset: -8.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow, offset -8
	// TODO crush
	linedefs.insert("linedef56.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
						target_height_offset: -8.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Linedef touch floors, highest neighbour floor
	*/

	// Retrigger, slow, offset 0
	linedefs.insert("linedef83.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::HighestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow, offset 0
	linedefs.insert("linedef19.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::HighestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast, offset +8
	linedefs.insert("linedef98.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 4.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::HighestNeighbourFloor,
						target_height_offset: 8.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast, offset +8
	linedefs.insert("linedef36.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				FloorLinedefTouch {
					params: FloorParams {
						speed: 4.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::HighestNeighbourFloor,
						target_height_offset: 8.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Switch plats, current - lowest neighbour floor
	*/

	// Retrigger, slow
	linedefs.insert("linedef62.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				PlatSwitchUse {
					params: PlatParams {
						speed: 4.0 * FRAME_RATE,
						wait_time: 105 * FRAME_TIME,
						can_reverse: true,

						start_sound: Some(asset_storage.load("dspstart.sound")),
						move_sound: None,
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),

						low_height_base: PlatTargetHeight::LowestNeighbourFloor,
						low_height_offset: 0.0,
						high_height_base: PlatTargetHeight::Current,
						high_height_offset: 0.0,
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast
	linedefs.insert("linedef123.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				PlatSwitchUse {
					params: PlatParams {
						speed: 8.0 * FRAME_RATE,
						wait_time: 105 * FRAME_TIME,
						can_reverse: true,

						start_sound: Some(asset_storage.load("dspstart.sound")),
						move_sound: None,
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),

						low_height_base: PlatTargetHeight::LowestNeighbourFloor,
						low_height_offset: 0.0,
						high_height_base: PlatTargetHeight::Current,
						high_height_offset: 0.0,
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: Some(35 * FRAME_TIME),
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow
	linedefs.insert("linedef21.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				PlatSwitchUse {
					params: PlatParams {
						speed: 4.0 * FRAME_RATE,
						wait_time: 105 * FRAME_TIME,
						can_reverse: true,

						start_sound: Some(asset_storage.load("dspstart.sound")),
						move_sound: None,
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),

						low_height_base: PlatTargetHeight::LowestNeighbourFloor,
						low_height_offset: 0.0,
						high_height_base: PlatTargetHeight::Current,
						high_height_offset: 0.0,
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast
	linedefs.insert("linedef122.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				PlatSwitchUse {
					params: PlatParams {
						speed: 8.0 * FRAME_RATE,
						wait_time: 105 * FRAME_TIME,
						can_reverse: true,

						start_sound: Some(asset_storage.load("dspstart.sound")),
						move_sound: None,
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),

						low_height_base: PlatTargetHeight::LowestNeighbourFloor,
						low_height_offset: 0.0,
						high_height_base: PlatTargetHeight::Current,
						high_height_offset: 0.0,
					},
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchn.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Linedef touch plats, current - lowest neighbour floor
	*/

	// Retrigger, slow
	linedefs.insert("linedef88.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				PlatLinedefTouch {
					params: PlatParams {
						speed: 4.0 * FRAME_RATE,
						wait_time: 105 * FRAME_TIME,
						can_reverse: true,

						start_sound: Some(asset_storage.load("dspstart.sound")),
						move_sound: None,
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),

						low_height_base: PlatTargetHeight::LowestNeighbourFloor,
						low_height_offset: 0.0,
						high_height_base: PlatTargetHeight::Current,
						high_height_offset: 0.0,
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Retrigger, fast
	linedefs.insert("linedef120.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				PlatLinedefTouch {
					params: PlatParams {
						speed: 8.0 * FRAME_RATE,
						wait_time: 105 * FRAME_TIME,
						can_reverse: true,

						start_sound: Some(asset_storage.load("dspstart.sound")),
						move_sound: None,
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),

						low_height_base: PlatTargetHeight::LowestNeighbourFloor,
						low_height_offset: 0.0,
						high_height_base: PlatTargetHeight::Current,
						high_height_offset: 0.0,
					},
					retrigger: true,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, slow
	linedefs.insert("linedef10.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				PlatLinedefTouch {
					params: PlatParams {
						speed: 4.0 * FRAME_RATE,
						wait_time: 105 * FRAME_TIME,
						can_reverse: true,

						start_sound: Some(asset_storage.load("dspstart.sound")),
						move_sound: None,
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),

						low_height_base: PlatTargetHeight::LowestNeighbourFloor,
						low_height_offset: 0.0,
						high_height_base: PlatTargetHeight::Current,
						high_height_offset: 0.0,
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// No retrigger, fast
	linedefs.insert("linedef121.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				Touchable,
			));
			world
		},
		touch: {
			let mut world = World::default();
			world.push((
				TouchEventDef,
				PlatLinedefTouch {
					params: PlatParams {
						speed: 8.0 * FRAME_RATE,
						wait_time: 105 * FRAME_TIME,
						can_reverse: true,

						start_sound: Some(asset_storage.load("dspstart.sound")),
						move_sound: None,
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),

						low_height_base: PlatTargetHeight::LowestNeighbourFloor,
						low_height_offset: 0.0,
						high_height_base: PlatTargetHeight::Current,
						high_height_offset: 0.0,
					},
					retrigger: false,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Exit triggers
	*/

	linedefs.insert("linedef11.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				ExitMapDef { secret: false },
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				ExitSwitchUse {
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchx.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef51.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				ExitMapDef { secret: true },
				Usable,
			));
			world
		},
		r#use: {
			let mut world = World::default();
			world.push((
				UseEventDef,
				ExitSwitchUse {
					switch_params: SwitchParams {
						sound: Some(asset_storage.load("dsswtchx.sound")),
						retrigger_time: None,
					},
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	/*
		Other
	*/

	linedefs.insert("linedef6.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef7.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef8.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef9.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef12.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef13.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef14.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef15.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef17.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef20.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef22.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef24.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef25.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef30.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef35.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef39.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef40.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef41.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef43.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef44.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef46.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef47.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef48.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
				TextureScroll {
					speed: Vector2::new(35.0, 0.0),
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef49.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef52.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef53.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef54.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef57.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef66.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef67.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef68.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef72.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef73.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef74.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef77.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef79.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef80.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef81.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef87.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef89.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef95.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef96.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef97.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef100.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef104.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef124.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef125.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef126.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef127.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef138.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef139.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs.insert("linedef141.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				LinedefRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	linedefs
});
