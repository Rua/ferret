use crate::assets::Asset;
use std::sync::Arc;
use vulkano::{
	buffer::{cpu_access::CpuAccessibleBuffer, BufferUsage},
	device::Queue,
	format::Format,
	image::{sys::ImageCreationError, Dimensions, ImageViewAccess, ImmutableImage},
	sync::GpuFuture,
};

pub struct Texture {
	inner: Arc<dyn ImageViewAccess + Send + Sync>,
}

impl Asset for Texture {
	type Intermediate = TextureBuilder;
	type Data = Self;
	const NAME: &'static str = "Texture";
}

impl Texture {
	pub fn inner(&self) -> Arc<dyn ImageViewAccess + Send + Sync> {
		self.inner.clone()
	}

	pub fn dimensions(&self) -> Dimensions {
		self.inner.dimensions()
	}
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

	pub fn with_data(mut self, data: Vec<u8>) -> Self {
		self.data = data;
		self
	}

	pub fn with_dimensions(mut self, dimensions: Dimensions) -> Self {
		self.dimensions = dimensions;
		self
	}

	pub fn with_format(mut self, format: Format) -> Self {
		self.format = format;
		self
	}

	pub fn build(
		self,
		queue: Arc<Queue>,
	) -> Result<(Texture, Box<dyn GpuFuture>), ImageCreationError> {
		// Create staging buffer
		let buffer = CpuAccessibleBuffer::from_iter(
			queue.device().clone(),
			BufferUsage::transfer_source(),
			self.data.into_iter(),
		)?;

		// Create the image
		let (image, future) =
			ImmutableImage::from_buffer(buffer, self.dimensions, self.format, queue)?;

		Ok((Texture { inner: image }, Box::from(future)))
	}
}
