//use crate::assets::AssetHandle;
use crate::{
	assets::AssetHandle,
	doom::{
		map::{Map, SectorDynamic},
		sprite::Sprite,
	},
	geometry::Angle,
};
use nalgebra::Vector3;
use specs::{Component, DenseVecStorage, Entity, HashMapStorage};
use specs_derive::Component;
use std::time::Duration;

/*#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct SpriteRenderComponent {
	pub sprite: AssetHandle<Sprite>,
}*/

#[derive(Clone, Component, Copy, Debug, Default)]
#[storage(HashMapStorage)]
pub struct LightFlash {
	pub on_time: Duration,
	pub off_time: Duration,
	pub time_left: Duration,
	pub state: bool,
}

#[derive(Clone, Component, Copy, Debug, Default)]
#[storage(HashMapStorage)]
pub struct LightGlow {
	pub speed: f32,
	pub state: bool,
}

#[derive(Clone, Component, Debug)]
#[storage(HashMapStorage)]
pub struct MapDynamic {
	pub map: AssetHandle<Map>,
	pub sectors: Vec<SectorDynamic>,
}

#[derive(Clone, Component, Debug)]
#[storage(HashMapStorage)]
pub struct SectorRef {
	pub map_entity: Entity,
	pub index: usize,
}

#[derive(Clone, Component, Copy, Debug)]
#[storage(HashMapStorage)]
pub struct SpawnOnCeiling {
	pub offset: f32,
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
