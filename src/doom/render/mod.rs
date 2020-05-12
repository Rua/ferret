pub mod map;
pub mod sprite;

use crate::{
	doom::{
		client::Client,
		components::Transform,
		render::{
			map::{MapRenderSystem, UniformBufferObject},
			sprite::SpriteRenderSystem,
		},
	},
	renderer::{RenderContext, RenderTarget},
};
use anyhow::Context;
use nalgebra::{Matrix4, Vector3};
use specs::{ReadExpect, ReadStorage, RunNow, World};
use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, CpuBufferPool},
	command_buffer::{AutoCommandBufferBuilder, DynamicState},
	descriptor::{
		descriptor::{DescriptorBufferDesc, DescriptorDesc, DescriptorDescTy, ShaderStages},
		descriptor_set::{FixedSizeDescriptorSetsPool, UnsafeDescriptorSetLayout},
	},
	framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract},
	pipeline::viewport::Viewport,
	sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
	single_pass_renderpass,
	swapchain::AcquireError,
	sync::GpuFuture,
};

pub struct RenderSystem {
	framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
	map: MapRenderSystem,
	matrix_uniform_pool: CpuBufferPool<UniformBufferObject>,
	matrix_set_pool: FixedSizeDescriptorSetsPool,
	render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
	sampler: Arc<Sampler>,
	sprites: SpriteRenderSystem,
	target: RenderTarget,
}

impl RenderSystem {
	pub fn new(world: &World) -> anyhow::Result<RenderSystem> {
		let render_context = world.fetch::<RenderContext>();

		// Create texture sampler
		let sampler = Sampler::new(
			render_context.device().clone(),
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
		)
		.context("Couldn't create sampler")?;

		// Create render target
		let size = render_context.surface().window().inner_size().into();
		let target = RenderTarget::new(
			render_context.surface().clone(),
			render_context.device().clone(),
			size,
			true,
		)
		.context("Couldn't create render target")?;

		// Create render pass
		let render_pass = Arc::new(
			single_pass_renderpass!(render_context.device().clone(),
				attachments: {
					color: {
						load: Clear,
						store: Store,
						format: target.image_format(),
						samples: 1,
					},
					depth: {
						load: Clear,
						store: DontCare,
						format: target.depth_format().unwrap(),
						samples: 1,
					}
				},
				pass: {
					color: [color],
					depth_stencil: {depth}
				}
			)
			.context("Couldn't create render pass")?,
		);

		// Create framebuffers
		let images = target.images();
		let mut framebuffers = Vec::with_capacity(images.len());

		for image in images.iter() {
			framebuffers.push(Arc::new(
				Framebuffer::start(render_pass.clone())
					.add(image.clone())?
					.add(target.depth_buffer().unwrap().clone())?
					.build()
					.context("Couldn't create framebuffers")?,
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

		let layout = Arc::new(
			UnsafeDescriptorSetLayout::new(
				render_context.device().clone(),
				descriptors.iter().cloned(),
			)
			.context("Couldn't create descriptor set layout")?,
		);
		let matrix_set_pool = FixedSizeDescriptorSetsPool::new(layout);

		Ok(RenderSystem {
			framebuffers,
			map: MapRenderSystem::new(render_pass.clone())
				.context("Couldn't create MapRenderSystem")?,
			matrix_uniform_pool: CpuBufferPool::new(
				render_context.device().clone(),
				BufferUsage::uniform_buffer(),
			),
			matrix_set_pool,
			render_pass: render_pass.clone(),
			sampler,
			sprites: SpriteRenderSystem::new(render_pass, &*render_context)
				.context("Couldn't create SpriteRenderSystem")?,
			target,
		})
	}

	pub fn recreate(&mut self) -> anyhow::Result<()> {
		let size = self
			.target
			.swapchain()
			.surface()
			.window()
			.inner_size()
			.into();
		self.target = self
			.target
			.recreate(size)
			.context("Couldn't recreate render target")?;

		let images = self.target.images();
		let depth_buffer = self.target.depth_buffer().unwrap();
		let mut framebuffers = Vec::with_capacity(images.len());

		for image in images.iter() {
			framebuffers.push(Arc::new(
				Framebuffer::start(self.render_pass.clone())
					.add(image.clone())?
					.add(depth_buffer.clone())?
					.build()
					.context("Couldn't recreate framebuffers")?,
			) as Arc<dyn FramebufferAbstract + Send + Sync>);
		}

		self.framebuffers = framebuffers;

		Ok(())
	}

	pub fn draw(&mut self, world: &World) -> anyhow::Result<()> {
		let render_context = world.fetch::<RenderContext>();
		let queues = render_context.queues();

		// Prepare for drawing
		let (image_num, future) = match self.target.acquire_next_image() {
			Ok((_, true, _)) => return self.recreate(),
			Ok((image_num, false, future)) => (image_num, future),
			Err(AcquireError::OutOfDate) => return self.recreate(),
			Err(x) => Err(x).context("Couldn't acquire swapchain framebuffer")?,
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
		.begin_render_pass(framebuffer, false, clear_value)
		.context("Couldn't begin render pass")?;

		// Projection matrix
		// Doom had non-square pixels, with a resolution of 320x200 (16:10) running on a 4:3
		// screen. This caused everything to be stretched vertically by some degree, and the game
		// art was made with that in mind.
		// The 1.2 factor here applies the same stretching as in the original.
		let aspect_ratio = (dimensions[0] / dimensions[1]) * 1.2;
		let proj = projection_matrix(90.0, aspect_ratio, 1.0, 20000.0);

		// View matrix
		let (client, transform_storage) =
			world.system_data::<(ReadExpect<Client>, ReadStorage<Transform>)>();

		if let Some(entity) = client.entity {
			let Transform {
				mut position,
				rotation,
			} = *transform_storage.get(entity).unwrap();
			position += Vector3::new(0.0, 0.0, 41.0);

			let view =
				Matrix4::new_rotation(Vector3::new(-rotation[0].to_radians() as f32, 0.0, 0.0))
					* Matrix4::new_rotation(Vector3::new(
						0.0,
						-rotation[1].to_radians() as f32,
						0.0,
					)) * Matrix4::new_rotation(Vector3::new(
					0.0,
					0.0,
					-rotation[2].to_radians() as f32,
				)) * Matrix4::new_translation(&-position);

			// Create UBO
			let data = UniformBufferObject {
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
			command_buffer_builder = self
				.map
				.draw(
					world,
					command_buffer_builder,
					dynamic_state.clone(),
					self.sampler.clone(),
					matrix_set.clone(),
					rotation,
				)
				.context("Draw error")?;

			// Draw sprites
			command_buffer_builder = self
				.sprites
				.draw(
					world,
					command_buffer_builder,
					dynamic_state,
					self.sampler.clone(),
					matrix_set,
					rotation[2],
					position,
				)
				.context("Draw error")?;
		}

		// Finalise
		let command_buffer = Arc::new(command_buffer_builder.end_render_pass()?.build()?);

		future
			.then_execute(queues.graphics.clone(), command_buffer)
			.context("Couldn't execute command buffer")?
			.then_swapchain_present(
				queues.graphics.clone(),
				self.target.swapchain().clone(),
				image_num,
			)
			.then_signal_fence_and_flush()
			.context("Couldn't flush command buffer")?
			.wait(None)
			.context("Couldn't flush command buffer")?;

		Ok(())
	}
}

impl<'a> RunNow<'a> for RenderSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		self.draw(world).unwrap_or_else(|e| {
			panic!("{:?}", e.context("Error while rendering"));
		});
	}
}

mod normal_frag {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/normal.frag",
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
