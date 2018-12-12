use byteorder::{LE, ReadBytesExt};
use nalgebra::Vector2;
use sdl2::surface::Surface;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use std::cmp::max;
use std::collections::hash_map::{Entry, HashMap};
use std::error::Error;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::rc::Rc;
use std::str;
use std::vec::Vec;

use crate::model::Texture;
use crate::palette::Palette;
use crate::sprite::{Sprite, SpriteFrame, SpriteImage, SpriteOrientation, SpriteRotation};
use crate::doom::wad::WadLoader;

#[cfg(target_endian = "big")]
const FORMAT : PixelFormatEnum = PixelFormatEnum::RGBA8888;
#[cfg(target_endian = "little")]
const FORMAT : PixelFormatEnum = PixelFormatEnum::ABGR8888;


pub mod flat {
	use super::*;
	
	pub fn from_data<T: Read>(data: &mut T, palette: &Palette) -> Result<Surface<'static>, Box<Error>> {
		let mut surface = Surface::new(64, 64, FORMAT)?;
		let pitch = surface.pitch() as usize;
		
		{
			let mut pixels = surface.without_lock_mut().unwrap();
			let mut flat_pixels = [0u8; 64 * 64];
			
			data.read_exact(&mut flat_pixels)?;
			
			for i in 0..flat_pixels.len() {
				let color = palette[flat_pixels[i] as usize];
				pixels[4 * i + 0] = color.r;
				pixels[4 * i + 1] = color.g;
				pixels[4 * i + 2] = color.b;
				pixels[4 * i + 3] = color.a;
			}
		}
		
		Ok(surface)
	}
	
	pub fn from_wad(name: &str, loader: &mut WadLoader, palette: &Palette) -> Result<Surface<'static>, Box<Error>> {
		let index = loader.index_for_name(name).unwrap();
		let mut data = loader.read_lump(index)?;
		from_data(&mut data, palette)
	}
}

pub mod image {
	use super::*;
	
	pub fn from_data<T: Read + Seek>(data: &mut T, palette: &Palette) -> Result<SpriteImage, Box<Error>> {
		let size_x = data.read_u16::<LE>()? as u32;
		let size_y = data.read_u16::<LE>()? as u32;
		let offset_x = data.read_i16::<LE>()? as i32;
		let offset_y = data.read_i16::<LE>()? as i32;
		
		let mut column_offsets = vec![0; size_x as usize];
		data.read_u32_into::<LE>(&mut column_offsets)?;
		
		#[cfg(target_endian = "big")]
		let target_format = PixelFormatEnum::RGBA8888;
		#[cfg(target_endian = "little")]
		let target_format = PixelFormatEnum::ABGR8888;
		
		let mut surface = Surface::new(size_x, size_y, FORMAT)?;
		let pitch = surface.pitch() as usize;
		
		{
			let mut pixels = surface.without_lock_mut().unwrap();
			
			for col in 0..size_x as usize {
				data.seek(SeekFrom::Start(column_offsets[col] as u64))?;
				let mut start_row = data.read_u8()? as usize;
				
				while start_row != 255 {
					// Read pixels in one vertical "post"
					let post_height = data.read_u8()?;
					let mut post_pixels = vec![0u8; post_height as usize];
					data.read_u8()?;  // Padding byte
					data.read_exact(&mut post_pixels)?;
					data.read_u8()?;  // Padding byte
					
					// Paint the pixels onto the main image
					for i in 0..post_pixels.len() {
						assert!(start_row + i < size_y as usize);
						let color = palette[post_pixels[i] as usize];
						pixels[pitch * (start_row + i) + 4 * col + 0] = color.r;
						pixels[pitch * (start_row + i) + 4 * col + 1] = color.g;
						pixels[pitch * (start_row + i) + 4 * col + 2] = color.b;
						pixels[pitch * (start_row + i) + 4 * col + 3] = color.a;
					}
					
					start_row = data.read_u8()? as usize;
				}
			}
		}
		
		Ok(SpriteImage::from_surface(surface, Vector2::new(offset_x, offset_y)))
	}
	
	pub fn from_wad(name: &str, loader: &mut WadLoader, palette: &Palette) -> Result<SpriteImage, Box<Error>> {
		let index = loader.index_for_name(name).unwrap();
		let mut data = loader.read_lump(index)?;
		from_data(&mut data, palette)
	}
}

pub mod palette {
	use super::*;
	
	pub fn from_data<T: Read>(data: &mut T) -> Result<Palette, io::Error> {
		let mut palette = [Color {r: 0, g: 0, b: 0, a: 0}; 256];
		
		for i in 0..256 {
			let r = data.read_u8()?;
			let g = data.read_u8()?;
			let b = data.read_u8()?;
			
			palette[i] = Color::RGB(r, g, b);
		}
		
		Ok(Palette(palette))
	}
	
	pub fn from_wad(name: &str, loader: &mut WadLoader) -> Result<Palette, Box<Error>> {
		let index = loader.index_for_name(name).unwrap();
		let mut data = loader.read_lump(index)?;
		Ok(from_data(&mut data)?)
	}
}

pub mod pnames {
	use super::*;
	
	pub fn from_data<T: Read>(data: &mut T) -> Result<Vec<String>, Box<Error>> {
		let count = data.read_u32::<LE>()? as usize;
		let mut pnames = Vec::with_capacity(count);
		
		for i in 0..count {
			let mut name = [0u8; 8];
			data.read_exact(&mut name)?;
			let mut name = String::from(str::from_utf8(&name)?.trim_right_matches('\0'));
			pnames.push(name);
		}
		
		Ok(pnames)
	}
	
	pub fn from_wad(name: &str, loader: &mut WadLoader) -> Result<Vec<String>, Box<Error>> {
		let index = loader.index_for_name(name).unwrap();
		let mut data = loader.read_lump(index)?;
		from_data(&mut data)
	}
}

pub mod sound {
	use super::*;
	
	pub struct DoomSound {
		sampling_rate: u16,
		samples: Vec<u8>,
	}
	
	pub fn from_data<T: Read>(data: &mut T) -> Result<DoomSound, Box<Error>> {
		let signature = data.read_u16::<LE>()?;
		
		if signature != 3 {
			panic!("No Doom sound file signature found.");
		}
		
		let sampling_rate = data.read_u16::<LE>()?;
		let num_samples = data.read_u32::<LE>()? as usize;
		let mut samples = vec![0u8; num_samples as usize];
		
		data.read_exact(&mut samples)?;
		
		// Remove padding bytes at start and end
		if samples.ends_with(&[samples[num_samples - 17]; 16]) {
			samples.drain(num_samples - 17..);
		}
		
		if samples.starts_with(&[samples[16]; 16]) {
			samples.drain(..16);
		}
		
		Ok(DoomSound {
			sampling_rate: sampling_rate,
			samples: samples,
		})
	}
	
	pub fn from_wad(name: &str, loader: &mut WadLoader) -> Result<DoomSound, Box<Error>> {
		let index = loader.index_for_name(name).unwrap();
		let mut data = loader.read_lump(index)?;
		from_data(&mut data)
	}
}

pub mod sprite {
	use super::*;
	
	pub fn from_wad(prefix: &str, loader: &mut WadLoader, palette: &Palette) -> Result<Sprite, Box<Error>> {
		let image_info = loader.find_with_prefix(prefix);
		let mut images = Vec::with_capacity(image_info.len());
		let mut name_indices = Vec::with_capacity(image_info.len());
		let mut max_size = Vector2::new(0, 0);
		
		for (i, info) in image_info.iter().enumerate() {
			assert!(info.0.starts_with(prefix) && (info.0.len() == 6 || info.0.len() == 8));
			
			let mut data = loader.read_lump(info.1)?;
			let image = super::image::from_data(&mut data, palette)?;
			let size = image.surface().size();
			max_size[0] = max(size.0, max_size[0]);
			max_size[1] = max(size.1, max_size[1]);
			
			images.push(image);
			name_indices.push((&info.0[4..], i));
		}
		
		let mut slice = name_indices.as_slice();
		let mut frames = Vec::new();
		
		while slice.len() > 0 {
			if slice[0].0.chars().nth(1).unwrap() == '0' {
				frames.push(SpriteFrame::new(vec![SpriteRotation::new(slice[0].1, false)]));
				slice = &slice[1..];
			} else {
				let next_frame = slice.iter().position(|i|
					i.0.chars().nth(0).unwrap() != slice[0].0.chars().nth(0).unwrap()
				).unwrap();
				let mut rotations = vec![None; 8];
				
				for info in &slice[..next_frame] {
					let rot = info.0.chars().nth(1).unwrap() as usize - '1' as usize;
					assert!(rotations[rot].is_none());
					rotations[rot] = Some(SpriteRotation::new(info.1, false));
					
					if info.0.len() == 4 {
						assert_eq!(info.0.chars().nth(2).unwrap(), info.0.chars().nth(0).unwrap());
						let rot = info.0.chars().nth(3).unwrap() as usize - '1' as usize;
						assert!(rotations[rot].is_none());
						rotations[rot] = Some(SpriteRotation::new(info.1, true));
					}
				}
				
				assert!(rotations.iter().all(|r| r.is_some()));
				frames.push(SpriteFrame::new(
					rotations.drain(..).map(Option::unwrap).collect::<Vec<_>>()
				));
				slice = &slice[next_frame..];
			}
		}
		
		Ok(Sprite::new(images, frames, SpriteOrientation::ViewPlaneParallelUpright, max_size))
	}
}

pub mod texture_info {
	use super::*;
	
	pub struct DoomPatchInfo {
		pub offset: Vector2<i32>,
		pub index: usize,
	}
	
	pub struct DoomTextureInfo {
		pub size: Vector2<u32>,
		pub patches: Vec<DoomPatchInfo>,
	}
	
	pub fn from_data<T: Read + Seek>(data: &mut T) -> Result<HashMap<String, DoomTextureInfo>, Box<Error>> {
		let mut texture_info = HashMap::new();
		
		let count = data.read_u32::<LE>()? as usize;
		let mut offsets = vec![0u32; count];
		data.read_u32_into::<LE>(&mut offsets)?;
		
		for i in 0..count {
			data.seek(SeekFrom::Start(offsets[i] as u64))?;
			
			let mut name = [0u8; 8];
			data.read_exact(&mut name)?;
			let mut name = String::from(str::from_utf8(&name)?.trim_right_matches('\0'));
			
			data.read_u32::<LE>()?; // unused bytes
			
			let size_x = data.read_u16::<LE>()? as u32;
			let size_y = data.read_u16::<LE>()? as u32;
			
			data.read_u32::<LE>()?; // unused bytes
			
			let patch_count = data.read_u16::<LE>()? as usize;
			let mut patches = Vec::with_capacity(patch_count);
			
			for _j in 0..patch_count {
				let offset_x = data.read_i16::<LE>()? as i32;
				let offset_y = data.read_i16::<LE>()? as i32;
				let patch_index = data.read_u16::<LE>()? as usize;
				
				data.read_u32::<LE>()?; // unused bytes
				
				patches.push(DoomPatchInfo {
					offset: Vector2::new(offset_x, offset_y),
					index: patch_index,
				});
			}
			
			texture_info.insert(name, DoomTextureInfo {
				size: Vector2::new(size_x, size_y),
				patches: patches,
			});
		}
		
		Ok(texture_info)
	}
	
	pub fn from_wad(name: &str, loader: &mut WadLoader) -> Result<HashMap<String, DoomTextureInfo>, Box<Error>> {
		let index = loader.index_for_name(name).unwrap();
		let mut data = loader.read_lump(index)?;
		from_data(&mut data)
	}
}

pub struct DoomTextureLoader {
	texture_cache: HashMap<String, Rc<Texture>>,
	patch_cache: HashMap<String, Surface<'static>>,
	pnames: Vec<String>,
	palette: Palette,
	texture_info: HashMap<String, texture_info::DoomTextureInfo>,
}

impl DoomTextureLoader {
	pub fn new(loader: &mut WadLoader) -> Result<DoomTextureLoader, Box<Error>> {
		let palette = palette::from_wad("PLAYPAL", loader)?;
		let pnames = pnames::from_wad("PNAMES", loader)?;
		let texture_info = texture_info::from_wad("TEXTURE1", loader)?;
		
		Ok(DoomTextureLoader {
			texture_cache: HashMap::new(),
			patch_cache: HashMap::new(),
			palette,
			pnames,
			texture_info,
		})
	}
	
	pub fn load(&mut self, name: &str, loader: &mut WadLoader) -> Result<Rc<Texture>, Box<Error>> {
		let texture = match self.texture_cache.entry(name.to_owned()) {
			Entry::Occupied(texture_item) => texture_item.into_mut(),
			Entry::Vacant(texture_item) => {
				let texture_info = &self.texture_info[name];
				let mut surface = Surface::new(texture_info.size[0] as u32, texture_info.size[1] as u32, FORMAT)?;
				
				// Read each patch, and paint it onto the main image
				for patch_info in &texture_info.patches {
					let name = &self.pnames[patch_info.index];
					
					// Return from cache if available, otherwise load and insert it
					let patch = match self.patch_cache.entry(name.clone()) {
						Entry::Occupied(patch_item) => patch_item.into_mut(),
						Entry::Vacant(patch_item) => {
							let image = image::from_wad(&name, loader, &self.palette)?;
							
							// Use to_surface because the offsets of patches are ignored anyway
							patch_item.insert(image.to_surface())
						}
					};
					
					patch.blit(None, &mut surface, Rect::new(patch_info.offset[0] as i32, patch_info.offset[1] as i32, 0, 0))?;
				}
				
				texture_item.insert(Rc::new(Texture::new(surface)))
			}
		};
		
		Ok(texture.clone())
	}
}

pub struct DoomFlatLoader {
	texture_cache: HashMap<String, Rc<Texture>>,
	palette: Palette,
}

impl DoomFlatLoader {
	pub fn new(loader: &mut WadLoader) -> Result<DoomFlatLoader, Box<Error>> {
		let palette = palette::from_wad("PLAYPAL", loader)?;
		
		Ok(DoomFlatLoader {
			texture_cache: HashMap::new(),
			palette,
		})
	}
	
	pub fn load(&mut self, name: &str, loader: &mut WadLoader) -> Result<Rc<Texture>, Box<Error>> {
		let texture = match self.texture_cache.entry(name.to_owned()) {
			Entry::Occupied(texture_item) => texture_item.into_mut(),
			Entry::Vacant(texture_item) => {
				let flat = flat::from_wad(&name, loader, &self.palette)?;
				
				// Use to_surface because the offsets of patches are ignored anyway
				texture_item.insert(Rc::new(Texture::new(flat)))
			},
		};
		
		Ok(texture.clone())
	}
}
