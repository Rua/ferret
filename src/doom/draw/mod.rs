pub mod map;
pub mod sprite;
pub mod ui;
pub mod world;
pub mod wsprite;

use crate::{
	common::video::{DrawContext, DrawTarget, PresentTarget, RenderContext},
	doom::{
		draw::{
			map::draw_map, sprite::draw_sprites, ui::draw_ui, world::draw_world,
			wsprite::draw_weapon_sprites,
		},
		ui::UiParams,
	},
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
	resources.insert::<Option<DrawContext>>(None);

	let mut draw_world = draw_world(resources)?;
	let mut draw_map = draw_map(resources)?;
	let mut draw_sprites = draw_sprites(resources)?;
	let mut draw_weapon_sprites = draw_weapon_sprites(resources)?;
	let mut draw_ui = draw_ui(resources)?;

	Ok(move |world: &mut World, resources: &mut Resources| {
		let mut draw_context = (|| -> anyhow::Result<DrawContext> {
			let (draw_target, render_context) =
				<(Read<DrawTarget>, Read<RenderContext>)>::fetch(resources);
			let graphics_queue = &render_context.queues().graphics;
			let mut draw_context = DrawContext {
				commands: AutoCommandBufferBuilder::primary(
					render_context.device().clone(),
					graphics_queue.family(),
					CommandBufferUsage::OneTimeSubmit,
				)
				.context("Couldn't create command buffer builder")?,
				descriptor_sets: Vec::with_capacity(12),
			};

			draw_context
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

			Ok(draw_context)
		})()
		.unwrap_or_else(|e| panic!("{:?}", e));

		draw_world(&mut draw_context, world, resources);
		draw_map(&mut draw_context, world, resources);
		draw_sprites(&mut draw_context, world, resources);
		draw_weapon_sprites(&mut draw_context, world, resources);
		draw_ui(&mut draw_context, world, resources);

		(|| -> anyhow::Result<()> {
			let (draw_target, render_context, mut present_target) =
				<(Read<DrawTarget>, Read<RenderContext>, Write<PresentTarget>)>::fetch_mut(
					resources,
				);
			let graphics_queue = &render_context.queues().graphics;

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
	})
}
