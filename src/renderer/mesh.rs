use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, ImmutableBuffer, TypedBufferAccess},
	device::Queue,
	memory::DeviceMemoryAllocError,
	sync::GpuFuture,
};

pub struct Mesh {
	vertex_buffer: Arc<dyn TypedBufferAccess<Content = [u8]> + Send + Sync>,
	index_buffer: Option<Arc<dyn TypedBufferAccess<Content = [u32]> + Send + Sync>>,
}

impl Mesh {
	pub fn vertex_buffer(&self) -> Arc<dyn TypedBufferAccess<Content = [u8]> + Send + Sync> {
		self.vertex_buffer.clone()
	}

	pub fn index_buffer(
		&self,
	) -> Option<Arc<dyn TypedBufferAccess<Content = [u32]> + Send + Sync>> {
		self.index_buffer.clone()
	}
}

pub struct MeshBuilder {
	vertices: Box<dyn AsBytes + Send + Sync>,
	indices: Option<Vec<u32>>,
}

impl MeshBuilder {
	pub fn new() -> MeshBuilder {
		MeshBuilder {
			vertices: Box::new(Vec::<u8>::new()),
			indices: None,
		}
	}

	pub fn with_vertices<V: AsBytes + Send + Sync + 'static>(mut self, vertices: V) -> MeshBuilder {
		self.vertices = Box::new(vertices);
		self
	}

	pub fn with_indices(mut self, indices: Vec<u32>) -> MeshBuilder {
		self.indices = Some(indices);
		self
	}

	pub fn build(
		self,
		queue: Arc<Queue>,
	) -> Result<(Mesh, Box<dyn GpuFuture>), DeviceMemoryAllocError> {
		let (vertex_buffer, future) = ImmutableBuffer::from_iter(
			self.vertices.as_bytes().iter().copied(),
			BufferUsage::vertex_buffer(),
			queue.clone(),
		)?;

		if let Some(indices) = self.indices {
			let (index_buffer, index_future) = ImmutableBuffer::from_iter(
				indices.iter().copied(),
				BufferUsage::index_buffer(),
				queue.clone(),
			)?;

			Ok((
				Mesh {
					vertex_buffer: vertex_buffer,
					index_buffer: Some(index_buffer),
				},
				Box::from(future.join(index_future)),
			))
		} else {
			Ok((
				Mesh {
					vertex_buffer: vertex_buffer,
					index_buffer: None,
				},
				Box::from(future),
			))
		}
	}
}

pub trait AsBytes {
	fn as_bytes<'a>(&'a self) -> &'a [u8];
}

impl<T> AsBytes for Vec<T> {
	fn as_bytes<'a>(&'a self) -> &'a [u8] {
		let slice = self.as_slice();
		unsafe {
			std::slice::from_raw_parts(slice.as_ptr() as _, std::mem::size_of::<T>() * slice.len())
		}
	}
}
