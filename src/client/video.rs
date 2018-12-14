use sdl2::{Sdl, VideoSubsystem};
use sdl2::video::{Window, WindowContext};
use std::error::Error;
use std::rc::Rc;
use std::sync::Arc;
use vulkano::device::{Device, DeviceExtensions, DeviceOwned, Queue};
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::format::Format;
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract};
use vulkano::image::{AttachmentImage, ImageCreationError, ImageViewAccess};
use vulkano::instance::{Instance, PhysicalDevice, QueueFamily};
use vulkano::instance::debug::DebugCallback;
use vulkano::sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode};
use vulkano::swapchain;
use vulkano::swapchain::{Surface, Swapchain};
use vulkano::sync::GpuFuture;

use crate::client::vulkan;
use crate::client::vulkan::Queues;


pub struct Video {
	debug_callback: DebugCallback,
	framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,
	queues: Queues,
	swapchain: Arc<Swapchain<()>>,
	
	sdl_video: VideoSubsystem,
	window: Window,
}

impl Video {
	pub fn init(sdl: &Sdl) -> Result<Video, Box<dyn Error>> {
		let sdl_video = sdl.video()?;
		
		let window = sdl_video
			.window("Ferret", 640, 480)
			.vulkan()
			.position_centered()
			.build()?;
		
		// Create Vulkan instance and surface
		let (instance, surface) = vulkan::create_instance(&window)?;
		
		// Setup debug callback for validation layers
		#[cfg(debug_assertions)]
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
		let dimensions = window.vulkan_drawable_size();
		let (swapchain, swapchain_images) = vulkan::create_swapchain(
			&surface,
			&device,
			&queues,
			[dimensions.0, dimensions.1],
		)?;
		
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
				framebuffers.push(
					Arc::new(Framebuffer::start(render_pass.clone())
						.add(image.clone())?
						.add(depth_buffer.clone())?
						.build()?
					) as Arc<FramebufferAbstract + Send + Sync>
				);
			}
			
			framebuffers
		};
		
		// Create texture sampler
		let _sampler = Sampler::new(device.clone(),
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
			debug_callback,
			framebuffers,
			queues,
			swapchain,
			
			sdl_video,
			window,
		};
		
		Ok(video)
	}
	
	pub fn draw_frame(&self) -> Result<(), Box<dyn Error>> {
		let (image_num, future) = match swapchain::acquire_next_image(self.swapchain.clone(), None) {
			Ok(r) => r,
			Err(err) => panic!("{:?}", err)
		};
		
		let clear_value = vec![[0.0, 0.0, 1.0, 1.0].into(), 1.0.into()];
		
		let command_buffer = Arc::new(AutoCommandBufferBuilder::primary_one_time_submit(self.swapchain.device().clone(), self.queues.graphics.family())?
			.begin_render_pass(
				self.framebuffers[image_num].clone(),
				false,
				clear_value,
			)?
			.end_render_pass()?
			.build()?
		);
		
		future
			.then_execute(self.queues.graphics.clone(), command_buffer)?
			.then_swapchain_present(self.queues.present.clone(), self.swapchain.clone(), image_num)
			.then_signal_fence_and_flush()?
			.wait(None)?;
		
		Ok(())
	}
}
