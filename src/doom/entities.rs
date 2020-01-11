#![allow(unused_variables)]
use crate::{
	assets::{AssetHandle, AssetStorage},
	component::EntityTemplate,
	doom::components::{SpawnPoint, SpriteRender, Transform},
};
use specs::{ReadExpect, World, WriteExpect};
use std::collections::HashMap;

pub struct EntityTypes {
	pub names: HashMap<&'static str, AssetHandle<EntityTemplate>>,
	pub doomednums: HashMap<u16, AssetHandle<EntityTemplate>>,
}

impl EntityTypes {
	pub fn new(world: &World) -> EntityTypes {
		let mut template_storage = world.system_data::<WriteExpect<AssetStorage<EntityTemplate>>>();
		let mut names = HashMap::new();
		let mut doomednums = HashMap::new();

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(SpawnPoint { player_num: 1 });
			template.add_component(Transform::default());
			template
		});
		doomednums.insert(1, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(SpawnPoint { player_num: 2 });
			template.add_component(Transform::default());
			template
		});
		doomednums.insert(2, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(SpawnPoint { player_num: 3 });
			template.add_component(Transform::default());
			template
		});
		doomednums.insert(3, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(SpawnPoint { player_num: 4 });
			template.add_component(Transform::default());
			template
		});
		doomednums.insert(4, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		doomednums.insert(11, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("PLAYER", handle.clone());

		let handle = template_storage.insert({
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
		names.insert("POSSESSED", handle.clone());
		doomednums.insert(3004, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SHOTGUY", handle.clone());
		doomednums.insert(9, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("VILE", handle.clone());
		doomednums.insert(64, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("FIRE", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("UNDEAD", handle.clone());
		doomednums.insert(66, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("TRACER", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SMOKE", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("FATSO", handle.clone());
		doomednums.insert(67, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("FATSHOT", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("CHAINGUY", handle.clone());
		doomednums.insert(65, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("TROOP", handle.clone());
		doomednums.insert(3001, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SERGEANT", handle.clone());
		doomednums.insert(3002, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SHADOWS", handle.clone());
		doomednums.insert(58, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("HEAD", handle.clone());
		doomednums.insert(3005, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BRUISER", handle.clone());
		doomednums.insert(3003, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BRUISERSHOT", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("KNIGHT", handle.clone());
		doomednums.insert(69, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SKULL", handle.clone());
		doomednums.insert(3006, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SPIDER", handle.clone());
		doomednums.insert(7, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BABY", handle.clone());
		doomednums.insert(68, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("CYBORG", handle.clone());
		doomednums.insert(16, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("PAIN", handle.clone());
		doomednums.insert(71, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("WOLFSS", handle.clone());
		doomednums.insert(84, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("KEEN", handle.clone());
		doomednums.insert(72, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BOSSBRAIN", handle.clone());
		doomednums.insert(88, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BOSSSPIT", handle.clone());
		doomednums.insert(89, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BOSSTARGET", handle.clone());
		doomednums.insert(87, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SPAWNSHOT", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SPAWNFIRE", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BARREL", handle.clone());
		doomednums.insert(2035, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("TROOPSHOT", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("HEADSHOT", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("ROCKET", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("PLASMA", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BFG", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("ARACHPLAZ", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("PUFF", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("BLOOD", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("TFOG", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("IFOG", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("TELEPORTMAN", handle.clone());
		doomednums.insert(14, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("EXTRABFG", handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC0", handle.clone());
		doomednums.insert(2018, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC1", handle.clone());
		doomednums.insert(2019, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC2", handle.clone());
		doomednums.insert(2014, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC3", handle.clone());
		doomednums.insert(2015, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC4", handle.clone());
		doomednums.insert(5, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC5", handle.clone());
		doomednums.insert(13, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC6", handle.clone());
		doomednums.insert(6, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC7", handle.clone());
		doomednums.insert(39, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC8", handle.clone());
		doomednums.insert(38, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC9", handle.clone());
		doomednums.insert(40, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC10", handle.clone());
		doomednums.insert(2011, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC11", handle.clone());
		doomednums.insert(2012, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC12", handle.clone());
		doomednums.insert(2013, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("INV", handle.clone());
		doomednums.insert(2022, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC13", handle.clone());
		doomednums.insert(2023, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("INS", handle.clone());
		doomednums.insert(2024, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC14", handle.clone());
		doomednums.insert(2025, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC15", handle.clone());
		doomednums.insert(2026, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC16", handle.clone());
		doomednums.insert(2045, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MEGA", handle.clone());
		doomednums.insert(83, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("CLIP", handle.clone());
		doomednums.insert(2007, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC17", handle.clone());
		doomednums.insert(2048, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC18", handle.clone());
		doomednums.insert(2010, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC19", handle.clone());
		doomednums.insert(2046, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC20", handle.clone());
		doomednums.insert(2047, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC21", handle.clone());
		doomednums.insert(17, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC22", handle.clone());
		doomednums.insert(2008, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC23", handle.clone());
		doomednums.insert(2049, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC24", handle.clone());
		doomednums.insert(8, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC25", handle.clone());
		doomednums.insert(2006, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("CHAINGUN", handle.clone());
		doomednums.insert(2002, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC26", handle.clone());
		doomednums.insert(2005, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC27", handle.clone());
		doomednums.insert(2003, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC28", handle.clone());
		doomednums.insert(2004, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SHOTGUN", handle.clone());
		doomednums.insert(2001, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("SUPERSHOTGUN", handle.clone());
		doomednums.insert(82, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC29", handle.clone());
		doomednums.insert(85, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC30", handle.clone());
		doomednums.insert(86, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC31", handle.clone());
		doomednums.insert(2028, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC32", handle.clone());
		doomednums.insert(30, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC33", handle.clone());
		doomednums.insert(31, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC34", handle.clone());
		doomednums.insert(32, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC35", handle.clone());
		doomednums.insert(33, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC36", handle.clone());
		doomednums.insert(37, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC37", handle.clone());
		doomednums.insert(36, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC38", handle.clone());
		doomednums.insert(41, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC39", handle.clone());
		doomednums.insert(42, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC40", handle.clone());
		doomednums.insert(43, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC41", handle.clone());
		doomednums.insert(44, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC42", handle.clone());
		doomednums.insert(45, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC43", handle.clone());
		doomednums.insert(46, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC44", handle.clone());
		doomednums.insert(55, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC45", handle.clone());
		doomednums.insert(56, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC46", handle.clone());
		doomednums.insert(57, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC47", handle.clone());
		doomednums.insert(47, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC48", handle.clone());
		doomednums.insert(48, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC49", handle.clone());
		doomednums.insert(34, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC50", handle.clone());
		doomednums.insert(35, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC51", handle.clone());
		doomednums.insert(49, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC52", handle.clone());
		doomednums.insert(50, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC53", handle.clone());
		doomednums.insert(51, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC54", handle.clone());
		doomednums.insert(52, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC55", handle.clone());
		doomednums.insert(53, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC56", handle.clone());
		doomednums.insert(59, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC57", handle.clone());
		doomednums.insert(60, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC58", handle.clone());
		doomednums.insert(61, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC59", handle.clone());
		doomednums.insert(62, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC60", handle.clone());
		doomednums.insert(63, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC61", handle.clone());
		doomednums.insert(22, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC62", handle.clone());
		doomednums.insert(15, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC63", handle.clone());
		doomednums.insert(18, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC64", handle.clone());
		doomednums.insert(21, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC65", handle.clone());
		doomednums.insert(23, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC66", handle.clone());
		doomednums.insert(20, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC67", handle.clone());
		doomednums.insert(19, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC68", handle.clone());
		doomednums.insert(10, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC69", handle.clone());
		doomednums.insert(12, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC70", handle.clone());
		doomednums.insert(28, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC71", handle.clone());
		doomednums.insert(24, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC72", handle.clone());
		doomednums.insert(27, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC73", handle.clone());
		doomednums.insert(29, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC74", handle.clone());
		doomednums.insert(25, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC75", handle.clone());
		doomednums.insert(26, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC76", handle.clone());
		doomednums.insert(54, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC77", handle.clone());
		doomednums.insert(70, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC78", handle.clone());
		doomednums.insert(73, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC79", handle.clone());
		doomednums.insert(74, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC80", handle.clone());
		doomednums.insert(75, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC81", handle.clone());
		doomednums.insert(76, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC82", handle.clone());
		doomednums.insert(77, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC83", handle.clone());
		doomednums.insert(78, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC84", handle.clone());
		doomednums.insert(79, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC85", handle.clone());
		doomednums.insert(80, handle.clone());

		let handle = template_storage.insert({
			let mut template = EntityTemplate::new();
			template.add_component(Transform::default());
			template
		});
		names.insert("MISC86", handle.clone());
		doomednums.insert(81, handle.clone());

		EntityTypes { names, doomednums }
	}
}
