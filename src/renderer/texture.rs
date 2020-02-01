use crate::renderer::AsBytes;
use std::sync::Arc;
use vulkano::{
	device::Queue,
	format::Format,
	image::{sys::ImageCreationError, Dimensions, ImageViewAccess, ImmutableImage},
	sync::GpuFuture,
};

pub struct Texture {
	inner: Arc<dyn ImageViewAccess + Send + Sync>,
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
		// Create the image
		let (image, future) = ImmutableImage::from_iter(
			self.data.as_bytes().iter().copied(),
			self.dimensions,
			self.format,
			queue,
		)?;

		Ok((Texture { inner: image }, Box::from(future)))
	}
}
