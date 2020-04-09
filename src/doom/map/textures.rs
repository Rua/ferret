use crate::{
	assets::{Asset, AssetFormat, AssetHandle, DataSource},
	doom::image::{IAColor, Image, ImageFormat},
};
use byteorder::{ReadBytesExt, LE};
use derivative::Derivative;
use std::{
	collections::HashMap,
	error::Error,
	io::{Cursor, Read, Seek, SeekFrom},
	str,
	sync::Arc,
};
use vulkano::image::ImageViewAccess;

pub struct Flat;

impl Asset for Flat {
	type Data = Arc<dyn ImageViewAccess + Send + Sync>;
	type Intermediate = Image;
	const NAME: &'static str = "Flat";

	fn import(
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Intermediate, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(name)?);
		let mut pixels = [0u8; 64 * 64];
		reader.read_exact(&mut pixels)?;

		Ok(Image {
			data: pixels.iter().map(|&i| IAColor { i, a: 0xFF }).collect(),
			size: [64, 64],
			offset: [0, 0],
		})
	}
}

#[derive(Clone, Copy)]
pub struct PNamesFormat;

impl AssetFormat for PNamesFormat {
	type Asset = Vec<[u8; 8]>;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(name)?);
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

pub struct WallTexture;

impl Asset for WallTexture {
	type Data = Arc<dyn ImageViewAccess + Send + Sync>;
	type Intermediate = Image;
	const NAME: &'static str = "WallTexture";

	fn import(
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Intermediate, Box<dyn Error + Send + Sync>> {
		let pnames = PNamesFormat.import("PNAMES", source)?;
		let mut texture_info = TexturesFormat.import("TEXTURE1", source)?;
		texture_info.extend(TexturesFormat.import("TEXTURE2", source)?);

		let name = name.to_ascii_uppercase();
		let texture_info = texture_info
			.get(&name)
			.ok_or(format!("Texture {} does not exist", name))?;

		let mut data = vec![IAColor::default(); texture_info.size[0] * texture_info.size[1]];

		texture_info.patches.iter().try_for_each(
			|patch_info| -> Result<(), Box<dyn Error + Send + Sync>> {
				let name =
					String::from(str::from_utf8(&pnames[patch_info.index])?.trim_end_matches('\0'));
				let patch = ImageFormat.import(&name, source)?;

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
			},
		)?;

		Ok(Image {
			data,
			size: texture_info.size,
			offset: [0, 0],
		})
	}
}

pub struct PatchInfo {
	pub offset: [isize; 2],
	pub index: usize,
}

pub struct TextureInfo {
	pub size: [usize; 2],
	pub patches: Vec<PatchInfo>,
}

#[derive(Clone, Copy)]
pub struct TexturesFormat;

impl AssetFormat for TexturesFormat {
	type Asset = HashMap<String, TextureInfo>;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		RawTexturesFormat
			.import(name, source)?
			.into_iter()
			.map(|(texture, patches)| {
				let mut name = String::from(str::from_utf8(&texture.name)?.trim_end_matches('\0'));
				name.make_ascii_uppercase();

				let patches = patches
					.into_iter()
					.map(|patch| PatchInfo {
						offset: [patch.offset[0] as isize, patch.offset[1] as isize],
						index: patch.index as usize,
					})
					.collect();

				Ok((
					name,
					TextureInfo {
						size: [texture.size[0] as usize, texture.size[1] as usize],
						patches,
					},
				))
			})
			.collect()
	}
}

pub struct RawPatchInfo {
	pub offset: [i16; 2],
	pub index: u16,
}

pub struct RawTextureInfo {
	pub name: [u8; 8],
	pub size: [u16; 2],
	pub patch_count: usize,
}

#[derive(Clone, Copy)]
pub struct RawTexturesFormat;

impl AssetFormat for RawTexturesFormat {
	type Asset = Vec<(RawTextureInfo, Vec<RawPatchInfo>)>;

	fn import(
		&self,
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Asset, Box<dyn Error + Send + Sync>> {
		let mut reader = Cursor::new(source.load(name)?);

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
				reader.read_u32::<LE>()?; // unused
				let size = [reader.read_u16::<LE>()?, reader.read_u16::<LE>()?];
				reader.read_u32::<LE>()?; // unused
				let patch_count = reader.read_u16::<LE>()? as usize;

				let mut patches: Vec<RawPatchInfo> = Vec::with_capacity(patch_count);

				for _ in 0..patch_count {
					let offset = [reader.read_i16::<LE>()?, reader.read_i16::<LE>()?];
					let index = reader.read_u16::<LE>()?;
					reader.read_u32::<LE>()?; // unused
					patches.push(RawPatchInfo { offset, index });
				}

				Ok((
					RawTextureInfo {
						name,
						size,
						patch_count,
					},
					patches,
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
