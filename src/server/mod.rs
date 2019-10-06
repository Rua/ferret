mod server_configvars;

use specs::{
	World,
	WorldExt,
};
use std::{
	error::Error,
	time::{Duration, Instant},
};
use crate::{
	components::{NetworkComponent, TransformComponent},
	doom,
};


pub struct Server {
	real_time: Instant,
	session: ServerSession,
	should_quit: bool,
}

impl Server {
	pub fn new() -> Result<Server, Box<dyn Error>> {
		Ok(Server {
			real_time: Instant::now(),
			session: ServerSession::new("E1M1")?,
			should_quit: false,
		})
	}

	pub fn frame(&mut self, delta: Duration) {
		self.real_time += delta;
	}

	pub fn quit(&mut self) {
		self.should_quit = true;
	}
}

struct ServerSession {
	world: World,
}

impl ServerSession {
	fn new(mapname: &str) -> Result<ServerSession, Box<dyn Error>> {
		let mut world = World::new();
		world.register::<NetworkComponent>();
		world.register::<TransformComponent>();

		let mut loader = doom::wad::WadLoader::new();
		loader.add("doom.wad")?;
		loader.add("doom.gwa")?;
		doom::map::spawn_map_entities(&mut world, mapname, &mut loader)?;

		Ok(ServerSession {
			world,
		})
	}

	fn world(&mut self) -> &mut World {
		&mut self.world
	}
}
