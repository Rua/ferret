use sdl2::video::Window;
use std::{
	cmp::{max, min},
	error::Error,
	ffi::CString,
	sync::Arc,
	u32,
};
use vulkano::{
	VulkanObject,
	device::{Device, DeviceExtensions, Features, Queue},
	format::Format,
	image::{AttachmentImage, ImageCreationError, ImageUsage, swapchain::SwapchainImage},
	instance::{Instance, PhysicalDevice, QueueFamily, RawInstanceExtensions},
	swapchain::{ColorSpace, CompositeAlpha, PresentMode, Surface, Swapchain},
	sync::SharingMode,
};


pub(super) fn create_instance(window: &Window) -> Result<(Arc<Instance>, Arc<Surface<()>>), Box<dyn Error>> {
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

	let instance = Instance::new(Some(&app_info_from_cargo_toml!()), instance_extensions, layers)?;

	// Create Vulkan surface
	let surface = unsafe { Arc::new(Surface::from_raw_surface(
		instance.clone(),
		window.vulkan_create_surface(instance.internal_object())?,
		(),
	)) };

	Ok((instance, surface))
}

fn select_queue_families<'a>(physical_device: PhysicalDevice<'a>, surface: &Surface<()>) -> Result<(Option<QueueFamily<'a>>, Option<QueueFamily<'a>>), Box<dyn Error>> {
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

fn find_suitable_physical_device<'a>(instance: &'a Arc<Instance>, surface: &Surface<()>) -> Result<Option<(PhysicalDevice<'a>, QueueFamily<'a>, QueueFamily<'a>)>, Box<dyn Error>> {
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

pub(super) struct Queues {
	pub graphics: Arc<Queue>,
	pub present: Arc<Queue>,
}

pub(super) fn create_device(instance: &Arc<Instance>, surface: &Arc<Surface<()>>) -> Result<(Arc<Device>, Queues), Box<dyn Error>> {
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

	Ok((device, Queues{ graphics: graphics_queue, present: present_queue }))
}

pub(super) fn create_swapchain(surface: &Arc<Surface<()>>, device: &Arc<Device>, queues: &Queues, dimensions: [u32; 2]) -> Result<(Arc<Swapchain<()>>, Vec<Arc<SwapchainImage<()>>>), Box<dyn Error>> {
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
		let mut actual_extent = dimensions;

		actual_extent[0] = max(capabilities.min_image_extent[0], min(capabilities.max_image_extent[0], actual_extent[0]));
		actual_extent[1] = max(capabilities.min_image_extent[1], min(capabilities.max_image_extent[1], actual_extent[1]));

		actual_extent
	});

	let image_count = min(
		capabilities.min_image_count + 1,
		capabilities.max_image_count.unwrap_or(u32::MAX)
	);

	let sharing_mode = {
		if queues.graphics.family().id() == queues.present.family().id() {
			SharingMode::Exclusive(queues.graphics.family().id())
		} else {
			SharingMode::Concurrent(vec![queues.graphics.family().id(), queues.present.family().id()])
		}
	};

	let image_usage = ImageUsage {
		color_attachment: true,
		transfer_source: true,
		.. ImageUsage::none()
	};

	Ok(Swapchain::new(
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
	)?)
}

pub(super) fn create_depth_buffer(device: &Arc<Device>, extent: [u32; 2]) -> Result<Arc<AttachmentImage>, Box<dyn Error>> {
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
