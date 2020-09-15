use crate::doom::data::FRAME_TIME;
use lazy_static::lazy_static;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct AnimData {
	pub frames: Vec<&'static str>,
	pub frame_time: Duration,
}

lazy_static! {
	pub static ref ANIMS: Vec<AnimData> = vec![
		// Doom 1
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["nukage1.flat", "nukage2.flat", "nukage3.flat"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["fwater1.flat", "fwater2.flat", "fwater3.flat", "fwater4.flat"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["lava1.flat", "lava2.flat", "lava3.flat", "lava4.flat"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["blood1.flat", "blood2.flat", "blood3.flat"]
		},

		// Doom 2
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["rrock5.flat", "rrock6.flat", "rrock7.flat", "rrock8.flat"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["slime1.flat", "slime2.flat", "slime3.flat", "slime4.flat"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["slime5.flat", "slime6.flat", "slime7.flat", "slime8.flat"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["slime9.flat", "slime10.flat", "slime11.flat", "slime12.flat"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["blodgr1.texture", "blodgr2.texture", "blodgr3.texture", "blodgr4.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["sladrip1.texture", "sladrip2.texture", "sladrip3.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["blodrip1.texture", "blodrip2.texture", "blodrip3.texture", "blodrip4.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["firewala.texture", "firewalb.texture", "firewall.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["gstfont1.texture", "gstfont2.texture", "gstfont3.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["firelav3.texture", "firelava.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["firemag1.texture", "firemag2.texture", "firemag3.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["fireblu1.texture", "fireblu2.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["rockred1.texture", "rockred2.texture", "rockred3.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["bfall1.texture", "bfall2.texture", "bfall3.texture", "bfall4.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["sfall1.texture", "sfall2.texture", "sfall3.texture", "sfall4.texture"]
		},
		AnimData {
			frame_time: 8 * FRAME_TIME,
			frames: vec!["dbrain1.texture", "dbrain2.texture", "dbrain3.texture", "dbrain4.texture"]
		},
	];
	pub static ref SWITCHES: Vec<[&'static str; 2]> = vec![
		// Doom 1
		["sw1brcom.texture", "sw2brcom.texture"],
		["sw1brn1.texture", "sw2brn1.texture"],
		["sw1brn2.texture", "sw2brn2.texture"],
		["sw1brngn.texture", "sw2brngn.texture"],
		["sw1brown.texture", "sw2brown.texture"],
		["sw1comm.texture", "sw2comm.texture"],
		["sw1comp.texture", "sw2comp.texture"],
		["sw1dirt.texture", "sw2dirt.texture"],
		["sw1exit.texture", "sw2exit.texture"],
		["sw1gray.texture", "sw2gray.texture"],
		["sw1gray1.texture", "sw2gray1.texture"],
		["sw1metal.texture", "sw2metal.texture"],
		["sw1pipe.texture", "sw2pipe.texture"],
		["sw1slad.texture", "sw2slad.texture"],
		["sw1starg.texture", "sw2starg.texture"],
		["sw1ston1.texture", "sw2ston1.texture"],
		["sw1ston2.texture", "sw2ston2.texture"],
		["sw1stone.texture", "sw2stone.texture"],
		["sw1strtn.texture", "sw2strtn.texture"],
		["sw1blue.texture", "sw2blue.texture"],
		["sw1cmt.texture", 	"sw2cmt.texture"],
		["sw1garg.texture", "sw2garg.texture"],
		["sw1gston.texture", "sw2gston.texture"],
		["sw1hot.texture", 	"sw2hot.texture"],
		["sw1lion.texture", "sw2lion.texture"],
		["sw1satyr.texture", "sw2satyr.texture"],
		["sw1skin.texture", "sw2skin.texture"],
		["sw1vine.texture", "sw2vine.texture"],
		["sw1wood.texture", "sw2wood.texture"],

		// doom 2
		["sw1panel.texture", "sw2panel.texture"],
		["sw1rock.texture", "sw2rock.texture"],
		["sw1met2.texture", "sw2met2.texture"],
		["sw1wdmet.texture", "sw2wdmet.texture"],
		["sw1brik.texture", "sw2brik.texture"],
		["sw1mod1.texture", "sw2mod1.texture"],
		["sw1zim.texture", 	"sw2zim.texture"],
		["sw1ston6.texture", "sw2ston6.texture"],
		["sw1tek.texture", 	"sw2tek.texture"],
		["sw1marb.texture", "sw2marb.texture"],
		["sw1skull.texture", "sw2skull.texture"],
	];
}
