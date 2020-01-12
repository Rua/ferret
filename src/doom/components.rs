//use crate::assets::AssetHandle;
use crate::{
	assets::AssetHandle,
	doom::{
		map::{meshes::MapModel, Map},
		sprite::Sprite,
	},
	geometry::Angle,
};
use nalgebra::Vector3;
use specs::{Component, DenseVecStorage, HashMapStorage};
use specs_derive::Component;

/*#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct SpriteRenderComponent {
	pub sprite: AssetHandle<Sprite>,
}*/

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct MapDynamic {
	pub map_model: MapModel,
	pub map: AssetHandle<Map>,
}

#[derive(Clone, Component, Copy, Debug)]
#[storage(HashMapStorage)]
pub struct SpawnPoint {
	pub player_num: usize,
}

#[derive(Clone, Component, Debug)]
pub struct SpriteRender {
	pub sprite: AssetHandle<Sprite>,
	pub frame: usize,
	pub full_bright: bool,
}

#[derive(Clone, Component, Copy, Debug)]
pub struct Transform {
	pub position: Vector3<f32>,
	pub rotation: Vector3<Angle>,
}

impl Default for Transform {
	fn default() -> Transform {
		Transform {
			position: Vector3::new(0.0, 0.0, 0.0),
			rotation: Vector3::new(0.into(), 0.into(), 0.into()),
		}
	}
}
