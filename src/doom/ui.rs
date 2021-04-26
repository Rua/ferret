use crate::{
	common::{
		assets::{AssetHandle, AssetStorage, ImportData},
		blit::blit,
		video::RenderContext,
	},
	doom::{data::FONTS, image::Image},
};
use anyhow::{bail, Context};
use derivative::Derivative;
use fnv::FnvHashMap;
use nalgebra::Vector2;
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	fs::File,
	io::{BufRead, BufReader},
	sync::Arc,
};
use vulkano::{
	format::Format,
	image::{
		view::{ComponentMapping, ComponentSwizzle, ImageView},
		ImageDimensions, ImageViewAbstract, ImmutableImage, MipmapsCount,
	},
};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct UiTransform {
	pub position: Vector2<f32>,
	pub depth: f32,
	pub alignment: [UiAlignment; 2],
	pub size: Vector2<f32>,
	pub stretch: [bool; 2],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiImage {
	pub image: AssetHandle<Image>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiText {
	pub text: String,
	pub font: AssetHandle<Font>,
}

#[derive(Clone, Copy, Debug, Derivative, Serialize, Deserialize)]
#[derivative(Default)]
pub enum UiAlignment {
	#[derivative(Default)]
	Near = 0,
	Middle = 1,
	Far = 2,
}

#[derive(Clone, Copy, Debug)]
pub struct UiParams {
	factors: Vector2<f32>,
	dimensions: Vector2<f32>,
	framebuffer_dimensions: Vector2<f32>,
	viewport_dimensions: Vector2<f32>,
	alignment_offsets: [Vector2<f32>; 3],
	stretch_offsets: [Vector2<f32>; 2],
}

impl UiParams {
	pub fn new(framebuffer_dimensions: [u32; 2]) -> UiParams {
		let framebuffer_dimensions = Vector2::new(
			framebuffer_dimensions[0] as f32,
			framebuffer_dimensions[1] as f32,
		);

		// If the current aspect ratio is wider than 4:3, stretch horizontally.
		// If narrower, stretch vertically.
		let ratio = (framebuffer_dimensions[0] / framebuffer_dimensions[1]) / (4.0 / 3.0);
		let factors = if ratio >= 1.0 {
			Vector2::new(ratio, 1.0)
		} else {
			Vector2::new(1.0, 1.0 / ratio)
		};

		let base_dimensions = Vector2::new(320.0, 200.0);
		let dimensions = base_dimensions.component_mul(&factors);
		let alignment_offsets = [
			Vector2::zeros(),
			(dimensions - base_dimensions) * 0.5,
			dimensions - base_dimensions,
		];
		let stretch_offsets = [Vector2::zeros(), dimensions - base_dimensions];

		let viewport_dimensions =
			Vector2::new(1.0, 1.0 - 32.0 / dimensions[1]).component_mul(&framebuffer_dimensions);

		UiParams {
			factors,
			dimensions,
			framebuffer_dimensions,
			viewport_dimensions,
			alignment_offsets,
			stretch_offsets,
		}
	}

	#[inline]
	pub fn factors(&self) -> Vector2<f32> {
		self.factors
	}

	#[inline]
	pub fn dimensions(&self) -> Vector2<f32> {
		self.dimensions
	}

	#[inline]
	pub fn framebuffer_dimensions(&self) -> Vector2<f32> {
		self.framebuffer_dimensions
	}

	#[inline]
	pub fn viewport_dimensions(&self) -> Vector2<f32> {
		self.viewport_dimensions
	}

	#[inline]
	pub fn align(&self, alignment: [UiAlignment; 2]) -> Vector2<f32> {
		Vector2::new(
			self.alignment_offsets[alignment[0] as usize][0],
			self.alignment_offsets[alignment[1] as usize][1],
		)
	}

	#[inline]
	pub fn stretch(&self, stretch: [bool; 2]) -> Vector2<f32> {
		Vector2::new(
			self.stretch_offsets[stretch[0] as usize][0],
			self.stretch_offsets[stretch[1] as usize][1],
		)
	}
}

#[derive(Clone, Debug)]
pub struct Font {
	pub characters: FnvHashMap<char, AssetHandle<Image>>,
	pub spacing: FontSpacing,
}

#[derive(Clone, Copy, Debug)]
pub enum FontSpacing {
	FixedWidth { width: f32 },
	VariableWidth { space_width: f32 },
}

pub fn import_font(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let func = FONTS
		.get(path.as_str())
		.with_context(|| format!("Font \"{}\" not found", path))?;
	let template = func(asset_storage);
	Ok(Box::new(template))
}

#[derive(Clone)]
pub struct HexFont {
	pub image_view: Arc<dyn ImageViewAbstract + Send + Sync>,
	pub locations: HashMap<char, (Vector2<usize>, Vector2<usize>)>,
	pub line_height: usize,
}

impl HexFont {
	#[inline]
	pub fn wrap_lines<'a>(&'a self, line_width: usize, slice: &'a str) -> WrapLines<'a> {
		WrapLines {
			font: self,
			line_width,
			slice,
		}
	}
}

#[derive(Clone)]
pub struct WrapLines<'a> {
	font: &'a HexFont,
	line_width: usize,
	slice: &'a str,
}

impl<'a> Iterator for WrapLines<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		let mut current_width = 0; // The current accumulated length of the line.
		let mut breakpoint = 0; // The most recent location for a wrapping break.

		for (index, ch) in self.slice.char_indices() {
			let is_space = matches!(ch, ' ');
			let is_break = matches!(ch, '\n');

			if is_space || is_break {
				// Always break after whitespace.
				breakpoint = index + ch.len_utf8();
			}

			if is_break {
				// Hit an explicit line break.
				let (line, slice) = self.slice.split_at(breakpoint);
				self.slice = slice;
				return Some(line);
			} else {
				let ch_width = self.font.locations[&ch].1[0];
				current_width += ch_width;

				// Overflowed the line?
				if !is_space && current_width > self.line_width {
					// A single word that's longer than a line. Just break it in the middle.
					if breakpoint == 0 {
						breakpoint = index;
					}

					let (line, slice) = self.slice.split_at(breakpoint);
					self.slice = slice;
					return Some(line);
				}
			}
		}

		// Iterated through all characters, and didn't find a newline or overflow.
		if self.slice.is_empty() {
			None
		} else {
			Some(self.slice)
		}
	}
}

#[derive(Clone, Debug)]
pub struct HexFontData {
	pub image_data: Vec<u8>,
	pub image_size: Vector2<usize>,
	pub locations: HashMap<char, (Vector2<usize>, Vector2<usize>)>,
	pub line_height: usize,
}

pub fn import_hexfont(
	path: &RelativePath,
	_asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let reader = BufReader::new(File::open(path.as_str())?);
	let mut pixels = HashMap::new();
	let mut locations = HashMap::new();
	let mut position = Vector2::new(0, 0);
	const IMAGE_WIDTH: usize = 4096; // Minimum value of maxImageDimension2D in Vulkan
	let mut image_height = 0;
	let mut line_height = 0;

	for (i, line) in reader.lines().enumerate() {
		let line = line?;
		let (ch, data) = line
			.split_once(':')
			.context("Missing colon")
			.and_then(|(first, second)| -> anyhow::Result<_> {
				let ch =
					char::from_u32(u32::from_str_radix(first, 16)?).context("Invalid codepoint")?;
				let data_bits = hex::decode(second)?;
				let mut data_bytes = vec![0u8; data_bits.len() * 8];

				for (mut bits, bytes) in data_bits.into_iter().zip(data_bytes.chunks_exact_mut(8)) {
					for i in (0..8).rev() {
						bytes[i] = (bits & 1).wrapping_neg(); // 0 -> 0, 1 -> 255
						bits >>= 1;
					}
				}

				Ok((ch, data_bytes))
			})
			.with_context(|| format!("Parse error on line {}", i))?;

		if !(data.len() == 128 || data.len() == 256) {
			bail!("Data is not 16 or 32 bytes long");
		}

		let size = Vector2::new(data.len() / 16, 16);

		// If we reach the end of the line, wrap to the next
		if position[0] + size[0] > IMAGE_WIDTH {
			position[0] = 0;
			position[1] = image_height;
		}

		pixels.insert(ch, data);
		locations.insert(ch, (position, size));
		position[0] += size[0];
		image_height = image_height.max(position[1] + size[1]);
		line_height = line_height.max(size[1]);
	}

	if image_height > IMAGE_WIDTH {
		bail!("Texture height exceeded maximum");
	}

	// Now that we know how big the final texture will be, blit all the characters onto it
	let image_size = Vector2::new(IMAGE_WIDTH, image_height);
	let mut image_data = vec![0u8; image_size[0] * image_size[1]];

	for (ch, &(ch_position, ch_size)) in &locations {
		let ch_pixels = &pixels[ch];
		blit(
			|src, dst| *dst = *src,
			ch_pixels,
			ch_size.into(),
			&mut image_data,
			[IMAGE_WIDTH, image_height],
			ch_position.map(|x| x as isize).into(),
		);
	}

	Ok(Box::new(HexFontData {
		image_data,
		image_size,
		locations,
		line_height,
	}))
}

pub fn process_hexfonts(render_context: &RenderContext, asset_storage: &mut AssetStorage) {
	asset_storage.process::<HexFont, _>(|data, _asset_storage| {
		let hexfont_data: HexFontData = *data.downcast().ok().expect("Not a HexFontData");

		// Create the image
		let (image, _future) = ImmutableImage::from_iter(
			hexfont_data.image_data.into_iter(),
			ImageDimensions::Dim2d {
				width: hexfont_data.image_size[0] as u32,
				height: hexfont_data.image_size[1] as u32,
				array_layers: 1,
			},
			MipmapsCount::One,
			Format::R8Unorm,
			render_context.queues().graphics.clone(),
		)?;
		let image_view = ImageView::start(image)
			.with_component_mapping(ComponentMapping {
				r: ComponentSwizzle::Red,
				g: ComponentSwizzle::Red,
				b: ComponentSwizzle::Red,
				a: ComponentSwizzle::Red,
			})
			.build()?;

		Ok(HexFont {
			image_view,
			locations: hexfont_data.locations,
			line_height: hexfont_data.line_height,
		})
	})
}

// TODO ewwww
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiHexFontText {
	pub lines: Vec<String>,
	pub font: AssetHandle<HexFont>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Hidden;
