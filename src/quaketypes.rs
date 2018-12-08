use byteorder::{LE, ReadBytesExt};
use nalgebra::Vector2;
use sdl2::surface::Surface;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use std::collections::hash_map::{Entry, HashMap};
use std::error::Error;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::str;
use std::vec::Vec;

use palette::Palette;
use sprite::{Sprite, SpriteFrame, SpriteImage, SpriteOrientation, SpriteRotation};
use wad::WadLoader;

#[cfg(target_endian = "big")]
const FORMAT : PixelFormatEnum = PixelFormatEnum::RGBA8888;
#[cfg(target_endian = "little")]
const FORMAT : PixelFormatEnum = PixelFormatEnum::ABGR8888;


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
}

pub mod sprite {
	use super::*;
	
	pub fn from_data<T: Read>(data: &mut T, palette: &Palette) -> Result<Sprite, Box<Error>> {
		let mut signature = [0u8; 4];
		data.read_exact(&mut signature)?;
		
		if &signature != b"IDSP" {
			return Err(Box::from("No IDSP signature found."));
		}
		
		let version = data.read_i32::<LE>()?;
		
		if version != 1 {
			return Err(Box::from(format!("IDSP version {} not supported.", version)));
		}
		
		let orientation = data.read_i32::<LE>()?;
		data.read_f32::<LE>()?;  // bounding radius, not used
		let max_size_x = data.read_i32::<LE>()? as u32;
		let max_size_y = data.read_i32::<LE>()? as u32;
		let num_frames = data.read_i32::<LE>()? as usize;
		data.read_f32::<LE>()?;  // beam length, not used
		let sync_type = data.read_i32::<LE>()?;
		
		let mut frames = Vec::new();
		let mut images = Vec::new();
		
		for i in 0..num_frames {
			let group = data.read_i32::<LE>()?;
			
			if group != 0 {
				return Err(Box::from("Sprite groups are not supported."));
			}
			
			let offset_x = data.read_i32::<LE>()?;
			let offset_y = data.read_i32::<LE>()?;
			let size_x = data.read_i32::<LE>()? as u32;
			let size_y = data.read_i32::<LE>()? as u32;
			
			let mut surface = Surface::new(size_x, size_y, FORMAT)?;
			let pitch = surface.pitch() as usize;
			
			{
				let mut pixels = surface.without_lock_mut().unwrap();
				
				for y in 0..size_y as usize {
					for x in 0..size_x as usize {
						let pixel = data.read_u8()? as usize;
						let mut color = palette[pixel];
						
						// Quake treats entry 255 as transparent for sprites
						if pixel == 255 {
							color.a = 0;
						}
						
						pixels[pitch * y + 4 * x + 0] = color.r;
						pixels[pitch * y + 4 * x + 1] = color.g;
						pixels[pitch * y + 4 * x + 2] = color.b;
						pixels[pitch * y + 4 * x + 3] = color.a;
					}
				}
			}
			
			images.push(SpriteImage::from_surface(surface, Vector2::new(offset_x, offset_y)));
			frames.push(SpriteFrame::new(vec![SpriteRotation::new(i, false)]));
		}
		
		let orientation = match(orientation) {
			0 => SpriteOrientation::ViewPlaneParallelUpright,
			1 => SpriteOrientation::FacingUpright,
			2 => SpriteOrientation::ViewPlaneParallel,
			3 => SpriteOrientation::Oriented,
			4 => SpriteOrientation::ViewPlaneParallelOriented,
			_ => return Err(Box::from("Invalid sprite orientation"))
		};
		
		Ok(Sprite::new(images, frames, orientation, Vector2::new(max_size_x, max_size_y)))
	}
}
