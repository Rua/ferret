use crate::common::video::RenderContext;
use anyhow::Context;
use std::sync::Arc;
use vulkano::{
	buffer::{BufferUsage, CpuAccessibleBuffer},
	command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBuffer},
	device::Device,
	format::Format,
	image::{
		view::{ImageView, ImageViewAbstract},
		AttachmentImage, ImageUsage,
	},
	render_pass::{Framebuffer, FramebufferAbstract, RenderPass},
	single_pass_renderpass,
	sync::GpuFuture,
	DeviceSize,
};

pub struct DrawTarget {
	colour_attachment: Arc<ImageView<Arc<AttachmentImage>>>,
	depth_stencil_attachment: Arc<ImageView<Arc<AttachmentImage>>>,
	framebuffer: Arc<dyn FramebufferAbstract + Send + Sync>,
}

impl DrawTarget {
	pub fn new(render_context: &RenderContext, dimensions: [u32; 2]) -> anyhow::Result<DrawTarget> {
		log::debug!("Creating DrawTarget");

		// Choose attachment formats
		let colour_format = [Format::R8G8B8A8_UNORM]
			.into_iter()
			.find(|format| {
				let physical_device = render_context.device().physical_device();
				let features = format.properties(physical_device).optimal_tiling_features;
				features.color_attachment && features.blit_src
			})
			.context("No supported colour buffer format found")?;

		let depth_stencil_format = [
			Format::D32_SFLOAT_S8_UINT,
			Format::D24_UNORM_S8_UINT,
			Format::D16_UNORM_S8_UINT,
		]
		.into_iter()
		.find(|format| {
			let physical_device = render_context.device().physical_device();
			let features = format.properties(physical_device).optimal_tiling_features;
			features.depth_stencil_attachment
		})
		.context("No supported depth-stencil buffer format found")?;

		// Create render pass
		let render_pass = Arc::new(
			single_pass_renderpass!(render_context.device().clone(),
				attachments: {
					color: {
						load: Clear,
						store: Store,
						format: colour_format,
						samples: 1,
					},
					depth_stencil: {
						load: Clear,
						store: DontCare,
						format: depth_stencil_format,
						samples: 1,
					}
				},
				pass: {
					color: [color],
					depth_stencil: {depth_stencil}
				}
			)
			.context("Couldn't create render pass")?,
		);

		let (colour_attachment, depth_stencil_attachment) = Self::create_attachments(
			&render_context.device(),
			dimensions,
			colour_format,
			depth_stencil_format,
		)?;

		// Create framebuffer
		let framebuffer = Arc::new(
			Framebuffer::start(render_pass)
				.add(colour_attachment.clone())?
				.add(depth_stencil_attachment.clone())?
				.build()
				.context("Couldn't create framebuffers")?,
		);

		Ok(DrawTarget {
			colour_attachment,
			depth_stencil_attachment,
			framebuffer,
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
			self.depth_stencil_attachment.format(),
		)?;
		self.colour_attachment = colour_attachment;
		self.depth_stencil_attachment = depth_attachment;

		// Create framebuffer
		self.framebuffer = Arc::new(
			Framebuffer::start(self.framebuffer.render_pass().clone())
				.add(self.colour_attachment.clone())?
				.add(self.depth_stencil_attachment.clone())?
				.build()
				.context("Couldn't create framebuffers")?,
		);

		Ok(())
	}

	fn create_attachments(
		device: &Arc<Device>,
		dimensions: [u32; 2],
		colour_format: Format,
		depth_stencil_format: Format,
	) -> anyhow::Result<(
		Arc<ImageView<Arc<AttachmentImage>>>,
		Arc<ImageView<Arc<AttachmentImage>>>,
	)> {
		// Create colour attachment
		let colour_attachment = ImageView::new(
			AttachmentImage::with_usage(
				device.clone(),
				dimensions,
				colour_format,
				ImageUsage {
					color_attachment: true,
					transfer_source: true,
					..ImageUsage::none()
				},
			)
			.context("Couldn't create colour attachment")?,
		)
		.context("Couldn't create colour attachment")?;

		// Create depth-stencil attachment
		let depth_stencil_attachment = ImageView::new(
			AttachmentImage::with_usage(
				device.clone(),
				dimensions,
				depth_stencil_format,
				ImageUsage {
					depth_stencil_attachment: true,
					transient_attachment: true,
					..ImageUsage::none()
				},
			)
			.context("Couldn't create depth-stencil attachment")?,
		)
		.context("Couldn't create depth-stencil attachment")?;

		Ok((colour_attachment, depth_stencil_attachment))
	}

	pub fn dimensions(&self) -> [u32; 2] {
		self.colour_attachment.image().dimensions().width_height()
	}

	pub fn framebuffer(&self) -> &Arc<dyn FramebufferAbstract + Send + Sync> {
		&self.framebuffer
	}

	pub fn colour_attachment(&self) -> &Arc<ImageView<Arc<AttachmentImage>>> {
		&self.colour_attachment
	}

	pub fn render_pass(&self) -> &Arc<RenderPass> {
		&self.framebuffer.render_pass()
	}

	pub fn copy_to_cpu(
		&self,
		render_context: &RenderContext,
	) -> anyhow::Result<(Arc<CpuAccessibleBuffer<[u8]>>, [u32; 2], impl GpuFuture)> {
		let graphics_queue = &render_context.queues().graphics;
		let format = self.colour_attachment.format();
		let dimensions = self.colour_attachment.image().dimensions();

		unsafe {
			// TODO: Would be nice to have a CpuAccessibleImage in Vulkano?
			let buffer = CpuAccessibleBuffer::<[u8]>::uninitialized_array(
				render_context.device().clone(),
				format.size().unwrap() * dimensions.num_texels() as DeviceSize,
				BufferUsage::transfer_destination(),
				true,
			)?;
			let mut builder = AutoCommandBufferBuilder::primary(
				render_context.device().clone(),
				graphics_queue.family(),
				CommandBufferUsage::OneTimeSubmit,
			)?;
			builder.copy_image_to_buffer(
				ImageView::image(&self.colour_attachment).clone(),
				buffer.clone(),
			)?;
			let future = builder.build()?.execute(graphics_queue.clone())?;
			Ok((buffer, dimensions.width_height(), future))
		}
	}
}
