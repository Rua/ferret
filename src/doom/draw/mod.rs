pub mod map;
pub mod sprite;
pub mod ui;
pub mod world;
pub mod wsprite;

use crate::{
	common::video::{DrawTarget, PresentTarget, RenderContext},
	doom::{draw::ui::draw_ui, ui::UiParams},
};
use anyhow::Context;
use legion::{
	systems::{ResourceSet, Runnable},
	Read, Resources, SystemBuilder, World, Write,
};
use vulkano::{
	command_buffer::{
		AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBuffer, SubpassContents,
	},
	format::ClearValue,
};

// Doom had non-square pixels, with a resolution of 320x200 (16:10) running on a 4:3
// screen. This caused everything to be stretched vertically by some degree, and the game
// art was made with that in mind.
pub const NON_SQUARE_CORRECTION: f32 = (16.0 / 10.0) / (4.0 / 3.0);

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

pub fn draw(resources: &mut Resources) -> anyhow::Result<impl FnMut(&mut World, &mut Resources)> {
	let mut draw_ui = draw_ui(resources)?;

	Ok(move |world: &mut World, resources: &mut Resources| {
		(|| -> anyhow::Result<()> {
			let mut command_buffer = {
				let (draw_target, render_context) =
					<(Read<DrawTarget>, Read<RenderContext>)>::fetch(resources);
				let graphics_queue = &render_context.queues().graphics;

				let mut command_buffer = AutoCommandBufferBuilder::primary(
					render_context.device().clone(),
					graphics_queue.family(),
					CommandBufferUsage::OneTimeSubmit,
				)
				.context("Couldn't create command buffer builder")?;
				command_buffer
					.begin_render_pass(
						draw_target.framebuffer().clone(),
						SubpassContents::Inline,
						std::array::IntoIter::new([
							ClearValue::Float([0.0, 0.0, 1.0, 1.0]),
							ClearValue::DepthStencil((1.0, 0)),
						]),
					)
					.context("Couldn't begin render pass")?;
				command_buffer
			};

			draw_ui(&mut command_buffer, world, resources)?;

			{
				let (draw_target, render_context, mut present_target) =
					<(Read<DrawTarget>, Read<RenderContext>, Write<PresentTarget>)>::fetch_mut(
						resources,
					);
				let graphics_queue = &render_context.queues().graphics;

				command_buffer
					.end_render_pass()
					.context("Couldn't end render pass")?;
				let future = command_buffer
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
			}

			Ok(())
		})()
		.unwrap_or_else(|e| panic!("{:?}", e));
	})
}
