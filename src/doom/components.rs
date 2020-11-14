use crate::{
	common::{
		frame::FrameState,
		geometry::{Angle, Interval},
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom},
	},
	doom::physics::{BoxCollider, DISTANCE_EPSILON},
};
use legion::{systems::ResourceSet, Read, Resources};
use nalgebra::Vector3;
use rand::{distributions::Uniform, Rng};

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
pub struct TransformDef {
	pub spawn_on_ceiling: bool,
}

impl SpawnFrom<TransformDef> for Transform {
	fn spawn(component: &TransformDef, accessor: ComponentAccessor, resources: &Resources) -> Self {
		let transform = <Read<SpawnContext<Transform>>>::fetch(resources);
		let mut transform = transform.0;

		if transform.position[2].is_nan() {
			let sector_interval = <Read<SpawnContext<Interval>>>::fetch(resources);

			if component.spawn_on_ceiling {
				transform.position[2] = sector_interval.0.max - DISTANCE_EPSILON;

				if let Some(box_collider) = accessor.get::<BoxCollider>() {
					transform.position[2] -= box_collider.height;
				}
			} else {
				transform.position[2] = sector_interval.0.min + DISTANCE_EPSILON;
			}
		}

		transform
	}
}

#[derive(Clone, Copy, Debug)]
pub struct RandomTransformDef(pub [Uniform<f32>; 3]);

impl SpawnFrom<RandomTransformDef> for Transform {
	fn spawn(
		component: &RandomTransformDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		let (transform, frame_state) =
			<(Read<SpawnContext<Transform>>, Read<FrameState>)>::fetch(resources);
		let mut rng = frame_state.rng.lock().unwrap();
		let mut transform = transform.0;
		let offset = Vector3::from_iterator(component.0.iter().map(|u| rng.sample(u)));
		transform.position += offset;
		transform
	}
}
