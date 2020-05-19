use crate::{
	assets::{AssetHandle, AssetStorage},
	audio::Sound,
	component::EntityTemplate,
	doom::{
		client::UseAction,
		data::{FRAME_RATE, FRAME_TIME},
		door::{DoorSwitchUse, DoorUse},
		update::TextureScroll,
		wad::WadLoader,
	},
};
use fnv::FnvHashMap;
use nalgebra::Vector2;
use specs::{World, WriteExpect};

pub struct LinedefTypes {
	pub doomednums: FnvHashMap<u16, AssetHandle<EntityTemplate>>,
}

impl LinedefTypes {
	#[rustfmt::skip]
	pub fn new(world: &World) -> LinedefTypes {
        let (mut template_storage, mut sound_storage, mut loader) = world.system_data::<(
			WriteExpect<AssetStorage<EntityTemplate>>,
			WriteExpect<AssetStorage<Sound>>,
			WriteExpect<WadLoader>,
		)>();

        let mut doomednums = FnvHashMap::default();

        let handle = template_storage.insert({
        	EntityTemplate::new()
				.with_component(UseAction::DoorUse(DoorUse {
					open_sound: sound_storage.load("DSDOROPN", &mut *loader),
					close_sound: sound_storage.load("DSDORCLS", &mut *loader),
					speed: 2.0 * FRAME_RATE,
					wait_time: 150 * FRAME_TIME,
				}))
        });
        doomednums.insert(1, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(2, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(3, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(4, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(5, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(6, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(7, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(8, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(9, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(10, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(11, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(12, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(13, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(14, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(15, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(16, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(17, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(18, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(19, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(20, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(21, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(22, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(23, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(24, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(25, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(26, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(27, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(28, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(29, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(30, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(31, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(32, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(33, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(34, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(35, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(36, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(37, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(38, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(39, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(40, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(41, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(42, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(43, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(44, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(45, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(46, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(47, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
				.with_component(TextureScroll {
					speed: Vector2::new(35.0, 0.0),
				})
        });
        doomednums.insert(48, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(49, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(50, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(51, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(52, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(53, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(54, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(55, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(56, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(57, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(58, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(59, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(60, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(61, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(62, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
				.with_component(UseAction::DoorSwitchUse(DoorSwitchUse {
					open_sound: sound_storage.load("DSDOROPN", &mut *loader),
					close_sound: sound_storage.load("DSDORCLS", &mut *loader),
					speed: 2.0 * FRAME_RATE,
					switch_sound: sound_storage.load("DSSWTCHN", &mut *loader),
					switch_time: 35 * FRAME_TIME,
					wait_time: 150 * FRAME_TIME,
				}))
        });
        doomednums.insert(63, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(64, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(65, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(66, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(67, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(68, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(69, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(70, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(71, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(72, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(73, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(74, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(75, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(76, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(77, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(79, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(80, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(81, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(82, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(83, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(84, handle);

    	let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(86, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(87, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(88, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(89, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(90, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(91, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(92, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(93, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(94, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(95, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(96, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(97, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(98, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(99, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(100, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(101, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(102, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(103, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(104, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(105, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(106, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(107, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(108, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(109, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(110, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(111, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(112, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(113, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(114, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(115, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(116, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(117, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(118, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(119, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(120, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(121, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(122, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(123, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(124, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(125, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(126, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(127, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(128, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(129, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(130, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(131, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(132, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(133, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(134, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(135, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(136, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(137, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(138, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(139, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(140, handle);

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(141, handle);

        LinedefTypes { doomednums }
    }
}
