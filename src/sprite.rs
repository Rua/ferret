use nalgebra::Vector2;
use sdl2::surface::Surface;

pub struct Sprite {
	images: Vec<SpriteImage>,
	frames: Vec<SpriteFrame>,
	orientation: SpriteOrientation,
	max_size: Vector2<u32>,
}

impl Sprite {
	pub fn new(images: Vec<SpriteImage>, frames: Vec<SpriteFrame>, orientation: SpriteOrientation, max_size: Vector2<u32>) -> Sprite {
		for frame in &frames {
			for rotation in &frame.rotations {
				assert!(rotation.image_index < images.len());
			}
		}

		Sprite {images, frames, orientation, max_size}
	}

	pub fn images(&self) -> &Vec<SpriteImage> {
		&self.images
	}

	pub fn frames(&self) -> &Vec<SpriteFrame> {
		&self.frames
	}
}

#[derive(Debug)]
pub struct SpriteFrame {
	rotations: Vec<SpriteRotation>,
}

impl SpriteFrame {
	pub fn new(rotations: Vec<SpriteRotation>) -> SpriteFrame {
		SpriteFrame {rotations}
	}
}

#[derive(Clone, Debug)]
pub struct SpriteRotation {
	image_index: usize,
	flipped: bool,
}

impl SpriteRotation {
	pub fn new(image_index: usize, flipped: bool) -> SpriteRotation {
		SpriteRotation {image_index, flipped}
	}
}

pub enum SpriteOrientation {
	Oriented = 3,
	ViewPlaneParallel = 2,
	ViewPlaneParallelOriented = 4,
	ViewPlaneParallelUpright = 0,
	FacingUpright = 1,
}


pub struct SpriteImage {
	surface: Surface<'static>,
	offset: Vector2<i32>,
	has_transparency: bool,
}

impl SpriteImage {
	pub fn from_surface(surface: Surface<'static>, offset: Vector2<i32>) -> SpriteImage {
		let mut has_transparency = false;

		{
			let pixels = surface.without_lock().unwrap();
			let mut i = 3;

			while i < pixels.len() {
				if pixels[i] != 0xFF {
					has_transparency = true;
					break;
				}

				i += 4;
			}
		}

		SpriteImage {surface, offset, has_transparency}
	}

	pub fn surface(&self) -> &Surface<'static> {
		&self.surface
	}

	pub fn to_surface(self) -> Surface<'static> {
		self.surface
	}
}
