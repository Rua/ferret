use crate::{
	common::{
		geometry::{Angle, Interval},
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom, SpawnMergerHandlerSet},
	},
	doom::{
		draw::{sprite::SpriteRender, wsprite::WeaponSpriteRender},
		map::{LinedefRef, LinedefRefDef, MapDynamic, SectorRef, SectorRefDef},
		physics::{BoxCollider, DISTANCE_EPSILON},
		template::{EntityTemplateRef, EntityTemplateRefDef},
		ui::{UiImage, UiTransform},
	},
};
use legion::{systems::ResourceSet, Read, Registry, Resources, Write};
use nalgebra::Vector3;
use rand::{distributions::Uniform, thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub fn register_components(resources: &mut Resources) {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<EntityTemplateRef>("EntityTemplateRef".into());
	handler_set.register_spawn::<EntityTemplateRefDef, EntityTemplateRef>();

	registry.register::<LinedefRef>("LinedefRef".into());
	handler_set.register_clone::<LinedefRef>();
	handler_set.register_spawn::<LinedefRefDef, LinedefRef>();

	registry.register::<MapDynamic>("MapDynamic".into());
	handler_set.register_clone::<MapDynamic>();

	registry.register::<SectorRef>("SectorRef".into());
	handler_set.register_clone::<SectorRef>();
	handler_set.register_spawn::<SectorRefDef, SectorRef>();

	registry.register::<SpawnPoint>("SpawnPoint".into());
	handler_set.register_clone::<SpawnPoint>();

	registry.register::<SpriteRender>("SpriteRender".into());
	handler_set.register_clone::<SpriteRender>();

	registry.register::<Transform>("Transform".into());
	handler_set.register_spawn::<TransformDef, Transform>();
	handler_set.register_spawn::<RandomTransformDef, Transform>();

	handler_set.register_clone::<UiTransform>();

	handler_set.register_clone::<UiImage>();

	registry.register::<WeaponSpriteRender>("WeaponSpriteRender".into());
	handler_set.register_clone::<WeaponSpriteRender>();
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpawnPoint {
	pub player_num: usize,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
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
		let transform = <Read<SpawnContext<Transform>>>::fetch(resources);
		let mut transform = transform.0;
		let offset = Vector3::from_iterator(component.0.iter().map(|u| thread_rng().sample(u)));
		transform.position += offset;
		transform
	}
}
