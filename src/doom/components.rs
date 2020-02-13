//use crate::assets::AssetHandle;
use crate::{
	assets::AssetHandle,
	audio::{Sink, Sound},
	doom::{map::Map, sprite::Sprite},
	geometry::Angle,
};
use nalgebra::{Vector2, Vector3};
use specs::{Component, DenseVecStorage, Entity, HashMapStorage};
use specs_derive::Component;
use std::time::Duration;

#[derive(Clone, Component, Debug)]
#[storage(HashMapStorage)]
pub struct DoorActive {
	pub open_sound: AssetHandle<Sound>,
	pub open_height: f32,

	pub close_sound: AssetHandle<Sound>,
	pub close_height: f32,

	pub state: DoorState,
	pub speed: f32,
	pub time_left: Duration,
}

#[derive(Clone, Copy, Debug)]
pub enum DoorState {
	Closed,
	Opening,
	Open,
	Closing,
}

#[derive(Clone, Component, Debug)]
#[storage(HashMapStorage)]
pub struct DoorUse {
	pub open_sound: AssetHandle<Sound>,
	pub close_sound: AssetHandle<Sound>,
	pub speed: f32,
	pub wait_time: Duration,
}

#[derive(Clone, Component, Copy, Debug, Default)]
#[storage(HashMapStorage)]
pub struct LightFlash {
	pub on_time: Duration,
	pub off_time: Duration,
	pub time_left: Duration,
	pub state: bool,
	pub flash_type: LightFlashType,
}

#[derive(Clone, Copy, Debug)]
pub enum LightFlashType {
	Broken,
	Strobe,
	StrobeUnSync(Duration),
}

impl Default for LightFlashType {
	fn default() -> LightFlashType {
		LightFlashType::Broken
	}
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
	pub linedefs: Vec<Entity>,
	pub sectors: Vec<Entity>,
}

#[derive(Clone, Component, Copy, Debug)]
#[storage(HashMapStorage)]
pub struct LinedefDynamic {
	pub map_entity: Entity,
	pub index: usize,

	pub texture_offset: Vector2<f32>,
}

#[derive(Clone, Component, Copy, Debug)]
#[storage(HashMapStorage)]
pub struct SectorDynamic {
	pub map_entity: Entity,
	pub index: usize,

	pub light_level: f32,
	pub floor_height: f32,
	pub ceiling_height: f32,
}

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct SoundPlaying {
	pub sink: Sink,
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
pub struct TextureScroll {
	pub speed: Vector2<f32>,
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
