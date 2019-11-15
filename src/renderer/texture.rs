use std::sync::Arc;
use vulkano::{
	buffer::{cpu_access::CpuAccessibleBuffer, BufferUsage},
	device::Queue,
	format::Format,
	image::{sys::ImageCreationError, Dimensions, ImageViewAccess, ImmutableImage},
	sync::GpuFuture,
};

pub struct Texture {
	pub(super) inner: Arc<dyn ImageViewAccess + Send + Sync>,
}

pub struct TextureBuilder {
	data: Vec<u8>,
	dimensions: Dimensions,
	format: Format,
}

impl TextureBuilder {
	pub fn new() -> TextureBuilder {
		TextureBuilder {
			data: Vec::new(),
			dimensions: Dimensions::Dim1d { width: 0 },
			format: Format::R8G8B8A8Unorm,
		}
	}

	pub fn with_data(mut self, data: Vec<u8>, format: Format) -> Self {
		self.data = data;
		self.format = format;
		self
	}

	pub fn with_dimensions(mut self, dimensions: Dimensions) -> Self {
		self.dimensions = dimensions;
		self
	}

	pub fn build(
		self,
		queue: &Arc<Queue>,
	) -> Result<(Texture, Box<dyn GpuFuture>), ImageCreationError> {
		// Create staging buffer
		let buffer = CpuAccessibleBuffer::from_iter(
			queue.device().clone(),
			BufferUsage::transfer_source(),
			self.data.into_iter(),
		)?;

		// Create the image
		let (image, future) =
			ImmutableImage::from_buffer(buffer, self.dimensions, self.format, queue.clone())?;

		Ok((Texture { inner: image }, Box::from(future)))
	}
}
