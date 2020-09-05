use anyhow::Context;
use std::sync::Arc;
use vulkano::{
	app_info_from_cargo_toml,
	device::{Device, DeviceExtensions, Features, Queue},
	instance::{
		debug::{DebugCallback, MessageSeverity, MessageType},
		Instance, InstanceExtensions, PhysicalDevice, QueueFamily,
	},
	swapchain::Surface,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
	dpi::Size,
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
};

pub struct RenderContext {
	device: Arc<Device>,
	queues: Queues,
	surface: Arc<Surface<Window>>,
}

impl RenderContext {
	pub fn new(
		event_loop: &EventLoop<()>,
	) -> anyhow::Result<(RenderContext, Option<DebugCallback>)> {
		log::debug!("Loading Vulkan library");
		// Load the Vulkan library
		vulkano::instance::loader::auto_loader().context("Couldn't load the Vulkan library")?;

		// Create Vulkan instance
		log::debug!("Creating Vulkan instance");
		let instance = create_instance().context("Couldn't create Vulkan instance")?;

		log::debug!("Creating Vulkan window and surface");
		let surface = WindowBuilder::new()
			.with_min_inner_size(Size::Physical([320, 240].into()))
			.with_inner_size(Size::Physical([800, 600].into()))
			.with_title("Ferret")
			.build_vk_surface(event_loop, instance.clone())
			.context("Couldn't create Vulkan rendering window")?;

		// Setup debug callback for validation layers
		#[cfg(debug_assertions)]
		let debug_callback = DebugCallback::new(
			&instance,
			MessageSeverity {
				error: true,
				warning: true,
				information: true,
				verbose: true,
			},
			MessageType::all(),
			|ref message| {
				if message.severity.error {
					log::error!("{}: {}", message.layer_prefix, message.description);
				} else if message.severity.warning {
					log::warn!("{}: {}", message.layer_prefix, message.description);
				} else {
					log::trace!("{}: {}", message.layer_prefix, message.description);
				}
			},
		)
		.ok();

		#[cfg(not(debug_assertions))]
		let debug_callback = None;

		// Create Vulkan device
		log::debug!("Creating Vulkan device");
		let (device, queues) =
			create_device(&instance, &surface).context("Couldn't create Vulkan device")?;
		log::info!(
			"Selected Vulkan device: {}",
			device.physical_device().name()
		);

		// All done!
		Ok((
			RenderContext {
				device,
				queues,
				surface,
			},
			debug_callback,
		))
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

fn create_instance() -> anyhow::Result<Arc<Instance>> {
	let mut instance_extensions = vulkano_win::required_extensions();
	let supported_extensions = InstanceExtensions::supported_by_core().unwrap();

	let mut layers = Vec::new();

	#[cfg(debug_assertions)]
	{
		if supported_extensions.ext_debug_utils {
			instance_extensions.ext_debug_utils = true;

			let available_layers: Vec<_> = vulkano::instance::layers_list()?.collect();

			for to_enable in [
				"VK_LAYER_LUNARG_standard_validation",
				"VK_LAYER_LUNARG_monitor",
			]
			.iter()
			{
				if available_layers.iter().any(|l| l.name() == *to_enable) {
					layers.push(*to_enable);
				}
			}

			log::debug!(
				"EXT_debug_utils is available, enabled Vulkan validation layers: {}",
				layers.join(", ")
			);
		} else {
			log::debug!("EXT_debug_utils not available, Vulkan validation layers disabled");
		}
	}

	let instance = Instance::new(
		Some(&app_info_from_cargo_toml!()),
		&instance_extensions,
		layers,
	)?;

	Ok(instance)
}

fn find_suitable_physical_device<'a>(
	instance: &'a Arc<Instance>,
	surface: &Surface<Window>,
) -> anyhow::Result<Option<(PhysicalDevice<'a>, QueueFamily<'a>)>> {
	for physical_device in PhysicalDevice::enumerate(&instance) {
		let family = {
			let mut val = None;

			for family in physical_device.queue_families() {
				if family.supports_graphics() && surface.is_supported(family)? {
					val = Some(family);
					break;
				}
			}

			val
		};

		if family.is_none() {
			continue;
		}

		let supported_extensions = DeviceExtensions::supported_by_device(physical_device);

		if !supported_extensions.khr_swapchain {
			continue;
		}

		let capabilities = surface.capabilities(physical_device)?;

		if capabilities.supported_formats.is_empty()
			|| capabilities.present_modes.iter().count() == 0
		{
			continue;
		}

		return Ok(Some((physical_device, family.unwrap())));
	}

	Ok(None)
}

pub struct Queues {
	pub graphics: Arc<Queue>,
}

fn create_device(
	instance: &Arc<Instance>,
	surface: &Arc<Surface<Window>>,
) -> anyhow::Result<(Arc<Device>, Queues)> {
	let (physical_device, family) = find_suitable_physical_device(&instance, &surface)?
		.context("No suitable physical device found")?;

	let features = Features::none();
	let extensions = DeviceExtensions {
		khr_swapchain: true,
		..DeviceExtensions::none()
	};

	let (device, mut queues) =
		Device::new(physical_device, &features, &extensions, vec![(family, 1.0)])?;

	Ok((
		device,
		Queues {
			graphics: queues.next().unwrap(),
		},
	))
}
