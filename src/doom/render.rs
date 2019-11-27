use crate::{
	assets::AssetStorage,
	doom::{components::{MapComponent, SpawnPointComponent, TransformComponent}, map::VertexData},
	renderer::{texture::Texture, video::Video},
};
use nalgebra::{Matrix4,  Vector3};
use specs::{join::Join, ReadExpect, ReadStorage, RunNow, SystemData, World};
use std::{error::Error, sync::Arc};
use vulkano::{
	buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer},
	command_buffer::{
		pool::standard::StandardCommandPoolBuilder, AutoCommandBufferBuilder, DynamicState,
	},
	descriptor::descriptor_set::FixedSizeDescriptorSetsPool,
	device::DeviceOwned,
	framebuffer::Subpass,
	pipeline::{viewport::Viewport, GraphicsPipeline, GraphicsPipelineAbstract},
	sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
	swapchain,
	sync::GpuFuture,
};

pub struct RenderSystem {
	map: MapRenderSystem,
	sampler: Arc<Sampler>,
}

impl RenderSystem {
	pub fn new(world: &World) -> Result<RenderSystem, Box<dyn Error>> {
		let video = world.fetch::<Video>();

		// Create texture sampler
		let sampler = Sampler::new(
			video.device(),
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

		Ok(RenderSystem {
			map: MapRenderSystem::new(world)?,
			sampler,
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
		command_buffer_builder = self.map.draw(
			world,
			command_buffer_builder,
			dynamic_state,
			self.sampler.clone(),
		)?;

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
		});
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
	matrix_pool: FixedSizeDescriptorSetsPool<Arc<dyn GraphicsPipelineAbstract + Send + Sync>>,
	texture_pool: FixedSizeDescriptorSetsPool<Arc<dyn GraphicsPipelineAbstract + Send + Sync>>,
	pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	uniform_buffer: Arc<CpuAccessibleBuffer<vs::ty::UniformBufferObject>>,
}

impl MapRenderSystem {
	fn new(world: &World) -> Result<MapRenderSystem, Box<dyn Error>> {
		let video = world.fetch::<Video>();
		let device = video.device();
		let render_pass = video.render_pass();

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
				.primitive_restart(true)
				.viewports_dynamic_scissors_irrelevant(1)
				.cull_mode_back()
				.depth_stencil_simple_depth()
				.build(device.clone())?,
		) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

		// Create descriptor sets pool
		let matrix_pool = FixedSizeDescriptorSetsPool::new(pipeline.clone(), 0);
		let texture_pool = FixedSizeDescriptorSetsPool::new(pipeline.clone(), 1);

		// Create uniform buffer
		let uniform_buffer = unsafe {
			CpuAccessibleBuffer::<vs::ty::UniformBufferObject>::uninitialized(
				device.clone(),
				BufferUsage::uniform_buffer(),
			)?
		};

		Ok(MapRenderSystem {
			matrix_pool,
			pipeline,
			texture_pool,
			uniform_buffer,
		})
	}

	fn draw(
		&mut self,
		world: &World,
		mut command_buffer_builder: AutoCommandBufferBuilder<StandardCommandPoolBuilder>,
		dynamic_state: DynamicState,
		sampler: Arc<Sampler>,
	) -> Result<AutoCommandBufferBuilder, Box<dyn Error>> {
		let (transform, spawn_point) = <(ReadStorage<TransformComponent>, ReadStorage<SpawnPointComponent>)>::fetch(world);
		let (mut position, rotation) = (&transform, &spawn_point).join().find_map(|(t, s)| if s.player_num == 1 { Some((t.position, t.rotation)) } else { None }).unwrap();
		position += Vector3::new(0.0, 0.0, 41.0);

		let view =
			Matrix4::new_rotation(Vector3::new(-rotation[0].to_radians(), 0.0, 0.0)) *
			Matrix4::new_rotation(Vector3::new(0.0, -rotation[1].to_radians(), 0.0)) *
			Matrix4::new_rotation(Vector3::new(0.0, 0.0, -rotation[2].to_radians())) *
			Matrix4::new_translation(&-position);

		let proj = projection_matrix(90.0, 4.0/3.0, 0.1, 10000.0);

		let data = vs::ty::UniformBufferObject {
			view: view.into(),
			proj: proj.into(),
		};

		*self.uniform_buffer.write()? = data;

		let matrix_set = Arc::new(
			self.matrix_pool
				.next()
				.add_buffer(self.uniform_buffer.clone())?
				.build()?,
		);

		let (texture_storage, map_component) =
			<(ReadExpect<AssetStorage<Texture>>, ReadStorage<MapComponent>)>::fetch(world);

		// Draw the map
		for component in map_component.join() {
			for (texture, mesh) in component.map_model.meshes() {
				let texture = texture_storage.get(&texture).unwrap();

				let texture_set = Arc::new(
					self.texture_pool
						.next()
						.add_sampled_image(texture.inner(), sampler.clone())?
						.build()?,
				);

				command_buffer_builder = command_buffer_builder.draw_indexed(
					self.pipeline.clone(),
					&dynamic_state,
					vec![Arc::new(mesh.vertex_buffer().into_buffer_slice())],
					mesh.index_buffer().unwrap(),
					(matrix_set.clone(), texture_set.clone()),
					(),
				)?;
			}
		}

		Ok(command_buffer_builder)
	}
}

// A projection matrix that creates a world coordinate system with
// x = forward
// y = left
// z = up
fn projection_matrix(fovx: f32, aspect: f32, near: f32, far: f32) -> Matrix4<f32> {
	let fovx = fovx.to_radians();
	let nmf = near - far;
	let f = 1.0 / (fovx * 0.5).tan();

	Matrix4::new(
		0.0       , -f , 0.0        , 0.0,
		0.0       , 0.0, -f * aspect, 0.0,
		-far / nmf, 0.0, 0.0        , (near * far) / nmf,
		1.0       , 0.0, 0.0        , 0.0,
	)
}
