use crate::renderer::vulkan;
pub use crate::renderer::vulkan::Queues;
use std::{error::Error, sync::Arc};
use vulkano::{
	device::{Device, DeviceOwned},
	format::Format,
	image::{swapchain::SwapchainImage, ImageUsage},
	instance::debug::DebugCallback,
	swapchain::{
		AcquireError, Capabilities, ColorSpace, CompositeAlpha, FullscreenExclusive, PresentMode,
		Surface, Swapchain, SwapchainAcquireFuture,
	},
	sync::SharingMode,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
	dpi::Size,
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
};

pub struct Video {
	device: Arc<Device>,
	queues: Queues,
	surface: Arc<Surface<Window>>,
}

impl Video {
	pub fn new(
		event_loop: &EventLoop<()>,
	) -> Result<(Video, DebugCallback), Box<dyn Error + Send + Sync>> {
		// Create Vulkan instance
		let instance = vulkan::create_instance()?;

		let surface = WindowBuilder::new()
			.with_min_inner_size(Size::Physical([320, 240].into()))
			.with_inner_size(Size::Physical([800, 600].into()))
			.with_title("Ferret")
			.build_vk_surface(event_loop, instance.clone())?;

		// Setup debug callback for validation layers
		let debug_callback = DebugCallback::errors_and_warnings(&instance, |ref message| {
			if message.ty.validation {
				log::error!("{}: {}", message.layer_prefix, message.description);
			} else {
				log::warn!("{}: {}", message.layer_prefix, message.description);
			}
		})?;

		// Create Vulkan device
		let (device, queues) = vulkan::create_device(&instance, &surface)?;
		log::info!(
			"Selected Vulkan device: {}",
			device.physical_device().name()
		);

		// All done!
		let video = Video {
			device,
			queues,
			surface,
		};

		Ok((video, debug_callback))
	}

	pub fn device(&self) -> &Arc<Device> {
		&self.device
	}

	pub fn queues(&self) -> &Queues {
		&self.queues
	}

	pub fn surface(&self) -> &Arc<Surface<Window>> {
		&self.surface
	}
}

pub struct RenderTarget {
	images: Vec<Arc<SwapchainImage<Window>>>,
	swapchain: Arc<Swapchain<Window>>,
}

impl RenderTarget {
	pub fn new(
		surface: Arc<Surface<Window>>,
		device: Arc<Device>,
		dimensions: [u32; 2],
	) -> Result<RenderTarget, Box<dyn Error + Send + Sync>> {
		let capabilities = surface.capabilities(device.physical_device())?;
		let surface_format =
			choose_format(&capabilities).ok_or("No suitable swapchain format found.")?;
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

		let (swapchain, images) = Swapchain::new(
			device,
			surface.clone(),
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
		)?;

		Ok(RenderTarget { images, swapchain })
	}

	pub fn acquire_next_image(
		&self,
	) -> Result<(usize, bool, SwapchainAcquireFuture<Window>), AcquireError> {
		vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None)
	}

	pub fn device(&self) -> &Arc<Device> {
		self.swapchain.device()
	}

	pub fn format(&self) -> Format {
		self.swapchain.format()
	}

	pub fn recreate(
		&mut self,
		dimensions: [u32; 2],
	) -> Result<RenderTarget, Box<dyn Error + Send + Sync>> {
		let capabilities = self
			.swapchain()
			.surface()
			.capabilities(self.swapchain.device().physical_device())?;
		let surface_format =
			choose_format(&capabilities).ok_or("No suitable swapchain format found.")?;

		let image_usage = ImageUsage {
			color_attachment: true,
			transfer_source: true,
			..ImageUsage::none()
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
		)?;

		Ok(RenderTarget { images, swapchain })
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
