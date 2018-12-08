use sdl2::{Sdl, VideoSubsystem};
use sdl2::video::{Window, WindowContext};
use std::cmp::{max, min};
use std::error::Error;
use std::ffi::CString;
use std::rc::Rc;
use std::sync::Arc;
use std::u32;
use vulkano::VulkanObject;
use vulkano::device::{Device, DeviceExtensions, Features};
use vulkano::format::Format;
use vulkano::framebuffer::Framebuffer;
use vulkano::image::{AttachmentImage, ImageCreationError, ImageUsage, ImageViewAccess};
use vulkano::instance::{Instance, PhysicalDevice, QueueFamily, RawInstanceExtensions};
use vulkano::instance::debug::DebugCallback;
use vulkano::sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode};
use vulkano::swapchain::{ColorSpace, CompositeAlpha, PresentMode, Surface, Swapchain};
use vulkano::sync::SharingMode;

pub struct Video {
	sdl_video: VideoSubsystem,
	window: Window,
}

fn select_queue_families<'a>(physical_device: PhysicalDevice<'a>, surface: &Surface<Rc<WindowContext>>) -> Result<(Option<QueueFamily<'a>>, Option<QueueFamily<'a>>), Box<dyn Error>> {
	for family in physical_device.queue_families() {
		if family.supports_graphics() && surface.is_supported(family)? {
			return Ok((Some(family), Some(family)));
		}
	}
	
	let mut graphics_family = None;
	let mut present_family = None;
	
	for family in physical_device.queue_families() {
		if family.supports_graphics() {
			graphics_family = Some(family);
			break;
		}
	}
	
	for family in physical_device.queue_families() {
		if surface.is_supported(family)? {
			present_family = Some(family);
			break;
		}
	}
	
	Ok((graphics_family, present_family))
}

fn find_suitable_physical_device<'a>(instance: &'a Arc<Instance>, surface: &Surface<Rc<WindowContext>>) -> Result<Option<(PhysicalDevice<'a>, QueueFamily<'a>, QueueFamily<'a>)>, Box<dyn Error>> {
	for physical_device in PhysicalDevice::enumerate(&instance) {
		let (graphics_family, present_family) = select_queue_families(physical_device, &surface)?;
		
		if graphics_family.is_none() || present_family.is_none() {
			continue;
		}
		
		let supported_extensions = DeviceExtensions::supported_by_device(physical_device);
		
		if !supported_extensions.khr_swapchain {
			continue;
		}
		
		let capabilities = surface.capabilities(physical_device)?;
		
		if capabilities.supported_formats.is_empty() || capabilities.present_modes.iter().count() == 0 {
			continue;
		}
		
		return Ok(Some((physical_device, graphics_family.unwrap(), present_family.unwrap())));
	};
	
	Ok(None)
}

fn create_depth_buffer(device: &Arc<Device>, extent: [u32; 2]) -> Result<Arc<AttachmentImage>, Box<dyn Error>> {
	let allowed_formats = [
		Format::D32Sfloat,
		Format::D32Sfloat_S8Uint,
		Format::D24Unorm_S8Uint,
		Format::D16Unorm,
		Format::D16Unorm_S8Uint
	];
	
	for format in allowed_formats.iter().cloned() {
		match AttachmentImage::transient(device.clone(), extent, format) {
			Ok(buf) => return Ok(buf),
			Err(ImageCreationError::FormatNotSupported) => continue,
			Err(any) => return Err(Box::from(any)),
		}
	}
	
	Err(Box::from("No suitable depth buffer format found."))
}

impl Video {
	pub fn init(sdl: &Sdl) -> Result<Video, Box<dyn Error>> {
		let sdl_video = sdl.video()?;
		
		let window = sdl_video
			.window("Ferret", 640, 480)
			.vulkan()
			.position_centered()
			.build()?;
		
		// Create Vulkan instance
		let instance = {
			let instance_extensions = {
				let sdl_instance_extensions = window.vulkan_instance_extensions()?;
				let mut instance_extensions = RawInstanceExtensions::new(sdl_instance_extensions.iter().map(|&v| CString::new(v).unwrap()));
				
				#[cfg(debug_assertions)]
				instance_extensions.insert(CString::new("VK_EXT_debug_report").unwrap());
				
				instance_extensions
			};
			
			let mut layers = Vec::new();
			
			#[cfg(debug_assertions)]
			layers.push("VK_LAYER_LUNARG_standard_validation");
		
			Instance::new(Some(&app_info_from_cargo_toml!()), instance_extensions, layers)?
		};
		
		// Setup debug callback for validation layers
		#[cfg(debug_assertions)]
		let _debug_callback = DebugCallback::errors_and_warnings(&instance, |ref message| {
			if message.ty.error {
				error!("{}: {}", message.layer_prefix, message.description);
			} else {
				warn!("{}: {}", message.layer_prefix, message.description);
			}
		})?;
		
		// Create Vulkan surface
		let surface = unsafe { Arc::new(Surface::from_raw_surface(
			instance.clone(),
			window.vulkan_create_surface(instance.internal_object())?,
			window.context()
		)) };
		
		// Create Vulkan device
		let (device, graphics_queue, present_queue) = {
			// Select physical device
			let (physical_device, graphics_family, present_family) =
				find_suitable_physical_device(&instance, &surface)?
				.ok_or("No suitable physical device found.")?;
			
			let mut queues = vec![(graphics_family, 1.0)];
			
			if graphics_family.id() != present_family.id() {
				queues.push((present_family, 1.0));
			}
			
			let features = Features::none();
			let mut extensions = DeviceExtensions::none();
			extensions.khr_swapchain = true;
			
			let (device, queues) = Device::new(physical_device, &features, &extensions, queues)?;
			let queues = queues.collect::<Vec<_>>();
			let graphics_queue = queues.iter().find(|queue| queue.family().id() == graphics_family.id()).unwrap().clone();
			let present_queue = queues.iter().find(|queue| queue.family().id() == present_family.id()).unwrap().clone();
			
			(device, graphics_queue, present_queue)
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
		
		// Create swapchain
		let (swapchain, swapchain_images) = {
			let capabilities = surface.capabilities(device.physical_device())?;
			
			let surface_format = {
				let srgb_formats = capabilities.supported_formats.iter()
					.filter(|f| f.1 == ColorSpace::SrgbNonLinear)
					.map(|f| f.0)
					.collect::<Vec<_>>();
				
				let allowed_formats = [
					Format::B8G8R8A8Unorm,
					Format::R8G8B8A8Unorm,
					Format::A8B8G8R8UnormPack32,
				];
				
				allowed_formats.iter().cloned()
					.find(|f| srgb_formats.iter().any(|g| g == f))
					.ok_or("No suitable swapchain format found.")?
			};
			
			let present_mode = if capabilities.present_modes.supports(PresentMode::Mailbox) {
				PresentMode::Mailbox
			} else {
				PresentMode::Fifo
			};
			
			let extent = capabilities.current_extent.unwrap_or_else(|| {
				let window_size = window.size();
				let mut actual_extent = [window_size.0, window_size.1];
				
				actual_extent[0] = max(capabilities.min_image_extent[0], min(capabilities.max_image_extent[0], actual_extent[0]));
				actual_extent[1] = max(capabilities.min_image_extent[1], min(capabilities.max_image_extent[1], actual_extent[1]));
				
				actual_extent
			});
			
			let image_count = min(
				capabilities.min_image_count + 1,
				capabilities.max_image_count.unwrap_or(u32::MAX)
			);
			
			let sharing_mode = {
				if graphics_queue.family().id() == present_queue.family().id() {
					SharingMode::Exclusive(graphics_queue.family().id())
				} else {
					SharingMode::Concurrent(vec![graphics_queue.family().id(), present_queue.family().id()])
				}
			};
			
			let image_usage = ImageUsage {
				color_attachment: true,
				.. ImageUsage::none()
			};
			
			Swapchain::new(
				device.clone(),
				surface.clone(),
				image_count,
				surface_format,
				extent,
				1,
				image_usage,
				sharing_mode,
				capabilities.current_transform,
				CompositeAlpha::Opaque,
				present_mode,
				true,
				None,
			)?
		};
		
		// Create depth buffer
		let depth_buffer = create_depth_buffer(&device, swapchain.dimensions())?;
		
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
		let _framebuffers = {
			let mut framebuffers = Vec::with_capacity(swapchain_images.len());
			
			for image in swapchain_images.iter() {
				framebuffers.push(
					Arc::new(Framebuffer::start(render_pass.clone())
						.add(image.clone())?
						.add(depth_buffer.clone())?
						.build()?
					)
				);
			}
			
			framebuffers
		};
		
		let video = Video {
			sdl_video: sdl_video,
			window: window,
		};
		
		Ok(video)
	}
}
