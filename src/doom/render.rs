use crate::{
	doom::{map, wad::WadLoader},
	renderer::{
		model::{BSPModel, VertexData},
		video::Video,
	},
};
use nalgebra::{Matrix4, Point3, Vector3};
use specs::{RunNow, World};
use std::{error::Error, f32::consts::FRAC_PI_4, sync::Arc};
use vulkano::{
	buffer::{BufferSlice, BufferUsage, CpuAccessibleBuffer},
	command_buffer::{
		pool::standard::StandardCommandPoolBuilder, AutoCommandBufferBuilder, DynamicState,
	},
	descriptor::descriptor_set::FixedSizeDescriptorSetsPool,
	device::DeviceOwned,
	framebuffer::Subpass,
	pipeline::{viewport::Viewport, GraphicsPipeline, GraphicsPipelineAbstract},
	swapchain,
	sync::GpuFuture,
};

pub struct RenderSystem {
	map: MapRenderSystem,
}

impl RenderSystem {
	pub fn new(world: &World) -> Result<RenderSystem, Box<dyn Error>> {
		Ok(RenderSystem {
			map: MapRenderSystem::new(world)?,
		})
	}

	pub fn draw(&mut self, world: &World) -> Result<(), Box<dyn Error>> {
		let video = world.fetch::<Video>();

		let swapchain = video.swapchain();
		let queues = video.queues();

		// Prepare for drawing
		let (image_num, future) = match swapchain::acquire_next_image(swapchain.clone(), None) {
			Ok(r) => r,
			Err(err) => panic!("{:?}", err),
		};

		let framebuffer = video.framebuffer(image_num);
		let clear_value = vec![[0.0, 0.0, 1.0, 1.0].into(), 1.0.into()];

		let viewport = Viewport {
			origin: [0.0; 2],
			dimensions: [framebuffer.width() as f32, framebuffer.height() as f32],
			depth_range: 0.0..1.0,
		};

		let dynamic_state = DynamicState {
			line_width: None,
			viewports: Some(vec![viewport]),
			scissors: None,
		};

		let mut command_buffer_builder = AutoCommandBufferBuilder::primary_one_time_submit(
			swapchain.device().clone(),
			queues.graphics.family(),
		)?
		.begin_render_pass(framebuffer.clone(), false, clear_value)?;

        // Draw the map
		command_buffer_builder = self
			.map
			.draw(world, command_buffer_builder, dynamic_state)?;

		// Finalise
		let command_buffer = Arc::new(command_buffer_builder.end_render_pass()?.build()?);

		future
			.then_execute(queues.graphics.clone(), command_buffer)?
			.then_swapchain_present(queues.present.clone(), swapchain.clone(), image_num)
			.then_signal_fence_and_flush()?
			.wait(None)?;

		Ok(())
	}
}

impl<'a> RunNow<'a> for RenderSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		self.draw(world).unwrap_or_else(|e| {
			panic!("Error while rendering: {}", e);
		})
	}
}

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

pub struct MapRenderSystem {
	descriptor_sets_pool:
		FixedSizeDescriptorSetsPool<Arc<dyn GraphicsPipelineAbstract + Send + Sync>>,
	map: BSPModel,
	pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	uniform_buffer: Arc<CpuAccessibleBuffer<vs::ty::UniformBufferObject>>,
}

impl MapRenderSystem {
	fn new(world: &World) -> Result<MapRenderSystem, Box<dyn Error>> {
		let video = world.fetch::<Video>();
		let device = video.device();
		let render_pass = video.render_pass();
		let queues = video.queues();

		// Create pipeline
		let vs = vs::Shader::load(device.clone())?;
		let fs = fs::Shader::load(device.clone())?;

		let pipeline = Arc::new(
			GraphicsPipeline::start()
				.render_pass(
					Subpass::from(render_pass.clone(), 0).ok_or("Subpass index out of range")?,
				)
				.vertex_input_single_buffer::<VertexData>()
				.vertex_shader(vs.main_entry_point(), ())
				.fragment_shader(fs.main_entry_point(), ())
				.triangle_fan()
				.viewports_dynamic_scissors_irrelevant(1)
				.cull_mode_back()
				.depth_stencil_simple_depth()
				.build(device.clone())?,
		) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

		let mut loader = WadLoader::new();
		loader.add("doom.wad")?;
		loader.add("doom.gwa")?;
		let mut map = map::from_wad("E1M1", &mut loader)?;
		map.upload(&queues.graphics)?
			.then_signal_fence_and_flush()?
			.wait(None)?;

		// Create descriptor sets pool
		let descriptor_sets_pool = FixedSizeDescriptorSetsPool::new(pipeline.clone(), 0);

		// Create uniform buffer
		let uniform_buffer = unsafe {
			CpuAccessibleBuffer::<vs::ty::UniformBufferObject>::uninitialized(
				device.clone(),
				BufferUsage::uniform_buffer(),
			)?
		};

		Ok(MapRenderSystem {
			descriptor_sets_pool,
			map,
			pipeline,
			uniform_buffer,
		})
	}

	fn draw(
		&mut self,
		world: &World,
		mut command_buffer_builder: AutoCommandBufferBuilder<StandardCommandPoolBuilder>,
		dynamic_state: DynamicState,
	) -> Result<AutoCommandBufferBuilder, Box<dyn Error>> {
		let video = world.fetch::<Video>();
		let sampler = video.sampler();

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

		// Draw the map
		for face in self.map.faces() {
			let texture = face.texture.borrow();
			let texture = texture.texture().unwrap();
			let lightmap = face.lightmap.borrow();
			let lightmap = lightmap.texture().unwrap();

			let descriptor_set = self
				.descriptor_sets_pool
				.next()
				.add_buffer(self.uniform_buffer.clone())?
				.add_sampled_image(texture.inner(), sampler.clone())?
				.add_sampled_image(lightmap.inner(), sampler.clone())?
				.build()?;

			let mesh = self.map.mesh().unwrap();
			let slice = BufferSlice::from_typed_buffer_access(mesh.inner());
			let range = face.first_vertex_index * std::mem::size_of::<VertexData>()
				..(face.first_vertex_index + face.vertex_count) * std::mem::size_of::<VertexData>();
			let slice2 = slice.slice(range).unwrap();

			command_buffer_builder = command_buffer_builder.draw(
				self.pipeline.clone(),
				&dynamic_state,
				vec![Arc::new(slice2)],
				descriptor_set,
				(),
			)?;
		}

		Ok(command_buffer_builder)
	}
}
