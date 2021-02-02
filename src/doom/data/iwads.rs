use crate::doom::wad::IWADInfo;

pub const IWADINFO: &[IWADInfo] = &[
	IWADInfo {
		files: &["doom2.wad"],
		load_fns: &[
			super::linedefs::load,
			super::sectors::load,
			super::mobjs::load_doom1sw,
			super::weapons::load_doom1sw,
			super::mobjs::load_doom1,
			super::weapons::load_doom1,
			super::mobjs::load_doom2,
			super::weapons::load_doom2,
		],
		map: "map01",
	},
	IWADInfo {
		files: &["plutonia.wad"],
		load_fns: &[
			super::linedefs::load,
			super::sectors::load,
			super::mobjs::load_doom1sw,
			super::weapons::load_doom1sw,
			super::mobjs::load_doom1,
			super::weapons::load_doom1,
			super::mobjs::load_doom2,
			super::weapons::load_doom2,
		],
		map: "map01",
	},
	IWADInfo {
		files: &["tnt.wad"],
		load_fns: &[
			super::linedefs::load,
			super::sectors::load,
			super::mobjs::load_doom1sw,
			super::weapons::load_doom1sw,
			super::mobjs::load_doom1,
			super::weapons::load_doom1,
			super::mobjs::load_doom2,
			super::weapons::load_doom2,
		],
		map: "map01",
	},
	IWADInfo {
		files: &["doom.wad", "doomu.wad"],
		load_fns: &[
			super::linedefs::load,
			super::sectors::load,
			super::mobjs::load_doom1sw,
			super::weapons::load_doom1sw,
			super::mobjs::load_doom1,
			super::weapons::load_doom1,
		],
		map: "e1m1",
	},
	IWADInfo {
		files: &["doom1.wad"],
		load_fns: &[
			super::linedefs::load,
			super::sectors::load,
			super::mobjs::load_doom1sw,
			super::weapons::load_doom1sw,
		],
		map: "e1m1",
	},
];
