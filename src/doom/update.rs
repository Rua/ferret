use crate::doom::map::{LinedefRef, MapDynamic};
use nalgebra::Vector2;
use specs::{
	Component, DenseVecStorage, Join, ReadExpect, ReadStorage, RunNow, World, WriteStorage,
};
use specs_derive::Component;
use std::time::Duration;

#[derive(Default)]
pub struct TextureScrollSystem;

impl<'a> RunNow<'a> for TextureScrollSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		let (delta, linedef_ref_component, texture_scroll_component, mut map_dynamic_component) =
			world.system_data::<(
				ReadExpect<Duration>,
				ReadStorage<LinedefRef>,
				ReadStorage<TextureScroll>,
				WriteStorage<MapDynamic>,
			)>();

		for (linedef_ref, texture_scroll) in
			(&linedef_ref_component, &texture_scroll_component).join()
		{
			let map_dynamic = map_dynamic_component
				.get_mut(linedef_ref.map_entity)
				.unwrap();
			let linedef_dynamic = &mut map_dynamic.linedefs[linedef_ref.index];
			linedef_dynamic.texture_offset += texture_scroll.speed * delta.as_secs_f32();
		}
	}
}

#[derive(Clone, Component, Copy, Debug)]
pub struct TextureScroll {
	pub speed: Vector2<f32>,
}
