//use crate::assets::AssetHandle;
use crate::{doom::map::MapModel, geometry::Angle};
use nalgebra::Vector3;
use specs::{Component, HashMapStorage, VecStorage};

/*#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct SpriteRenderComponent {
	pub sprite: AssetHandle<Sprite>,
}*/

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct MapComponent {
	pub map_model: MapModel,
}

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct SpawnPointComponent {
	pub player_num: usize,
}

#[derive(Clone, Component, Copy, Debug)]
#[storage(VecStorage)]
pub struct TransformComponent {
	pub position: Vector3<f32>,
	pub rotation: Vector3<Angle>,
}

impl Default for TransformComponent {
	fn default() -> TransformComponent {
		TransformComponent {
			position: Vector3::new(0.0, 0.0, 0.0),
			rotation: Vector3::new(0.into(), 0.into(), 0.into()),
		}
	}
}
