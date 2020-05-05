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
}
