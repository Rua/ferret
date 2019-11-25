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
	data: Box<dyn AsBytes>,
}

impl MeshBuilder {
	pub fn new() -> MeshBuilder {
		MeshBuilder { data: Box::new(Vec::<u8>::new()) }
	}

	pub fn with_data<V: AsBytes + 'static>(mut self, data: V) -> MeshBuilder {
		self.data = Box::new(data);
		self
	}

	pub fn build(
		self,
		queue: &Arc<Queue>,
	) -> Result<(Mesh, Box<dyn GpuFuture>), DeviceMemoryAllocError> {
		let (buffer, future) = ImmutableBuffer::from_iter(
			self.data.as_bytes().iter().copied(),
			BufferUsage::vertex_buffer(),
			queue.clone(),
		)?;

		Ok((Mesh { inner: buffer }, Box::from(future)))
	}
}

pub trait AsBytes {
	fn as_bytes<'a>(&'a self) -> &'a [u8];
}

impl<T> AsBytes for Vec<T> {
	fn as_bytes<'a>(&'a self) -> &'a [u8] {
		let slice = self.as_slice();
		unsafe {
			std::slice::from_raw_parts(
				slice.as_ptr() as _,
				std::mem::size_of::<T>() * slice.len(),
			)
		}
	}
}
