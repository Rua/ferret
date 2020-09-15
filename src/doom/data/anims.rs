use crate::doom::data::FRAME_TIME;
use lazy_static::lazy_static;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct AnimData {
	pub frames: Vec<&'static str>,
	pub frame_time: Duration,
}

lazy_static! {
	pub static ref ANIMS_FLAT: Vec<AnimData> = vec![
		// Doom 1
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["NUKAGE1.flat", "NUKAGE2.flat", "NUKAGE3.flat"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["FWATER1.flat", "FWATER2.flat", "FWATER3.flat", "FWATER4.flat"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["LAVA1.flat", "LAVA2.flat", "LAVA3.flat", "LAVA4.flat"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["BLOOD1.flat", "BLOOD2.flat", "BLOOD3.flat"]
		},

		// Doom 2
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["RROCK5.flat", "RROCK6.flat", "RROCK7.flat", "RROCK8.flat"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["SLIME1.flat", "SLIME2.flat", "SLIME3.flat", "SLIME4.flat"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["SLIME5.flat", "SLIME6.flat", "SLIME7.flat", "SLIME8.flat"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["SLIME9.flat", "SLIME10.flat", "SLIME11.flat", "SLIME12.flat"]
		},
	];
	pub static ref ANIMS_WALL: Vec<AnimData> = vec![
		// Doom 2
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["BLODGR1.texture", "BLODGR2.texture", "BLODGR3.texture", "BLODGR4.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["SLADRIP1.texture", "SLADRIP2.texture", "SLADRIP3.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["BLODRIP1.texture", "BLODRIP2.texture", "BLODRIP3.texture", "BLODRIP4.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["FIREWALA.texture", "FIREWALB.texture", "FIREWALL.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["GSTFONT1.texture", "GSTFONT2.texture", "GSTFONT3.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["FIRELAV3.texture", "FIRELAVA.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["FIREMAG1.texture", "FIREMAG2.texture", "FIREMAG3.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["FIREBLU1.texture", "FIREBLU2.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["ROCKRED1.texture", "ROCKRED2.texture", "ROCKRED3.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["BFALL1.texture", "BFALL2.texture", "BFALL3.texture", "BFALL4.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["SFALL1.texture", "SFALL2.texture", "SFALL3.texture", "SFALL4.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["DBRAIN1.texture", "DBRAIN2.texture", "DBRAIN3.texture", "DBRAIN4.texture"]
		},
	];
	pub static ref SWITCHES: Vec<[&'static str; 2]> = vec![
		// Doom 1
		["SW1BRCOM.texture", "SW2BRCOM.texture"],
		["SW1BRN1.texture", "SW2BRN1.texture"],
		["SW1BRN2.texture", "SW2BRN2.texture"],
		["SW1BRNGN.texture", "SW2BRNGN.texture"],
		["SW1BROWN.texture", "SW2BROWN.texture"],
		["SW1COMM.texture", "SW2COMM.texture"],
		["SW1COMP.texture", "SW2COMP.texture"],
		["SW1DIRT.texture", "SW2DIRT.texture"],
		["SW1EXIT.texture", "SW2EXIT.texture"],
		["SW1GRAY.texture", "SW2GRAY.texture"],
		["SW1GRAY1.texture", "SW2GRAY1.texture"],
		["SW1METAL.texture", "SW2METAL.texture"],
		["SW1PIPE.texture", "SW2PIPE.texture"],
		["SW1SLAD.texture", "SW2SLAD.texture"],
		["SW1STARG.texture", "SW2STARG.texture"],
		["SW1STON1.texture", "SW2STON1.texture"],
		["SW1STON2.texture", "SW2STON2.texture"],
		["SW1STONE.texture", "SW2STONE.texture"],
		["SW1STRTN.texture", "SW2STRTN.texture"],
		["SW1BLUE.texture", "SW2BLUE.texture"],
		["SW1CMT.texture", 	"SW2CMT.texture"],
		["SW1GARG.texture", "SW2GARG.texture"],
		["SW1GSTON.texture", "SW2GSTON.texture"],
		["SW1HOT.texture", 	"SW2HOT.texture"],
		["SW1LION.texture", "SW2LION.texture"],
		["SW1SATYR.texture", "SW2SATYR.texture"],
		["SW1SKIN.texture", "SW2SKIN.texture"],
		["SW1VINE.texture", "SW2VINE.texture"],
		["SW1WOOD.texture", "SW2WOOD.texture"],

		// Doom 2
		["SW1PANEL.texture", "SW2PANEL.texture"],
		["SW1ROCK.texture", "SW2ROCK.texture"],
		["SW1MET2.texture", "SW2MET2.texture"],
		["SW1WDMET.texture", "SW2WDMET.texture"],
		["SW1BRIK.texture", "SW2BRIK.texture"],
		["SW1MOD1.texture", "SW2MOD1.texture"],
		["SW1ZIM.texture", 	"SW2ZIM.texture"],
		["SW1STON6.texture", "SW2STON6.texture"],
		["SW1TEK.texture", 	"SW2TEK.texture"],
		["SW1MARB.texture", "SW2MARB.texture"],
		["SW1SKULL.texture", "SW2SKULL.texture"],
	];
}
