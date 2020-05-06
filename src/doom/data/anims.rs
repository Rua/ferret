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
			frames: vec!["NUKAGE1", "NUKAGE2", "NUKAGE3"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["FWATER1", "FWATER2", "FWATER3", "FWATER4"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["LAVA1", "LAVA2", "LAVA3", "LAVA4"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["BLOOD1", "BLOOD2", "BLOOD3"]
		},

		// Doom 2
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["RROCK5", "RROCK6", "RROCK7", "RROCK8"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["SLIME1", "SLIME2", "SLIME3", "SLIME4"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["SLIME5", "SLIME6", "SLIME7", "SLIME8"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["SLIME9", "SLIME10", "SLIME11", "SLIME12"]
		},
	];
	pub static ref ANIMS_WALL: Vec<AnimData> = vec![
		// Doom 2
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["BLODGR1", "BLODGR2", "BLODGR3", "BLODGR4"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["SLADRIP1", "SLADRIP2", "SLADRIP3"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["BLODRIP1", "BLODRIP2", "BLODRIP3", "BLODRIP4"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["FIREWALA", "FIREWALB", "FIREWALL"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["GSTFONT1", "GSTFONT2", "GSTFONT3"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["FIRELAV3", "FIRELAVA"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["FIREMAG1", "FIREMAG2", "FIREMAG3"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["FIREBLU1", "FIREBLU2"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["ROCKRED1", "ROCKRED2", "ROCKRED3"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["BFALL1", "BFALL2", "BFALL3", "BFALL4"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["SFALL1", "SFALL2", "SFALL3", "SFALL4"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["DBRAIN1", "DBRAIN2", "DBRAIN3", "DBRAIN4"]
		},
	];
	pub static ref SWITCHES: Vec<[&'static str; 2]> = vec![
		// Doom 1
		["SW1BRCOM", "SW2BRCOM"],
		["SW1BRN1", "SW2BRN1"],
		["SW1BRN2", "SW2BRN2"],
		["SW1BRNGN", "SW2BRNGN"],
		["SW1BROWN", "SW2BROWN"],
		["SW1COMM", "SW2COMM"],
		["SW1COMP", "SW2COMP"],
		["SW1DIRT", "SW2DIRT"],
		["SW1EXIT", "SW2EXIT"],
		["SW1GRAY", "SW2GRAY"],
		["SW1GRAY1", "SW2GRAY1"],
		["SW1METAL", "SW2METAL"],
		["SW1PIPE", "SW2PIPE"],
		["SW1SLAD", "SW2SLAD"],
		["SW1STARG", "SW2STARG"],
		["SW1STON1", "SW2STON1"],
		["SW1STON2", "SW2STON2"],
		["SW1STONE", "SW2STONE"],
		["SW1STRTN", "SW2STRTN"],
		["SW1BLUE", "SW2BLUE"],
		["SW1CMT", 	"SW2CMT"],
		["SW1GARG", "SW2GARG"],
		["SW1GSTON", "SW2GSTON"],
		["SW1HOT", 	"SW2HOT"],
		["SW1LION", "SW2LION"],
		["SW1SATYR", "SW2SATYR"],
		["SW1SKIN", "SW2SKIN"],
		["SW1VINE", "SW2VINE"],
		["SW1WOOD", "SW2WOOD"],

		// Doom 2
		["SW1PANEL", "SW2PANEL"],
		["SW1ROCK", "SW2ROCK"],
		["SW1MET2", "SW2MET2"],
		["SW1WDMET", "SW2WDMET"],
		["SW1BRIK", "SW2BRIK"],
		["SW1MOD1", "SW2MOD1"],
		["SW1ZIM", 	"SW2ZIM"],
		["SW1STON6", "SW2STON6"],
		["SW1TEK", 	"SW2TEK"],
		["SW1MARB", "SW2MARB"],
		["SW1SKULL", "SW2SKULL"],
	];
}
