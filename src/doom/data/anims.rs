use crate::doom::data::FRAME_TIME;
use once_cell::sync::Lazy;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct AnimData {
	pub frames: &'static [&'static str],
	pub frame_time: Duration,
}

#[rustfmt::skip]
pub static ANIMS: Lazy<Box<[AnimData]>> = Lazy::new(|| vec![
	// Doom 1
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["nukage1.flat", "nukage2.flat", "nukage3.flat"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["fwater1.flat", "fwater2.flat", "fwater3.flat", "fwater4.flat"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["lava1.flat", "lava2.flat", "lava3.flat", "lava4.flat"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["blood1.flat", "blood2.flat", "blood3.flat"]
	},

	// Doom 2
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["rrock5.flat", "rrock6.flat", "rrock7.flat", "rrock8.flat"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["slime1.flat", "slime2.flat", "slime3.flat", "slime4.flat"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["slime5.flat", "slime6.flat", "slime7.flat", "slime8.flat"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["slime9.flat", "slime10.flat", "slime11.flat", "slime12.flat"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["blodgr1.texture", "blodgr2.texture", "blodgr3.texture", "blodgr4.texture"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["sladrip1.texture", "sladrip2.texture", "sladrip3.texture"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["blodrip1.texture", "blodrip2.texture", "blodrip3.texture", "blodrip4.texture"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["firewala.texture", "firewalb.texture", "firewall.texture"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["gstfont1.texture", "gstfont2.texture", "gstfont3.texture"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["firelav3.texture", "firelava.texture"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["firemag1.texture", "firemag2.texture", "firemag3.texture"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["fireblu1.texture", "fireblu2.texture"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["rockred1.texture", "rockred2.texture", "rockred3.texture"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["bfall1.texture", "bfall2.texture", "bfall3.texture", "bfall4.texture"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["sfall1.texture", "sfall2.texture", "sfall3.texture", "sfall4.texture"]
	},
	AnimData {
		frame_time: 8 * FRAME_TIME,
		frames: &["dbrain1.texture", "dbrain2.texture", "dbrain3.texture", "dbrain4.texture"]
	},
].into_boxed_slice());

pub static SWITCHES: &[[&'static str; 2]] = &[
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
	["sw1cmt.texture", "sw2cmt.texture"],
	["sw1garg.texture", "sw2garg.texture"],
	["sw1gston.texture", "sw2gston.texture"],
	["sw1hot.texture", "sw2hot.texture"],
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
	["sw1zim.texture", "sw2zim.texture"],
	["sw1ston6.texture", "sw2ston6.texture"],
	["sw1tek.texture", "sw2tek.texture"],
	["sw1marb.texture", "sw2marb.texture"],
	["sw1skull.texture", "sw2skull.texture"],
];
