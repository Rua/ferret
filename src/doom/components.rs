use crate::{
	common::{
		geometry::Angle,
		spawn::{ComponentAccessor, SpawnFrom},
	},
	doom::map::spawn::SpawnContext,
};
use legion::{systems::ResourceSet, Read, Resources};
use nalgebra::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct SpawnOnCeiling {
	pub offset: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct SpawnPoint {
	pub player_num: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Transform {
	pub position: Vector3<f32>,
	pub rotation: Vector3<Angle>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransformDef;

impl SpawnFrom<TransformDef> for Transform {
	fn from_with_resources(
		_component: &TransformDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		let spawn_context = <Read<SpawnContext>>::fetch(resources);
		spawn_context.transform
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Velocity {
	pub velocity: Vector3<f32>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct VelocityDef;

impl From<VelocityDef> for Velocity {
	fn from(_src_component: VelocityDef) -> Self {
		Velocity::default()
	}
}
