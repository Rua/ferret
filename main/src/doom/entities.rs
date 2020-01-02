#![allow(unused_variables)]
use crate::{assets::AssetStorage, doom::components::SpawnPoint};
use specs::{Entity, ReadExpect, World, WriteExpect, WriteStorage};
use std::collections::HashMap;

lazy_static! {
	pub(crate) static ref ENTITIES: HashMap<&'static str, fn(Entity, &World)> = {
		let mut m: HashMap<&'static str, fn(Entity, &World)> = HashMap::new();
		m.insert("SPAWN1", |entity, world| {
			world
				.system_data::<WriteStorage<SpawnPoint>>()
				.insert(entity, SpawnPoint { player_num: 1 })
				.unwrap();
		});
		m.insert("SPAWN2", |entity, world| {
			world
				.system_data::<WriteStorage<SpawnPoint>>()
				.insert(entity, SpawnPoint { player_num: 2 })
				.unwrap();
		});
		m.insert("SPAWN3", |entity, world| {
			world
				.system_data::<WriteStorage<SpawnPoint>>()
				.insert(entity, SpawnPoint { player_num: 3 })
				.unwrap();
		});
		m.insert("SPAWN4", |entity, world| {
			world
				.system_data::<WriteStorage<SpawnPoint>>()
				.insert(entity, SpawnPoint { player_num: 4 })
				.unwrap();
		});
		m.insert("DMSPAWN", |entity, world| {});
		m.insert("PLAYER", |entity, world| {});
		m.insert("POSSESSED", |entity, world| {
			let sprite = {
				let (mut loader, mut sprite_storage, video) = world.system_data::<(
					WriteExpect<crate::doom::wad::WadLoader>,
					WriteExpect<AssetStorage<crate::doom::sprite::Sprite>>,
					ReadExpect<crate::renderer::video::Video>,
				)>();
				let sprite =
					sprite_storage.load("POSS", crate::doom::sprite::SpriteFormat, &mut *loader);
				sprite_storage
					.build_waiting(|data| Ok(data.build(video.queues().graphics.clone())?.0));

				sprite
			};
		});
		m.insert("SHOTGUY", |entity, world| {});
		m.insert("VILE", |entity, world| {});
		m.insert("FIRE", |entity, world| {});
		m.insert("UNDEAD", |entity, world| {});
		m.insert("TRACER", |entity, world| {});
		m.insert("SMOKE", |entity, world| {});
		m.insert("FATSO", |entity, world| {});
		m.insert("FATSHOT", |entity, world| {});
		m.insert("CHAINGUY", |entity, world| {});
		m.insert("TROOP", |entity, world| {});
		m.insert("SERGEANT", |entity, world| {});
		m.insert("SHADOWS", |entity, world| {});
		m.insert("HEAD", |entity, world| {});
		m.insert("BRUISER", |entity, world| {});
		m.insert("BRUISERSHOT", |entity, world| {});
		m.insert("KNIGHT", |entity, world| {});
		m.insert("SKULL", |entity, world| {});
		m.insert("SPIDER", |entity, world| {});
		m.insert("BABY", |entity, world| {});
		m.insert("CYBORG", |entity, world| {});
		m.insert("PAIN", |entity, world| {});
		m.insert("WOLFSS", |entity, world| {});
		m.insert("KEEN", |entity, world| {});
		m.insert("BOSSBRAIN", |entity, world| {});
		m.insert("BOSSSPIT", |entity, world| {});
		m.insert("BOSSTARGET", |entity, world| {});
		m.insert("SPAWNSHOT", |entity, world| {});
		m.insert("SPAWNFIRE", |entity, world| {});
		m.insert("BARREL", |entity, world| {});
		m.insert("TROOPSHOT", |entity, world| {});
		m.insert("HEADSHOT", |entity, world| {});
		m.insert("ROCKET", |entity, world| {});
		m.insert("PLASMA", |entity, world| {});
		m.insert("BFG", |entity, world| {});
		m.insert("ARACHPLAZ", |entity, world| {});
		m.insert("PUFF", |entity, world| {});
		m.insert("BLOOD", |entity, world| {});
		m.insert("TFOG", |entity, world| {});
		m.insert("IFOG", |entity, world| {});
		m.insert("TELEPORTMAN", |entity, world| {});
		m.insert("EXTRABFG", |entity, world| {});
		m.insert("MISC0", |entity, world| {});
		m.insert("MISC1", |entity, world| {});
		m.insert("MISC2", |entity, world| {});
		m.insert("MISC3", |entity, world| {});
		m.insert("MISC4", |entity, world| {});
		m.insert("MISC5", |entity, world| {});
		m.insert("MISC6", |entity, world| {});
		m.insert("MISC7", |entity, world| {});
		m.insert("MISC8", |entity, world| {});
		m.insert("MISC9", |entity, world| {});
		m.insert("MISC10", |entity, world| {});
		m.insert("MISC11", |entity, world| {});
		m.insert("MISC12", |entity, world| {});
		m.insert("INV", |entity, world| {});
		m.insert("MISC13", |entity, world| {});
		m.insert("INS", |entity, world| {});
		m.insert("MISC14", |entity, world| {});
		m.insert("MISC15", |entity, world| {});
		m.insert("MISC16", |entity, world| {});
		m.insert("MEGA", |entity, world| {});
		m.insert("CLIP", |entity, world| {});
		m.insert("MISC17", |entity, world| {});
		m.insert("MISC18", |entity, world| {});
		m.insert("MISC19", |entity, world| {});
		m.insert("MISC20", |entity, world| {});
		m.insert("MISC21", |entity, world| {});
		m.insert("MISC22", |entity, world| {});
		m.insert("MISC23", |entity, world| {});
		m.insert("MISC24", |entity, world| {});
		m.insert("MISC25", |entity, world| {});
		m.insert("CHAINGUN", |entity, world| {});
		m.insert("MISC26", |entity, world| {});
		m.insert("MISC27", |entity, world| {});
		m.insert("MISC28", |entity, world| {});
		m.insert("SHOTGUN", |entity, world| {});
		m.insert("SUPERSHOTGUN", |entity, world| {});
		m.insert("MISC29", |entity, world| {});
		m.insert("MISC30", |entity, world| {});
		m.insert("MISC31", |entity, world| {});
		m.insert("MISC32", |entity, world| {});
		m.insert("MISC33", |entity, world| {});
		m.insert("MISC34", |entity, world| {});
		m.insert("MISC35", |entity, world| {});
		m.insert("MISC36", |entity, world| {});
		m.insert("MISC37", |entity, world| {});
		m.insert("MISC38", |entity, world| {});
		m.insert("MISC39", |entity, world| {});
		m.insert("MISC40", |entity, world| {});
		m.insert("MISC41", |entity, world| {});
		m.insert("MISC42", |entity, world| {});
		m.insert("MISC43", |entity, world| {});
		m.insert("MISC44", |entity, world| {});
		m.insert("MISC45", |entity, world| {});
		m.insert("MISC46", |entity, world| {});
		m.insert("MISC47", |entity, world| {});
		m.insert("MISC48", |entity, world| {});
		m.insert("MISC49", |entity, world| {});
		m.insert("MISC50", |entity, world| {});
		m.insert("MISC51", |entity, world| {});
		m.insert("MISC52", |entity, world| {});
		m.insert("MISC53", |entity, world| {});
		m.insert("MISC54", |entity, world| {});
		m.insert("MISC55", |entity, world| {});
		m.insert("MISC56", |entity, world| {});
		m.insert("MISC57", |entity, world| {});
		m.insert("MISC58", |entity, world| {});
		m.insert("MISC59", |entity, world| {});
		m.insert("MISC60", |entity, world| {});
		m.insert("MISC61", |entity, world| {});
		m.insert("MISC62", |entity, world| {});
		m.insert("MISC63", |entity, world| {});
		m.insert("MISC64", |entity, world| {});
		m.insert("MISC65", |entity, world| {});
		m.insert("MISC66", |entity, world| {});
		m.insert("MISC67", |entity, world| {});
		m.insert("MISC68", |entity, world| {});
		m.insert("MISC69", |entity, world| {});
		m.insert("MISC70", |entity, world| {});
		m.insert("MISC71", |entity, world| {});
		m.insert("MISC72", |entity, world| {});
		m.insert("MISC73", |entity, world| {});
		m.insert("MISC74", |entity, world| {});
		m.insert("MISC75", |entity, world| {});
		m.insert("MISC76", |entity, world| {});
		m.insert("MISC77", |entity, world| {});
		m.insert("MISC78", |entity, world| {});
		m.insert("MISC79", |entity, world| {});
		m.insert("MISC80", |entity, world| {});
		m.insert("MISC81", |entity, world| {});
		m.insert("MISC82", |entity, world| {});
		m.insert("MISC83", |entity, world| {});
		m.insert("MISC84", |entity, world| {});
		m.insert("MISC85", |entity, world| {});
		m.insert("MISC86", |entity, world| {});
		m
	};
	pub(crate) static ref DOOMEDNUMS: HashMap<u16, &'static str> = {
		let mut m: HashMap<u16, &'static str> = HashMap::new();
		m.insert(1, "SPAWN1");
		m.insert(2, "SPAWN2");
		m.insert(3, "SPAWN3");
		m.insert(4, "SPAWN4");
		m.insert(11, "DMSPAWN");
		m.insert(3004, "POSSESSED");
		m.insert(9, "SHOTGUY");
		m.insert(64, "VILE");
		m.insert(66, "UNDEAD");
		m.insert(67, "FATSO");
		m.insert(65, "CHAINGUY");
		m.insert(3001, "TROOP");
		m.insert(3002, "SERGEANT");
		m.insert(58, "SHADOWS");
		m.insert(3005, "HEAD");
		m.insert(3003, "BRUISER");
		m.insert(69, "KNIGHT");
		m.insert(3006, "SKULL");
		m.insert(7, "SPIDER");
		m.insert(68, "BABY");
		m.insert(16, "CYBORG");
		m.insert(71, "PAIN");
		m.insert(84, "WOLFSS");
		m.insert(72, "KEEN");
		m.insert(88, "BOSSBRAIN");
		m.insert(89, "BOSSSPIT");
		m.insert(87, "BOSSTARGET");
		m.insert(2035, "BARREL");
		m.insert(14, "TELEPORTMAN");
		m.insert(2018, "MISC0");
		m.insert(2019, "MISC1");
		m.insert(2014, "MISC2");
		m.insert(2015, "MISC3");
		m.insert(5, "MISC4");
		m.insert(13, "MISC5");
		m.insert(6, "MISC6");
		m.insert(39, "MISC7");
		m.insert(38, "MISC8");
		m.insert(40, "MISC9");
		m.insert(2011, "MISC10");
		m.insert(2012, "MISC11");
		m.insert(2013, "MISC12");
		m.insert(2022, "INV");
		m.insert(2023, "MISC13");
		m.insert(2024, "INS");
		m.insert(2025, "MISC14");
		m.insert(2026, "MISC15");
		m.insert(2045, "MISC16");
		m.insert(83, "MEGA");
		m.insert(2007, "CLIP");
		m.insert(2048, "MISC17");
		m.insert(2010, "MISC18");
		m.insert(2046, "MISC19");
		m.insert(2047, "MISC20");
		m.insert(17, "MISC21");
		m.insert(2008, "MISC22");
		m.insert(2049, "MISC23");
		m.insert(8, "MISC24");
		m.insert(2006, "MISC25");
		m.insert(2002, "CHAINGUN");
		m.insert(2005, "MISC26");
		m.insert(2003, "MISC27");
		m.insert(2004, "MISC28");
		m.insert(2001, "SHOTGUN");
		m.insert(82, "SUPERSHOTGUN");
		m.insert(85, "MISC29");
		m.insert(86, "MISC30");
		m.insert(2028, "MISC31");
		m.insert(30, "MISC32");
		m.insert(31, "MISC33");
		m.insert(32, "MISC34");
		m.insert(33, "MISC35");
		m.insert(37, "MISC36");
		m.insert(36, "MISC37");
		m.insert(41, "MISC38");
		m.insert(42, "MISC39");
		m.insert(43, "MISC40");
		m.insert(44, "MISC41");
		m.insert(45, "MISC42");
		m.insert(46, "MISC43");
		m.insert(55, "MISC44");
		m.insert(56, "MISC45");
		m.insert(57, "MISC46");
		m.insert(47, "MISC47");
		m.insert(48, "MISC48");
		m.insert(34, "MISC49");
		m.insert(35, "MISC50");
		m.insert(49, "MISC51");
		m.insert(50, "MISC52");
		m.insert(51, "MISC53");
		m.insert(52, "MISC54");
		m.insert(53, "MISC55");
		m.insert(59, "MISC56");
		m.insert(60, "MISC57");
		m.insert(61, "MISC58");
		m.insert(62, "MISC59");
		m.insert(63, "MISC60");
		m.insert(22, "MISC61");
		m.insert(15, "MISC62");
		m.insert(18, "MISC63");
		m.insert(21, "MISC64");
		m.insert(23, "MISC65");
		m.insert(20, "MISC66");
		m.insert(19, "MISC67");
		m.insert(10, "MISC68");
		m.insert(12, "MISC69");
		m.insert(28, "MISC70");
		m.insert(24, "MISC71");
		m.insert(27, "MISC72");
		m.insert(29, "MISC73");
		m.insert(25, "MISC74");
		m.insert(26, "MISC75");
		m.insert(54, "MISC76");
		m.insert(70, "MISC77");
		m.insert(73, "MISC78");
		m.insert(74, "MISC79");
		m.insert(75, "MISC80");
		m.insert(76, "MISC81");
		m.insert(77, "MISC82");
		m.insert(78, "MISC83");
		m.insert(79, "MISC84");
		m.insert(80, "MISC85");
		m.insert(81, "MISC86");
		m
	};
}
