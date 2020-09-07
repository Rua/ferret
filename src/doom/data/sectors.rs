use crate::{
	common::{assets::AssetStorage, component::EntityComponents},
	doom::{
		data::{FRAME_RATE, FRAME_TIME},
		entitytemplate::{EntityTemplate, EntityTypeId},
		light::{LightFlash, LightFlashType, LightGlow},
	},
};
use legion::{systems::ResourceSet, Resources, Write};

#[rustfmt::skip]
pub fn load(resources: &mut Resources) {
	let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);

	// Blink random
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(1)),
		components: EntityComponents::new()
			.with_component(LightFlash {
				off_time: 8 * FRAME_TIME,
				on_time: 64 * FRAME_TIME,
				..LightFlash::default()
			})
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Fast strobe unsynchronised
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(2)),
		components: EntityComponents::new()
			.with_component(LightFlash {
				flash_type: LightFlashType::StrobeUnSync(8 * FRAME_TIME),
				off_time: 15 * FRAME_TIME,
				on_time: 5 * FRAME_TIME,
				..LightFlash::default()
			})
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Slow strobe unsynchronised
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(3)),
		components: EntityComponents::new()
			.with_component(LightFlash {
				flash_type: LightFlashType::StrobeUnSync(8 * FRAME_TIME),
				off_time: 35 * FRAME_TIME,
				on_time: 5 * FRAME_TIME,
				..LightFlash::default()
			})
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Fast strobe unsynchronised + 20% damage
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(4)),
		components: EntityComponents::new()
			.with_component(LightFlash {
				flash_type: LightFlashType::StrobeUnSync(8 * FRAME_TIME),
				off_time: 15 * FRAME_TIME,
				on_time: 5 * FRAME_TIME,
				..LightFlash::default()
			})
	};
	asset_storage.insert::<EntityTemplate>(template);

	// 10% damage
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(5)),
		components: EntityComponents::new()
	};
	asset_storage.insert::<EntityTemplate>(template);

	// 5% damage
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(7)),
		components: EntityComponents::new()
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Glow
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(8)),
		components: EntityComponents::new()
			.with_component(LightGlow {
				speed: (8.0 / 256.0) * FRAME_RATE,
				..LightGlow::default()
			})
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Secret
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(9)),
		components: EntityComponents::new()
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Door close 30 s after level start
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(10)),
		components: EntityComponents::new()
	};
	asset_storage.insert::<EntityTemplate>(template);

	// 20% damage, end map on death
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(11)),
		components: EntityComponents::new()
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Slow strobe
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(12)),
		components: EntityComponents::new()
			.with_component(LightFlash {
				flash_type: LightFlashType::Strobe,
				off_time: 35 * FRAME_TIME,
				on_time: 5 * FRAME_TIME,
				..LightFlash::default()
			})
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Fast strobe
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(13)),
		components: EntityComponents::new()
			.with_component(LightFlash {
				flash_type: LightFlashType::Strobe,
				off_time: 15 * FRAME_TIME,
				on_time: 5 * FRAME_TIME,
				..LightFlash::default()
			})
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Door open 300 s after level start
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(14)),
		components: EntityComponents::new()
	};
	asset_storage.insert::<EntityTemplate>(template);

	// 20% damage
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(16)),
		components: EntityComponents::new()
	};
	asset_storage.insert::<EntityTemplate>(template);

	// Random flicker
	let template = EntityTemplate {
		name: None,
		type_id: Some(EntityTypeId::Sector(17)),
		components: EntityComponents::new()
	};
	asset_storage.insert::<EntityTemplate>(template);
}
