pub mod map;
pub mod sprite;
pub mod world;

use crate::renderer::{DrawList, RenderContext, RenderTarget};
use legion::prelude::{Read, ResourceSet, Resources, World, Write};

pub fn render_system() -> Box<dyn FnMut(&mut World, &mut Resources)> {
	Box::new(move |world, resources| {
		let (render_context, mut draw_list, mut render_target) =
			<(Read<RenderContext>, Write<DrawList>, Write<RenderTarget>)>::fetch_mut(resources);

		if render_target.needs_recreate() {
			render_target
				.recreate()
				.expect("Couldn't recreate RenderTarget");

			if render_target.dimensions() != draw_list.dimensions() {
				draw_list
					.resize(&render_context, render_target.dimensions())
					.expect("Couldn't resize DrawList");
			}
		}

		let (image, draw_future) = draw_list
			.draw(world, resources)
			.expect("Couldn't execute DrawList");
		render_target
			.present(&render_context.queues().graphics, image, draw_future)
			.expect("Couldn't present swapchain");
	})
}
