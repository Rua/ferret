pub mod map;
pub mod sprite;
pub mod world;

use crate::{
	doom::render::world::DrawWorld,
	renderer::{RenderContext, RenderTarget},
};
use anyhow::Context;
use legion::prelude::{Read, ResourceSet, Resources, World};
use std::sync::Arc;
use vulkano::{
	command_buffer::{AutoCommandBufferBuilder, DynamicState},
	descriptor::descriptor_set::DescriptorSet,
	framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract},
	pipeline::viewport::Viewport,
	single_pass_renderpass,
	swapchain::AcquireError,
	sync::GpuFuture,
};

pub struct DrawContext {
	commands: AutoCommandBufferBuilder,
	descriptor_sets: Vec<Arc<dyn DescriptorSet + Send + Sync>>,
	dynamic_state: DynamicState,
}

pub struct RenderSystem {
	framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
	render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
	target: RenderTarget,

	world: DrawWorld,
}

impl RenderSystem {
	pub fn new(render_context: &RenderContext) -> anyhow::Result<RenderSystem> {
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

		Ok(RenderSystem {
			framebuffers,
			render_pass: render_pass.clone(),
			target,

			world: DrawWorld::new(render_context, render_pass)
				.context("Couldn't create DrawWorld")?,
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

	pub fn draw(&mut self, world: &World, resources: &Resources) -> anyhow::Result<()> {
		let render_context = <Read<RenderContext>>::fetch(resources);
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

		let mut draw_context = DrawContext {
			commands: AutoCommandBufferBuilder::primary_one_time_submit(
				self.target.device().clone(),
				queues.graphics.family(),
			)?,
			descriptor_sets: Vec::with_capacity(12),
			dynamic_state: DynamicState {
				viewports: Some(vec![viewport]),
				..DynamicState::none()
			},
		};

		draw_context
			.commands
			.begin_render_pass(framebuffer, false, clear_value)
			.context("Couldn't begin render pass")?;

		self.world.draw(&mut draw_context, world, resources)?;

		draw_context.commands.end_render_pass()?;
		let command_buffer = Arc::new(draw_context.commands.build()?);

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

mod normal_frag {
	vulkano_shaders::shader! {
		ty: "fragment",
		path: "shaders/normal.frag",
	}
}
