use crate::{
	common::{assets::AssetStorage, frame::FrameRngDef},
	doom::{
		data::{FRAME_RATE, FRAME_TIME},
		entitytemplate::{EntityTemplate, EntityTypeId},
		light::{LightFlashDef, LightFlashType, LightGlow},
	},
};
use legion::{systems::ResourceSet, Resources, World, Write};

#[rustfmt::skip]
pub fn load(resources: &mut Resources) {
	let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);

	// Blink random
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(1)),
		world: {
			let mut world = World::default();
			world.push((
				FrameRngDef,
				LightFlashDef {
					flash_type: LightFlashType::Broken,
					off_time: 8 * FRAME_TIME,
					on_time: 64 * FRAME_TIME,
				},
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Fast strobe unsynchronised
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(2)),
		world: {
			let mut world = World::default();
			world.push((
				FrameRngDef,
				LightFlashDef {
					flash_type: LightFlashType::StrobeUnSync(8 * FRAME_TIME),
					off_time: 15 * FRAME_TIME,
					on_time: 5 * FRAME_TIME,
				},
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Slow strobe unsynchronised
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(3)),
		world: {
			let mut world = World::default();
			world.push((
				FrameRngDef,
				LightFlashDef {
					flash_type: LightFlashType::StrobeUnSync(8 * FRAME_TIME),
					off_time: 35 * FRAME_TIME,
					on_time: 5 * FRAME_TIME,
				},
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Fast strobe unsynchronised + 20% damage
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(4)),
		world: {
			let mut world = World::default();
			world.push((
				FrameRngDef,
				LightFlashDef {
					flash_type: LightFlashType::StrobeUnSync(8 * FRAME_TIME),
					off_time: 15 * FRAME_TIME,
					on_time: 5 * FRAME_TIME,
				},
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// 10% damage
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(5)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// 5% damage
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(7)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Glow
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(8)),
		world: {
			let mut world = World::default();
			world.push((
				LightGlow {
					speed: (8.0 / 256.0) * FRAME_RATE,
					..LightGlow::default()
				},
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Secret
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(9)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Door close 30 s after level start
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(10)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// 20% damage, end map on death
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(11)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Slow strobe
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(12)),
		world: {
			let mut world = World::default();
			world.push((
				FrameRngDef,
				LightFlashDef {
					flash_type: LightFlashType::Strobe,
					off_time: 35 * FRAME_TIME,
					on_time: 5 * FRAME_TIME,
				},
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Fast strobe
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(13)),
		world: {
			let mut world = World::default();
			world.push((
				FrameRngDef,
				LightFlashDef {
					flash_type: LightFlashType::Strobe,
					off_time: 15 * FRAME_TIME,
					on_time: 5 * FRAME_TIME,
				},
			));
			world
		},
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Door open 300 s after level start
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(14)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// 20% damage
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(16)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);

	// Random flicker
	let template = EntityTemplate {
		type_id: Some(EntityTypeId::Sector(17)),
		.. EntityTemplate::default()
	};
	asset_storage.insert(template);
}
