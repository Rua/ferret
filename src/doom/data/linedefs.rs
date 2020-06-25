use crate::{
	assets::{AssetHandle, AssetStorage},
	component::EntityTemplate,
	doom::{
		client::UseAction,
		data::{FRAME_RATE, FRAME_TIME},
		door::{DoorParams, DoorState, DoorSwitchUse, DoorTouch, DoorUse},
		floor::{FloorParams, FloorSwitchUse, FloorTargetHeight, FloorTouch},
		physics::TouchAction,
		plat::{PlatParams, PlatSwitchUse, PlatTargetHeight, PlatTouch},
		switch::SwitchParams,
		texture::TextureScroll,
		wad::WadLoader,
	},
};
use fnv::FnvHashMap;
use legion::prelude::{ResourceSet, Resources, Write};
use nalgebra::Vector2;
use std::time::Duration;

pub struct LinedefTypes {
	pub doomednums: FnvHashMap<u16, AssetHandle<EntityTemplate>>,
}

impl LinedefTypes {
	#[rustfmt::skip]
	pub fn new(resources: &mut Resources) -> LinedefTypes {
        let (mut asset_storage, mut loader) = <(
			Write<AssetStorage>,
			Write<WadLoader>,
		)>::fetch_mut(resources);

        let mut doomednums = FnvHashMap::default();

		/*
			Push doors, open-close
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				retrigger: true,
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
			}));
        let handle = asset_storage.insert(template);
        doomednums.insert(1, handle);

		// Retrigger, slow
		// TODO blue key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(26, handle);

		// Retrigger, slow
		// TODO red key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(28, handle);

		// Retrigger, slow
		// TODO yellow key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(27, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(117, handle);

		/*
			Push doors, open only
		*/

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(31, handle);

		// No retrigger, slow
		// TODO blue key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(32, handle);

		// No retrigger, slow
		// TODO red key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(33, handle);

		// No retrigger, slow
		// TODO yellow key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(34, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(118, handle);

		/*
			Switch doors, open-close
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(63, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(114, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(29, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(111, handle);

		/*
			Switch doors, open only
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(61, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(115, handle);

		// Retrigger, fast
		// TODO blue key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(99, handle);

		// Retrigger, fast
		// TODO red key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(134, handle);

		// Retrigger, fast
		// TODO yellow key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(136, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(103, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(112, handle);

		// No retrigger, fast
		// TODO blue key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(133, handle);

		// No retrigger, fast
		// TODO red key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(135, handle);

		// No retrigger, fast
		// TODO yellow key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(137, handle);

		/*
			Switch doors, close only
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(42, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(116, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(113, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(50, handle);

		/*
			Linedef touch doors, open-close
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(90, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(105, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(4, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(108, handle);

		/*
			Linedef touch doors, open only
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(86, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(106, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(2, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(109, handle);

		/*
			Linedef touch doors, close only
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(75, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(107, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(3, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,
					wait_time: Duration::default(),
					can_reverse: false,

					open_sound: Some(asset_storage.load("DSBDOPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSBDCLS", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(110, handle);

		/*
			Linedef touch doors, close-open
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: 30 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(76, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				params: DoorParams {
					start_state: DoorState::Open,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,
					wait_time: 30 * FRAME_TIME,
					can_reverse: true,

					open_sound: Some(asset_storage.load("DSDOROPN", &mut *loader)),
					close_sound: Some(asset_storage.load("DSDORCLS", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(16, handle);

		/*
			Switch floors, current height
		*/

		// No retrigger, slow, offset 512
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::Current,
					target_height_offset: 512.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(140, handle);

		/*
			Switch floors, lowest neighbour floor
		*/

		// Retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(60, handle);

		// No retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(23, handle);

		/*
			Switch floors, lowest neighbour floor above
		*/

		// Retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(69, handle);

		// No retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(18, handle);

		// Retrigger, fast, offset 0
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(132, handle);

		// No retrigger, fast, offset 0
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(131, handle);

		/*
			Switch floors, lowest neighbour ceiling
		*/

		// Retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(64, handle);

		// No retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(101, handle);

		// Retrigger, slow, offset -8
		// TODO crush
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: -8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(65, handle);

		// No retrigger, slow, offset -8
		// TODO crush
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: -8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(55, handle);

		/*
			Switch floors, highest neighbour floor
		*/

		// Retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(45, handle);

		// No retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(102, handle);

		// Retrigger, fast, offset +8
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(70, handle);

		// No retrigger, fast, offset +8
		let template = EntityTemplate::new()
			.with_component(UseAction::FloorSwitchUse(FloorSwitchUse {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(71, handle);

		/*
			Linedef touch floors, current height
		*/

		// Retrigger, slow, offset 24
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::Current,
					target_height_offset: 24.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(92, handle);

		// No retrigger, slow, offset 24
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::Current,
					target_height_offset: 24.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(58, handle);

		// Retrigger, slow, offset 24
		// TODO change type
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::Current,
					target_height_offset: 24.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(93, handle);

		// No retrigger, slow, offset 24
		// TODO change type
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::Current,
					target_height_offset: 24.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(59, handle);

		/*
			Linedef touch floors, lowest neighbour floor
		*/

		// Retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(82, handle);

		// No retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(38, handle);

		// Retrigger, slow, offset 0
		// TODO type change
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(84, handle);

		// No retrigger, slow, offset 0
		// TODO type change
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(37, handle);

		/*
			Linedef touch floors, lowest neighbour floor above
		*/

		// Retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(128, handle);

		// No retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(119, handle);

		// Retrigger, fast, offset 0
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(129, handle);

		// No retrigger, fast, offset 0
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourFloorAbove,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(130, handle);

		/*
			Linedef touch floors, lowest neighbour ceiling
		*/

		// Retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(91, handle);

		// No retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(5, handle);

		// Retrigger, slow, offset -8
		// TODO crush
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: -8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(94, handle);

		// No retrigger, slow, offset -8
		// TODO crush
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::LowestNeighbourCeiling,
					target_height_offset: -8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(56, handle);

		/*
			Linedef touch floors, highest neighbour floor
		*/

		// Retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(83, handle);

		// No retrigger, slow, offset 0
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 1.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 0.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(19, handle);

		// Retrigger, fast, offset +8
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(98, handle);

		// No retrigger, fast, offset +8
		let template = EntityTemplate::new()
			.with_component(TouchAction::FloorTouch(FloorTouch {
				params: FloorParams {
					speed: 4.0 * FRAME_RATE,
					target_height_base: FloorTargetHeight::HighestNeighbourFloor,
					target_height_offset: 8.0,
					move_sound: Some(asset_storage.load("DSSTNMOV", &mut *loader)),
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(36, handle);

		/*
			Switch plats, current - lowest neighbour floor
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::PlatSwitchUse(PlatSwitchUse {
				params: PlatParams {
					speed: 4.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART", &mut *loader)),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(62, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::PlatSwitchUse(PlatSwitchUse {
				params: PlatParams {
					speed: 8.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART", &mut *loader)),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: Some(35 * FRAME_TIME),
				},
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(123, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::PlatSwitchUse(PlatSwitchUse {
				params: PlatParams {
					speed: 4.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART", &mut *loader)),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(21, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::PlatSwitchUse(PlatSwitchUse {
				params: PlatParams {
					speed: 8.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART", &mut *loader)),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				switch_params: SwitchParams {
					sound: Some(asset_storage.load("DSSWTCHN", &mut *loader)),
					retrigger_time: None,
				},
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(122, handle);

		/*
			Linedef touch plats, current - lowest neighbour floor
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::PlatTouch(PlatTouch {
				params: PlatParams {
					speed: 4.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART", &mut *loader)),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(88, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::PlatTouch(PlatTouch {
				params: PlatParams {
					speed: 8.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART", &mut *loader)),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(120, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::PlatTouch(PlatTouch {
				params: PlatParams {
					speed: 4.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART", &mut *loader)),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(10, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::PlatTouch(PlatTouch {
				params: PlatParams {
					speed: 8.0 * FRAME_RATE,
					wait_time: 105 * FRAME_TIME,
					can_reverse: true,

					start_sound: Some(asset_storage.load("DSPSTART", &mut *loader)),
					move_sound: None,
					move_sound_time: 8 * FRAME_TIME,
					finish_sound: Some(asset_storage.load("DSPSTOP", &mut *loader)),

					low_height_base: PlatTargetHeight::LowestNeighbourFloor,
					low_height_offset: 0.0,
					high_height_base: PlatTargetHeight::Current,
					high_height_offset: 0.0,
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(121, handle);

		/*
			Other
		*/

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(6, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(7, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(8, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(9, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(11, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(12, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(13, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(14, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(15, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(17, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(20, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(22, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(24, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(25, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(30, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(35, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(39, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(40, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(41, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(43, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(44, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(46, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(47, handle);

        let template = EntityTemplate::new()
			.with_component(TextureScroll {
				speed: Vector2::new(35.0, 0.0),
			});
		let handle = asset_storage.insert(template);
        doomednums.insert(48, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(49, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(51, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(52, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(53, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(54, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(57, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(66, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(67, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(68, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(72, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(73, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(74, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(77, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(79, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(80, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(81, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(87, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(89, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(95, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(96, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(97, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(100, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(104, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(124, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(125, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(126, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(127, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(138, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(139, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(141, handle);

        LinedefTypes { doomednums }
    }
}
