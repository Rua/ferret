use crate::{
	common::assets::AssetStorage,
	doom::{
		client::UseAction,
		data::{FRAME_RATE, FRAME_TIME},
		door::{DoorParams, DoorState, DoorSwitchUse, DoorTouch, DoorUse},
		floor::{FloorParams, FloorSwitchUse, FloorTargetHeight, FloorTouch},
		physics::TouchAction,
		plat::{PlatParams, PlatSwitchUse, PlatTargetHeight, PlatTouch},
		switch::SwitchParams,
		template::{EntityTemplate, EntityTypeId},
		texture::TextureScroll,
	},
};
use legion::{systems::ResourceSet, Resources, World, Write};
use nalgebra::Vector2;
use std::time::Duration;

#[rustfmt::skip]
pub fn load(resources: &mut Resources) {
	let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);

	/*
		Push doors, open-close
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(1)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorUse(DoorUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, slow
	// TODO blue key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(26)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorUse(DoorUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, slow
	// TODO red key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(28)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorUse(DoorUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, slow
	// TODO yellow key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(27)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorUse(DoorUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(117)),
		world: {
			let mut world = World::default();
			world.push((
			UseAction::DoorUse(DoorUse {
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
			}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Push doors, open only
	*/

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(31)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorUse(DoorUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow
	// TODO blue key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(32)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorUse(DoorUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow
	// TODO red key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(33)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorUse(DoorUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow
	// TODO yellow key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(34)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorUse(DoorUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(118)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorUse(DoorUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Switch doors, open-close
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(63)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(114)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(29)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(111)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Switch doors, open only
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(61)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(115)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast
	// TODO blue key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(99)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast
	// TODO red key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(134)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast
	// TODO yellow key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(136)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(103)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(112)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast
	// TODO blue key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(133)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast
	// TODO red key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(135)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast
	// TODO yellow key
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(137)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Switch doors, close only
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(42)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(116)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(113)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(50)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::DoorSwitchUse(DoorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Linedef touch doors, open-close
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(90)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(105)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(4)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(108)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Linedef touch doors, open only
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(86)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(106)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(2)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(109)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Linedef touch doors, close only
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(75)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(107)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(3)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(110)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Linedef touch doors, close-open
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(76)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(16)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::DoorTouch(DoorTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Switch floors, current height
	*/

	// No retrigger, slow, offset 512
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(140)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Switch floors, lowest neighbour floor
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(60)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(23)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Switch floors, lowest neighbour floor above
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(69)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(18)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(132)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(131)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Switch floors, lowest neighbour ceiling
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(64)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(101)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, slow, offset -8
	// TODO crush
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(65)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow, offset -8
	// TODO crush
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(55)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Switch floors, highest neighbour floor
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(45)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(102)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast, offset +8
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(70)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast, offset +8
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(71)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::FloorSwitchUse(FloorSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Linedef touch floors, current height
	*/

	// Retrigger, slow, offset 24
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(92)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::Current,
						target_height_offset: 24.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow, offset 24
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(58)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::Current,
						target_height_offset: 24.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, slow, offset 24
	// TODO change type
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(93)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::Current,
						target_height_offset: 24.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow, offset 24
	// TODO change type
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(59)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::Current,
						target_height_offset: 24.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Linedef touch floors, lowest neighbour floor
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(82)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(38)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, slow, offset 0
	// TODO type change
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(84)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow, offset 0
	// TODO type change
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(37)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Linedef touch floors, lowest neighbour floor above
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(128)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(119)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(129)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 4.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(130)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 4.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Linedef touch floors, lowest neighbour ceiling
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(91)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(5)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, slow, offset -8
	// TODO crush
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(94)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
						target_height_offset: -8.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow, offset -8
	// TODO crush
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(56)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
						target_height_offset: -8.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Linedef touch floors, highest neighbour floor
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(83)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::HighestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(19)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 1.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::HighestNeighbourFloor,
						target_height_offset: 0.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast, offset +8
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(98)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 4.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::HighestNeighbourFloor,
						target_height_offset: 8.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: true,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast, offset +8
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(36)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::FloorTouch(FloorTouch {
					params: FloorParams {
						speed: 4.0 * FRAME_RATE,
						target_height_base: FloorTargetHeight::HighestNeighbourFloor,
						target_height_offset: 8.0,
						move_sound: Some(asset_storage.load("dsstnmov.sound")),
						move_sound_time: 8 * FRAME_TIME,
						finish_sound: Some(asset_storage.load("dspstop.sound")),
					},
					retrigger: false,
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Switch plats, current - lowest neighbour floor
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(62)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::PlatSwitchUse(PlatSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(123)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::PlatSwitchUse(PlatSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(21)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::PlatSwitchUse(PlatSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(122)),
		world: {
			let mut world = World::default();
			world.push((
				UseAction::PlatSwitchUse(PlatSwitchUse {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Linedef touch plats, current - lowest neighbour floor
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(88)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::PlatTouch(PlatTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(120)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::PlatTouch(PlatTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, slow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(10)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::PlatTouch(PlatTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// No retrigger, fast
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(121)),
		world: {
			let mut world = World::default();
			world.push((
				TouchAction::PlatTouch(PlatTouch {
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
				}),
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	/*
		Other
	*/

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(6)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(7)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(8)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(9)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(11)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(12)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(13)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(14)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(15)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(17)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(20)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(22)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(24)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(25)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(30)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(35)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(39)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(40)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(41)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(43)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(44)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(46)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(47)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

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
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(49)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(51)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(52)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(53)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(54)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(57)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(66)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(67)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(68)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(72)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(73)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(74)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(77)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(79)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(80)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(81)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(87)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(89)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(95)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(96)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(97)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(100)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(104)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(124)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(125)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(126)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(127)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(138)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(139)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Linedef(141)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);
}
