use crate::{
	assets::{AssetHandle, AssetStorage},
	doom::{
		components::{MapDynamic, SpriteRender, Transform},
		map::{
			textures::{Flat, WallTexture},
			Map,
		},
		sprite::{Sprite, SpriteImage},
	},
	geometry::Angle,
	renderer::{
		video::{RenderTarget, Video},
		vulkan, AsBytes,
	},
};
use nalgebra::{Matrix4, Vector2, Vector3};
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, RunNow, World};
use std::{
	collections::{hash_map::Entry, HashMap},
	error::Error,
	sync::Arc,
};
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool, ImmutableBuffer},
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
	impl_vertex,
	pipeline::{
		vertex::OneVertexOneInstanceDefinition, viewport::Viewport, GraphicsPipeline,
		GraphicsPipelineAbstract,
	},
	sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
	single_pass_renderpass,
	swapchain::AcquireError,
	sync::GpuFuture,
};

pub struct RenderSystem {
	framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
	map: MapRenderSystem,
	matrix_uniform_pool: CpuBufferPool<map_normal_vert::ty::UniformBufferObject>,
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
		let target = RenderTarget::new(video.surface().clone(), video.device().clone(), size)?;

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

		// Create descriptor sets pool for matrices
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
			matrix_uniform_pool: CpuBufferPool::new(
				video.device().clone(),
				BufferUsage::uniform_buffer(),
			),
			matrix_set_pool,
			render_pass: render_pass.clone(),
			sampler,
			sprites: SpriteRenderSystem::new(render_pass.clone(), &*video)?,
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
		let (view_entity, transform_storage) =
			world.system_data::<(ReadExpect<Entity>, ReadStorage<Transform>)>();
		let Transform {
			mut position,
			rotation,
		} = *transform_storage.get(*view_entity).unwrap();
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

		let matrix_buffer = self.matrix_uniform_pool.next(data)?;
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
			position,
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

mod normal_frag {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/normal.frag",
	}
}

mod map_sky_vert {
	vulkano_shaders::shader! {
		ty: "vertex",
		path: "shaders/map_sky.vert",
	}
}

mod sky_frag {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/sky.frag",
	}
}

pub struct MapRenderSystem {
	index_buffer_pool: CpuBufferPool<u32>,
	normal_pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	normal_texture_set_pool: FixedSizeDescriptorSetsPool,
	sky_pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	sky_texture_set_pool: FixedSizeDescriptorSetsPool,
	sky_uniform_pool: CpuBufferPool<sky_frag::ty::FragParams>,
	vertex_buffer_pool: CpuBufferPool<u8>,
}

impl MapRenderSystem {
	fn new(
		render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
	) -> Result<MapRenderSystem, Box<dyn Error + Send + Sync>> {
		let device = render_pass.device();

		// Create pipeline for normal parts of the map
		let normal_vert = map_normal_vert::Shader::load(device.clone())?;
		let normal_frag = normal_frag::Shader::load(device.clone())?;

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

		// Create pipeline for sky
		let sky_vert = map_sky_vert::Shader::load(device.clone())?;
		let sky_frag = sky_frag::Shader::load(device.clone())?;

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

		Ok(MapRenderSystem {
			index_buffer_pool: CpuBufferPool::new(device.clone(), BufferUsage::index_buffer()),
			vertex_buffer_pool: CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer()),

			normal_texture_set_pool: FixedSizeDescriptorSetsPool::new(
				normal_pipeline.descriptor_set_layout(1).unwrap().clone(),
			),
			normal_pipeline,

			sky_uniform_pool: CpuBufferPool::new(device.clone(), BufferUsage::uniform_buffer()),
			sky_texture_set_pool: FixedSizeDescriptorSetsPool::new(
				sky_pipeline.descriptor_set_layout(1).unwrap().clone(),
			),
			sky_pipeline,
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
		let (flat_storage, map_storage, wall_texture_storage, map_component) = world
			.system_data::<(
				ReadExpect<AssetStorage<Flat>>,
				ReadExpect<AssetStorage<Map>>,
				ReadExpect<AssetStorage<WallTexture>>,
				ReadStorage<MapDynamic>,
			)>();

		for map_dynamic in map_component.join() {
			let map = map_storage.get(&map_dynamic.map).unwrap();
			let (flat_meshes, sky_mesh, wall_meshes) =
				crate::doom::map::meshes::make_meshes(map, map_dynamic, world)?;

			// Draw the walls
			for (handle, mesh) in wall_meshes {
				let vertex_buffer = self
					.vertex_buffer_pool
					.chunk(mesh.0.as_bytes().iter().copied())?;
				let index_buffer = self.index_buffer_pool.chunk(mesh.1)?;
				let texture = wall_texture_storage.get(&handle).unwrap();

				let texture_set = Arc::new(
					self.normal_texture_set_pool
						.next()
						.add_sampled_image(texture.inner(), sampler.clone())?
						.build()?,
				);

				command_buffer_builder = command_buffer_builder.draw_indexed(
					self.normal_pipeline.clone(),
					&dynamic_state,
					vec![Arc::new(vertex_buffer)],
					index_buffer,
					(matrix_set.clone(), texture_set.clone()),
					(),
				)?;
			}

			// Draw the flats
			for (handle, mesh) in flat_meshes {
				let vertex_buffer = self
					.vertex_buffer_pool
					.chunk(mesh.0.as_bytes().iter().copied())?;
				let index_buffer = self.index_buffer_pool.chunk(mesh.1)?;
				let texture = flat_storage.get(&handle).unwrap();

				let texture_set = Arc::new(
					self.normal_texture_set_pool
						.next()
						.add_sampled_image(texture.inner(), sampler.clone())?
						.build()?,
				);

				command_buffer_builder = command_buffer_builder.draw_indexed(
					self.normal_pipeline.clone(),
					&dynamic_state,
					vec![Arc::new(vertex_buffer)],
					index_buffer,
					(matrix_set.clone(), texture_set.clone()),
					(),
				)?;
			}

			// Draw the sky
			let vertex_buffer = self
				.vertex_buffer_pool
				.chunk(sky_mesh.0.as_bytes().iter().copied())?;
			let index_buffer = self.index_buffer_pool.chunk(sky_mesh.1)?;
			let texture = wall_texture_storage.get(&map.sky).unwrap();
			let sky_buffer = self.sky_uniform_pool.next(sky_frag::ty::FragParams {
				screenSize: [800.0, 600.0],
				pitch: rotation[1].to_degrees() as f32,
				yaw: rotation[2].to_degrees() as f32,
			})?;

			let texture_params_set = Arc::new(
				self.sky_texture_set_pool
					.next()
					.add_sampled_image(texture.inner(), sampler.clone())?
					.add_buffer(sky_buffer)?
					.build()?,
			);

			command_buffer_builder = command_buffer_builder.draw_indexed(
				self.sky_pipeline.clone(),
				&dynamic_state,
				vec![Arc::new(vertex_buffer)],
				index_buffer,
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

#[derive(Clone, Debug, Default)]
pub struct VertexData {
	pub in_position: [f32; 3],
	pub in_texture_coord: [f32; 2],
}
impl_vertex!(VertexData, in_position, in_texture_coord);

#[derive(Clone, Debug, Default)]
pub struct InstanceData {
	pub in_flip: f32,
	pub in_light_level: f32,
	pub in_matrix: [[f32; 4]; 4],
}
impl_vertex!(InstanceData, in_flip, in_light_level, in_matrix);

pub struct SpriteRenderSystem {
	instance_buffer_pool: CpuBufferPool<InstanceData>,
	vertex_buffer: Arc<ImmutableBuffer<[u8]>>,
	pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
	texture_set_pool: FixedSizeDescriptorSetsPool,
}

impl SpriteRenderSystem {
	fn new(
		render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
		video: &Video,
	) -> Result<SpriteRenderSystem, Box<dyn Error + Send + Sync>> {
		let device = render_pass.device();

		// Create pipeline
		let vert = sprite_vert::Shader::load(device.clone())?;
		let frag = normal_frag::Shader::load(device.clone())?;

		let pipeline = Arc::new(
			GraphicsPipeline::start()
				.render_pass(
					Subpass::from(render_pass.clone(), 0).ok_or("Subpass index out of range")?,
				)
				.vertex_input(OneVertexOneInstanceDefinition::<VertexData, InstanceData>::new())
				.vertex_shader(vert.main_entry_point(), ())
				.fragment_shader(frag.main_entry_point(), ())
				.triangle_fan()
				.primitive_restart(true)
				.viewports_dynamic_scissors_irrelevant(1)
				.cull_mode_disabled()
				.depth_stencil_simple_depth()
				.build(device.clone())?,
		) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

		// Create mesh
		let (vertex_buffer, future) = ImmutableBuffer::from_iter(
			vec![
				VertexData {
					in_position: [0.0, -1.0, 0.0],
					in_texture_coord: [1.0, 0.0],
				},
				VertexData {
					in_position: [0.0, 0.0, 0.0],
					in_texture_coord: [0.0, 0.0],
				},
				VertexData {
					in_position: [0.0, 0.0, -1.0],
					in_texture_coord: [0.0, 1.0],
				},
				VertexData {
					in_position: [0.0, -1.0, -1.0],
					in_texture_coord: [1.0, 1.0],
				},
			]
			.as_bytes()
			.iter()
			.copied(),
			BufferUsage::vertex_buffer(),
			video.queues().graphics.clone(),
		)?;

		Ok(SpriteRenderSystem {
			vertex_buffer,

			instance_buffer_pool: CpuBufferPool::new(device.clone(), BufferUsage::vertex_buffer()),
			texture_set_pool: FixedSizeDescriptorSetsPool::new(
				pipeline.descriptor_set_layout(1).unwrap().clone(),
			),
			pipeline,
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
		view_pos: Vector3<f32>,
	) -> Result<AutoCommandBufferBuilder, Box<dyn Error + Send + Sync>> {
		let (
			entities,
			view_entity,
			map_storage,
			sprite_storage,
			sprite_image_storage,
			map_component,
			sprite_component,
			transform_component,
		) = world.system_data::<(
			Entities,
			ReadExpect<Entity>,
			ReadExpect<AssetStorage<Map>>,
			ReadExpect<AssetStorage<Sprite>>,
			ReadExpect<AssetStorage<SpriteImage>>,
			ReadStorage<MapDynamic>,
			ReadStorage<SpriteRender>,
			ReadStorage<Transform>,
		)>();

		let map_handle = &map_component.join().next().unwrap().map;
		let map = map_storage.get(map_handle).unwrap();

		// Group draws into batches by texture
		let mut batches: HashMap<AssetHandle<SpriteImage>, Vec<InstanceData>> = HashMap::new();

		for (entity, sprite_render, transform) in
			(&entities, &sprite_component, &transform_component).join()
		{
			// Don't render the player's own sprite
			if entity == *view_entity {
				continue;
			}

			let sprite = sprite_storage.get(&sprite_render.sprite).unwrap();
			let frame = &sprite.frames()[sprite_render.frame];

			// This frame has no images, nothing to render
			if frame.len() == 0 {
				continue;
			}

			// Figure out which rotation image to use
			// Treat non-rotating frames specially for efficiency
			let index = if frame.len() == 1 {
				0
			} else {
				let to_view_vec = view_pos - transform.position;
				let to_view_angle =
					Angle::from_radians(f64::atan2(to_view_vec[1] as f64, to_view_vec[0] as f64));
				let delta = to_view_angle - transform.rotation[2]
					+ Angle::from_units(0.5 / frame.len() as f64);
				(delta.to_units_unsigned() * frame.len() as f64) as usize % frame.len()
			};

			let image_info = &frame[index];
			let sprite_image = sprite_image_storage.get(&image_info.handle).unwrap();

			// Determine light level
			let light_level = if sprite_render.full_bright {
				1.0
			} else {
				let ssect =
					map.find_subsector(Vector2::new(transform.position[0], transform.position[1]));
				let sector = &map.sectors[ssect.sector_index];
				sector.light_level
			};

			// Set up instance data
			let instance_matrix = Matrix4::new_translation(&transform.position)
				* Matrix4::new_rotation(Vector3::new(0.0, 0.0, yaw.to_radians() as f32))
				* sprite_image.matrix;
			let instance_data = InstanceData {
				in_flip: image_info.flip,
				in_light_level: light_level,
				in_matrix: instance_matrix.into(),
			};

			// Add to batches
			match batches.entry(image_info.handle.clone()) {
				Entry::Occupied(mut entry) => {
					entry.get_mut().push(instance_data);
				}
				Entry::Vacant(entry) => {
					entry.insert(vec![instance_data]);
				}
			}
		}

		// Draw the batches
		for (sprite_image_handle, instance_data) in batches {
			let sprite_image = sprite_image_storage.get(&sprite_image_handle).unwrap();
			let texture_set = Arc::new(
				self.texture_set_pool
					.next()
					.add_sampled_image(sprite_image.texture.inner(), sampler.clone())?
					.build()?,
			);

			let instance_buffer = self.instance_buffer_pool.chunk(instance_data)?;

			command_buffer_builder = command_buffer_builder.draw(
				self.pipeline.clone(),
				&dynamic_state,
				vec![self.vertex_buffer.clone(), Arc::new(instance_buffer)],
				(matrix_set.clone(), texture_set.clone()),
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
#[rustfmt::skip]
fn projection_matrix(fovx: f32, aspect: f32, near: f32, far: f32) -> Matrix4<f32> {
	let fovx = fovx.to_radians();
	let nmf = near - far;
	let f = 1.0 / (fovx * 0.5).tan();

	Matrix4::new(
		0.0       , -f , 0.0        , 0.0               ,
		0.0       , 0.0, -f * aspect, 0.0               ,
		-far / nmf, 0.0, 0.0        , (near * far) / nmf,
		1.0       , 0.0, 0.0        , 0.0               ,
	)
}
