use anyhow::{anyhow, Context};
use std::sync::Arc;
use vulkano::{
	device::{Device, DeviceOwned},
	format::Format,
	image::{
		swapchain::SwapchainImage, AttachmentImage, ImageCreationError, ImageUsage, ImageViewAccess,
	},
	swapchain::{
		AcquireError, Capabilities, ColorSpace, CompositeAlpha, FullscreenExclusive, PresentMode,
		Surface, Swapchain, SwapchainAcquireFuture,
	},
	sync::SharingMode,
};
use winit::window::Window;

pub struct RenderTarget {
	depth_buffer: Option<Arc<AttachmentImage>>,
	images: Vec<Arc<SwapchainImage<Window>>>,
	swapchain: Arc<Swapchain<Window>>,
}

impl RenderTarget {
	pub fn new(
		surface: Arc<Surface<Window>>,
		device: Arc<Device>,
		dimensions: [u32; 2],
		with_depth_buffer: bool,
	) -> anyhow::Result<RenderTarget> {
		let capabilities = surface.capabilities(device.physical_device())?;
		let surface_format =
			choose_format(&capabilities).context("No suitable swapchain format found")?;
		let present_mode = [PresentMode::Mailbox, PresentMode::Fifo]
			.iter()
			.copied()
			.find(|mode| capabilities.present_modes.supports(*mode))
			.unwrap();

		let image_count = u32::min(
			capabilities.min_image_count + 1,
			capabilities.max_image_count.unwrap_or(std::u32::MAX),
		);

		let image_usage = ImageUsage {
			color_attachment: true,
			transfer_source: true,
			..ImageUsage::none()
		};

		// Create depth buffer
		let depth_buffer = if with_depth_buffer {
			Some(
				create_depth_buffer(&device, capabilities.current_extent.unwrap_or(dimensions))
					.context("Couldn't create depth buffer")?,
			)
		} else {
			None
		};

		// Create swapchain and images
		let (swapchain, images) = Swapchain::new(
			device.clone(),
			surface,
			image_count,
			surface_format,
			capabilities.current_extent.unwrap_or(dimensions),
			1,
			image_usage,
			SharingMode::Exclusive,
			capabilities.current_transform,
			CompositeAlpha::Opaque,
			present_mode,
			FullscreenExclusive::Default,
			true,
			ColorSpace::SrgbNonLinear,
		)
		.context("Couldn't create swapchain")?;

		Ok(RenderTarget {
			depth_buffer,
			images,
			swapchain,
		})
	}

	pub fn acquire_next_image(
		&self,
	) -> Result<(usize, bool, SwapchainAcquireFuture<Window>), AcquireError> {
		vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None)
	}

	pub fn device(&self) -> &Arc<Device> {
		self.swapchain.device()
	}

	pub fn image_format(&self) -> Format {
		self.swapchain.format()
	}

	pub fn depth_format(&self) -> Option<Format> {
		self.depth_buffer
			.as_ref()
			.map(|b| ImageViewAccess::inner(&*b).format())
	}

	pub fn recreate(&mut self, dimensions: [u32; 2]) -> anyhow::Result<RenderTarget> {
		let capabilities = self
			.swapchain()
			.surface()
			.capabilities(self.swapchain.device().physical_device())?;
		let surface_format =
			choose_format(&capabilities).context("No suitable swapchain format found")?;

		let image_usage = ImageUsage {
			color_attachment: true,
			transfer_source: true,
			..ImageUsage::none()
		};

		let depth_buffer = if self.depth_buffer.is_some() {
			Some(
				create_depth_buffer(
					self.swapchain.device(),
					capabilities.current_extent.unwrap_or(dimensions),
				)
				.context("Couldn't create depth buffer")?,
			)
		} else {
			None
		};

		let (swapchain, images) = Swapchain::with_old_swapchain(
			self.swapchain.device().clone(),
			self.swapchain.surface().clone(),
			self.swapchain.num_images(),
			surface_format,
			capabilities.current_extent.unwrap_or(dimensions),
			1,
			image_usage,
			SharingMode::Exclusive,
			capabilities.current_transform,
			CompositeAlpha::Opaque,
			self.swapchain.present_mode(),
			FullscreenExclusive::Default,
			true,
			ColorSpace::SrgbNonLinear,
			self.swapchain.clone(),
		)
		.context("Couldn't create swapchain")?;

		Ok(RenderTarget {
			depth_buffer,
			images,
			swapchain,
		})
	}

	pub fn depth_buffer(&self) -> Option<&Arc<AttachmentImage>> {
		self.depth_buffer.as_ref()
	}

	pub fn images(&self) -> &Vec<Arc<SwapchainImage<Window>>> {
		&self.images
	}

	pub fn swapchain(&self) -> &Arc<Swapchain<Window>> {
		&self.swapchain
	}
}

fn choose_format(capabilities: &Capabilities) -> Option<Format> {
	let srgb_formats = capabilities
		.supported_formats
		.iter()
		.filter_map(|f| {
			if f.1 == ColorSpace::SrgbNonLinear {
				Some(f.0)
			} else {
				None
			}
		})
		.collect::<Vec<_>>();

	let allowed_formats = [
		Format::B8G8R8A8Unorm,
		Format::R8G8B8A8Unorm,
		Format::A8B8G8R8UnormPack32,
	];

	allowed_formats
		.iter()
		.cloned()
		.find(|f| srgb_formats.iter().any(|g| g == f))
}

pub fn create_depth_buffer(
	device: &Arc<Device>,
	extent: [u32; 2],
) -> anyhow::Result<Arc<AttachmentImage>> {
	let allowed_formats = [
		Format::D32Sfloat,
		Format::D32Sfloat_S8Uint,
		Format::D24Unorm_S8Uint,
		Format::D16Unorm,
		Format::D16Unorm_S8Uint,
	];

	for format in allowed_formats.iter().cloned() {
		match AttachmentImage::transient(device.clone(), extent, format) {
			Ok(buf) => return Ok(buf),
			Err(ImageCreationError::FormatNotSupported) => continue,
			Err(any) => Err(any)?,
		}
	}

	Err(anyhow!("No suitable depth buffer format found"))
}
