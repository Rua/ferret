use crate::renderer::vulkan;
pub use crate::renderer::vulkan::Queues;
use std::{error::Error, sync::Arc};
use vulkano::{
	device::{Device, DeviceOwned},
	format::Format,
	image::{swapchain::SwapchainImage, ImageUsage},
	instance::debug::DebugCallback,
	swapchain::{
		AcquireError, Capabilities, ColorSpace, CompositeAlpha, PresentMode, Surface, Swapchain,
		SwapchainAcquireFuture,
	},
	sync::SharingMode,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
};

pub struct Video {
	device: Arc<Device>,
	queues: Queues,
	surface: Arc<Surface<Window>>,
}

impl Video {
	pub fn new(event_loop: &EventLoop<()>) -> Result<(Video, DebugCallback), Box<dyn Error>> {
		// Create Vulkan instance
		let instance = vulkan::create_instance()?;

		let surface = WindowBuilder::new()
			.with_min_inner_size((320, 240).into())
			.with_inner_size((800, 600).into())
			.with_title("Ferret")
			.build_vk_surface(event_loop, instance.clone())?;

		// Setup debug callback for validation layers
		let debug_callback = DebugCallback::errors_and_warnings(&instance, |ref message| {
			if message.ty.error {
				error!("{}: {}", message.layer_prefix, message.description);
			} else {
				warn!("{}: {}", message.layer_prefix, message.description);
			}
		})?;

		// Create Vulkan device
		let (device, queues) = vulkan::create_device(&instance, &surface)?;

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
	queue_family_id: u32,
	surface: Arc<Surface<Window>>,
	swapchain: Arc<Swapchain<Window>>,
}

impl RenderTarget {
	pub fn new(
		surface: Arc<Surface<Window>>,
		device: Arc<Device>,
		queue_family_id: u32,
		dimensions: [u32; 2],
	) -> Result<RenderTarget, Box<dyn Error>> {
		let capabilities = surface.capabilities(device.physical_device())?;
		let surface_format =
			choose_format(&capabilities).ok_or("No suitable swapchain format found.")?;
		let present_mode = [PresentMode::Mailbox, PresentMode::Fifo]
			.into_iter()
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
			SharingMode::Exclusive(queue_family_id),
			capabilities.current_transform,
			CompositeAlpha::Opaque,
			present_mode,
			true,
			None,
		)?;

		Ok(RenderTarget {
			images,
			queue_family_id,
			surface,
			swapchain,
		})
	}

	pub fn acquire_next_image(
		&self,
	) -> Result<(usize, SwapchainAcquireFuture<Window>), AcquireError> {
		vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None)
	}

	pub fn device(&self) -> &Arc<Device> {
		self.swapchain.device()
	}

	pub fn format(&self) -> Format {
		self.swapchain.format()
	}

	pub fn recreate(&self, dimensions: [u32; 2]) -> Result<RenderTarget, Box<dyn Error>> {
		let capabilities = self
			.surface
			.capabilities(self.swapchain.device().physical_device())?;
		let surface_format =
			choose_format(&capabilities).ok_or("No suitable swapchain format found.")?;

		let image_usage = ImageUsage {
			color_attachment: true,
			transfer_source: true,
			..ImageUsage::none()
		};

		let (swapchain, images) = Swapchain::new(
			self.swapchain.device().clone(),
			self.surface.clone(),
			self.swapchain.num_images(),
			surface_format,
			capabilities.current_extent.unwrap_or(dimensions),
			1,
			image_usage,
			SharingMode::Exclusive(self.queue_family_id),
			capabilities.current_transform,
			CompositeAlpha::Opaque,
			self.swapchain.present_mode(),
			true,
			Some(&self.swapchain),
		)?;

		Ok(RenderTarget {
			images,
			queue_family_id: self.queue_family_id,
			surface: self.surface.clone(),
			swapchain,
		})
	}

	pub fn images(&self) -> &Vec<Arc<SwapchainImage<Window>>> {
		&self.images
	}

	pub fn surface(&self) -> &Arc<Surface<Window>> {
		&self.surface
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
