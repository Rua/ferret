pub mod map;
pub mod sprite;
pub mod ui;
pub mod world;
pub mod wsprite;

use crate::{
	common::video::{DrawContext, DrawTarget, PresentTarget, RenderContext},
	doom::ui::UiParams,
};
use anyhow::Context;
use legion::{systems::Runnable, Resources, SystemBuilder};
use vulkano::{
	command_buffer::{
		AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBuffer, SubpassContents,
	},
	format::ClearValue,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct FramebufferResizeEvent;

pub fn check_recreate() -> impl Runnable {
	SystemBuilder::new("check_recreate")
		.read_resource::<RenderContext>()
		.write_resource::<DrawTarget>()
		.write_resource::<PresentTarget>()
		.write_resource::<UiParams>()
		.build(move |command_buffer, _world, resources, _queries| {
			let (render_context, draw_target, present_target, ui_params) = resources;

			if present_target.needs_recreate() {
				present_target
					.recreate()
					.expect("Couldn't recreate PresentTarget");

				if present_target.dimensions() != draw_target.dimensions() {
					draw_target
						.resize(&render_context, present_target.dimensions())
						.expect("Couldn't resize DrawTarget");

					**ui_params = UiParams::new(present_target.dimensions());
					command_buffer.push((FramebufferResizeEvent,));
				}
			}
		})
}

pub fn start_draw(resources: &mut Resources) -> anyhow::Result<impl Runnable> {
	resources.insert::<Option<DrawContext>>(None);

	Ok(SystemBuilder::new("start_draw")
		.read_resource::<RenderContext>()
		.write_resource::<Option<DrawContext>>()
		.write_resource::<DrawTarget>()
		.build(move |_command_buffer, _world, resources, _queries| {
			(|| -> anyhow::Result<()> {
				let (render_context, draw_context, draw_target) = resources;
				let graphics_queue = &render_context.queues().graphics;
				let draw_context: &mut Option<DrawContext> = &mut *draw_context;
				*draw_context = Some(DrawContext {
					commands: AutoCommandBufferBuilder::primary(
						render_context.device().clone(),
						graphics_queue.family(),
						CommandBufferUsage::OneTimeSubmit,
					)
					.context("Couldn't create command buffer builder")?,
					descriptor_sets: Vec::with_capacity(12),
				});

				draw_context
					.as_mut()
					.unwrap()
					.commands
					.begin_render_pass(
						draw_target.framebuffer().clone(),
						SubpassContents::Inline,
						std::array::IntoIter::new([
							ClearValue::Float([0.0, 0.0, 1.0, 1.0]),
							ClearValue::DepthStencil((1.0, 0)),
						]),
					)
					.context("Couldn't begin render pass")?;

				Ok(())
			})()
			.unwrap_or_else(|e| panic!("{:?}", e));
		}))
}

pub fn finish_draw(_resources: &mut Resources) -> anyhow::Result<impl Runnable> {
	Ok(SystemBuilder::new("finish_draw")
		.read_resource::<DrawTarget>()
		.read_resource::<RenderContext>()
		.write_resource::<Option<DrawContext>>()
		.write_resource::<PresentTarget>()
		.build(move |_command_buffer, _world, resources, _queries| {
			(|| -> anyhow::Result<()> {
				let (draw_target, render_context, draw_context, present_target) = resources;
				let graphics_queue = &render_context.queues().graphics;

				let mut draw_context = draw_context.take().unwrap();

				draw_context
					.commands
					.end_render_pass()
					.context("Couldn't end render pass")?;
				let future = draw_context
					.commands
					.build()
					.context("Couldn't build command buffer")?
					.execute(graphics_queue.clone())
					.context("Couldn't execute command buffer")?;

				present_target
					.present(
						&render_context.queues().graphics,
						draw_target.colour_attachment().image().clone(),
						future,
					)
					.context("Couldn't present swapchain")?;
				Ok(())
			})()
			.unwrap_or_else(|e| panic!("{:?}", e));
		}))
}
