use crate::doom::sprite::SpriteRender;
use nalgebra::Vector2;

#[derive(Clone, Debug)]
pub struct PlayerSpriteRender {
	pub position: Vector2<f32>,
	pub slots: [Option<SpriteRender>; 2],
}

pub enum PlayerSpriteSlot {
	Weapon = 0,
	Flash = 1,
}
