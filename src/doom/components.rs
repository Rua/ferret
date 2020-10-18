use crate::common::geometry::Angle;
use legion::{
	storage::{Archetype, ComponentWriter, Components},
	Resources,
};
use legion_prefab::SpawnFrom;
use legion_transaction::iter_component_slice_from_archetype;
use nalgebra::Vector3;
use std::ops::Range;

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

impl SpawnFrom<Self> for Transform {
	fn spawn_from(
		_resources: &Resources,
		src_entity_range: Range<usize>,
		src_arch: &Archetype,
		src_components: &Components,
		dst: &mut ComponentWriter<Self>,
		push_fn: fn(&mut ComponentWriter<Self>, Self),
	) {
		println!("Foo");
		for component in
			iter_component_slice_from_archetype::<Self>(src_components, src_arch, src_entity_range)
				.flatten()
		{
			push_fn(dst, component.clone());
		}
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Velocity {
	pub velocity: Vector3<f32>,
}
