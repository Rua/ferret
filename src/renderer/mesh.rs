use crate::renderer::model::VertexData;
use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, ImmutableBuffer, TypedBufferAccess},
	device::Queue,
	memory::DeviceMemoryAllocError,
	sync::GpuFuture,
};

pub struct Mesh {
	inner: Arc<dyn TypedBufferAccess<Content = [u8]> + Send + Sync>,
}

impl Mesh {
	pub fn inner(&self) -> Arc<dyn TypedBufferAccess<Content = [u8]> + Send + Sync> {
		self.inner.clone()
	}
}

pub struct MeshBuilder {
	data: Vec<VertexData>,
}

impl MeshBuilder {
	pub fn new() -> MeshBuilder {
		MeshBuilder { data: Vec::new() }
	}

	pub fn with_data(mut self, data: Vec<VertexData>) -> MeshBuilder {
		self.data = data;
		self
	}

	pub fn build(
		self,
		queue: &Arc<Queue>,
	) -> Result<(Mesh, Box<dyn GpuFuture>), DeviceMemoryAllocError> {
		let slice = {
			let slice = self.data.as_slice();
			unsafe {
				std::slice::from_raw_parts(
					slice.as_ptr() as _,
					std::mem::size_of::<VertexData>() * slice.len(),
				)
			}
		};

		let (buffer, future) = ImmutableBuffer::from_iter(
			slice.iter().copied(),
			BufferUsage::vertex_buffer(),
			queue.clone(),
		)?;

		Ok((Mesh { inner: buffer }, Box::from(future)))
	}
}
