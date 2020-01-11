#![allow(unused_variables)]
use crate::{
	assets::AssetStorage,
	component::EntityTemplate,
	doom::components::{SpawnPoint, SpriteRender, Transform},
};
use specs::{ReadExpect, World, WriteExpect};
use std::collections::HashMap;

pub struct EntityTypes {
	pub names: HashMap<&'static str, EntityTemplate>,
	pub doomednums: HashMap<u16, &'static str>,
}

impl EntityTypes {
	pub fn new(world: &World) -> EntityTypes {
		let mut names: HashMap<&'static str, EntityTemplate> = HashMap::new();
		names.insert("SPAWN1", {
			let mut template = EntityTemplate::new();
			template.add_component(SpawnPoint { player_num: 1 });
			template.add_component(Transform::default());
			template
		});

		names.insert("SPAWN2", {
			let mut template = EntityTemplate::new();
			template.add_component(SpawnPoint { player_num: 2 });
			template.add_component(Transform::default());
			template
		});
		names.insert("SPAWN3", {
			let mut template = EntityTemplate::new();
			template.add_component(SpawnPoint { player_num: 3 });
			template.add_component(Transform::default());
			template
		});
		names.insert("SPAWN4", {
			let mut template = EntityTemplate::new();
			template.add_component(SpawnPoint { player_num: 4 });
			template.add_component(Transform::default());
			template
		});
		names.insert("DMSPAWN", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("PLAYER", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("POSSESSED", {
			let mut template = EntityTemplate::new();
			template.add_component({
				let sprite = {
					let (mut loader, mut sprite_storage, video) = world.system_data::<(
						WriteExpect<crate::doom::wad::WadLoader>,
						WriteExpect<AssetStorage<crate::doom::sprite::Sprite>>,
						ReadExpect<crate::renderer::video::Video>,
					)>();
					let sprite = sprite_storage.load(
						"POSS",
						crate::doom::sprite::SpriteFormat,
						&mut *loader,
					);
					sprite_storage
						.build_waiting(|data| Ok(data.build(video.queues().graphics.clone())?.0));

					sprite
				};
				SpriteRender { sprite, frame: 0 }
			});
			template.add_component(Transform::default());
			template
		});
		names.insert("SHOTGUY", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("VILE", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("FIRE", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("UNDEAD", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("TRACER", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SMOKE", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("FATSO", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("FATSHOT", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("CHAINGUY", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("TROOP", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SERGEANT", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SHADOWS", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("HEAD", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BRUISER", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BRUISERSHOT", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("KNIGHT", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SKULL", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SPIDER", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BABY", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("CYBORG", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("PAIN", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("WOLFSS", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("KEEN", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BOSSBRAIN", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BOSSSPIT", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BOSSTARGET", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SPAWNSHOT", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SPAWNFIRE", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BARREL", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("TROOPSHOT", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("HEADSHOT", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("ROCKET", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("PLASMA", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BFG", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("ARACHPLAZ", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("PUFF", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BLOOD", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("TFOG", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("IFOG", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("TELEPORTMAN", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("EXTRABFG", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC0", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC1", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC2", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC3", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC4", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC5", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC6", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC7", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC8", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC9", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC10", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC11", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC12", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("INV", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC13", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("INS", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC14", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC15", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC16", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MEGA", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("CLIP", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC17", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC18", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC19", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC20", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC21", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC22", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC23", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC24", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC25", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("CHAINGUN", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC26", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC27", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC28", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SHOTGUN", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SUPERSHOTGUN", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC29", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC30", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC31", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC32", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC33", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC34", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC35", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC36", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC37", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC38", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC39", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC40", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC41", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC42", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC43", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC44", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC45", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC46", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC47", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC48", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC49", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC50", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC51", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC52", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC53", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC54", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC55", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC56", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC57", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC58", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC59", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC60", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC61", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC62", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC63", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC64", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC65", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC66", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC67", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC68", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC69", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC70", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC71", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC72", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC73", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC74", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC75", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC76", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC77", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC78", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC79", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC80", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC81", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC82", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC83", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC84", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC85", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC86", {
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});

		let mut doomednums = HashMap::new();
		doomednums.insert(1, "SPAWN1");
		doomednums.insert(2, "SPAWN2");
		doomednums.insert(3, "SPAWN3");
		doomednums.insert(4, "SPAWN4");
		doomednums.insert(11, "DMSPAWN");
		doomednums.insert(3004, "POSSESSED");
		doomednums.insert(9, "SHOTGUY");
		doomednums.insert(64, "VILE");
		doomednums.insert(66, "UNDEAD");
		doomednums.insert(67, "FATSO");
		doomednums.insert(65, "CHAINGUY");
		doomednums.insert(3001, "TROOP");
		doomednums.insert(3002, "SERGEANT");
		doomednums.insert(58, "SHADOWS");
		doomednums.insert(3005, "HEAD");
		doomednums.insert(3003, "BRUISER");
		doomednums.insert(69, "KNIGHT");
		doomednums.insert(3006, "SKULL");
		doomednums.insert(7, "SPIDER");
		doomednums.insert(68, "BABY");
		doomednums.insert(16, "CYBORG");
		doomednums.insert(71, "PAIN");
		doomednums.insert(84, "WOLFSS");
		doomednums.insert(72, "KEEN");
		doomednums.insert(88, "BOSSBRAIN");
		doomednums.insert(89, "BOSSSPIT");
		doomednums.insert(87, "BOSSTARGET");
		doomednums.insert(2035, "BARREL");
		doomednums.insert(14, "TELEPORTMAN");
		doomednums.insert(2018, "MISC0");
		doomednums.insert(2019, "MISC1");
		doomednums.insert(2014, "MISC2");
		doomednums.insert(2015, "MISC3");
		doomednums.insert(5, "MISC4");
		doomednums.insert(13, "MISC5");
		doomednums.insert(6, "MISC6");
		doomednums.insert(39, "MISC7");
		doomednums.insert(38, "MISC8");
		doomednums.insert(40, "MISC9");
		doomednums.insert(2011, "MISC10");
		doomednums.insert(2012, "MISC11");
		doomednums.insert(2013, "MISC12");
		doomednums.insert(2022, "INV");
		doomednums.insert(2023, "MISC13");
		doomednums.insert(2024, "INS");
		doomednums.insert(2025, "MISC14");
		doomednums.insert(2026, "MISC15");
		doomednums.insert(2045, "MISC16");
		doomednums.insert(83, "MEGA");
		doomednums.insert(2007, "CLIP");
		doomednums.insert(2048, "MISC17");
		doomednums.insert(2010, "MISC18");
		doomednums.insert(2046, "MISC19");
		doomednums.insert(2047, "MISC20");
		doomednums.insert(17, "MISC21");
		doomednums.insert(2008, "MISC22");
		doomednums.insert(2049, "MISC23");
		doomednums.insert(8, "MISC24");
		doomednums.insert(2006, "MISC25");
		doomednums.insert(2002, "CHAINGUN");
		doomednums.insert(2005, "MISC26");
		doomednums.insert(2003, "MISC27");
		doomednums.insert(2004, "MISC28");
		doomednums.insert(2001, "SHOTGUN");
		doomednums.insert(82, "SUPERSHOTGUN");
		doomednums.insert(85, "MISC29");
		doomednums.insert(86, "MISC30");
		doomednums.insert(2028, "MISC31");
		doomednums.insert(30, "MISC32");
		doomednums.insert(31, "MISC33");
		doomednums.insert(32, "MISC34");
		doomednums.insert(33, "MISC35");
		doomednums.insert(37, "MISC36");
		doomednums.insert(36, "MISC37");
		doomednums.insert(41, "MISC38");
		doomednums.insert(42, "MISC39");
		doomednums.insert(43, "MISC40");
		doomednums.insert(44, "MISC41");
		doomednums.insert(45, "MISC42");
		doomednums.insert(46, "MISC43");
		doomednums.insert(55, "MISC44");
		doomednums.insert(56, "MISC45");
		doomednums.insert(57, "MISC46");
		doomednums.insert(47, "MISC47");
		doomednums.insert(48, "MISC48");
		doomednums.insert(34, "MISC49");
		doomednums.insert(35, "MISC50");
		doomednums.insert(49, "MISC51");
		doomednums.insert(50, "MISC52");
		doomednums.insert(51, "MISC53");
		doomednums.insert(52, "MISC54");
		doomednums.insert(53, "MISC55");
		doomednums.insert(59, "MISC56");
		doomednums.insert(60, "MISC57");
		doomednums.insert(61, "MISC58");
		doomednums.insert(62, "MISC59");
		doomednums.insert(63, "MISC60");
		doomednums.insert(22, "MISC61");
		doomednums.insert(15, "MISC62");
		doomednums.insert(18, "MISC63");
		doomednums.insert(21, "MISC64");
		doomednums.insert(23, "MISC65");
		doomednums.insert(20, "MISC66");
		doomednums.insert(19, "MISC67");
		doomednums.insert(10, "MISC68");
		doomednums.insert(12, "MISC69");
		doomednums.insert(28, "MISC70");
		doomednums.insert(24, "MISC71");
		doomednums.insert(27, "MISC72");
		doomednums.insert(29, "MISC73");
		doomednums.insert(25, "MISC74");
		doomednums.insert(26, "MISC75");
		doomednums.insert(54, "MISC76");
		doomednums.insert(70, "MISC77");
		doomednums.insert(73, "MISC78");
		doomednums.insert(74, "MISC79");
		doomednums.insert(75, "MISC80");
		doomednums.insert(76, "MISC81");
		doomednums.insert(77, "MISC82");
		doomednums.insert(78, "MISC83");
		doomednums.insert(79, "MISC84");
		doomednums.insert(80, "MISC85");
		doomednums.insert(81, "MISC86");

		EntityTypes { names, doomednums }
	}
}
