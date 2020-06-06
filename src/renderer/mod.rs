mod context;
mod target;

use anyhow::Context;
use legion::prelude::{Read, ResourceSet, Resources, World};
use std::sync::Arc;
use vulkano::{
	command_buffer::{AutoCommandBufferBuilder, CommandBuffer, DynamicState},
	descriptor::descriptor_set::DescriptorSet,
	device::Device,
	format::Format,
	framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract},
	image::{AttachmentImage, ImageAccess, ImageUsage},
	pipeline::viewport::Viewport,
	single_pass_renderpass,
	sync::GpuFuture,
};

pub use {context::RenderContext, target::RenderTarget};

pub trait AsBytes {
	fn as_bytes(&self) -> &[u8];
}

impl<T> AsBytes for Vec<T> {
	fn as_bytes(&self) -> &[u8] {
		let slice = self.as_slice();
		unsafe {
			std::slice::from_raw_parts(slice.as_ptr() as _, std::mem::size_of::<T>() * slice.len())
		}
	}
}

pub struct DrawList {
	steps: Vec<Box<dyn DrawStep>>,

	colour_attachment: Arc<AttachmentImage>,
	depth_attachment: Arc<AttachmentImage>,
	framebuffer: Arc<dyn FramebufferAbstract + Send + Sync>,
	render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
}

impl DrawList {
	pub fn new(render_context: &RenderContext, dimensions: [u32; 2]) -> anyhow::Result<DrawList> {
		log::debug!("Creating DrawList");

		// Choose attachment formats
		let colour_format = [Format::R8G8B8A8Unorm]
			.iter()
			.cloned()
			.find(|format| {
				let physical_device = render_context.device().physical_device();
				let features = format.properties(physical_device).optimal_tiling_features;
				features.color_attachment && features.blit_src
			})
			.context("No supported colour buffer format found")?;

		let depth_format = [
			Format::D32Sfloat,
			Format::X8_D24UnormPack32,
			Format::D16Unorm,
		]
		.iter()
		.cloned()
		.find(|format| {
			let physical_device = render_context.device().physical_device();
			let features = format.properties(physical_device).optimal_tiling_features;
			features.depth_stencil_attachment
		})
		.context("No supported depth buffer format found")?;

		// Create render pass
		let render_pass: Arc<dyn RenderPassAbstract + Send + Sync> = Arc::new(
			single_pass_renderpass!(render_context.device().clone(),
				attachments: {
					color: {
						load: Clear,
						store: Store,
						format: colour_format,
						samples: 1,
					},
					depth: {
						load: Clear,
						store: DontCare,
						format: depth_format,
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

		let (colour_attachment, depth_attachment) = Self::create_attachments(
			&render_context.device(),
			dimensions,
			colour_format,
			depth_format,
		)?;

		// Create framebuffer
		let framebuffer = Arc::new(
			Framebuffer::start(render_pass.clone())
				.add(colour_attachment.clone())?
				.add(depth_attachment.clone())?
				.build()
				.context("Couldn't create framebuffers")?,
		);

		Ok(DrawList {
			steps: Vec::new(),
			colour_attachment,
			depth_attachment,
			framebuffer,
			render_pass,
		})
	}

	pub fn add_step(&mut self, step: impl DrawStep + 'static) {
		self.steps.push(Box::from(step));
	}

	pub fn resize(
		&mut self,
		render_context: &RenderContext,
		dimensions: [u32; 2],
	) -> anyhow::Result<()> {
		log::debug!("Resizing DrawList");

		// Create attachments
		let (colour_attachment, depth_attachment) = Self::create_attachments(
			&render_context.device(),
			dimensions,
			self.colour_attachment.format(),
			self.depth_attachment.format(),
		)?;
		self.colour_attachment = colour_attachment;
		self.depth_attachment = depth_attachment;

		// Create framebuffer
		self.framebuffer = Arc::new(
			Framebuffer::start(self.render_pass.clone())
				.add(self.colour_attachment.clone())?
				.add(self.depth_attachment.clone())?
				.build()
				.context("Couldn't create framebuffers")?,
		);

		Ok(())
	}

	fn create_attachments(
		device: &Arc<Device>,
		dimensions: [u32; 2],
		colour_format: Format,
		depth_format: Format,
	) -> anyhow::Result<(Arc<AttachmentImage>, Arc<AttachmentImage>)> {
		// Create colour attachment
		let colour_attachment = AttachmentImage::with_usage(
			device.clone(),
			dimensions,
			colour_format,
			ImageUsage {
				color_attachment: true,
				transfer_source: true,
				..ImageUsage::none()
			},
		)
		.context("Couldn't create colour attachment")?;

		// Create depth attachment
		let depth_attachment = AttachmentImage::with_usage(
			device.clone(),
			dimensions,
			depth_format,
			ImageUsage {
				depth_stencil_attachment: true,
				transient_attachment: true,
				..ImageUsage::none()
			},
		)
		.context("Couldn't create depth attachment")?;

		Ok((colour_attachment, depth_attachment))
	}

	pub fn dimensions(&self) -> [u32; 2] {
		self.colour_attachment.dimensions().width_height()
	}

	pub fn render_pass(&self) -> &Arc<dyn RenderPassAbstract + Send + Sync> {
		&self.render_pass
	}

	pub fn draw(
		&mut self,
		world: &World,
		resources: &Resources,
	) -> anyhow::Result<(Arc<AttachmentImage>, impl GpuFuture)> {
		let render_context = <Read<RenderContext>>::fetch(resources);
		let graphics_queue = &render_context.queues().graphics;

		let clear_value = vec![[0.0, 0.0, 1.0, 1.0].into(), 1.0.into()];
		let dimensions = [
			self.framebuffer.width() as f32,
			self.framebuffer.height() as f32,
		];

		let viewport = Viewport {
			origin: [0.0; 2],
			dimensions,
			depth_range: 0.0..1.0,
		};

		let mut draw_context = DrawContext {
			commands: AutoCommandBufferBuilder::primary_one_time_submit(
				render_context.device().clone(),
				graphics_queue.family(),
			)?,
			descriptor_sets: Vec::with_capacity(12),
			dynamic_state: DynamicState {
				viewports: Some(vec![viewport]),
				..DynamicState::none()
			},
		};

		draw_context
			.commands
			.begin_render_pass(self.framebuffer.clone(), false, clear_value)
			.context("Couldn't begin render pass")?;
		self.steps
			.iter_mut()
			.try_for_each(|step| step.draw(&mut draw_context, world, resources))?;
		draw_context
			.commands
			.end_render_pass()
			.context("Couldn't end render pass")?;
		let future = draw_context
			.commands
			.build()?
			.execute(graphics_queue.clone())
			.context("Couldn't execute draw commands")?;

		Ok((self.colour_attachment.clone(), future))
	}
}

pub trait DrawStep: Send + Sync {
	fn draw(
		&mut self,
		draw_context: &mut DrawContext,
		world: &World,
		resources: &Resources,
	) -> anyhow::Result<()>;
}

pub struct DrawContext {
	pub commands: AutoCommandBufferBuilder,
	pub descriptor_sets: Vec<Arc<dyn DescriptorSet + Send + Sync>>,
	pub dynamic_state: DynamicState,
}
