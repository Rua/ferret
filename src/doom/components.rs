use crate::{
	common::{
		frame::FrameState,
		geometry::Angle,
		spawn::{ComponentAccessor, SpawnFrom},
	},
	doom::{
		map::spawn::SpawnContext,
		physics::{BoxCollider, DISTANCE_EPSILON},
	},
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
		let spawn_context = <Read<SpawnContext>>::fetch(resources);
		let mut transform = spawn_context.transform;

		if transform.position[2].is_nan() {
			if component.spawn_on_ceiling {
				transform.position[2] = spawn_context.sector_interval.max - DISTANCE_EPSILON;

				if let Some(box_collider) = accessor.get::<BoxCollider>() {
					transform.position[2] -= box_collider.height;
				}
			} else {
				transform.position[2] = spawn_context.sector_interval.min + DISTANCE_EPSILON;
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
		let (spawn_context, frame_state) =
			<(Read<SpawnContext>, Read<FrameState>)>::fetch(resources);
		let mut rng = frame_state.rng.lock().unwrap();
		let mut transform = spawn_context.transform;
		let offset = Vector3::from_iterator(component.0.iter().map(|u| rng.sample(u)));
		transform.position += offset;
		transform
	}
}
