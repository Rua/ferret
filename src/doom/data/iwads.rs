use crate::doom::iwad::{IWADInfo, MapInfo};
use once_cell::sync::Lazy;
use std::{collections::HashMap, iter::FromIterator};

pub static IWADINFO: Lazy<Vec<IWADInfo>> = Lazy::new(|| {
	vec![
		IWADInfo {
			files: &["doom2.wad"],
			name: "Doom II",
			map: "map01",
			weapons: &[
				"fist.weapon",
				"chainsaw.weapon",
				"pistol.weapon",
				"shotgun.weapon",
				"supershotgun.weapon",
				"chaingun.weapon",
				"missile.weapon",
				"plasma.weapon",
				"bfg.weapon",
			],
			maps: HashMap::from_iter(std::array::IntoIter::new([
				(
					"map01.map",
					MapInfo {
						name: "level 1: entryway",
						sky: "rsky1.patch",
						music: "runnin.music",
						exit: Some("map02.map"),
						secret_exit: None,
					},
				),
				(
					"map02.map",
					MapInfo {
						name: "level 2: underhalls",
						sky: "rsky1.patch",
						music: "stalks.music",
						exit: Some("map03.map"),
						secret_exit: None,
					},
				),
				(
					"map03.map",
					MapInfo {
						name: "level 3: the gantlet",
						sky: "rsky1.patch",
						music: "countd.music",
						exit: Some("map04.map"),
						secret_exit: None,
					},
				),
				(
					"map04.map",
					MapInfo {
						name: "level 4: the focus",
						sky: "rsky1.patch",
						music: "betwee.music",
						exit: Some("map05.map"),
						secret_exit: None,
					},
				),
				(
					"map05.map",
					MapInfo {
						name: "level 5: the waste tunnels",
						sky: "rsky1.patch",
						music: "doom.music",
						exit: Some("map06.map"),
						secret_exit: None,
					},
				),
				(
					"map06.map",
					MapInfo {
						name: "level 6: the crusher",
						sky: "rsky1.patch",
						music: "the_da.music",
						exit: Some("map07.map"),
						secret_exit: None,
					},
				),
				(
					"map07.map",
					MapInfo {
						name: "level 7: dead simple",
						sky: "rsky1.patch",
						music: "shawn.music",
						exit: Some("map08.map"),
						secret_exit: None,
					},
				),
				(
					"map08.map",
					MapInfo {
						name: "level 8: tricks and traps",
						sky: "rsky1.patch",
						music: "ddtblu.music",
						exit: Some("map09.map"),
						secret_exit: None,
					},
				),
				(
					"map09.map",
					MapInfo {
						name: "level 9: the pit",
						sky: "rsky1.patch",
						music: "in_cit.music",
						exit: Some("map10.map"),
						secret_exit: None,
					},
				),
				(
					"map10.map",
					MapInfo {
						name: "level 10: refueling base",
						sky: "rsky1.patch",
						music: "dead.music",
						exit: Some("map11.map"),
						secret_exit: None,
					},
				),
				(
					"map11.map",
					MapInfo {
						name: "level 11: 'o' of destruction!",
						sky: "rsky1.patch",
						music: "stlks2.music",
						exit: Some("map12.map"),
						secret_exit: None,
					},
				),
				(
					"map12.map",
					MapInfo {
						name: "level 12: the factory",
						sky: "rsky2.patch",
						music: "theda2.music",
						exit: Some("map13.map"),
						secret_exit: None,
					},
				),
				(
					"map13.map",
					MapInfo {
						name: "level 13: downtown",
						sky: "rsky2.patch",
						music: "doom2.music",
						exit: Some("map14.map"),
						secret_exit: None,
					},
				),
				(
					"map14.map",
					MapInfo {
						name: "level 14: the inmost dens",
						sky: "rsky2.patch",
						music: "ddtbl2.music",
						exit: Some("map15.map"),
						secret_exit: None,
					},
				),
				(
					"map15.map",
					MapInfo {
						name: "level 15: industrial zone",
						sky: "rsky2.patch",
						music: "runni2.music",
						exit: Some("map16.map"),
						secret_exit: Some("map31.map"),
					},
				),
				(
					"map16.map",
					MapInfo {
						name: "level 16: suburbs",
						sky: "rsky2.patch",
						music: "dead2.music",
						exit: Some("map17.map"),
						secret_exit: None,
					},
				),
				(
					"map17.map",
					MapInfo {
						name: "level 17: tenements",
						sky: "rsky2.patch",
						music: "stlks3.music",
						exit: Some("map18.map"),
						secret_exit: None,
					},
				),
				(
					"map18.map",
					MapInfo {
						name: "level 18: the courtyard",
						sky: "rsky2.patch",
						music: "romero.music",
						exit: Some("map19.map"),
						secret_exit: None,
					},
				),
				(
					"map19.map",
					MapInfo {
						name: "level 19: the citadel",
						sky: "rsky2.patch",
						music: "shawn2.music",
						exit: Some("map20.map"),
						secret_exit: None,
					},
				),
				(
					"map20.map",
					MapInfo {
						name: "level 20: gotcha!",
						sky: "rsky2.patch",
						music: "messag.music",
						exit: Some("map21.map"),
						secret_exit: None,
					},
				),
				(
					"map21.map",
					MapInfo {
						name: "level 21: nirvana",
						sky: "rsky3.patch",
						music: "count2.music",
						exit: Some("map22.map"),
						secret_exit: None,
					},
				),
				(
					"map22.map",
					MapInfo {
						name: "level 22: the catacombs",
						sky: "rsky3.patch",
						music: "ddtbl3.music",
						exit: Some("map23.map"),
						secret_exit: None,
					},
				),
				(
					"map23.map",
					MapInfo {
						name: "level 23: barrels o' fun",
						sky: "rsky3.patch",
						music: "ampie.music",
						exit: Some("map24.map"),
						secret_exit: None,
					},
				),
				(
					"map24.map",
					MapInfo {
						name: "level 24: the chasm",
						sky: "rsky3.patch",
						music: "theda3.music",
						exit: Some("map25.map"),
						secret_exit: None,
					},
				),
				(
					"map25.map",
					MapInfo {
						name: "level 25: bloodfalls",
						sky: "rsky3.patch",
						music: "adrian.music",
						exit: Some("map26.map"),
						secret_exit: None,
					},
				),
				(
					"map26.map",
					MapInfo {
						name: "level 26: the abandoned mines",
						sky: "rsky3.patch",
						music: "messg2.music",
						exit: Some("map27.map"),
						secret_exit: None,
					},
				),
				(
					"map27.map",
					MapInfo {
						name: "level 27: monster condo",
						sky: "rsky3.patch",
						music: "romer2.music",
						exit: Some("map28.map"),
						secret_exit: None,
					},
				),
				(
					"map28.map",
					MapInfo {
						name: "level 28: the spirit world",
						sky: "rsky3.patch",
						music: "tense.music",
						exit: Some("map29.map"),
						secret_exit: None,
					},
				),
				(
					"map29.map",
					MapInfo {
						name: "level 29: the living end",
						sky: "rsky3.patch",
						music: "shawn3.music",
						exit: Some("map30.map"),
						secret_exit: None,
					},
				),
				(
					"map30.map",
					MapInfo {
						name: "level 30: icon of sin",
						sky: "rsky3.patch",
						music: "openin.music",
						exit: None,
						secret_exit: None,
					},
				),
				(
					"map31.map",
					MapInfo {
						name: "level 31: wolfenstein",
						sky: "rsky3.patch",
						music: "evil.music",
						exit: Some("map16.map"),
						secret_exit: Some("map32.map"),
					},
				),
				(
					"map32.map",
					MapInfo {
						name: "level 32: grosse",
						sky: "rsky3.patch",
						music: "ultima.music",
						exit: Some("map16.map"),
						secret_exit: None,
					},
				),
			])),
		},
		IWADInfo {
			files: &["plutonia.wad"],
			name: "The Plutonia Experiment",
			map: "map01",
			weapons: &[
				"fist.weapon",
				"chainsaw.weapon",
				"pistol.weapon",
				"shotgun.weapon",
				"supershotgun.weapon",
				"chaingun.weapon",
				"missile.weapon",
				"plasma.weapon",
				"bfg.weapon",
			],
			maps: HashMap::from_iter(std::array::IntoIter::new([
				(
					"map01.map",
					MapInfo {
						name: "level 1: congo",
						sky: "rsky1.patch",
						music: "runnin.music",
						exit: Some("map02.map"),
						secret_exit: None,
					},
				),
				(
					"map02.map",
					MapInfo {
						name: "level 2: well of souls",
						sky: "rsky1.patch",
						music: "stalks.music",
						exit: Some("map03.map"),
						secret_exit: None,
					},
				),
				(
					"map03.map",
					MapInfo {
						name: "level 3: aztec",
						sky: "rsky1.patch",
						music: "countd.music",
						exit: Some("map04.map"),
						secret_exit: None,
					},
				),
				(
					"map04.map",
					MapInfo {
						name: "level 4: caged",
						sky: "rsky1.patch",
						music: "betwee.music",
						exit: Some("map05.map"),
						secret_exit: None,
					},
				),
				(
					"map05.map",
					MapInfo {
						name: "level 5: ghost town",
						sky: "rsky1.patch",
						music: "doom.music",
						exit: Some("map06.map"),
						secret_exit: None,
					},
				),
				(
					"map06.map",
					MapInfo {
						name: "level 6: baron's lair",
						sky: "rsky1.patch",
						music: "the_da.music",
						exit: Some("map07.map"),
						secret_exit: None,
					},
				),
				(
					"map07.map",
					MapInfo {
						name: "level 7: caughtyard",
						sky: "rsky1.patch",
						music: "shawn.music",
						exit: Some("map08.map"),
						secret_exit: None,
					},
				),
				(
					"map08.map",
					MapInfo {
						name: "level 8: realm",
						sky: "rsky1.patch",
						music: "ddtblu.music",
						exit: Some("map09.map"),
						secret_exit: None,
					},
				),
				(
					"map09.map",
					MapInfo {
						name: "level 9: abattoire",
						sky: "rsky1.patch",
						music: "in_cit.music",
						exit: Some("map10.map"),
						secret_exit: None,
					},
				),
				(
					"map10.map",
					MapInfo {
						name: "level 10: onslaught",
						sky: "rsky1.patch",
						music: "dead.music",
						exit: Some("map11.map"),
						secret_exit: None,
					},
				),
				(
					"map11.map",
					MapInfo {
						name: "level 11: hunted",
						sky: "rsky1.patch",
						music: "stlks2.music",
						exit: Some("map12.map"),
						secret_exit: None,
					},
				),
				(
					"map12.map",
					MapInfo {
						name: "level 12: speed",
						sky: "rsky2.patch",
						music: "theda2.music",
						exit: Some("map13.map"),
						secret_exit: None,
					},
				),
				(
					"map13.map",
					MapInfo {
						name: "level 13: the crypt",
						sky: "rsky2.patch",
						music: "doom2.music",
						exit: Some("map14.map"),
						secret_exit: None,
					},
				),
				(
					"map14.map",
					MapInfo {
						name: "level 14: genesis",
						sky: "rsky2.patch",
						music: "ddtbl2.music",
						exit: Some("map15.map"),
						secret_exit: None,
					},
				),
				(
					"map15.map",
					MapInfo {
						name: "level 15: the twilight",
						sky: "rsky2.patch",
						music: "runni2.music",
						exit: Some("map16.map"),
						secret_exit: Some("map31.map"),
					},
				),
				(
					"map16.map",
					MapInfo {
						name: "level 16: the omen",
						sky: "rsky2.patch",
						music: "dead2.music",
						exit: Some("map17.map"),
						secret_exit: None,
					},
				),
				(
					"map17.map",
					MapInfo {
						name: "level 17: compound",
						sky: "rsky2.patch",
						music: "stlks3.music",
						exit: Some("map18.map"),
						secret_exit: None,
					},
				),
				(
					"map18.map",
					MapInfo {
						name: "level 18: neurosphere",
						sky: "rsky2.patch",
						music: "romero.music",
						exit: Some("map19.map"),
						secret_exit: None,
					},
				),
				(
					"map19.map",
					MapInfo {
						name: "level 19: nme",
						sky: "rsky2.patch",
						music: "shawn2.music",
						exit: Some("map20.map"),
						secret_exit: None,
					},
				),
				(
					"map20.map",
					MapInfo {
						name: "level 20: the death domain",
						sky: "rsky2.patch",
						music: "messag.music",
						exit: Some("map21.map"),
						secret_exit: None,
					},
				),
				(
					"map21.map",
					MapInfo {
						name: "level 21: slayer",
						sky: "rsky3.patch",
						music: "count2.music",
						exit: Some("map22.map"),
						secret_exit: None,
					},
				),
				(
					"map22.map",
					MapInfo {
						name: "level 22: impossible mission",
						sky: "rsky3.patch",
						music: "ddtbl3.music",
						exit: Some("map23.map"),
						secret_exit: None,
					},
				),
				(
					"map23.map",
					MapInfo {
						name: "level 23: tombstone",
						sky: "rsky3.patch",
						music: "ampie.music",
						exit: Some("map24.map"),
						secret_exit: None,
					},
				),
				(
					"map24.map",
					MapInfo {
						name: "level 24: the final frontier",
						sky: "rsky3.patch",
						music: "theda3.music",
						exit: Some("map25.map"),
						secret_exit: None,
					},
				),
				(
					"map25.map",
					MapInfo {
						name: "level 25: the temple of darkness",
						sky: "rsky3.patch",
						music: "adrian.music",
						exit: Some("map26.map"),
						secret_exit: None,
					},
				),
				(
					"map26.map",
					MapInfo {
						name: "level 26: bunker",
						sky: "rsky3.patch",
						music: "messg2.music",
						exit: Some("map27.map"),
						secret_exit: None,
					},
				),
				(
					"map27.map",
					MapInfo {
						name: "level 27: anti-christ",
						sky: "rsky3.patch",
						music: "romer2.music",
						exit: Some("map28.map"),
						secret_exit: None,
					},
				),
				(
					"map28.map",
					MapInfo {
						name: "level 28: the sewers",
						sky: "rsky3.patch",
						music: "tense.music",
						exit: Some("map29.map"),
						secret_exit: None,
					},
				),
				(
					"map29.map",
					MapInfo {
						name: "level 29: odyssey of noises",
						sky: "rsky3.patch",
						music: "shawn3.music",
						exit: Some("map30.map"),
						secret_exit: None,
					},
				),
				(
					"map30.map",
					MapInfo {
						name: "level 30: the gateway of hell",
						sky: "rsky3.patch",
						music: "openin.music",
						exit: None,
						secret_exit: None,
					},
				),
				(
					"map31.map",
					MapInfo {
						name: "level 31: cyberden",
						sky: "rsky3.patch",
						music: "evil.music",
						exit: Some("map16.map"),
						secret_exit: Some("map32.map"),
					},
				),
				(
					"map32.map",
					MapInfo {
						name: "level 32: go 2 it",
						sky: "rsky3.patch",
						music: "ultima.music",
						exit: Some("map16.map"),
						secret_exit: None,
					},
				),
			])),
		},
		IWADInfo {
			files: &["tnt.wad"],
			name: "TNT: Evilution",
			map: "map01",
			weapons: &[
				"fist.weapon",
				"chainsaw.weapon",
				"pistol.weapon",
				"shotgun.weapon",
				"supershotgun.weapon",
				"chaingun.weapon",
				"missile.weapon",
				"plasma.weapon",
				"bfg.weapon",
			],
			maps: HashMap::from_iter(std::array::IntoIter::new([
				(
					"map01.map",
					MapInfo {
						name: "level 1: system control",
						sky: "rsky1.patch",
						music: "runnin.music",
						exit: Some("map02.map"),
						secret_exit: None,
					},
				),
				(
					"map02.map",
					MapInfo {
						name: "level 2: human bbq",
						sky: "rsky1.patch",
						music: "stalks.music",
						exit: Some("map03.map"),
						secret_exit: None,
					},
				),
				(
					"map03.map",
					MapInfo {
						name: "level 3: power control",
						sky: "rsky1.patch",
						music: "countd.music",
						exit: Some("map04.map"),
						secret_exit: None,
					},
				),
				(
					"map04.map",
					MapInfo {
						name: "level 4: wormhole",
						sky: "rsky1.patch",
						music: "betwee.music",
						exit: Some("map05.map"),
						secret_exit: None,
					},
				),
				(
					"map05.map",
					MapInfo {
						name: "level 5: hanger",
						sky: "rsky1.patch",
						music: "doom.music",
						exit: Some("map06.map"),
						secret_exit: None,
					},
				),
				(
					"map06.map",
					MapInfo {
						name: "level 6: open season",
						sky: "rsky1.patch",
						music: "the_da.music",
						exit: Some("map07.map"),
						secret_exit: None,
					},
				),
				(
					"map07.map",
					MapInfo {
						name: "level 7: prison",
						sky: "rsky1.patch",
						music: "shawn.music",
						exit: Some("map08.map"),
						secret_exit: None,
					},
				),
				(
					"map08.map",
					MapInfo {
						name: "level 8: metal",
						sky: "rsky1.patch",
						music: "ddtblu.music",
						exit: Some("map09.map"),
						secret_exit: None,
					},
				),
				(
					"map09.map",
					MapInfo {
						name: "level 9: stronghold",
						sky: "rsky1.patch",
						music: "in_cit.music",
						exit: Some("map10.map"),
						secret_exit: None,
					},
				),
				(
					"map10.map",
					MapInfo {
						name: "level 10: redemption",
						sky: "rsky1.patch",
						music: "dead.music",
						exit: Some("map11.map"),
						secret_exit: None,
					},
				),
				(
					"map11.map",
					MapInfo {
						name: "level 11: storage facility",
						sky: "rsky1.patch",
						music: "stlks2.music",
						exit: Some("map12.map"),
						secret_exit: None,
					},
				),
				(
					"map12.map",
					MapInfo {
						name: "level 12: crater",
						sky: "rsky2.patch",
						music: "theda2.music",
						exit: Some("map13.map"),
						secret_exit: None,
					},
				),
				(
					"map13.map",
					MapInfo {
						name: "level 13: nukage processing",
						sky: "rsky2.patch",
						music: "doom2.music",
						exit: Some("map14.map"),
						secret_exit: None,
					},
				),
				(
					"map14.map",
					MapInfo {
						name: "level 14: steel works",
						sky: "rsky2.patch",
						music: "ddtbl2.music",
						exit: Some("map15.map"),
						secret_exit: None,
					},
				),
				(
					"map15.map",
					MapInfo {
						name: "level 15: dead zone",
						sky: "rsky2.patch",
						music: "runni2.music",
						exit: Some("map16.map"),
						secret_exit: Some("map31.map"),
					},
				),
				(
					"map16.map",
					MapInfo {
						name: "level 16: deepest reaches",
						sky: "rsky2.patch",
						music: "dead2.music",
						exit: Some("map17.map"),
						secret_exit: None,
					},
				),
				(
					"map17.map",
					MapInfo {
						name: "level 17: processing area",
						sky: "rsky2.patch",
						music: "stlks3.music",
						exit: Some("map18.map"),
						secret_exit: None,
					},
				),
				(
					"map18.map",
					MapInfo {
						name: "level 18: mill",
						sky: "rsky2.patch",
						music: "romero.music",
						exit: Some("map19.map"),
						secret_exit: None,
					},
				),
				(
					"map19.map",
					MapInfo {
						name: "level 19: shipping/respawning",
						sky: "rsky2.patch",
						music: "shawn2.music",
						exit: Some("map20.map"),
						secret_exit: None,
					},
				),
				(
					"map20.map",
					MapInfo {
						name: "level 20: central processing",
						sky: "rsky2.patch",
						music: "messag.music",
						exit: Some("map21.map"),
						secret_exit: None,
					},
				),
				(
					"map21.map",
					MapInfo {
						name: "level 21: administration center",
						sky: "rsky3.patch",
						music: "count2.music",
						exit: Some("map22.map"),
						secret_exit: None,
					},
				),
				(
					"map22.map",
					MapInfo {
						name: "level 22: habitat",
						sky: "rsky3.patch",
						music: "ddtbl3.music",
						exit: Some("map23.map"),
						secret_exit: None,
					},
				),
				(
					"map23.map",
					MapInfo {
						name: "level 23: lunar mining project",
						sky: "rsky3.patch",
						music: "ampie.music",
						exit: Some("map24.map"),
						secret_exit: None,
					},
				),
				(
					"map24.map",
					MapInfo {
						name: "level 24: quarry",
						sky: "rsky3.patch",
						music: "theda3.music",
						exit: Some("map25.map"),
						secret_exit: None,
					},
				),
				(
					"map25.map",
					MapInfo {
						name: "level 25: baron's den",
						sky: "rsky3.patch",
						music: "adrian.music",
						exit: Some("map26.map"),
						secret_exit: None,
					},
				),
				(
					"map26.map",
					MapInfo {
						name: "level 26: ballistyx",
						sky: "rsky3.patch",
						music: "messg2.music",
						exit: Some("map27.map"),
						secret_exit: None,
					},
				),
				(
					"map27.map",
					MapInfo {
						name: "level 27: mount pain",
						sky: "rsky3.patch",
						music: "romer2.music",
						exit: Some("map28.map"),
						secret_exit: None,
					},
				),
				(
					"map28.map",
					MapInfo {
						name: "level 28: heck",
						sky: "rsky3.patch",
						music: "tense.music",
						exit: Some("map29.map"),
						secret_exit: None,
					},
				),
				(
					"map29.map",
					MapInfo {
						name: "level 29: river styx",
						sky: "rsky3.patch",
						music: "shawn3.music",
						exit: Some("map30.map"),
						secret_exit: None,
					},
				),
				(
					"map30.map",
					MapInfo {
						name: "level 30: last call",
						sky: "rsky3.patch",
						music: "openin.music",
						exit: None,
						secret_exit: None,
					},
				),
				(
					"map31.map",
					MapInfo {
						name: "level 31: pharaoh",
						sky: "rsky3.patch",
						music: "evil.music",
						exit: Some("map16.map"),
						secret_exit: Some("map32.map"),
					},
				),
				(
					"map32.map",
					MapInfo {
						name: "level 32: caribbean",
						sky: "rsky3.patch",
						music: "ultima.music",
						exit: Some("map16.map"),
						secret_exit: None,
					},
				),
			])),
		},
		IWADInfo {
			files: &["doom.wad", "doomu.wad"],
			name: "Doom",
			map: "e1m1",
			weapons: &[
				"fist.weapon",
				"chainsaw.weapon",
				"pistol.weapon",
				"shotgun.weapon",
				"chaingun.weapon",
				"missile.weapon",
				"plasma.weapon",
				"bfg.weapon",
			],
			maps: HashMap::from_iter(std::array::IntoIter::new([
				(
					"e1m1.map",
					MapInfo {
						name: "E1M1: Hangar",
						sky: "sky1.patch",
						music: "e1m1.music",
						exit: Some("e1m2.map"),
						secret_exit: None,
					},
				),
				(
					"e1m2.map",
					MapInfo {
						name: "E1M2: Nuclear Plant",
						sky: "sky1.patch",
						music: "e1m2.music",
						exit: Some("e1m3.map"),
						secret_exit: None,
					},
				),
				(
					"e1m3.map",
					MapInfo {
						name: "E1M3: Toxin Refinery",
						sky: "sky1.patch",
						music: "e1m3.music",
						exit: Some("e1m4.map"),
						secret_exit: Some("e1m9.map"),
					},
				),
				(
					"e1m4.map",
					MapInfo {
						name: "E1M4: Command Control",
						sky: "sky1.patch",
						music: "e1m4.music",
						exit: Some("e1m5.map"),
						secret_exit: None,
					},
				),
				(
					"e1m5.map",
					MapInfo {
						name: "E1M5: Phobos Lab",
						sky: "sky1.patch",
						music: "e1m5.music",
						exit: Some("e1m6.map"),
						secret_exit: None,
					},
				),
				(
					"e1m6.map",
					MapInfo {
						name: "E1M6: Central Processing",
						sky: "sky1.patch",
						music: "e1m6.music",
						exit: Some("e1m7.map"),
						secret_exit: None,
					},
				),
				(
					"e1m7.map",
					MapInfo {
						name: "E1M7: Computer Station",
						sky: "sky1.patch",
						music: "e1m7.music",
						exit: Some("e1m8.map"),
						secret_exit: None,
					},
				),
				(
					"e1m8.map",
					MapInfo {
						name: "E1M8: Phobos Anomaly",
						sky: "sky1.patch",
						music: "e1m8.music",
						exit: None,
						secret_exit: None,
					},
				),
				(
					"e1m9.map",
					MapInfo {
						name: "E1M9: Military Base",
						sky: "sky1.patch",
						music: "e1m9.music",
						exit: Some("e1m4.map"),
						secret_exit: None,
					},
				),
				(
					"e2m1.map",
					MapInfo {
						name: "E2M1: Deimos Anomaly",
						sky: "sky2.patch",
						music: "e2m1.music",
						exit: Some("e2m2.map"),
						secret_exit: None,
					},
				),
				(
					"e2m2.map",
					MapInfo {
						name: "E2M2: Containment Area",
						sky: "sky2.patch",
						music: "e2m2.music",
						exit: Some("e2m3.map"),
						secret_exit: None,
					},
				),
				(
					"e2m3.map",
					MapInfo {
						name: "E2M3: Refinery",
						sky: "sky2.patch",
						music: "e2m3.music",
						exit: Some("e2m4.map"),
						secret_exit: None,
					},
				),
				(
					"e2m4.map",
					MapInfo {
						name: "E2M4: Deimos Lab",
						sky: "sky2.patch",
						music: "e2m4.music",
						exit: Some("e2m5.map"),
						secret_exit: None,
					},
				),
				(
					"e2m5.map",
					MapInfo {
						name: "E2M5: Command Center",
						sky: "sky2.patch",
						music: "e2m5.music",
						exit: Some("e2m6.map"),
						secret_exit: Some("e2m9.map"),
					},
				),
				(
					"e2m6.map",
					MapInfo {
						name: "E2M6: Halls of the Damned",
						sky: "sky2.patch",
						music: "e2m6.music",
						exit: Some("e2m7.map"),
						secret_exit: None,
					},
				),
				(
					"e2m7.map",
					MapInfo {
						name: "E2M7: Spawning Vats",
						sky: "sky2.patch",
						music: "e2m7.music",
						exit: Some("e2m8.map"),
						secret_exit: None,
					},
				),
				(
					"e2m8.map",
					MapInfo {
						name: "E2M8: Tower of Babel",
						sky: "sky2.patch",
						music: "e2m8.music",
						exit: None,
						secret_exit: None,
					},
				),
				(
					"e2m9.map",
					MapInfo {
						name: "E2M9: Fortress of Mystery",
						sky: "sky2.patch",
						music: "e2m9.music",
						exit: Some("e2m6.map"),
						secret_exit: None,
					},
				),
				(
					"e3m1.map",
					MapInfo {
						name: "E3M1: Hell Keep",
						sky: "sky3.patch",
						music: "e3m1.music",
						exit: Some("e3m2.map"),
						secret_exit: None,
					},
				),
				(
					"e3m2.map",
					MapInfo {
						name: "E3M2: Slough of Despair",
						sky: "sky3.patch",
						music: "e3m2.music",
						exit: Some("e3m3.map"),
						secret_exit: None,
					},
				),
				(
					"e3m3.map",
					MapInfo {
						name: "E3M3: Pandemonium",
						sky: "sky3.patch",
						music: "e3m3.music",
						exit: Some("e3m4.map"),
						secret_exit: None,
					},
				),
				(
					"e3m4.map",
					MapInfo {
						name: "E3M4: House of Pain",
						sky: "sky3.patch",
						music: "e3m4.music",
						exit: Some("e3m5.map"),
						secret_exit: None,
					},
				),
				(
					"e3m5.map",
					MapInfo {
						name: "E3M5: Unholy Cathedral",
						sky: "sky3.patch",
						music: "e3m5.music",
						exit: Some("e3m6.map"),
						secret_exit: None,
					},
				),
				(
					"e3m6.map",
					MapInfo {
						name: "E3M6: Mt. Erebus",
						sky: "sky3.patch",
						music: "e3m6.music",
						exit: Some("e3m7.map"),
						secret_exit: Some("e3m9.map"),
					},
				),
				(
					"e3m7.map",
					MapInfo {
						name: "E3M7: Limbo",
						sky: "sky3.patch",
						music: "e3m7.music",
						exit: Some("e3m8.map"),
						secret_exit: None,
					},
				),
				(
					"e3m8.map",
					MapInfo {
						name: "E3M8: Dis",
						sky: "sky3.patch",
						music: "em.music",
						exit: None,
						secret_exit: None,
					},
				),
				(
					"e3m9.map",
					MapInfo {
						name: "E3M9: Warrens",
						sky: "sky3.patch",
						music: "e3m9.music",
						exit: Some("e3m7.map"),
						secret_exit: None,
					},
				),
				(
					"e4m1.map",
					MapInfo {
						name: "E4M1: Hell Beneath",
						sky: "sky4.patch",
						music: "e3m4.music",
						exit: Some("e4m2.map"),
						secret_exit: None,
					},
				),
				(
					"e4m2.map",
					MapInfo {
						name: "E4M2: Perfect Hatred",
						sky: "sky4.patch",
						music: "e3m2.music",
						exit: Some("e4m3.map"),
						secret_exit: Some("e4m9.map"),
					},
				),
				(
					"e4m3.map",
					MapInfo {
						name: "E4M3: Sever The Wicked",
						sky: "sky4.patch",
						music: "e3m3.music",
						exit: Some("e4m4.map"),
						secret_exit: None,
					},
				),
				(
					"e4m4.map",
					MapInfo {
						name: "E4M4: Unruly Evil",
						sky: "sky4.patch",
						music: "e1m5.music",
						exit: Some("e4m5.map"),
						secret_exit: None,
					},
				),
				(
					"e4m5.map",
					MapInfo {
						name: "E4M5: They Will Repent",
						sky: "sky4.patch",
						music: "e2m7.music",
						exit: Some("e4m6.map"),
						secret_exit: None,
					},
				),
				(
					"e4m6.map",
					MapInfo {
						name: "E4M6: Against Thee Wickedly",
						sky: "sky4.patch",
						music: "e2m4.music",
						exit: Some("e4m7.map"),
						secret_exit: None,
					},
				),
				(
					"e4m7.map",
					MapInfo {
						name: "E4M7: And Hell Followed",
						sky: "sky4.patch",
						music: "e2m6.music",
						exit: Some("e4m8.map"),
						secret_exit: None,
					},
				),
				(
					"e4m8.map",
					MapInfo {
						name: "E4M8: Unto The Cruel",
						sky: "sky4.patch",
						music: "e2m5.music",
						exit: None,
						secret_exit: None,
					},
				),
				(
					"e4m9.map",
					MapInfo {
						name: "E4M9: Fear",
						sky: "sky4.patch",
						music: "e1m9.music",
						exit: Some("e4m3.map"),
						secret_exit: None,
					},
				),
			])),
		},
		IWADInfo {
			files: &["doom1.wad"],
			name: "Doom Shareware",
			map: "e1m1",
			weapons: &[
				"fist.weapon",
				"chainsaw.weapon",
				"pistol.weapon",
				"shotgun.weapon",
				"chaingun.weapon",
				"missile.weapon",
			],
			maps: HashMap::from_iter(std::array::IntoIter::new([
				(
					"e1m1.map",
					MapInfo {
						name: "E1M1: Hangar",
						sky: "sky1.patch",
						music: "e1m1.music",
						exit: Some("e1m2.map"),
						secret_exit: None,
					},
				),
				(
					"e1m2.map",
					MapInfo {
						name: "E1M2: Nuclear Plant",
						sky: "sky1.patch",
						music: "e1m2.music",
						exit: Some("e1m3.map"),
						secret_exit: None,
					},
				),
				(
					"e1m3.map",
					MapInfo {
						name: "E1M3: Toxin Refinery",
						sky: "sky1.patch",
						music: "e1m3.music",
						exit: Some("e1m4.map"),
						secret_exit: Some("e1m9.map"),
					},
				),
				(
					"e1m4.map",
					MapInfo {
						name: "E1M4: Command Control",
						sky: "sky1.patch",
						music: "e1m4.music",
						exit: Some("e1m5.map"),
						secret_exit: None,
					},
				),
				(
					"e1m5.map",
					MapInfo {
						name: "E1M5: Phobos Lab",
						sky: "sky1.patch",
						music: "e1m5.music",
						exit: Some("e1m6.map"),
						secret_exit: None,
					},
				),
				(
					"e1m6.map",
					MapInfo {
						name: "E1M6: Central Processing",
						sky: "sky1.patch",
						music: "e1m6.music",
						exit: Some("e1m7.map"),
						secret_exit: None,
					},
				),
				(
					"e1m7.map",
					MapInfo {
						name: "E1M7: Computer Station",
						sky: "sky1.patch",
						music: "e1m7.music",
						exit: Some("e1m8.map"),
						secret_exit: None,
					},
				),
				(
					"e1m8.map",
					MapInfo {
						name: "E1M8: Phobos Anomaly",
						sky: "sky1.patch",
						music: "e1m8.music",
						exit: None,
						secret_exit: None,
					},
				),
				(
					"e1m9.map",
					MapInfo {
						name: "E1M9: Military Base",
						sky: "sky1.patch",
						music: "e1m9.music",
						exit: Some("e1m4.map"),
						secret_exit: None,
					},
				),
			])),
		},
	]
});
