use crate::{
	common::assets::AssetStorage,
	doom::{
		client::{Usable, UseEventDef},
		data::{FRAME_RATE, FRAME_TIME},
		door::{DoorLinedefTouch, DoorParams, DoorState, DoorSwitchUse, DoorUse},
		exit::{ExitSwitchUse, NextMapDef},
		floor::{FloorLinedefTouch, FloorParams, FloorSwitchUse, FloorTargetHeight},
		physics::{TouchEventDef, Touchable},
		plat::{PlatLinedefTouch, PlatParams, PlatSwitchUse, PlatTargetHeight},
		switch::SwitchParams,
		template::{EntityTemplate, EntityTypeId},
		texture::TextureScroll,
	},
};
use legion::World;
use nalgebra::Vector2;
use std::time::Duration;

#[rustfmt::skip]
pub fn load(asset_storage: &mut AssetStorage) {
	/*
		Push doors, open-close
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(1)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef1", template);

	// Retrigger, slow
	// TODO blue key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(26)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef26", template);

	// Retrigger, slow
	// TODO red key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(28)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef28", template);

	// Retrigger, slow
	// TODO yellow key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(27)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef27", template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(117)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef117", template);

	/*
		Push doors, open only
	*/

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(31)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef31", template);

	// No retrigger, slow
	// TODO blue key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(32)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef32", template);

	// No retrigger, slow
	// TODO red key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(33)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef33", template);

	// No retrigger, slow
	// TODO yellow key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(34)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef34", template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(118)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef118", template);

	/*
		Switch doors, open-close
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(63)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef63", template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(114)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef114", template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(29)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef29", template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(111)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef111", template);

	/*
		Switch doors, open only
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(61)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef61", template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(115)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef115", template);

	// Retrigger, fast
	// TODO blue key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(99)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef99", template);

	// Retrigger, fast
	// TODO red key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(134)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef134", template);

	// Retrigger, fast
	// TODO yellow key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(136)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef136", template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(103)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef103", template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(112)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef112", template);

	// No retrigger, fast
	// TODO blue key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(133)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef133", template);

	// No retrigger, fast
	// TODO red key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(135)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef135", template);

	// No retrigger, fast
	// TODO yellow key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(137)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef137", template);

	/*
		Switch doors, close only
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(42)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef42", template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(116)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef116", template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(113)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef113", template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(50)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef50", template);

	/*
		Linedef touch doors, open-close
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(90)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef90", template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(105)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef105", template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(4)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef4", template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(108)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef108", template);

	/*
		Linedef touch doors, open only
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(86)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef86", template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(106)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef106", template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(2)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef2", template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(109)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef109", template);

	/*
		Linedef touch doors, close only
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(75)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef75", template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(107)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef107", template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(3)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef3", template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(110)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
						wait_time: Duration::default(),
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
	};
	asset_storage.insert("linedef110", template);

	/*
		Linedef touch doors, close-open
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(76)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef76", template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(16)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef16", template);

	/*
		Switch floors, current height
	*/

	// No retrigger, slow, offset 512
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(140)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef140", template);

	/*
		Switch floors, lowest neighbour floor
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(60)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef60", template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(23)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef23", template);

	/*
		Switch floors, lowest neighbour floor above
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(69)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef69", template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(18)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef18", template);

	// Retrigger, fast, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(132)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef132", template);

	// No retrigger, fast, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(131)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef131", template);

	/*
		Switch floors, lowest neighbour ceiling
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(64)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef64", template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(101)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef101", template);

	// Retrigger, slow, offset -8
	// TODO crush
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(65)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef65", template);

	// No retrigger, slow, offset -8
	// TODO crush
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(55)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef55", template);

	/*
		Switch floors, highest neighbour floor
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(45)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef45", template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(102)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef102", template);

	// Retrigger, fast, offset +8
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(70)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef70", template);

	// No retrigger, fast, offset +8
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(71)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef71", template);

	/*
		Linedef touch floors, current height
	*/

	// Retrigger, slow, offset 24
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(92)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef92", template);

	// No retrigger, slow, offset 24
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(58)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef58", template);

	// Retrigger, slow, offset 24
	// TODO change type
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(93)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef93", template);

	// No retrigger, slow, offset 24
	// TODO change type
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(59)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef59", template);

	/*
		Linedef touch floors, lowest neighbour floor
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(82)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef82", template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(38)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef38", template);

	// Retrigger, slow, offset 0
	// TODO type change
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(84)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef84", template);

	// No retrigger, slow, offset 0
	// TODO type change
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(37)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef37", template);

	/*
		Linedef touch floors, lowest neighbour floor above
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(128)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef128", template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(119)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef119", template);

	// Retrigger, fast, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(129)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef129", template);

	// No retrigger, fast, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(130)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef130", template);

	/*
		Linedef touch floors, lowest neighbour ceiling
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(91)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef91", template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(5)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef5", template);

	// Retrigger, slow, offset -8
	// TODO crush
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(94)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef94", template);

	// No retrigger, slow, offset -8
	// TODO crush
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(56)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef56", template);

	/*
		Linedef touch floors, highest neighbour floor
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(83)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef83", template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(19)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef19", template);

	// Retrigger, fast, offset +8
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(98)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef98", template);

	// No retrigger, fast, offset +8
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(36)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef36", template);

	/*
		Switch plats, current - lowest neighbour floor
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(62)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef62", template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(123)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef123", template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(21)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef21", template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(122)),
		world: {
			let mut world = World::default();
			world.push((Usable,));
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
	};
	asset_storage.insert("linedef122", template);

	/*
		Linedef touch plats, current - lowest neighbour floor
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(88)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef88", template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(120)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef120", template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(10)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef10", template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(121)),
		world: {
			let mut world = World::default();
			world.push((Touchable,));
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
	};
	asset_storage.insert("linedef121", template);

	/*
		Other
	*/

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(11)),
		world: {
			let mut world = World::default();
			world.push((
				NextMapDef,
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
	};
	asset_storage.insert("linedef11", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(6)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef6", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(7)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef7", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(8)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef8", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(9)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef9", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(12)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef12", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(13)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef13", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(14)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef14", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(15)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef15", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(17)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef17", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(20)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef20", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(22)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef22", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(24)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef24", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(25)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef25", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(30)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef30", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(35)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef35", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(39)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef39", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(40)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef40", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(41)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef41", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(43)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef43", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(44)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef44", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(46)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef46", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(47)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef47", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(48)),
		world: {
			let mut world = World::default();
			world.push((
				TextureScroll {
					speed: Vector2::new(35.0, 0.0),
				},
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef48", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(49)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef49", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(51)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef51", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(52)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef52", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(53)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef53", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(54)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef54", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(57)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef57", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(66)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef66", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(67)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef67", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(68)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef68", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(72)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef72", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(73)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef73", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(74)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef74", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(77)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef77", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(79)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef79", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(80)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef80", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(81)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef81", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(87)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef87", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(89)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef89", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(95)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef95", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(96)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef96", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(97)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef97", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(100)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef100", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(104)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef104", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(124)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef124", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(125)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef125", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(126)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef126", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(127)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef127", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(138)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef138", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(139)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef139", template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(141)),
		.. EntityTemplate::default()
	};
	asset_storage.insert("linedef141", template);
}
