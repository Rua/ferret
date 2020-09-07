use crate::{
	common::assets::{Asset, AssetFormat, AssetHandle, AssetStorage, ImportData},
	doom::image::{IAColor, ImageData, ImageFormat},
};
use anyhow::anyhow;
use byteorder::{ReadBytesExt, LE};
use derivative::Derivative;
use fnv::FnvHashMap;
use std::{
	io::{Cursor, Read, Seek, SeekFrom},
	str,
	sync::Arc,
};
use vulkano::image::ImageViewAccess;

#[derive(Clone, Copy, Debug)]
pub struct Flat;

impl Asset for Flat {
	type Data = Arc<dyn ImageViewAccess + Send + Sync>;
	const NAME: &'static str = "Flat";

	fn import(name: &str, asset_storage: &mut AssetStorage) -> anyhow::Result<Box<dyn ImportData>> {
		let mut reader = Cursor::new(asset_storage.source().load(name)?);
		let mut pixels = [0u8; 64 * 64];
		reader.read_exact(&mut pixels)?;

		Ok(Box::new(ImageData {
			data: pixels.iter().map(|&i| IAColor { i, a: 0xFF }).collect(),
			size: [64, 64],
			offset: [0, 0],
		}))
	}
}

#[derive(Clone, Copy, Debug)]
pub struct PNamesFormat;

impl AssetFormat for PNamesFormat {
	type Asset = Vec<[u8; 8]>;

	fn import(&self, name: &str, asset_storage: &mut AssetStorage) -> anyhow::Result<Self::Asset> {
		let mut reader = Cursor::new(asset_storage.source().load(name)?);
		let count = reader.read_u32::<LE>()? as usize;
		let mut ret = Vec::with_capacity(count);

		for _ in 0..count {
			let mut name = [0u8; 8];
			reader.read_exact(&mut name)?;
			ret.push(name);
		}

		Ok(ret)
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Wall;

impl Asset for Wall {
	type Data = Arc<dyn ImageViewAccess + Send + Sync>;
	const NAME: &'static str = "Wall";

	fn import(name: &str, asset_storage: &mut AssetStorage) -> anyhow::Result<Box<dyn ImportData>> {
		let pnames = PNamesFormat.import("PNAMES", asset_storage)?;
		let mut texture_info = TexturesFormat.import("TEXTURE1", asset_storage)?;

		// TODO better error handing
		if let Ok(info) = TexturesFormat.import("TEXTURE2", asset_storage) {
			texture_info.extend(info);
		}

		let name = name.to_ascii_uppercase();
		let texture_info = texture_info
			.get(&name)
			.ok_or(anyhow!("Texture {} does not exist", name))?;

		let mut data = vec![IAColor::default(); texture_info.size[0] * texture_info.size[1]];

		texture_info
			.patches
			.iter()
			.try_for_each(|patch_info| -> anyhow::Result<()> {
				let name =
					String::from(str::from_utf8(&pnames[patch_info.index])?.trim_end_matches('\0'));
				let patch = ImageFormat.import(&name, asset_storage)?;

				// Blit the patch onto the main image
				let dest_start = [
					std::cmp::max(patch_info.offset[0], 0),
					std::cmp::max(patch_info.offset[1], 0),
				];
				let dest_end = [
					std::cmp::min(
						patch_info.offset[0] + patch.size[0] as isize,
						texture_info.size[0] as isize,
					),
					std::cmp::min(
						patch_info.offset[1] + patch.size[1] as isize,
						texture_info.size[1] as isize,
					),
				];

				for dest_y in dest_start[1]..dest_end[1] {
					let src_y = dest_y - patch_info.offset[1];

					let dest_y_index = dest_y * texture_info.size[0] as isize;
					let src_y_index = src_y * patch.size[0] as isize;

					for dest_x in dest_start[0]..dest_end[0] {
						let src_x = dest_x - patch_info.offset[0];

						let src_index = (src_x + src_y_index) as usize;
						let dest_index = (dest_x + dest_y_index) as usize;

						data[dest_index] = patch.data[src_index];
					}
				}

				Ok(())
			})?;

		Ok(Box::new(ImageData {
			data,
			size: texture_info.size,
			offset: [0, 0],
		}))
	}
}

#[derive(Clone, Copy, Debug)]
pub struct PatchInfo {
	pub offset: [isize; 2],
	pub index: usize,
}

#[derive(Clone, Debug)]
pub struct TextureInfo {
	pub size: [usize; 2],
	pub patches: Vec<PatchInfo>,
}

#[derive(Clone, Copy, Debug)]
pub struct TexturesFormat;

impl AssetFormat for TexturesFormat {
	type Asset = FnvHashMap<String, TextureInfo>;

	fn import(&self, name: &str, asset_storage: &mut AssetStorage) -> anyhow::Result<Self::Asset> {
		let mut reader = Cursor::new(asset_storage.source().load(name)?);

		let count = reader.read_u32::<LE>()? as usize;
		let mut offsets = Vec::with_capacity(count);

		for _ in 0..count {
			offsets.push(reader.read_u32::<LE>()? as u64);
		}

		offsets
			.into_iter()
			.map(|offset| {
				reader.seek(SeekFrom::Start(offset))?;

				let mut name = [0u8; 8];
				reader.read_exact(&mut name)?;
				let mut name = String::from(str::from_utf8(&name)?.trim_end_matches('\0'));
				name.make_ascii_uppercase();

				reader.read_u32::<LE>()?; // unused
				let size = [reader.read_u16::<LE>()?, reader.read_u16::<LE>()?];
				reader.read_u32::<LE>()?; // unused
				let patch_count = reader.read_u16::<LE>()? as usize;

				let mut patches = Vec::with_capacity(patch_count);

				for _ in 0..patch_count {
					let offset = [reader.read_i16::<LE>()?, reader.read_i16::<LE>()?];
					let index = reader.read_u16::<LE>()?;
					reader.read_u32::<LE>()?; // unused
					patches.push(PatchInfo {
						offset: [offset[0] as isize, offset[1] as isize],
						index: index as usize,
					})
				}

				Ok((
					name,
					TextureInfo {
						size: [size[0] as usize, size[1] as usize],
						patches,
					},
				))
			})
			.collect()
	}
}

#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub enum TextureType<T> {
	Normal(AssetHandle<T>),
	Sky,
	None,
}

impl<T> TextureType<T> {
	pub fn is_sky(&self) -> bool {
		if let TextureType::Sky = *self {
			true
		} else {
			false
		}
	}
}
