use crate::renderer::texture::{Texture, TextureBuilder};
use nalgebra::Vector2;

pub struct Sprite {
	frames: Vec<Vec<usize>>,
	textures: Vec<Texture>,
}

pub struct SpriteBuilder {
	images: Vec<(TextureBuilder, Vector2<f32>, usize, usize, Option<usize>)>,
}

impl SpriteBuilder {
	pub fn new() -> SpriteBuilder {
		SpriteBuilder { images: Vec::new() }
	}

	pub fn add_image(
		mut self,
		texture: TextureBuilder,
		offset: Vector2<f32>,
		frame: usize,
		rotation: usize,
		rotation2: Option<usize>,
	) -> Self {
		self.images
			.push((texture, offset, frame, rotation, rotation2));
		self
	}

	/*pub fn build(self) -> Sprite {
	let mut images = Vec::with_capacity(lumpnames.len());
	let mut name_indices = Vec::with_capacity(lumpnames.len());
	let mut max_size = Vector2::new(0, 0);

	for (i, lump) in lumpnames.iter().enumerate() {
		assert!(lump.starts_with(name) && (lump.len() == 6 || lump.len() == 8));

		let image = DoomImage.import(lump, source)?;
		let size = image.surface().size();
		max_size[0] = max(size.0, max_size[0]);
		max_size[1] = max(size.1, max_size[1]);

		images.push(image);
		name_indices.push((&lump[4..], i));
	}

	let mut slice = name_indices.as_slice();
	let mut frames = Vec::new();

	while slice.len() > 0 {
		if slice[0].0.chars().nth(1).unwrap() == '0' {
			frames.push(SpriteFrame::new(vec![SpriteRotation::new(
				slice[0].1, false,
			)]));
			slice = &slice[1..];
		} else {
			let next_frame = slice
				.iter()
				.position(|i| i.0.chars().nth(0).unwrap() != slice[0].0.chars().nth(0).unwrap())
				.unwrap();
			let mut rotations = vec![None; 8];

			for info in &slice[..next_frame] {
				let rot = info.0.chars().nth(1).unwrap() as usize - '1' as usize;
				assert!(rotations[rot].is_none());
				rotations[rot] = Some(SpriteRotation::new(info.1, false));

				if info.0.len() == 4 {
					assert_eq!(
						info.0.chars().nth(2).unwrap(),
						info.0.chars().nth(0).unwrap()
					);
					let rot = info.0.chars().nth(3).unwrap() as usize - '1' as usize;
					assert!(rotations[rot].is_none());
					rotations[rot] = Some(SpriteRotation::new(info.1, true));
				}
			}

			assert!(rotations.iter().all(|r| r.is_some()));
			frames.push(SpriteFrame::new(
				rotations.drain(..).map(Option::unwrap).collect::<Vec<_>>(),
			));
			slice = &slice[next_frame..];
		}
	}
	}*/
}

/*
	pub fn new(
		images: Vec<SpriteImage>,
		frames: Vec<SpriteFrame>,
		orientation: SpriteOrientation,
		max_size: Vector2<u32>,
	) -> Sprite {
		for frame in &frames {
			for rotation in &frame.rotations {
				assert!(rotation.image_index < images.len());
			}
		}

		Sprite {
			images,
			frames,
			orientation,
			max_size,
		}
	}

	pub fn images(&self) -> &Vec<SpriteImage> {
		&self.images
	}

	pub fn frames(&self) -> &Vec<SpriteFrame> {
		&self.frames
	}
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

		SpriteImage {
			surface,
			offset,
			has_transparency,
		}
	}

	pub fn surface(&self) -> &Surface<'static> {
		&self.surface
	}

	pub fn to_surface(self) -> Surface<'static> {
		self.surface
	}
}
*/
