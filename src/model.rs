use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, ImmutableBuffer};
use vulkano::device::Queue;
use vulkano::format::Format;
use vulkano::image::{Dimensions, ImmutableImage, sys::ImageCreationError};
use vulkano::memory::DeviceMemoryAllocError;
use vulkano::sync;
use vulkano::sync::GpuFuture;

use crate::geometry::{BoundingBox3, Plane};
use crate::sprite::SpriteFrame;


#[derive(Clone)]
pub struct VertexData {
	pub in_position: [f32; 3],
	pub in_tex_coord: [f32; 2],
}
impl_vertex!(VertexData, in_position, in_tex_coord);

pub struct BSPModel {
	vertices: DataOrBuffer,
	faces: Vec<(usize, usize)>,
}

impl BSPModel {
	pub fn new(vertices: Vec<VertexData>, faces: Vec<(usize, usize)>) -> BSPModel {
		BSPModel {
			vertices: DataOrBuffer::Data(vertices),
			faces,
		}
	}
	
	pub fn upload(&mut self, queue: &Arc<Queue>) -> Result<Box<dyn GpuFuture>, DeviceMemoryAllocError> {
		match &self.vertices {
			DataOrBuffer::Data(data) => {
				let (buffer, future) = ImmutableBuffer::from_iter(
					data.iter().cloned(),
					BufferUsage::vertex_buffer(),
					queue.clone(),
				)?;
				
				self.vertices = DataOrBuffer::Buffer(buffer);
				Ok(Box::from(future))
			},
			DataOrBuffer::Buffer(buffer) => {
				Ok(Box::from(sync::now(queue.device().clone())))
			},
		}
	}
}

pub struct BSPBranch {
	plane: Plane,
	bounding_box: BoundingBox3,
	children: [usize; 2],
}

pub struct BSPLeaf {
	first_face_index: usize,
	count: usize,
	bounding_box: BoundingBox3,
}

pub enum BSPNode {
	Leaf(BSPLeaf),
	Branch(BSPBranch),
}

pub struct SpriteModel {
	frames: Vec<SpriteFrame>,
	//orientation: SpriteOrientation,
	// bounding_box
}

impl SpriteModel {
	
}

pub struct Texture {
	image: DataOrImage,
}

impl Texture {
	pub fn new(surface: Surface<'static>) -> Texture {
		#[cfg(target_endian = "big")]
		assert_eq!(surface.pixel_format_enum(), PixelFormatEnum::RGBA8888);
		#[cfg(target_endian = "little")]
		assert_eq!(surface.pixel_format_enum(), PixelFormatEnum::ABGR8888);
		
		Texture {
			image: DataOrImage::Data(surface),
		}
	}
	
	pub fn upload(&mut self, queue: &Arc<Queue>) -> Result<Box<dyn GpuFuture>, ImageCreationError> {
		match &self.image {
			DataOrImage::Data(surface) => {
				let (image, future) = ImmutableImage::from_iter(
					surface.without_lock().unwrap().iter().cloned(),
					Dimensions::Dim2d { width: surface.width(), height: surface.height() },
					Format::R8G8B8A8Unorm,
					queue.clone(),
				)?;
				
				self.image = DataOrImage::Image(image);
				Ok(Box::from(future))
			},
			DataOrImage::Image(image) => {
				Ok(Box::from(sync::now(queue.device().clone())))
			},
		}
	}
}

enum DataOrBuffer {
	Data(Vec<VertexData>),
	Buffer(Arc<ImmutableBuffer<[VertexData]>>),
}

enum DataOrImage {
	Data(Surface<'static>),
	Image(Arc<ImmutableImage<Format>>),
}
