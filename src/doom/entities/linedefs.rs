use crate::{
    assets::{AssetHandle, AssetStorage},
    component::EntityTemplate,
};
use std::collections::HashMap;
use specs::{World, WriteExpect};

pub struct LinedefTypes {
	pub doomednums: HashMap<u16, AssetHandle<EntityTemplate>>,
}

impl LinedefTypes {
	#[rustfmt::skip]
	pub fn new(world: &World) -> LinedefTypes {
        let mut template_storage = world.system_data::<
			WriteExpect<AssetStorage<EntityTemplate>>,
		>();

        let mut doomednums = HashMap::new();

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(1, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(2, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(3, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(4, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(5, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(6, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(7, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(8, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(9, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(10, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(11, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(12, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(13, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(14, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(15, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(16, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(17, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(18, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(19, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(20, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(21, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(22, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(23, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(24, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(25, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(26, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(27, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(28, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(29, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(30, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(31, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(32, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(33, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(34, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(35, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(36, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(37, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(38, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(39, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(40, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(41, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(42, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(43, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(44, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(45, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(46, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(47, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(48, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(49, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(50, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(51, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(52, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(53, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(54, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(55, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(56, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(57, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(58, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(59, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(60, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(61, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(62, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(63, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(64, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(65, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(66, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(67, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(68, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(69, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(70, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(71, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(72, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(73, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(74, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(75, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(76, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(77, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(78, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(79, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(80, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(81, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(82, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(83, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(84, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(85, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(86, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(87, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(88, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(89, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(90, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(91, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(92, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(93, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(94, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(95, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(96, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(97, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(98, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(99, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(100, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(101, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(102, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(103, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(104, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(105, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(106, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(107, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(108, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(109, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(110, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(111, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(112, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(113, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(114, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(115, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(116, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(117, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(118, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(119, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(120, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(121, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(122, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(123, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(124, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(125, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(126, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(127, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(128, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(129, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(130, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(131, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(132, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(133, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(134, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(135, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(136, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(137, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(138, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(139, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(140, handle.clone());

        let handle = template_storage.insert({
        	EntityTemplate::new()
        });
        doomednums.insert(141, handle.clone());

        LinedefTypes { doomednums }
    }
}
