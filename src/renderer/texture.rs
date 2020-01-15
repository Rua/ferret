use crate::{
	assets::{Asset, DataSource},
	renderer::AsBytes,
};
use std::{error::Error, sync::Arc};
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

	fn import(
		name: &str,
		source: &impl DataSource,
	) -> Result<Self::Intermediate, Box<dyn Error + Send + Sync>> {
		unimplemented!();
	}
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
	data: Box<dyn AsBytes + Send + Sync>,
	dimensions: Dimensions,
	format: Format,
}

impl TextureBuilder {
	pub fn new() -> TextureBuilder {
		TextureBuilder {
			data: Box::new(Vec::<u8>::new()),
			dimensions: Dimensions::Dim1d { width: 0 },
			format: Format::R8G8B8A8Unorm,
		}
	}

	pub fn with_data<V: AsBytes + Send + Sync + 'static>(mut self, data: V) -> Self {
		self.data = Box::new(data);
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
			self.data.as_bytes().iter().copied(),
		)?;

		// Create the image
		let (image, future) =
			ImmutableImage::from_buffer(buffer, self.dimensions, self.format, queue)?;

		Ok((Texture { inner: image }, Box::from(future)))
	}
}
