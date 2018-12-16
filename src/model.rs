use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, ImmutableBuffer};
use vulkano::buffer::cpu_access::CpuAccessibleBuffer;
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
	faces: Vec<Face>,
	leaves: Vec<BSPLeaf>,
	branches: Vec<BSPBranch>,
}

impl BSPModel {
	pub fn new(vertices: Vec<VertexData>, faces: Vec<Face>, leaves: Vec<BSPLeaf>, branches: Vec<BSPBranch>) -> BSPModel {
		BSPModel {
			vertices: DataOrBuffer::Data(vertices),
			faces,
			leaves,
			branches,
		}
	}
	
	pub fn upload(&mut self, queue: &Arc<Queue>) -> Result<Box<dyn GpuFuture>, Box<dyn Error>> {
		for face in &mut self.faces {
			face.texture.borrow_mut().upload(queue)?;
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
	
	pub fn buffer(&self) -> Option<Arc<ImmutableBuffer<[VertexData]>>> {
		if let DataOrBuffer::Buffer(buffer) = &self.vertices {
			Some(buffer.clone())
		} else {
			None
		}
	}
	
	pub fn faces(&self) -> &Vec<Face> {
		&self.faces
	}
}

#[derive(Debug, Clone)]
pub struct VertexData {
	pub in_position: [f32; 3],
	pub in_tex_coord: [f32; 3],
}
impl_vertex!(VertexData, in_position, in_tex_coord);

pub struct Face {
	pub first_vertex_index: usize,
	pub vertex_count: usize,
	pub texture: Rc<RefCell<Texture>>,
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
	pub fn new(surfaces: Vec<Surface<'static>>) -> Texture {
		assert!(!surfaces.is_empty());
		let size = surfaces[0].size();
		
		for surface in &surfaces {
			// All surfaces must be the same size
			assert_eq!(surface.size(), size);
			
			// All surfaces must store pixels in byte-wise RGBA order
			#[cfg(target_endian = "big")]
			assert_eq!(surface.pixel_format_enum(), PixelFormatEnum::RGBA8888);
			#[cfg(target_endian = "little")]
			assert_eq!(surface.pixel_format_enum(), PixelFormatEnum::ABGR8888);
		}
		
		Texture {
			image: DataOrImage::Data(surfaces),
		}
	}
	
	pub fn upload(&mut self, queue: &Arc<Queue>) -> Result<Box<dyn GpuFuture>, ImageCreationError> {
		match &self.image {
			DataOrImage::Data(surfaces) => {
				// Create staging buffer
				let layer_size = surfaces[0].without_lock().unwrap().len();
				
				let buffer = unsafe { CpuAccessibleBuffer::uninitialized_array(
					queue.device().clone(),
					layer_size * surfaces.len(),
					BufferUsage::transfer_source(),
				) }?;
				
				// Copy all the layers into the buffer
				for (chunk, surface) in (&mut *buffer.write().unwrap()).chunks_exact_mut(layer_size).zip(surfaces) {
					let slice = surface.without_lock().unwrap();
					chunk.copy_from_slice(slice);
				}
				
				// Create image
				let (image, future) = ImmutableImage::from_buffer(
					buffer,
					Dimensions::Dim2dArray { width: surfaces[0].width(), height: surfaces[0].height(), array_layers: surfaces.len() as u32 },
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
	
	pub fn image(&self) -> Option<Arc<ImmutableImage<Format>>> {
		if let DataOrImage::Image(image) = &self.image {
			Some(image.clone())
		} else {
			None
		}
	}
}

enum DataOrBuffer {
	Data(Vec<VertexData>),
	Buffer(Arc<ImmutableBuffer<[VertexData]>>),
}

enum DataOrImage {
	Data(Vec<Surface<'static>>),
	Image(Arc<ImmutableImage<Format>>),
}
