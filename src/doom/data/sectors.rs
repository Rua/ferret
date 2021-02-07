use crate::{
	common::assets::AssetStorage,
	doom::{
		data::{FRAME_RATE, FRAME_TIME},
		light::{LightFlashDef, LightFlashType, LightGlow},
		map::SectorRefDef,
		template::{EntityTemplate, EntityTemplateRefDef},
	},
};
use legion::World;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[allow(unused_variables)]
#[rustfmt::skip]
pub static SECTORS: Lazy<HashMap<&'static str, fn(&mut AssetStorage) -> EntityTemplate>> = Lazy::new(|| {
	let mut sectors: HashMap<&'static str, fn(&mut AssetStorage) -> EntityTemplate> = HashMap::new();

	// The default, boring, do-nothing sector
	sectors.insert("sector0.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Blink random
	sectors.insert("sector1.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
				LightFlashDef {
					flash_type: LightFlashType::Broken,
					off_time: 8 * FRAME_TIME,
					on_time: 64 * FRAME_TIME,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Fast strobe unsynchronised
	sectors.insert("sector2.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
				LightFlashDef {
					flash_type: LightFlashType::StrobeUnSync(8 * FRAME_TIME),
					off_time: 15 * FRAME_TIME,
					on_time: 5 * FRAME_TIME,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Slow strobe unsynchronised
	sectors.insert("sector3.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
				LightFlashDef {
					flash_type: LightFlashType::StrobeUnSync(8 * FRAME_TIME),
					off_time: 35 * FRAME_TIME,
					on_time: 5 * FRAME_TIME,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Fast strobe unsynchronised + 20% damage
	sectors.insert("sector4.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
				LightFlashDef {
					flash_type: LightFlashType::StrobeUnSync(8 * FRAME_TIME),
					off_time: 15 * FRAME_TIME,
					on_time: 5 * FRAME_TIME,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// 10% damage
	sectors.insert("sector5.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	// 5% damage
	sectors.insert("sector7.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Glow
	sectors.insert("sector8.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
				LightGlow {
					speed: (8.0 / 256.0) * FRAME_RATE,
					..LightGlow::default()
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Secret
	sectors.insert("sector9.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Door close 30 s after level start
	sectors.insert("sector10.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	// 20% damage, end map on death
	sectors.insert("sector11.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Slow strobe
	sectors.insert("sector12.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
				LightFlashDef {
					flash_type: LightFlashType::Strobe,
					off_time: 35 * FRAME_TIME,
					on_time: 5 * FRAME_TIME,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Fast strobe
	sectors.insert("sector13.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
				LightFlashDef {
					flash_type: LightFlashType::Strobe,
					off_time: 15 * FRAME_TIME,
					on_time: 5 * FRAME_TIME,
				},
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Door open 300 s after level start
	sectors.insert("sector14.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	// 20% damage
	sectors.insert("sector16.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	// Random flicker
	sectors.insert("sector17.entity", |asset_storage| EntityTemplate {
		world: {
			let mut world = World::default();
			world.push((
				EntityTemplateRefDef,
				SectorRefDef,
			));
			world
		},
		.. EntityTemplate::default()
	});

	sectors
});
