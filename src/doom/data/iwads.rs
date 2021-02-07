use crate::doom::wad::IWADInfo;

pub const IWADINFO: &[IWADInfo] = &[
	IWADInfo {
		files: &["doom2.wad"],
		map: "map01",
	},
	IWADInfo {
		files: &["plutonia.wad"],
		map: "map01",
	},
	IWADInfo {
		files: &["tnt.wad"],
		map: "map01",
	},
	IWADInfo {
		files: &["doom.wad", "doomu.wad"],
		map: "e1m1",
	},
	IWADInfo {
		files: &["doom1.wad"],
		map: "e1m1",
	},
];
