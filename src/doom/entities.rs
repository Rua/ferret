#![allow(unused_variables)]
use crate::{
	assets::AssetStorage,
	component::DynComponent,
	doom::components::{SpawnPoint, SpriteRender, Transform},
};
use specs::{ReadExpect, World, WriteExpect};
use std::collections::HashMap;

pub struct EntityTypes {
	pub names: HashMap<&'static str, Vec<Box<dyn DynComponent>>>,
	pub doomednums: HashMap<u16, &'static str>,
}

impl EntityTypes {
	pub fn new(world: &World) -> EntityTypes {
		let mut names: HashMap<&'static str, Vec<Box<dyn DynComponent>>> = HashMap::new();
		names.insert("SPAWN1", vec![
			Box::from(SpawnPoint { player_num: 1 }),
			Box::from(Transform::default()),
		]);
		names.insert("SPAWN2", vec![
			Box::from(SpawnPoint { player_num: 2 }),
			Box::from(Transform::default()),
		]);
		names.insert("SPAWN3", vec![
			Box::from(SpawnPoint { player_num: 3 }),
			Box::from(Transform::default()),
		]);
		names.insert("SPAWN4", vec![
			Box::from(SpawnPoint { player_num: 4 }),
			Box::from(Transform::default()),
		]);
		names.insert("DMSPAWN", vec![
			Box::from(Transform::default()),
		]);
		names.insert("PLAYER", vec![
			Box::from(Transform::default()),
		]);
		names.insert("POSSESSED", vec![
			Box::from({
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
				SpriteRender { sprite, frame: 0 }
			}),
			Box::from(Transform::default()),
		]);
		names.insert("SHOTGUY", vec![
			Box::from(Transform::default()),
		]);
		names.insert("VILE", vec![
			Box::from(Transform::default()),
		]);
		names.insert("FIRE", vec![
			Box::from(Transform::default()),
		]);
		names.insert("UNDEAD", vec![
			Box::from(Transform::default()),
		]);
		names.insert("TRACER", vec![
			Box::from(Transform::default()),
		]);
		names.insert("SMOKE", vec![
			Box::from(Transform::default()),
		]);
		names.insert("FATSO", vec![
			Box::from(Transform::default()),
		]);
		names.insert("FATSHOT", vec![
			Box::from(Transform::default()),
		]);
		names.insert("CHAINGUY", vec![
			Box::from(Transform::default()),
		]);
		names.insert("TROOP", vec![
			Box::from(Transform::default()),
		]);
		names.insert("SERGEANT", vec![
			Box::from(Transform::default()),
		]);
		names.insert("SHADOWS", vec![
			Box::from(Transform::default()),
		]);
		names.insert("HEAD", vec![
			Box::from(Transform::default()),
		]);
		names.insert("BRUISER", vec![
			Box::from(Transform::default()),
		]);
		names.insert("BRUISERSHOT", vec![
			Box::from(Transform::default()),
		]);
		names.insert("KNIGHT", vec![
			Box::from(Transform::default()),
		]);
		names.insert("SKULL", vec![
			Box::from(Transform::default()),
		]);
		names.insert("SPIDER", vec![
			Box::from(Transform::default()),
		]);
		names.insert("BABY", vec![
			Box::from(Transform::default()),
		]);
		names.insert("CYBORG", vec![
			Box::from(Transform::default()),
		]);
		names.insert("PAIN", vec![
			Box::from(Transform::default()),
		]);
		names.insert("WOLFSS", vec![
			Box::from(Transform::default()),
		]);
		names.insert("KEEN", vec![
			Box::from(Transform::default()),
		]);
		names.insert("BOSSBRAIN", vec![
			Box::from(Transform::default()),
		]);
		names.insert("BOSSSPIT", vec![
			Box::from(Transform::default()),
		]);
		names.insert("BOSSTARGET", vec![
			Box::from(Transform::default()),
		]);
		names.insert("SPAWNSHOT", vec![
			Box::from(Transform::default()),
		]);
		names.insert("SPAWNFIRE", vec![
			Box::from(Transform::default()),
		]);
		names.insert("BARREL", vec![
			Box::from(Transform::default()),
		]);
		names.insert("TROOPSHOT", vec![
			Box::from(Transform::default()),
		]);
		names.insert("HEADSHOT", vec![
			Box::from(Transform::default()),
		]);
		names.insert("ROCKET", vec![
			Box::from(Transform::default()),
		]);
		names.insert("PLASMA", vec![
			Box::from(Transform::default()),
		]);
		names.insert("BFG", vec![
			Box::from(Transform::default()),
		]);
		names.insert("ARACHPLAZ", vec![
			Box::from(Transform::default()),
		]);
		names.insert("PUFF", vec![
			Box::from(Transform::default()),
		]);
		names.insert("BLOOD", vec![
			Box::from(Transform::default()),
		]);
		names.insert("TFOG", vec![
			Box::from(Transform::default()),
		]);
		names.insert("IFOG", vec![
			Box::from(Transform::default()),
		]);
		names.insert("TELEPORTMAN", vec![
			Box::from(Transform::default()),
		]);
		names.insert("EXTRABFG", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC0", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC1", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC2", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC3", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC4", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC5", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC6", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC7", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC8", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC9", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC10", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC11", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC12", vec![
			Box::from(Transform::default()),
		]);
		names.insert("INV", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC13", vec![
			Box::from(Transform::default()),
		]);
		names.insert("INS", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC14", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC15", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC16", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MEGA", vec![
			Box::from(Transform::default()),
		]);
		names.insert("CLIP", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC17", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC18", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC19", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC20", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC21", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC22", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC23", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC24", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC25", vec![
			Box::from(Transform::default()),
		]);
		names.insert("CHAINGUN", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC26", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC27", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC28", vec![
			Box::from(Transform::default()),
		]);
		names.insert("SHOTGUN", vec![
			Box::from(Transform::default()),
		]);
		names.insert("SUPERSHOTGUN", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC29", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC30", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC31", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC32", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC33", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC34", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC35", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC36", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC37", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC38", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC39", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC40", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC41", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC42", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC43", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC44", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC45", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC46", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC47", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC48", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC49", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC50", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC51", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC52", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC53", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC54", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC55", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC56", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC57", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC58", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC59", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC60", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC61", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC62", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC63", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC64", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC65", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC66", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC67", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC68", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC69", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC70", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC71", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC72", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC73", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC74", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC75", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC76", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC77", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC78", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC79", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC80", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC81", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC82", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC83", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC84", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC85", vec![
			Box::from(Transform::default()),
		]);
		names.insert("MISC86", vec![
			Box::from(Transform::default()),
		]);

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

		EntityTypes {
			names,
			doomednums,
		}
	}
}
