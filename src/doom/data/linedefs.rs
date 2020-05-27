use crate::{
	assets::{AssetHandle, AssetStorage},
	component::EntityTemplate,
	doom::{
		client::UseAction,
		data::{FRAME_RATE, FRAME_TIME},
		door::{DoorState, DoorSwitchUse, DoorTouch, DoorTrigger, DoorUse},
		physics::TouchAction,
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
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: 150 * FRAME_TIME,

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
			}));
        let handle = asset_storage.insert(template);
        doomednums.insert(1, handle);

		// Retrigger, slow
		// TODO blue key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: 150 * FRAME_TIME,

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(26, handle);

		// Retrigger, slow
		// TODO red key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: 150 * FRAME_TIME,

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(28, handle);

		// Retrigger, slow
		// TODO yellow key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: 150 * FRAME_TIME,

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(27, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: 150 * FRAME_TIME,

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
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
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(31, handle);

		// No retrigger, slow
		// TODO blue key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(32, handle);

		// No retrigger, slow
		// TODO red key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(33, handle);

		// No retrigger, slow
		// TODO yellow key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(34, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorUse(DoorUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
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
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: 150 * FRAME_TIME,

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: Some(35 * FRAME_TIME),
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(63, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: 150 * FRAME_TIME,

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: Some(35 * FRAME_TIME),
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(114, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: 150 * FRAME_TIME,

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: None,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(29, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: 150 * FRAME_TIME,

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: None,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(111, handle);

		/*
			Switch doors, open only
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: Some(35 * FRAME_TIME),
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(61, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: Some(35 * FRAME_TIME),
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(115, handle);

		// Retrigger, fast
		// TODO blue key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: Some(35 * FRAME_TIME),
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(99, handle);

		// Retrigger, fast
		// TODO red key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: Some(35 * FRAME_TIME),
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(134, handle);

		// Retrigger, fast
		// TODO yellow key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: Some(35 * FRAME_TIME),
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(136, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: None,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(103, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: None,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(112, handle);

		// No retrigger, fast
		// TODO blue key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: None,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(133, handle);

		// No retrigger, fast
		// TODO red key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: None,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(135, handle);

		// No retrigger, fast
		// TODO yellow key
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: None,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(137, handle);

		/*
			Switch doors, close only
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: Some(35 * FRAME_TIME),
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(42, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: Some(35 * FRAME_TIME),
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(116, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: None,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(113, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
				trigger: DoorTrigger {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				switch_sound: asset_storage.load("DSSWTCHN", &mut *loader),
				retrigger_time: None,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(50, handle);

		/*
			Touch doors, open-close
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: 150 * FRAME_TIME,

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(90, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: 150 * FRAME_TIME,

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(105, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: 150 * FRAME_TIME,

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(4, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: 150 * FRAME_TIME,

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(108, handle);

		/*
			Touch doors, open only
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(86, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(106, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(2, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Closed,
					end_state: DoorState::Open,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(109, handle);

		/*
			Touch doors, close only
			TODO these shouldn't go back up if they bump something
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(75, handle);

		// Retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(107, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(3, handle);

		// No retrigger, fast
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Open,
					end_state: DoorState::Closed,
					speed: 8.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSBDOPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSBDCLS", &mut *loader),
					close_time: Duration::default(),
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
        doomednums.insert(110, handle);

		/*
			Touch doors, close-open
		*/

		// Retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Open,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: 30 * FRAME_TIME,
				},
				retrigger: true,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(76, handle);

		// No retrigger, slow
		let template = EntityTemplate::new()
			.with_component(TouchAction::DoorTouch(DoorTouch {
				trigger: DoorTrigger {
					start_state: DoorState::Open,
					end_state: DoorState::Open,
					speed: 2.0 * FRAME_RATE,

					open_sound: asset_storage.load("DSDOROPN", &mut *loader),
					open_time: Duration::default(),

					close_sound: asset_storage.load("DSDORCLS", &mut *loader),
					close_time: 30 * FRAME_TIME,
				},
				retrigger: false,
			}));
		let handle = asset_storage.insert(template);
		doomednums.insert(16, handle);

		/*
			Other
		*/

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(5, handle);

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
        doomednums.insert(10, handle);

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
        doomednums.insert(18, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(19, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(20, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(21, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(22, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(23, handle);

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
        doomednums.insert(36, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(37, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(38, handle);

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
        doomednums.insert(45, handle);

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
        doomednums.insert(55, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(56, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(57, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(58, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(59, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(60, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(62, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(64, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(65, handle);

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
        doomednums.insert(69, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(70, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(71, handle);

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
        doomednums.insert(82, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(83, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(84, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(87, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(88, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(89, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(91, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(92, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(93, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(94, handle);

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
        doomednums.insert(98, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(100, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(101, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(102, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(104, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(119, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(120, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(121, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(122, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(123, handle);

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
        doomednums.insert(128, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(129, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(130, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(131, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(132, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(138, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(139, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(140, handle);

        let template = EntityTemplate::new();
		let handle = asset_storage.insert(template);
        doomednums.insert(141, handle);

        LinedefTypes { doomednums }
    }
}
