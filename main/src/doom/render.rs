use crate::{
	assets::AssetStorage,
	doom::{
		components::{MapDynamic, SpriteRender, Transform},
		sprite::Sprite,
	},
	geometry::Angle,
	renderer::{
		texture::Texture,
		video::{RenderTarget, Video},
		vulkan,
	},
};
use nalgebra::{Matrix4, Vector3};
use specs::{Entity, Join, ReadExpect, ReadStorage, RunNow, World};
use std::{error::Error, sync::Arc};
use vulkano::{
	buffer::{BufferAccess, CpuBufferPool},
	command_buffer::{
		pool::standard::StandardCommandPoolBuilder, AutoCommandBufferBuilder, DynamicState,
	},
	descriptor::{
		descriptor::{DescriptorBufferDesc, DescriptorDesc, DescriptorDescTy, ShaderStages},
		descriptor_set::{DescriptorSet, FixedSizeDescriptorSetsPool, UnsafeDescriptorSetLayout},
		PipelineLayoutAbstract,
	},
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
	matrix_buffer_pool: CpuBufferPool<map_normal_vert::ty::UniformBufferObject>,
	matrix_set_pool: FixedSizeDescriptorSetsPool,
	render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
	sampler: Arc<Sampler>,
	sprites: SpriteRenderSystem,
	target: RenderTarget,
}

impl RenderSystem {
	pub fn new(world: &World) -> Result<RenderSystem, Box<dyn Error + Send + Sync>> {
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

		// Create uniform buffer and descriptor sets pool for matrices
		let matrix_buffer_pool =
			CpuBufferPool::<map_normal_vert::ty::UniformBufferObject>::uniform_buffer(
				video.device().clone(),
			);

		let descriptors = [Some(DescriptorDesc {
			ty: DescriptorDescTy::Buffer(DescriptorBufferDesc {
				dynamic: Some(false),
				storage: false,
			}),
			array_count: 1,
			stages: ShaderStages {
				vertex: true,
				..ShaderStages::none()
			},
			readonly: true,
		})];

		let layout = Arc::new(UnsafeDescriptorSetLayout::new(
			video.device().clone(),
			descriptors.iter().cloned(),
		)?);
		let matrix_set_pool = FixedSizeDescriptorSetsPool::new(layout);

		Ok(RenderSystem {
			framebuffers,
			map: MapRenderSystem::new(render_pass.clone())?,
			matrix_buffer_pool,
			matrix_set_pool,
			render_pass: render_pass.clone(),
			sampler,
			sprites: SpriteRenderSystem::new(render_pass.clone())?,
			target,
		})
	}

	pub fn recreate(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
		let (width, height) = self
			.target
			.swapchain()
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

	pub fn draw(&mut self, world: &World) -> Result<(), Box<dyn Error + Send + Sync>> {
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

		// Projection matrix
		// Doom had non-square pixels, with a resolution of 320x200 (16:10) running on a 4:3
		// screen. This caused everything to be stretched vertically by some degree, and the game
		// art was made with that in mind.
		// The 1.2 factor here applies the same stretching as in the original.
		let aspect_ratio = (dimensions[0] / dimensions[1]) * 1.2;
		let proj = projection_matrix(90.0, aspect_ratio, 0.1, 10000.0);

		// View matrix
		let (entity, transform_storage) =
			world.system_data::<(ReadExpect<Entity>, ReadStorage<Transform>)>();
		let Transform {
			mut position,
			rotation,
		} = *transform_storage.get(*entity).unwrap();
		position += Vector3::new(0.0, 0.0, 41.0);

		let view = Matrix4::new_rotation(Vector3::new(-rotation[0].to_radians() as f32, 0.0, 0.0))
			* Matrix4::new_rotation(Vector3::new(0.0, -rotation[1].to_radians() as f32, 0.0))
			* Matrix4::new_rotation(Vector3::new(0.0, 0.0, -rotation[2].to_radians() as f32))
			* Matrix4::new_translation(&-position);

		// Create UBO
		let data = map_normal_vert::ty::UniformBufferObject {
			view: view.into(),
			proj: proj.into(),
		};

		let matrix_buffer = self.matrix_buffer_pool.next(data)?;
		let matrix_set = Arc::new(
			self.matrix_set_pool
				.next()
				.add_buffer(matrix_buffer)?
				.build()?,
		);

		// Draw the map
		command_buffer_builder = self.map.draw(
			world,
			command_buffer_builder,
			dynamic_state.clone(),
			self.sampler.clone(),
			matrix_set.clone(),
			rotation,
		)?;

		// Draw sprites
		command_buffer_builder = self.sprites.draw(
			world,
			command_buffer_builder,
			dynamic_state,
			self.sampler.clone(),
			matrix_set,
			rotation[2],
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

mod map_normal_vert {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/map_normal.vert",
	}
}

mod map_normal_frag {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/map_normal.frag",
	}
}

mod map_sky_vert {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/map_sky.vert",
	}
}

mod map_sky_frag {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/map_sky.frag",
	}
}

pub struct MapRenderSystem {
	normal_pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	normal_texture_pool: FixedSizeDescriptorSetsPool,
	sky_buffer_pool: CpuBufferPool<map_sky_frag::ty::FragParams>,
	sky_pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	sky_texture_pool: FixedSizeDescriptorSetsPool,
}

impl MapRenderSystem {
	fn new(
		render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
	) -> Result<MapRenderSystem, Box<dyn Error + Send + Sync>> {
		let device = render_pass.device();

		// Create pipeline for normal parts of the map
		let normal_vert = map_normal_vert::Shader::load(device.clone())?;
		let normal_frag = map_normal_frag::Shader::load(device.clone())?;

		let normal_pipeline = Arc::new(
			GraphicsPipeline::start()
				.render_pass(
					Subpass::from(render_pass.clone(), 0).ok_or("Subpass index out of range")?,
				)
				.vertex_input_single_buffer::<super::map::meshes::VertexData>()
				.vertex_shader(normal_vert.main_entry_point(), ())
				.fragment_shader(normal_frag.main_entry_point(), ())
				.triangle_fan()
				.primitive_restart(true)
				.viewports_dynamic_scissors_irrelevant(1)
				.cull_mode_back()
				.depth_stencil_simple_depth()
				.build(device.clone())?,
		) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

		let layout = normal_pipeline.descriptor_set_layout(1).unwrap();
		let normal_texture_pool = FixedSizeDescriptorSetsPool::new(layout.clone());

		// Create pipeline for sky
		let sky_vert = map_sky_vert::Shader::load(device.clone())?;
		let sky_frag = map_sky_frag::Shader::load(device.clone())?;

		let sky_pipeline = Arc::new(
			GraphicsPipeline::start()
				.render_pass(
					Subpass::from(render_pass.clone(), 0).ok_or("Subpass index out of range")?,
				)
				.vertex_input_single_buffer::<super::map::meshes::SkyVertexData>()
				.vertex_shader(sky_vert.main_entry_point(), ())
				.fragment_shader(sky_frag.main_entry_point(), ())
				.triangle_fan()
				.primitive_restart(true)
				.viewports_dynamic_scissors_irrelevant(1)
				.cull_mode_back()
				.depth_stencil_simple_depth()
				.build(device.clone())?,
		) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

		let layout = sky_pipeline.descriptor_set_layout(1).unwrap();
		let sky_texture_pool = FixedSizeDescriptorSetsPool::new(layout.clone());
		let sky_buffer_pool =
			CpuBufferPool::<map_sky_frag::ty::FragParams>::uniform_buffer(device.clone());

		Ok(MapRenderSystem {
			normal_pipeline,
			normal_texture_pool,
			sky_buffer_pool,
			sky_pipeline,
			sky_texture_pool,
		})
	}

	fn draw(
		&mut self,
		world: &World,
		mut command_buffer_builder: AutoCommandBufferBuilder<StandardCommandPoolBuilder>,
		dynamic_state: DynamicState,
		sampler: Arc<Sampler>,
		matrix_set: Arc<dyn DescriptorSet + Send + Sync>,
		rotation: Vector3<Angle>,
	) -> Result<AutoCommandBufferBuilder, Box<dyn Error + Send + Sync>> {
		let (texture_storage, map_component) =
			world.system_data::<(ReadExpect<AssetStorage<Texture>>, ReadStorage<MapDynamic>)>();

		for component in map_component.join() {
			// Draw the normal parts of the map
			for (texture, mesh) in component.map_model.meshes() {
				let texture = texture_storage.get(&texture).unwrap();

				let texture_set = Arc::new(
					self.normal_texture_pool
						.next()
						.add_sampled_image(texture.inner(), sampler.clone())?
						.build()?,
				);

				command_buffer_builder = command_buffer_builder.draw_indexed(
					self.normal_pipeline.clone(),
					&dynamic_state,
					vec![Arc::new(mesh.vertex_buffer().into_buffer_slice())],
					mesh.index_buffer().unwrap(),
					(matrix_set.clone(), texture_set.clone()),
					(),
				)?;
			}

			// Draw the sky
			let (texture, mesh) = component.map_model.sky_mesh();
			let texture = texture_storage.get(&texture).unwrap();
			let sky_buffer = self.sky_buffer_pool.next(map_sky_frag::ty::FragParams {
				screenSize: [800.0, 600.0],
				pitch: rotation[1].to_degrees() as f32,
				yaw: rotation[2].to_degrees() as f32,
			})?;

			let texture_params_set = Arc::new(
				self.sky_texture_pool
					.next()
					.add_sampled_image(texture.inner(), sampler.clone())?
					.add_buffer(sky_buffer)?
					.build()?,
			);

			command_buffer_builder = command_buffer_builder.draw_indexed(
				self.sky_pipeline.clone(),
				&dynamic_state,
				vec![Arc::new(mesh.vertex_buffer().into_buffer_slice())],
				mesh.index_buffer().unwrap(),
				(matrix_set.clone(), texture_params_set.clone()),
				(),
			)?;
		}

		Ok(command_buffer_builder)
	}
}

mod sprite_vert {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/sprite.vert",
	}
}

mod sprite_frag {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/sprite.frag",
	}
}

pub struct SpriteRenderSystem {
	instance_buffer_pool: CpuBufferPool<sprite_vert::ty::Instance>,
	instance_set_pool: FixedSizeDescriptorSetsPool,
	pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	texture_pool: FixedSizeDescriptorSetsPool,
}

impl SpriteRenderSystem {
	fn new(
		render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
	) -> Result<SpriteRenderSystem, Box<dyn Error + Send + Sync>> {
		let device = render_pass.device();

		// Create pipeline for normal parts of the map
		let vert = sprite_vert::Shader::load(device.clone())?;
		let frag = sprite_frag::Shader::load(device.clone())?;

		let pipeline = Arc::new(
			GraphicsPipeline::start()
				.render_pass(
					Subpass::from(render_pass.clone(), 0).ok_or("Subpass index out of range")?,
				)
				.vertex_input_single_buffer::<super::sprite::VertexData>()
				.vertex_shader(vert.main_entry_point(), ())
				.fragment_shader(frag.main_entry_point(), ())
				.triangle_fan()
				.primitive_restart(true)
				.viewports_dynamic_scissors_irrelevant(1)
				.cull_mode_disabled()
				.depth_stencil_simple_depth()
				.build(device.clone())?,
		) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

		let layout = pipeline.descriptor_set_layout(1).unwrap();
		let texture_pool = FixedSizeDescriptorSetsPool::new(layout.clone());

		let layout = pipeline.descriptor_set_layout(2).unwrap();
		let instance_set_pool = FixedSizeDescriptorSetsPool::new(layout.clone());
		let instance_buffer_pool =
			CpuBufferPool::<sprite_vert::ty::Instance>::uniform_buffer(device.clone());

		Ok(SpriteRenderSystem {
			instance_buffer_pool,
			instance_set_pool,
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
		matrix_set: Arc<dyn DescriptorSet + Send + Sync>,
		yaw: Angle,
	) -> Result<AutoCommandBufferBuilder, Box<dyn Error + Send + Sync>> {
		/*let billboard_matrix = Matrix4::new(
			-camera_axes[1][0], 0.0, camera_axes[2][0], 0.0,
			-camera_axes[1][1], 0.0, camera_axes[2][1], 0.0,
			-camera_axes[1][2], 0.0, camera_axes[2][2], 0.0,
			0.0               , 0.0, 0.0, 1.0,
		);*/

		let (sprite_storage, sprite_component, transform_component) =
			world.system_data::<(ReadExpect<AssetStorage<Sprite>>, ReadStorage<SpriteRender>, ReadStorage<Transform>)>();

		for (sprite_render, transform) in (&sprite_component, &transform_component).join() {
			let sprite = sprite_storage.get(&sprite_render.sprite).unwrap();
			let frame = &sprite.frames()[sprite_render.frame];
			let mesh = &sprite.meshes()[frame[0].mesh_index];
			let texture = &sprite.textures()[frame[0].texture_index];

			let texture_set = Arc::new(
				self.texture_pool
					.next()
					.add_sampled_image(texture.inner(), sampler.clone())?
					.build()?,
			);

			let instance_matrix = Matrix4::new_translation(&transform.position)
				* Matrix4::new_rotation(Vector3::new(0.0, 0.0, yaw.to_radians() as f32));
			let instance_buffer = self.instance_buffer_pool.next(sprite_vert::ty::Instance {
				matrix: instance_matrix.into(),
			})?;

			let instance_set = Arc::new(
				self.instance_set_pool
					.next()
					.add_buffer(instance_buffer)?
					.build()?,
			);

			command_buffer_builder = command_buffer_builder.draw(
				self.pipeline.clone(),
				&dynamic_state,
				vec![Arc::new(mesh.vertex_buffer().into_buffer_slice())],
				(matrix_set.clone(), texture_set.clone(), instance_set),
				(),
			)?;
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
