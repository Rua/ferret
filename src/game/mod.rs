mod entity;

use self::entity::Entity;

pub struct Game {
	entities: Vec<Entity>,
}

impl Game {
	pub fn new(mapname: &str) -> Game {
		Game {
			entities: Vec::new(),
		}
	}
}
