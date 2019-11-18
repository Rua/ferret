//use crate::assets::AssetHandle;
use nalgebra::Vector3;
use specs::{Component, VecStorage};

/*#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct SpriteRenderComponent {
	pub sprite: AssetHandle<Sprite>,
}*/

#[derive(Clone, Component, Copy, Debug)]
#[storage(VecStorage)]
pub struct TransformComponent {
	pub position: Vector3<f32>,
	pub rotation: Vector3<f32>,
}

impl Default for TransformComponent {
	fn default() -> TransformComponent {
		TransformComponent {
			position: Vector3::new(0.0, 0.0, 0.0),
			rotation: Vector3::new(0.0, 0.0, 0.0),
		}
	}
}
