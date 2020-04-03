use crate::doom::{
	client::{PlayerCommandSystem, PlayerMoveSystem, PlayerUseSystem},
	components::TextureScroll,
	door::DoorUpdateSystem,
	light::LightUpdateSystem,
	map::{LinedefRef, MapDynamic},
	physics::PhysicsSystem,
};
use specs::{Join, ReadExpect, ReadStorage, RunNow, World, WriteStorage};
use std::time::Duration;

#[derive(Default)]
pub struct UpdateSystem {
	player_command: PlayerCommandSystem,
	player_move: PlayerMoveSystem,
	player_use: PlayerUseSystem,

	physics: PhysicsSystem,

	door_update: DoorUpdateSystem,
	light_update: LightUpdateSystem,
	texture_scroll: TextureScrollSystem,
}

impl<'a> RunNow<'a> for UpdateSystem {
	fn setup(&mut self, _world: &mut World) {}

	fn run_now(&mut self, world: &'a World) {
		self.player_command.run_now(world);
		self.player_move.run_now(world);
		self.player_use.run_now(world);

		self.physics.run_now(world);

		self.door_update.run_now(world);
		self.light_update.run_now(world);
		self.texture_scroll.run_now(world);
	}
}

#[derive(Default)]
struct TextureScrollSystem;

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
