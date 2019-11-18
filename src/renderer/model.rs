use crate::{
	geometry::{BoundingBox3, Plane},
	renderer::mesh::{Mesh, MeshBuilder},
	renderer::texture::{Texture, TextureBuilder},
};
use nalgebra::Vector3;
use sdl2::{pixels::PixelFormatEnum, surface::Surface};
use std::{cell::RefCell, error::Error, rc::Rc, sync::Arc};
use vulkano::{
	device::Queue,
	format::Format,
	image::{sys::ImageCreationError, Dimensions, ImageViewAccess},
	sync::{self, GpuFuture},
};

pub struct BSPModel {
	mesh: DataOrMesh,
	faces: Vec<Face>,
	_leaves: Vec<BSPLeaf>,
	_branches: Vec<BSPBranch>,
}

impl BSPModel {
	pub fn new(
		vertices: Vec<VertexData>,
		faces: Vec<Face>,
		leaves: Vec<BSPLeaf>,
		branches: Vec<BSPBranch>,
	) -> BSPModel {
		BSPModel {
			mesh: DataOrMesh::Data(vertices),
			faces,
			_leaves: leaves,
			_branches: branches,
		}
	}

	pub fn upload(&mut self, queue: &Arc<Queue>) -> Result<Box<dyn GpuFuture>, Box<dyn Error>> {
		for face in &mut self.faces {
			face.texture.borrow_mut().upload(queue)?;
			face.lightmap.borrow_mut().upload(queue)?;
		}

		match &self.mesh {
			DataOrMesh::Data(data) => {
				let (mesh, future) = MeshBuilder::new().with_data(data.clone()).build(queue)?;

				self.mesh = DataOrMesh::Mesh(mesh);
				Ok(Box::from(future))
			}
			DataOrMesh::Mesh(_) => Ok(Box::from(sync::now(queue.device().clone()))),
		}
	}

	pub fn mesh(&self) -> Option<&Mesh> {
		if let DataOrMesh::Mesh(mesh) = &self.mesh {
			Some(mesh)
		} else {
			None
		}
	}

	pub fn faces(&self) -> &Vec<Face> {
		&self.faces
	}
}

#[derive(Debug, Default, Clone)]
pub struct VertexData {
	pub in_position: [f32; 3],
	pub in_texture_coord: [f32; 3],
	pub in_lightmap_coord: [f32; 3],
}
impl_vertex!(VertexData, in_position, in_texture_coord, in_lightmap_coord);

pub struct Face {
	pub first_vertex_index: usize,
	pub vertex_count: usize,
	pub texture: Rc<RefCell<OldTexture>>,
	pub lightmap: Rc<RefCell<OldTexture>>,
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

pub struct OldTexture {
	texture: DataOrTexture,
}

impl OldTexture {
	pub fn new(surfaces: Vec<Surface<'static>>) -> OldTexture {
		assert!(!surfaces.is_empty());
		let size = surfaces[0].size();
		let pixel_format = surfaces[0].pixel_format_enum();

		for surface in &surfaces {
			// All surfaces must be the same size
			assert_eq!(surface.size(), size);

			// All surfaces must have the same pixel format
			assert_eq!(surface.pixel_format_enum(), pixel_format);
		}

		OldTexture {
			texture: DataOrTexture::Data(surfaces),
		}
	}

	pub fn size(&self) -> Vector3<u32> {
		match &self.texture {
			DataOrTexture::Data(surfaces) => Vector3::new(
				surfaces[0].width(),
				surfaces[0].height(),
				surfaces.len() as u32,
			),
			DataOrTexture::Texture(image) => Vector3::new(
				image.inner.dimensions().width(),
				image.inner.dimensions().height(),
				image.inner.dimensions().array_layers(),
			),
		}
	}

	pub fn upload(&mut self, queue: &Arc<Queue>) -> Result<Box<dyn GpuFuture>, ImageCreationError> {
		match &self.texture {
			DataOrTexture::Data(surfaces) => {
				let layer_size = surfaces[0].without_lock().unwrap().len();
				let mut data = vec![0u8; layer_size * surfaces.len()];

				// Copy all the layers into the buffer
				for (chunk, surface) in data.chunks_exact_mut(layer_size).zip(surfaces) {
					chunk.copy_from_slice(surface.without_lock().unwrap());
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
				let (texture, future) = TextureBuilder::new()
					.with_data(data, format)
					.with_dimensions(Dimensions::Dim2dArray {
						width: surfaces[0].width(),
						height: surfaces[0].height(),
						array_layers: surfaces.len() as u32,
					})
					.build(queue)?;

				self.texture = DataOrTexture::Texture(texture);
				Ok(Box::from(future))
			}
			DataOrTexture::Texture(_) => Ok(Box::from(sync::now(queue.device().clone()))),
		}
	}

	pub fn texture(&self) -> Option<&Texture> {
		if let DataOrTexture::Texture(texture) = &self.texture {
			Some(texture)
		} else {
			None
		}
	}
}

enum DataOrMesh {
	Data(Vec<VertexData>),
	Mesh(Mesh),
}

enum DataOrTexture {
	Data(Vec<Surface<'static>>),
	Texture(Texture),
}
