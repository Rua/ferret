pub mod anim;
pub mod door;
pub mod exit;
pub mod floor;
pub mod plat;
pub mod sector_move;
pub mod switch;

use crate::{
	common::{
		assets::AssetHandle,
		geometry::Interval,
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom},
		time::Timer,
	},
	doom::assets::{
		image::Image,
		map::{textures::TextureType, Map},
	},
};
use fnv::FnvHashMap;
use legion::{systems::ResourceSet, Entity, Read, Resources};
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapDynamic {
	pub anim_states: FnvHashMap<AssetHandle<Image>, AnimState>,
	pub map: AssetHandle<Map>,
	pub linedefs: Vec<LinedefDynamic>,
	pub sectors: Vec<SectorDynamic>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AnimState {
	pub frame: usize,
	pub timer: Timer,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinedefDynamic {
	pub entity: Entity,
	pub sidedefs: [Option<SidedefDynamic>; 2],
	pub texture_offset: Vector2<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SidedefDynamic {
	pub textures: [TextureType; 3],
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LinedefRef {
	pub map_entity: Entity,
	pub index: usize,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct LinedefRefDef;

impl SpawnFrom<LinedefRefDef> for LinedefRef {
	fn spawn(
		_component: &LinedefRefDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		<Read<SpawnContext<LinedefRef>>>::fetch(resources).0
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SectorDynamic {
	pub entity: Entity,
	pub light_level: f32,
	pub interval: Interval,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SectorRef {
	pub map_entity: Entity,
	pub index: usize,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct SectorRefDef;

impl SpawnFrom<SectorRefDef> for SectorRef {
	fn spawn(
		_component: &SectorRefDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		<Read<SpawnContext<SectorRef>>>::fetch(resources).0
	}
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpawnPoint {
	pub player_num: usize,
}
