use crate::{
	doom::{map, wad::WadLoader},
	model::{BSPModel, VertexData},
	vulkan::{self, Queues},
};
use nalgebra::{Matrix4, Point3, Vector3};
use std::{error::Error, f32::consts::FRAC_PI_4, ops::Range, sync::Arc};
use vulkano::{
	buffer::{BufferSlice, BufferUsage, CpuAccessibleBuffer},
	command_buffer::{AutoCommandBufferBuilder, DynamicState},
	descriptor::descriptor_set::FixedSizeDescriptorSetsPool,
	device::DeviceOwned,
	framebuffer::{Framebuffer, FramebufferAbstract, Subpass},
	image::ImageViewAccess,
	instance::debug::DebugCallback,
	pipeline::{viewport::Viewport, GraphicsPipeline, GraphicsPipelineAbstract},
	sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
	swapchain::{self, Surface, Swapchain},
	sync::GpuFuture,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
};

mod vs {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/world.vert",
	}
}

mod fs {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/world.frag",
	}
}

pub struct Video {
	map: BSPModel,

	debug_callback: DebugCallback,
	descriptor_sets_pool:
		FixedSizeDescriptorSetsPool<Arc<dyn GraphicsPipelineAbstract + Send + Sync>>,
	framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
	pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	queues: Queues,
	sampler: Arc<Sampler>,
	surface: Arc<Surface<Window>>,
	swapchain: Arc<Swapchain<Window>>,
	uniform_buffer: Arc<CpuAccessibleBuffer<vs::ty::UniformBufferObject>>,
}

impl Video {
	pub fn init(event_loop: &EventLoop<()>) -> Result<Video, Box<dyn Error>> {
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

		// Create pipeline
		let vs = vs::Shader::load(device.clone())?;
		let fs = fs::Shader::load(device.clone())?;

		let pipeline = Arc::new(
			GraphicsPipeline::start()
				.render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
				.vertex_input_single_buffer::<VertexData>()
				.vertex_shader(vs.main_entry_point(), ())
				.fragment_shader(fs.main_entry_point(), ())
				.triangle_fan()
				.viewports_dynamic_scissors_irrelevant(1)
				.cull_mode_back()
				.depth_stencil_simple_depth()
				.build(device.clone())?,
		);

		let mut loader = WadLoader::new();
		loader.add("doom.wad")?;
		loader.add("doom.gwa")?;
		let mut map = map::from_wad("E1M1", &mut loader)?;
		map.upload(&queues.graphics)?
			.then_signal_fence_and_flush()?
			.wait(None)?;

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

		// Create descriptor sets pool
		let descriptor_sets_pool = FixedSizeDescriptorSetsPool::new(
			pipeline.clone() as Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
			0,
		);

		// Create uniform buffer
		let uniform_buffer = unsafe {
			CpuAccessibleBuffer::<vs::ty::UniformBufferObject>::uninitialized(
				device.clone(),
				BufferUsage::uniform_buffer(),
			)?
		};

		// All done!
		let video = Video {
			map,

			debug_callback,
			descriptor_sets_pool,
			framebuffers,
			pipeline,
			queues,
			sampler,
			surface,
			swapchain,
			uniform_buffer,
		};

		Ok(video)
	}

	pub fn draw_frame(&mut self) -> Result<(), Box<dyn Error>> {
		// Update uniform buffer
		let model = Matrix4::identity();
		let view = Matrix4::look_at_rh(
			&Point3::new(1670.0, -2500.0, 50.0),
			&Point3::new(1671.0, -2500.0, 50.0),
			&Vector3::new(0.0, 0.0, 1.0),
		);
		let proj = Matrix4::new(
			1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
		) * Matrix4::new_perspective(4.0 / 3.0, FRAC_PI_4, 0.1, 10000.0);

		let data = vs::ty::UniformBufferObject {
			model: model.into(),
			view: view.into(),
			proj: proj.into(),
		};

		*self.uniform_buffer.write()? = data;

		// Prepare for drawing
		let (image_num, future) = match swapchain::acquire_next_image(self.swapchain.clone(), None)
		{
			Ok(r) => r,
			Err(err) => panic!("{:?}", err),
		};

		let framebuffer = &self.framebuffers[image_num];
		let clear_value = vec![[0.0, 0.0, 1.0, 1.0].into(), 1.0.into()];

		let viewport = Viewport {
			origin: [0.0; 2],
			dimensions: [framebuffer.width() as f32, framebuffer.height() as f32],
			depth_range: Range {
				start: 0.0,
				end: 1.0,
			},
		};

		let dynamic_state = DynamicState {
			line_width: None,
			viewports: Some(vec![viewport]),
			scissors: None,
		};

		let mut command_buffer_builder = AutoCommandBufferBuilder::primary_one_time_submit(
			self.swapchain.device().clone(),
			self.queues.graphics.family(),
		)?
		.begin_render_pass(framebuffer.clone(), false, clear_value)?;

		for face in self.map.faces() {
			let texture = face.texture.borrow().image().unwrap();
			let lightmap = face.lightmap.borrow().image().unwrap();

			let descriptor_set = self
				.descriptor_sets_pool
				.next()
				.add_buffer(self.uniform_buffer.clone())?
				.add_sampled_image(texture, self.sampler.clone())?
				.add_sampled_image(lightmap, self.sampler.clone())?
				.build()?;

			let buffer = self.map.buffer().unwrap();
			let slice = BufferSlice::from_typed_buffer_access(buffer.clone());
			let range = Range {
				start: face.first_vertex_index,
				end: face.first_vertex_index + face.vertex_count,
			};
			let slice2 = slice.slice(range).unwrap();

			command_buffer_builder = command_buffer_builder.draw(
				self.pipeline.clone(),
				&dynamic_state,
				vec![Arc::new(slice2)],
				descriptor_set,
				(),
			)?;
		}

		// Finalise
		let command_buffer = Arc::new(command_buffer_builder.end_render_pass()?.build()?);

		future
			.then_execute(self.queues.graphics.clone(), command_buffer)?
			.then_swapchain_present(
				self.queues.present.clone(),
				self.swapchain.clone(),
				image_num,
			)
			.then_signal_fence_and_flush()?
			.wait(None)?;

		Ok(())
	}
}
