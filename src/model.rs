use nalgebra::Vector3;
use sdl2::{
	pixels::PixelFormatEnum,
	surface::Surface,
};
use std::{
	cell::RefCell,
	error::Error,
	rc::Rc,
	sync::Arc,
};
use vulkano::{
	buffer::{BufferUsage, ImmutableBuffer, cpu_access::CpuAccessibleBuffer},
	device::Queue,
	format::Format,
	image::{Dimensions, ImmutableImage, sys::ImageCreationError},
	sync::{self, GpuFuture},
};
use crate::{
	geometry::{BoundingBox3, Plane},
	sprite::SpriteFrame,
};


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
			face.lightmap.borrow_mut().upload(queue)?;
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
	pub in_texture_coord: [f32; 3],
	pub in_lightmap_coord: [f32; 3],
}
impl_vertex!(VertexData, in_position, in_texture_coord, in_lightmap_coord);

pub struct Face {
	pub first_vertex_index: usize,
	pub vertex_count: usize,
	pub texture: Rc<RefCell<Texture>>,
	pub lightmap: Rc<RefCell<Texture>>,
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
		let pixel_format = surfaces[0].pixel_format_enum();

		for surface in &surfaces {
			// All surfaces must be the same size
			assert_eq!(surface.size(), size);

			// All surfaces must have the same pixel format
			assert_eq!(surface.pixel_format_enum(), pixel_format);
		}

		Texture {
			image: DataOrImage::Data(surfaces),
		}
	}

	pub fn size(&self) -> Vector3<u32> {
		match &self.image {
			DataOrImage::Data(surfaces) => {
				Vector3::new(
					surfaces[0].width(),
					surfaces[0].height(),
					surfaces.len() as u32,
				)
			},
			DataOrImage::Image(image) => {
				Vector3::new(
					image.dimensions().width(),
					image.dimensions().height(),
					image.dimensions().array_layers(),
				)
			},
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
				{
					let slice = &mut *buffer.write().unwrap();

					for (chunk, surface) in slice.chunks_exact_mut(layer_size).zip(surfaces) {
						chunk.copy_from_slice(surface.without_lock().unwrap());
					}
				}

				// Find the corresponding Vulkan pixel format
				let format = match surfaces[0].pixel_format_enum() {
					PixelFormatEnum::RGB24 => Format::R8G8B8Unorm,
					PixelFormatEnum::BGR24 => Format::B8G8R8Unorm,
					PixelFormatEnum::RGBA32 => Format::R8G8B8A8Unorm,
					PixelFormatEnum::BGRA32 => Format::B8G8R8A8Unorm,
					_ => unimplemented!(),
				};

				// Create the image
				let (image, future) = ImmutableImage::from_buffer(
					buffer,
					Dimensions::Dim2dArray {
						width: surfaces[0].width(),
						height: surfaces[0].height(),
						array_layers: surfaces.len() as u32,
					},
					format,
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
