use crate::{
	assets::{AssetHandle, AssetStorage},
	component::EntityTemplate,
	doom::components::{LightFlash, LightFlashType, LightGlow},
};
use specs::{World, WriteExpect};
use std::collections::HashMap;

pub struct SectorTypes {
	pub doomednums: HashMap<u16, AssetHandle<EntityTemplate>>,
}

impl SectorTypes {
	#[rustfmt::skip]
	pub fn new(world: &World) -> SectorTypes {
        let mut template_storage = world.system_data::<
			WriteExpect<AssetStorage<EntityTemplate>>,
		>();

        let mut doomednums = HashMap::new();

        // Blink random
        let handle = template_storage.insert({
            EntityTemplate::new()
                .with_component(LightFlash {
                    off_time: 8 * crate::doom::FRAME_TIME,
                    on_time: 64 * crate::doom::FRAME_TIME,
                    ..LightFlash::default()
                })
        });
        doomednums.insert(1, handle.clone());

        // Fast strobe unsynchronised
        let handle = template_storage.insert({
            EntityTemplate::new()
                .with_component(LightFlash {
                    flash_type: LightFlashType::StrobeUnSync(8 * crate::doom::FRAME_TIME),
                    off_time: 15 * crate::doom::FRAME_TIME,
                    on_time: 5 * crate::doom::FRAME_TIME,
                    ..LightFlash::default()
                })
        });
        doomednums.insert(2, handle.clone());

        // Slow strobe unsynchronised
        let handle = template_storage.insert({
            EntityTemplate::new()
                .with_component(LightFlash {
                    flash_type: LightFlashType::StrobeUnSync(8 * crate::doom::FRAME_TIME),
                    off_time: 35 * crate::doom::FRAME_TIME,
                    on_time: 5 * crate::doom::FRAME_TIME,
                    ..LightFlash::default()
                })
        });
        doomednums.insert(3, handle.clone());

        // Fast strobe unsynchronised + 20% damage
        let handle = template_storage.insert({
            EntityTemplate::new()
                .with_component(LightFlash {
                    flash_type: LightFlashType::StrobeUnSync(8 * crate::doom::FRAME_TIME),
                    off_time: 15 * crate::doom::FRAME_TIME,
                    on_time: 5 * crate::doom::FRAME_TIME,
                    ..LightFlash::default()
                })
        });
        doomednums.insert(4, handle.clone());

        // 10% damage
        let handle = template_storage.insert({
            EntityTemplate::new()
        });
        doomednums.insert(5, handle.clone());

        // 5% damage
        let handle = template_storage.insert({
            EntityTemplate::new()
        });
        doomednums.insert(7, handle.clone());

        // Glow
        let handle = template_storage.insert({
            EntityTemplate::new()
                .with_component(LightGlow {
                    speed: 1.09375,
                    ..LightGlow::default()
                })
        });
        doomednums.insert(8, handle.clone());

        // Secret
        let handle = template_storage.insert({
            EntityTemplate::new()
        });
        doomednums.insert(9, handle.clone());

        // Door close 30 s after level start
        let handle = template_storage.insert({
            EntityTemplate::new()
        });
        doomednums.insert(10, handle.clone());

        // 20% damage, end map on death
        let handle = template_storage.insert({
            EntityTemplate::new()
        });
        doomednums.insert(11, handle.clone());

        // Slow strobe
        let handle = template_storage.insert({
            EntityTemplate::new()
                .with_component(LightFlash {
                    flash_type: LightFlashType::Strobe,
                    off_time: 35 * crate::doom::FRAME_TIME,
                    on_time: 5 * crate::doom::FRAME_TIME,
                    ..LightFlash::default()
                })
        });
        doomednums.insert(12, handle.clone());

        // Fast strobe
        let handle = template_storage.insert({
            EntityTemplate::new()
                .with_component(LightFlash {
                    flash_type: LightFlashType::Strobe,
                    off_time: 15 * crate::doom::FRAME_TIME,
                    on_time: 5 * crate::doom::FRAME_TIME,
                    ..LightFlash::default()
                })
        });
        doomednums.insert(13, handle.clone());

        // Door open 300 s after level start
        let handle = template_storage.insert({
            EntityTemplate::new()
        });
        doomednums.insert(14, handle.clone());

        // 20% damage
        let handle = template_storage.insert({
            EntityTemplate::new()
        });
        doomednums.insert(16, handle.clone());

        // Random flicker
        let handle = template_storage.insert({
            EntityTemplate::new()
        });
        doomednums.insert(17, handle.clone());

        SectorTypes { doomednums }
    }
}
