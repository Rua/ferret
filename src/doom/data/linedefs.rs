use crate::{
	common::{assets::AssetStorage, component::EntityComponents},
	doom::{
		client::UseAction,
		data::{FRAME_RATE, FRAME_TIME},
		door::{DoorParams, DoorState, DoorSwitchUse, DoorTouch, DoorUse},
		entitytemplate::{EntityTemplate, EntityTypeId},
		floor::{FloorParams, FloorSwitchUse, FloorTargetHeight, FloorTouch},
		physics::TouchAction,
		plat::{PlatParams, PlatSwitchUse, PlatTargetHeight, PlatTouch},
		switch::SwitchParams,
		texture::TextureScroll,
	},
};
use legion::prelude::{ResourceSet, Resources, Write};
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
		name: None,
		type_id: Some(EntityTypeId::Linedef(1)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorUse(DoorUse {
				retrigger: true,
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, slow
	// TODO blue key
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(26)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, slow
	// TODO red key
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(28)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, slow
	// TODO yellow key
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(27)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(117)),
		components: EntityComponents::new()
		.with_component(UseAction::DoorUse(DoorUse {
			params: DoorParams {
				start_state: DoorState::Closed,
				end_state: DoorState::Closed,
				speed: 8.0 * FRAME_RATE,
				wait_time: 150 * FRAME_TIME,
				can_reverse: true,

				open_sound: Some(asset_storage.load("DSBDOPN")),
				close_sound: Some(asset_storage.load("DSBDCLS")),
			},
			retrigger: true,
		})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Push doors, open only
	*/

	// No retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(31)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow
	// TODO blue key
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(32)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow
	// TODO red key
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(33)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow
	// TODO yellow key
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(34)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(118)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Switch doors, open-close
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(63)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(114)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(29)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(111)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Switch doors, open only
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(61)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(115)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast
	// TODO blue key
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(99)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast
	// TODO red key
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(134)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast
	// TODO yellow key
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(136)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(103)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(112)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast
	// TODO blue key
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(133)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast
	// TODO red key
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(135)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast
	// TODO yellow key
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(137)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Switch doors, close only
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(42)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(116)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(113)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(50)),
		components: EntityComponents::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Linedef touch doors, open-close
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(90)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(105)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(4)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(108)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Linedef touch doors, open only
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(86)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(106)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(2)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(109)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Linedef touch doors, close only
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(75)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(107)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(3)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(110)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN")),
					close_sound: Some(asset_storage.load("DSBDCLS")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Linedef touch doors, close-open
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(76)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: 30 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(16)),
		components: EntityComponents::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: 30 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN")),
					close_sound: Some(asset_storage.load("DSDORCLS")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Switch floors, current height
	*/

	// No retrigger, slow, offset 512
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(140)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::Current,
					target_height_offset: 512.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Switch floors, lowest neighbour floor
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(60)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(23)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Switch floors, lowest neighbour floor above
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(69)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(18)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(132)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(131)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Switch floors, lowest neighbour ceiling
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(64)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(101)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, slow, offset -8
	// TODO crush
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(65)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: -8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow, offset -8
	// TODO crush
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(55)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: -8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Switch floors, highest neighbour floor
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(45)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(102)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast, offset +8
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(70)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast, offset +8
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(71)),
		components: EntityComponents::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Linedef touch floors, current height
	*/

	// Retrigger, slow, offset 24
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(92)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::Current,
					target_height_offset: 24.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow, offset 24
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(58)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::Current,
					target_height_offset: 24.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, slow, offset 24
	// TODO change type
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(93)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::Current,
					target_height_offset: 24.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow, offset 24
	// TODO change type
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(59)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::Current,
					target_height_offset: 24.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Linedef touch floors, lowest neighbour floor
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(82)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(38)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, slow, offset 0
	// TODO type change
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(84)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow, offset 0
	// TODO type change
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(37)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Linedef touch floors, lowest neighbour floor above
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(128)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(119)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(129)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(130)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Linedef touch floors, lowest neighbour ceiling
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(91)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(5)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, slow, offset -8
	// TODO crush
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(94)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: -8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow, offset -8
	// TODO crush
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(56)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: -8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Linedef touch floors, highest neighbour floor
	*/

	// Retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(83)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow, offset 0
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(19)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast, offset +8
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(98)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast, offset +8
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(36)),
		components: EntityComponents::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV")),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Switch plats, current - lowest neighbour floor
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(62)),
		components: EntityComponents::new()
			.with_component(UseAction::PlatSwitchUse(PlatSwitchUse {
				params: PlatParams {
					speed: 4.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART")),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(123)),
		components: EntityComponents::new()
			.with_component(UseAction::PlatSwitchUse(PlatSwitchUse {
				params: PlatParams {
					speed: 8.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART")),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(21)),
		components: EntityComponents::new()
			.with_component(UseAction::PlatSwitchUse(PlatSwitchUse {
				params: PlatParams {
					speed: 4.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART")),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(122)),
		components: EntityComponents::new()
			.with_component(UseAction::PlatSwitchUse(PlatSwitchUse {
				params: PlatParams {
					speed: 8.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART")),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN")),
					retrigger_time: None,
				},
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Linedef touch plats, current - lowest neighbour floor
	*/

	// Retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(88)),
		components: EntityComponents::new()
			.with_component(TouchAction::PlatTouch(PlatTouch {
				params: PlatParams {
					speed: 4.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART")),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(120)),
		components: EntityComponents::new()
			.with_component(TouchAction::PlatTouch(PlatTouch {
				params: PlatParams {
					speed: 8.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART")),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				retrigger: true,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, slow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(10)),
		components: EntityComponents::new()
			.with_component(TouchAction::PlatTouch(PlatTouch {
				params: PlatParams {
					speed: 4.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART")),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	// No retrigger, fast
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(121)),
		components: EntityComponents::new()
			.with_component(TouchAction::PlatTouch(PlatTouch {
				params: PlatParams {
					speed: 8.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART")),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP")),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				retrigger: false,
			})),
	};
	asset_storage.insert::<EntityTemplate>(template);

	/*
		Other
	*/

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(6)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(7)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(8)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(9)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(11)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(12)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(13)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(14)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(15)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(17)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(20)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(22)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(24)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(25)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(30)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(35)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(39)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(40)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(41)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(43)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(44)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(46)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(47)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Linedef(48)),
		components: EntityComponents::new()
			.with_component(TextureScroll {
				speed: Vector2::new(35.0, 0.0),
			}),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(49)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(51)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(52)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(53)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(54)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(57)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(66)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(67)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(68)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(72)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(73)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(74)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(77)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(79)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(80)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(81)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(87)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(89)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(95)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(96)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(97)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(100)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(104)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(124)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(125)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(126)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(127)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(138)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(139)),
	};
	asset_storage.insert::<EntityTemplate>(template);

	let template = EntityTemplate {
		name: None,
		components: EntityComponents::new(),
		type_id: Some(EntityTypeId::Linedef(141)),
	};
	asset_storage.insert::<EntityTemplate>(template);
}
