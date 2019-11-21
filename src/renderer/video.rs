use crate::renderer::vulkan;
pub use crate::renderer::vulkan::Queues;
use std::{error::Error, sync::Arc};
use vulkano::{
	device::Device,
	framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract},
	image::ImageViewAccess,
	instance::debug::DebugCallback,
	sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
	swapchain::{Surface, Swapchain},
};
use vulkano_win::VkSurfaceBuild;
use winit::{
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
};

pub struct Video {
	device: Arc<Device>,
	framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
	queues: Queues,
	render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
	sampler: Arc<Sampler>,
	_surface: Arc<Surface<Window>>,
	swapchain: Arc<Swapchain<Window>>,
}

impl Video {
	pub fn new(event_loop: &EventLoop<()>) -> Result<(Video, DebugCallback), Box<dyn Error>> {
		// Create Vulkan instance
		let instance = vulkan::create_instance()?;

		let surface = WindowBuilder::new()
			.with_inner_size((640, 480).into())
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

		// Create swapchain
		let (width, height) = surface.window().inner_size().into();
		let (swapchain, swapchain_images) =
			vulkan::create_swapchain(&surface, &device, &queues, [width, height])?;

		// Create depth buffer
		let depth_buffer = vulkan::create_depth_buffer(&device, swapchain.dimensions())?;

		// Create render pass
		let render_pass = Arc::new(single_pass_renderpass!(device.clone(),
			attachments: {
				color: {
					load: Clear,
					store: Store,
					format: swapchain.format(),
					samples: 1,
				},
				depth: {
					load: Clear,
					store: DontCare,
					format: ImageViewAccess::inner(&depth_buffer).format(),
					samples: 1,
				}
			},
			pass: {
				color: [color],
				depth_stencil: {depth}
			}
		)?);

		// Create framebuffers
		let framebuffers = {
			let mut framebuffers = Vec::with_capacity(swapchain_images.len());

			for image in swapchain_images.iter() {
				framebuffers.push(Arc::new(
					Framebuffer::start(render_pass.clone())
						.add(image.clone())?
						.add(depth_buffer.clone())?
						.build()?,
				) as Arc<dyn FramebufferAbstract + Send + Sync>);
			}

			framebuffers
		};

		// Create texture sampler
		let sampler = Sampler::new(
			device.clone(),
			Filter::Nearest,
			Filter::Nearest,
			MipmapMode::Nearest,
			SamplerAddressMode::Repeat,
			SamplerAddressMode::Repeat,
			SamplerAddressMode::Repeat,
			0.0,
			1.0,
			0.0,
			0.0,
		)?;

		// All done!
		let video = Video {
			device,
			framebuffers,
			queues,
			render_pass,
			sampler,
			_surface: surface,
			swapchain,
		};

		Ok((video, debug_callback))
	}

	pub fn device(&self) -> Arc<Device> {
		self.device.clone()
	}

	pub fn framebuffer(&self, index: usize) -> Arc<dyn FramebufferAbstract + Send + Sync> {
		self.framebuffers[index].clone()
	}

	pub fn queues(&self) -> &Queues {
		&self.queues
	}

	pub fn render_pass(&self) -> Arc<dyn RenderPassAbstract + Send + Sync> {
		self.render_pass.clone()
	}

	pub fn sampler(&self) -> Arc<Sampler> {
		self.sampler.clone()
	}

	pub fn swapchain(&self) -> Arc<Swapchain<Window>> {
		self.swapchain.clone()
	}
}
