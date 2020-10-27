use crate::doom::sprite::SpriteRender;
use nalgebra::Vector2;

#[derive(Clone, Copy, Debug)]
pub enum WeaponSpriteSlot {
	Weapon = 0,
	Flash = 1,
}

#[derive(Clone, Debug)]
pub struct WeaponSpriteRender {
	pub position: Vector2<f32>,
	pub slots: [Option<SpriteRender>; 2],
}
