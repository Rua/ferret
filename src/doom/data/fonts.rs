use crate::{
	common::assets::AssetStorage,
	doom::ui::{Font, FontSpacing},
};
use fnv::FnvHashMap;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[allow(unused_variables)]
#[rustfmt::skip]
pub static FONTS: Lazy<HashMap<&'static str, fn(&mut AssetStorage) -> Font>> = Lazy::new(|| {
	let mut fonts: HashMap<&'static str, fn(&mut AssetStorage) -> Font> = HashMap::new();

    fonts.insert("ammnum.font", |asset_storage| Font {
        characters: {
            let mut characters = FnvHashMap::default();
            characters.insert('0', asset_storage.load("ammnum0.patch"));
            characters.insert('1', asset_storage.load("ammnum1.patch"));
            characters.insert('2', asset_storage.load("ammnum2.patch"));
            characters.insert('3', asset_storage.load("ammnum3.patch"));
            characters.insert('4', asset_storage.load("ammnum4.patch"));
            characters.insert('5', asset_storage.load("ammnum5.patch"));
            characters.insert('6', asset_storage.load("ammnum6.patch"));
            characters.insert('7', asset_storage.load("ammnum7.patch"));
            characters.insert('8', asset_storage.load("ammnum8.patch"));
            characters.insert('9', asset_storage.load("ammnum9.patch"));
            characters
        },
		spacing: FontSpacing::VariableWidth { space_width: 4.0 },
    });

    fonts.insert("stcfn.font", |asset_storage| Font {
        characters: {
            let mut characters = FnvHashMap::default();
            characters.insert('!', asset_storage.load("stcfn033.patch"));
            characters.insert('"', asset_storage.load("stcfn034.patch"));
            characters.insert('#', asset_storage.load("stcfn035.patch"));
            characters.insert('$', asset_storage.load("stcfn036.patch"));
            characters.insert('%', asset_storage.load("stcfn037.patch"));
            characters.insert('&', asset_storage.load("stcfn038.patch"));
            characters.insert('\'', asset_storage.load("stcfn039.patch"));
            characters.insert('(', asset_storage.load("stcfn040.patch"));
            characters.insert(')', asset_storage.load("stcfn041.patch"));
            characters.insert('*', asset_storage.load("stcfn042.patch"));
            characters.insert('+', asset_storage.load("stcfn043.patch"));
            characters.insert(',', asset_storage.load("stcfn044.patch"));
            characters.insert('-', asset_storage.load("stcfn045.patch"));
            characters.insert('.', asset_storage.load("stcfn046.patch"));
            characters.insert('/', asset_storage.load("stcfn047.patch"));
            characters.insert('0', asset_storage.load("stcfn048.patch"));
            characters.insert('1', asset_storage.load("stcfn049.patch"));
            characters.insert('2', asset_storage.load("stcfn050.patch"));
            characters.insert('3', asset_storage.load("stcfn051.patch"));
            characters.insert('4', asset_storage.load("stcfn052.patch"));
            characters.insert('5', asset_storage.load("stcfn053.patch"));
            characters.insert('6', asset_storage.load("stcfn054.patch"));
            characters.insert('7', asset_storage.load("stcfn055.patch"));
            characters.insert('8', asset_storage.load("stcfn056.patch"));
            characters.insert('9', asset_storage.load("stcfn057.patch"));
            characters.insert(':', asset_storage.load("stcfn058.patch"));
            characters.insert(';', asset_storage.load("stcfn059.patch"));
            characters.insert('<', asset_storage.load("stcfn060.patch"));
            characters.insert('=', asset_storage.load("stcfn061.patch"));
            characters.insert('>', asset_storage.load("stcfn062.patch"));
            characters.insert('?', asset_storage.load("stcfn063.patch"));
            characters.insert('@', asset_storage.load("stcfn064.patch"));
            characters.insert('A', asset_storage.load("stcfn065.patch"));
            characters.insert('B', asset_storage.load("stcfn066.patch"));
            characters.insert('C', asset_storage.load("stcfn067.patch"));
            characters.insert('D', asset_storage.load("stcfn068.patch"));
            characters.insert('E', asset_storage.load("stcfn069.patch"));
            characters.insert('F', asset_storage.load("stcfn070.patch"));
            characters.insert('G', asset_storage.load("stcfn071.patch"));
            characters.insert('H', asset_storage.load("stcfn072.patch"));
            characters.insert('I', asset_storage.load("stcfn073.patch"));
            characters.insert('J', asset_storage.load("stcfn074.patch"));
            characters.insert('K', asset_storage.load("stcfn075.patch"));
            characters.insert('L', asset_storage.load("stcfn076.patch"));
            characters.insert('M', asset_storage.load("stcfn077.patch"));
            characters.insert('N', asset_storage.load("stcfn078.patch"));
            characters.insert('O', asset_storage.load("stcfn079.patch"));
            characters.insert('P', asset_storage.load("stcfn080.patch"));
            characters.insert('Q', asset_storage.load("stcfn081.patch"));
            characters.insert('R', asset_storage.load("stcfn082.patch"));
            characters.insert('S', asset_storage.load("stcfn083.patch"));
            characters.insert('T', asset_storage.load("stcfn084.patch"));
            characters.insert('U', asset_storage.load("stcfn085.patch"));
            characters.insert('V', asset_storage.load("stcfn086.patch"));
            characters.insert('W', asset_storage.load("stcfn087.patch"));
            characters.insert('X', asset_storage.load("stcfn088.patch"));
            characters.insert('Y', asset_storage.load("stcfn089.patch"));
            characters.insert('Z', asset_storage.load("stcfn090.patch"));
            characters.insert('[', asset_storage.load("stcfn091.patch"));
            characters.insert('\\', asset_storage.load("stcfn092.patch"));
            characters.insert(']', asset_storage.load("stcfn093.patch"));
            characters.insert('^', asset_storage.load("stcfn094.patch"));
            characters.insert('_', asset_storage.load("stcfn095.patch"));
            characters
        },
		spacing: FontSpacing::VariableWidth { space_width: 4.0 },
    });

    fonts.insert("stgnum.font", |asset_storage| Font {
        characters: {
            let mut characters = FnvHashMap::default();
            characters.insert('0', asset_storage.load("stgnum0.patch"));
            characters.insert('1', asset_storage.load("stgnum1.patch"));
            characters.insert('2', asset_storage.load("stgnum2.patch"));
            characters.insert('3', asset_storage.load("stgnum3.patch"));
            characters.insert('4', asset_storage.load("stgnum4.patch"));
            characters.insert('5', asset_storage.load("stgnum5.patch"));
            characters.insert('6', asset_storage.load("stgnum6.patch"));
            characters.insert('7', asset_storage.load("stgnum7.patch"));
            characters.insert('8', asset_storage.load("stgnum8.patch"));
            characters.insert('9', asset_storage.load("stgnum9.patch"));
            characters
        },
		spacing: FontSpacing::FixedWidth { width: 4.0 },
    });

    fonts.insert("sttnum.font", |asset_storage| Font {
        characters: {
            let mut characters = FnvHashMap::default();
            characters.insert('%', asset_storage.load("sttprcnt.patch"));
            characters.insert('-', asset_storage.load("sttminus.patch"));
            characters.insert('0', asset_storage.load("sttnum0.patch"));
            characters.insert('1', asset_storage.load("sttnum1.patch"));
            characters.insert('2', asset_storage.load("sttnum2.patch"));
            characters.insert('3', asset_storage.load("sttnum3.patch"));
            characters.insert('4', asset_storage.load("sttnum4.patch"));
            characters.insert('5', asset_storage.load("sttnum5.patch"));
            characters.insert('6', asset_storage.load("sttnum6.patch"));
            characters.insert('7', asset_storage.load("sttnum7.patch"));
            characters.insert('8', asset_storage.load("sttnum8.patch"));
            characters.insert('9', asset_storage.load("sttnum9.patch"));
            characters
        },
		spacing: FontSpacing::FixedWidth { width: 14.0 },
    });

    fonts.insert("stysnum.font", |asset_storage| Font {
        characters: {
            let mut characters = FnvHashMap::default();
            characters.insert('0', asset_storage.load("stysnum0.patch"));
            characters.insert('1', asset_storage.load("stysnum1.patch"));
            characters.insert('2', asset_storage.load("stysnum2.patch"));
            characters.insert('3', asset_storage.load("stysnum3.patch"));
            characters.insert('4', asset_storage.load("stysnum4.patch"));
            characters.insert('5', asset_storage.load("stysnum5.patch"));
            characters.insert('6', asset_storage.load("stysnum6.patch"));
            characters.insert('7', asset_storage.load("stysnum7.patch"));
            characters.insert('8', asset_storage.load("stysnum8.patch"));
            characters.insert('9', asset_storage.load("stysnum9.patch"));
            characters
        },
		spacing: FontSpacing::FixedWidth { width: 4.0 },
    });

    fonts.insert("winum.font", |asset_storage| Font {
        characters: {
            let mut characters = FnvHashMap::default();
            characters.insert('%', asset_storage.load("wipcnt.patch"));
            characters.insert('-', asset_storage.load("wiminus.patch"));
            characters.insert('0', asset_storage.load("winum0.patch"));
            characters.insert('1', asset_storage.load("winum1.patch"));
            characters.insert('2', asset_storage.load("winum2.patch"));
            characters.insert('3', asset_storage.load("winum3.patch"));
            characters.insert('4', asset_storage.load("winum4.patch"));
            characters.insert('5', asset_storage.load("winum5.patch"));
            characters.insert('6', asset_storage.load("winum6.patch"));
            characters.insert('7', asset_storage.load("winum7.patch"));
            characters.insert('8', asset_storage.load("winum8.patch"));
            characters.insert('9', asset_storage.load("winum9.patch"));
            characters.insert(':', asset_storage.load("wicolon.patch"));
            characters
        },
		spacing: FontSpacing::VariableWidth { space_width: 4.0 },
    });

    fonts
});
