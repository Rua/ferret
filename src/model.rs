use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use std::error::Error;
use std::rc::Rc;
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


pub struct BSPModel {
	vertices: DataOrBuffer,
	textures: Vec<Texture>,
	faces: Vec<Face>,
	leaves: Vec<BSPLeaf>,
	branches: Vec<BSPBranch>,
}

impl BSPModel {
	pub fn new(vertices: Vec<VertexData>, textures: Vec<Surface<'static>>, faces: Vec<Face>, leaves: Vec<BSPLeaf>, branches: Vec<BSPBranch>) -> BSPModel {
		BSPModel {
			vertices: DataOrBuffer::Data(vertices),
			textures: textures.into_iter().map(Texture::new).collect(),
			faces,
			leaves,
			branches,
		}
	}
	
	pub fn upload(&mut self, queue: &Arc<Queue>) -> Result<Box<dyn GpuFuture>, Box<dyn Error>> {
		for texture in &mut self.textures {
			texture.upload(queue)?;
		}
		
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

#[derive(Debug, Clone)]
pub struct VertexData {
	pub in_position: [f32; 3],
	pub in_tex_coord: [f32; 2],
}
impl_vertex!(VertexData, in_position, in_tex_coord);

pub struct Face {
	pub first_vertex_index: usize,
	pub vertex_count: usize,
	pub texture: Rc<Texture>,
}

#[derive(Debug, Clone)]
pub struct BSPBranch {
	pub plane: Plane,
	pub bounding_box: BoundingBox3,
	pub children: [BSPNode; 2],
}

#[derive(Debug, Clone)]
pub struct BSPLeaf {
	pub first_face_index: usize,
	pub face_count: usize,
	pub bounding_box: BoundingBox3,
}

#[derive(Debug, Copy, Clone)]
pub enum BSPNode {
	Leaf(usize),
	Branch(usize),
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
