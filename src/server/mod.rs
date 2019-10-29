mod server_configvars;

use crate::{components::TransformComponent, doom};
use specs::{World, WorldExt};
use std::{
	error::Error,
	time::{Duration, Instant},
};

pub struct Server {
	real_time: Instant,
	world: World,
	should_quit: bool,
}

impl Server {
	pub fn new() -> Result<Server, Box<dyn Error>> {
		let mut world = World::new();
		world.register::<TransformComponent>();

		Ok(Server {
			real_time: Instant::now(),
			world,
			should_quit: false,
		})
	}

	pub fn frame(&mut self, delta: Duration) {
		self.real_time += delta;
	}

	pub fn quit(&mut self) {
		self.should_quit = true;
	}

	pub fn world(&mut self) -> &mut World {
		&mut self.world
	}

	pub fn new_map(&mut self, mapname: &str) -> Result<(), Box<dyn Error>> {
		let mut loader = doom::wad::WadLoader::new();
		loader.add("doom.wad")?;
		loader.add("doom.gwa")?;
		doom::map::spawn_map_entities(&mut self.world, mapname, &mut loader)?;
		Ok(())
	}
}
