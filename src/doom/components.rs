//use crate::assets::AssetHandle;
use crate::{assets::AssetHandle, doom::map::{Map, meshes::MapModel}, geometry::Angle};
use nalgebra::Vector3;
use specs::{Component, HashMapStorage, VecStorage};

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

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct SpawnPoint {
	pub player_num: usize,
}

#[derive(Clone, Component, Copy, Debug)]
#[storage(VecStorage)]
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
