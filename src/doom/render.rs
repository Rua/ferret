use crate::{
	assets::AssetStorage,
	doom::{
		components::{MapComponent, TransformComponent},
		map::VertexData,
	},
	renderer::{
		texture::Texture,
		video::{RenderTarget, Video},
		vulkan,
	},
};
use nalgebra::{Matrix4, Vector3};
use specs::{Entity, Join, ReadExpect, ReadStorage, RunNow, SystemData, World};
use std::{error::Error, sync::Arc};
use vulkano::{
	buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer},
	command_buffer::{
		pool::standard::StandardCommandPoolBuilder, AutoCommandBufferBuilder, DynamicState,
	},
	descriptor::descriptor_set::FixedSizeDescriptorSetsPool,
	device::DeviceOwned,
	framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass},
	image::ImageViewAccess,
	pipeline::{viewport::Viewport, GraphicsPipeline, GraphicsPipelineAbstract},
	sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
	swapchain::AcquireError,
	sync::GpuFuture,
};

pub struct RenderSystem {
	framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
	map: MapRenderSystem,
	matrix_buffer: Arc<CpuAccessibleBuffer<vs::ty::UniformBufferObject>>,
	render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
	sampler: Arc<Sampler>,
	target: RenderTarget,
}

impl RenderSystem {
	pub fn new(world: &World) -> Result<RenderSystem, Box<dyn Error>> {
		let video = world.fetch::<Video>();

		// Create texture sampler
		let sampler = Sampler::new(
			video.device().clone(),
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

		// Create render target
		let (width, height) = video.surface().window().get_inner_size().unwrap().into();
		let size = [width, height];
		let target = RenderTarget::new(
			video.surface().clone(),
			video.device().clone(),
			video.queues().graphics.family().id(),
			size,
		)?;

		// Create depth buffer
		let depth_buffer = vulkan::create_depth_buffer(&video.device(), size)?;

		// Create render pass
		let render_pass = Arc::new(single_pass_renderpass!(video.device().clone(),
			attachments: {
				color: {
					load: Clear,
					store: Store,
					format: target.format(),
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
		let images = target.images();
		let mut framebuffers = Vec::with_capacity(images.len());

		for image in images.iter() {
			framebuffers.push(Arc::new(
				Framebuffer::start(render_pass.clone())
					.add(image.clone())?
					.add(depth_buffer.clone())?
					.build()?,
			) as Arc<dyn FramebufferAbstract + Send + Sync>);
		}

		// Create uniform buffer for matrices
		let matrix_buffer = unsafe {
			CpuAccessibleBuffer::<vs::ty::UniformBufferObject>::uninitialized(
				video.device().clone(),
				BufferUsage::uniform_buffer(),
			)?
		};

		Ok(RenderSystem {
			framebuffers,
			map: MapRenderSystem::new(render_pass.clone())?,
			matrix_buffer,
			render_pass,
			sampler,
			target,
		})
	}

	pub fn recreate(&mut self) -> Result<(), Box<dyn Error>> {
		let (width, height) = self
			.target
			.surface()
			.window()
			.get_inner_size()
			.unwrap()
			.into();
		let size = [width, height];
		self.target = self.target.recreate(size)?;
		let depth_buffer = vulkan::create_depth_buffer(self.target.device(), size)?;

		let images = self.target.images();
		let mut framebuffers = Vec::with_capacity(images.len());

		for image in images.iter() {
			framebuffers.push(Arc::new(
				Framebuffer::start(self.render_pass.clone())
					.add(image.clone())?
					.add(depth_buffer.clone())?
					.build()?,
			) as Arc<dyn FramebufferAbstract + Send + Sync>);
		}

		self.framebuffers = framebuffers;

		Ok(())
	}

	pub fn draw(&mut self, world: &World) -> Result<(), Box<dyn Error>> {
		let video = world.fetch::<Video>();
		let queues = video.queues();

		// Prepare for drawing
		let (image_num, future) = match self.target.acquire_next_image() {
			Ok(x) => x,
			Err(AcquireError::OutOfDate) => {
				self.recreate()?;
				return Ok(());
			}
			Err(x) => Err(x)?,
		};

		let framebuffer = self.framebuffers[image_num].clone();
		let clear_value = vec![[0.0, 0.0, 1.0, 1.0].into(), 1.0.into()];
		let dimensions = [framebuffer.width() as f32, framebuffer.height() as f32];

		let viewport = Viewport {
			origin: [0.0; 2],
			dimensions,
			depth_range: 0.0..1.0,
		};

		let dynamic_state = DynamicState {
			viewports: Some(vec![viewport]),
			..DynamicState::none()
		};

		let mut command_buffer_builder = AutoCommandBufferBuilder::primary_one_time_submit(
			self.target.device().clone(),
			queues.graphics.family(),
		)?
		.begin_render_pass(framebuffer, false, clear_value)?;

		// Set up matrices
		let (entity, transform_storage) =
			<(ReadExpect<Entity>, ReadStorage<TransformComponent>)>::fetch(world);
		let TransformComponent {
			mut position,
			rotation,
		} = *transform_storage.get(*entity).unwrap();
		position += Vector3::new(0.0, 0.0, 41.0);

		let view = Matrix4::new_rotation(Vector3::new(-rotation[0].to_radians() as f32, 0.0, 0.0))
			* Matrix4::new_rotation(Vector3::new(0.0, -rotation[1].to_radians() as f32, 0.0))
			* Matrix4::new_rotation(Vector3::new(0.0, 0.0, -rotation[2].to_radians() as f32))
			* Matrix4::new_translation(&-position);

		// Doom had non-square pixels, with a resolution of 320x200 (16:10) running on a 4:3
		// screen. This caused everything to be stretched vertically by some degree, and the game
		// art was made with that in mind.
		// The 1.2 factor here applies the same stretching as in the original.
		let aspect_ratio = (dimensions[0] / dimensions[1]) * 1.2;
		let proj = projection_matrix(90.0, aspect_ratio, 0.1, 10000.0);

		let data = vs::ty::UniformBufferObject {
			view: view.into(),
			proj: proj.into(),
		};

		*self.matrix_buffer.write()? = data;

		// Draw the map
		command_buffer_builder = self.map.draw(
			world,
			command_buffer_builder,
			dynamic_state,
			self.sampler.clone(),
			self.matrix_buffer.clone(),
		)?;

		// Finalise
		let command_buffer = Arc::new(command_buffer_builder.end_render_pass()?.build()?);

		future
			.then_execute(queues.graphics.clone(), command_buffer)?
			.then_swapchain_present(
				queues.graphics.clone(),
				self.target.swapchain().clone(),
				image_num,
			)
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
}

impl MapRenderSystem {
	fn new(
		render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
	) -> Result<MapRenderSystem, Box<dyn Error>> {
		let device = render_pass.device();

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

		Ok(MapRenderSystem {
			matrix_pool,
			pipeline,
			texture_pool,
		})
	}

	fn draw(
		&mut self,
		world: &World,
		mut command_buffer_builder: AutoCommandBufferBuilder<StandardCommandPoolBuilder>,
		dynamic_state: DynamicState,
		sampler: Arc<Sampler>,
		matrix_buffer: Arc<CpuAccessibleBuffer<vs::ty::UniformBufferObject>>,
	) -> Result<AutoCommandBufferBuilder, Box<dyn Error>> {
		let (texture_storage, map_component) =
			<(ReadExpect<AssetStorage<Texture>>, ReadStorage<MapComponent>)>::fetch(world);

		let matrix_set = Arc::new(self.matrix_pool.next().add_buffer(matrix_buffer)?.build()?);

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
		0.0,
		-f,
		0.0,
		0.0,
		0.0,
		0.0,
		-f * aspect,
		0.0,
		-far / nmf,
		0.0,
		0.0,
		(near * far) / nmf,
		1.0,
		0.0,
		0.0,
		0.0,
	)
}
