use crate::common::video::RenderContext;
use anyhow::Context;
use std::sync::Arc;
use vulkano::{
	command_buffer::AutoCommandBufferBuilder,
	descriptor::descriptor_set::DescriptorSet,
	device::Device,
	format::Format,
	framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract},
	image::{AttachmentImage, ImageAccess, ImageUsage},
	single_pass_renderpass,
};

pub struct DrawTarget {
	colour_attachment: Arc<AttachmentImage>,
	depth_attachment: Arc<AttachmentImage>,
	framebuffer: Arc<dyn FramebufferAbstract + Send + Sync>,
	render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
}

impl DrawTarget {
	pub fn new(render_context: &RenderContext, dimensions: [u32; 2]) -> anyhow::Result<DrawTarget> {
		log::debug!("Creating DrawTarget");

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

		Ok(DrawTarget {
			colour_attachment,
			depth_attachment,
			framebuffer,
			render_pass,
		})
	}

	pub fn resize(
		&mut self,
		render_context: &RenderContext,
		dimensions: [u32; 2],
	) -> anyhow::Result<()> {
		log::debug!("Resizing DrawTarget");

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

	pub fn framebuffer(&self) -> &Arc<dyn FramebufferAbstract + Send + Sync> {
		&self.framebuffer
	}

	pub fn colour_attachment(&self) -> &Arc<AttachmentImage> {
		&self.colour_attachment
	}

	pub fn render_pass(&self) -> &Arc<dyn RenderPassAbstract + Send + Sync> {
		&self.render_pass
	}
}

pub struct DrawContext {
	pub commands: AutoCommandBufferBuilder,
	pub descriptor_sets: Vec<Arc<dyn DescriptorSet + Send + Sync>>,
}
