use crate::{
	common::{
		assets::{AssetHandle, AssetStorage, ImportData},
		blit::blit,
	},
	doom::assets::{
		image::{IAColor, Image, ImageData},
		wad::read_string,
	},
};
use anyhow::{anyhow, Context};
use arrayvec::ArrayString;
use byteorder::{ReadBytesExt, LE};
use fnv::FnvHashMap;
use nalgebra::Vector2;
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read, Seek, SeekFrom};

pub fn import_flat(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let mut reader = Cursor::new(asset_storage.source().load(path)?);
	let mut pixels = [0u8; 64 * 64];
	reader.read_exact(&mut pixels)?;

	Ok(Box::new(ImageData {
		data: pixels.iter().map(|&i| IAColor { i, a: 0xFF }).collect(),
		size: Vector2::new(64, 64),
		offset: Vector2::zeros(),
	}))
}

pub fn import_wall(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let texture1_handle = asset_storage.load::<Textures>("texture1");
	let texture2_handle = if asset_storage.source().exists(RelativePath::new("texture2")) {
		Some(asset_storage.load::<Textures>("texture2"))
	} else {
		None
	};

	let texture1 = asset_storage.get(&texture1_handle).unwrap();
	let texture2 = texture2_handle.map(|h| asset_storage.get(&h).unwrap());

	let name = path.file_stem().context("Empty file name")?;
	let texture_info = texture1
		.get(name)
		.or(texture2.and_then(|t| t.get(name)))
		.ok_or(anyhow!("Texture {} does not exist", name))?
		.clone();
	let mut data = vec![IAColor::default(); texture_info.size[0] * texture_info.size[1]];

	for patch_info in &texture_info.patches {
		let patch_handle = asset_storage.load::<ImageData>(&patch_info.name);
		let patch = asset_storage.get(&patch_handle).unwrap();
		blit(
			|src, dst| {
				if src.a != 0 {
					*dst = *src;
				}
			},
			&patch.data,
			patch.size.into(),
			&mut data,
			texture_info.size.into(),
			patch_info.offset.into(),
		);
	}

	Ok(Box::new(ImageData {
		data,
		size: texture_info.size,
		offset: Vector2::zeros(),
	}))
}

pub type PNames = Vec<ArrayString<8>>;

pub fn import_pnames(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let mut reader = Cursor::new(asset_storage.source().load(path)?);
	let count = reader.read_u32::<LE>()? as usize;
	let mut ret = Vec::with_capacity(count);

	for _ in 0..count {
		ret.push(read_string(&mut reader)?);
	}

	Ok(Box::new(ret))
}

#[derive(Clone, Debug)]
pub struct PatchInfo {
	pub offset: Vector2<isize>,
	pub name: String,
}

#[derive(Clone, Debug)]
pub struct TextureInfo {
	pub size: Vector2<usize>,
	pub patches: Vec<PatchInfo>,
}

pub type Textures = FnvHashMap<String, TextureInfo>;

pub fn import_textures(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let pnames_handle = asset_storage.load::<PNames>("pnames");
	let pnames = asset_storage.get(&pnames_handle).unwrap();
	let mut reader = Cursor::new(asset_storage.source().load(path)?);

	let count = reader.read_u32::<LE>()? as usize;
	let mut offsets = Vec::with_capacity(count);

	for _ in 0..count {
		offsets.push(reader.read_u32::<LE>()? as u64);
	}

	Ok(Box::new(
		offsets
			.into_iter()
			.map(|offset| {
				reader.seek(SeekFrom::Start(offset))?;

				let name = read_string(&mut reader)?;
				reader.read_u32::<LE>()?; // unused
				let size = Vector2::new(
					reader.read_u16::<LE>()? as usize,
					reader.read_u16::<LE>()? as usize,
				);
				reader.read_u32::<LE>()?; // unused
				let patch_count = reader.read_u16::<LE>()? as usize;

				let mut patches = Vec::with_capacity(patch_count);

				for _ in 0..patch_count {
					let offset = Vector2::new(
						reader.read_i16::<LE>()? as isize,
						reader.read_i16::<LE>()? as isize,
					);
					let index = reader.read_u16::<LE>()? as usize;
					let name = format!("{}.patch", pnames[index]);
					reader.read_u32::<LE>()?; // unused
					patches.push(PatchInfo { offset, name })
				}

				Ok((name.as_str().to_owned(), TextureInfo { size, patches }))
			})
			.collect::<anyhow::Result<Textures>>()?,
	))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TextureType {
	Normal(AssetHandle<Image>),
	Sky,
	None,
}

impl TextureType {
	pub fn is_sky(&self) -> bool {
		if let TextureType::Sky = *self {
			true
		} else {
			false
		}
	}
}
