use crate::{
	common::{
		assets::AssetStorage,
		frame::FrameRng,
		spawn::{ComponentAccessor, SpawnFrom, SpawnMergerHandlerSet},
	},
	doom::{
		state::{State, StateAction, StateName},
		template::EntityTemplateRef,
	},
};
use legion::{
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Resources, SystemBuilder, Write,
};
use nalgebra::Vector3;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Health {
	pub current: f32,
	pub max: f32,
	pub pain_chance: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct HealthDef {
	pub max: f32,
	pub pain_chance: f32,
}

impl SpawnFrom<HealthDef> for Health {
	fn spawn(component: &HealthDef, _accessor: ComponentAccessor, _resources: &Resources) -> Self {
		Health {
			current: component.max,
			max: component.max,
			pain_chance: component.pain_chance,
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Damage {
	pub amount: f32,
	pub origin_point: Vector3<f32>,
	pub source_entity: Entity,
}

pub fn apply_damage(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_spawn::<HealthDef, Health>();

	SystemBuilder::new("apply_damage")
		.read_resource::<AssetStorage>()
		.with_query(<(Entity, &Entity, &Damage)>::query())
		.with_query(<(&EntityTemplateRef, &mut FrameRng, &mut Health, &mut State)>::query())
		.build(move |command_buffer, world, resources, queries| {
			let asset_storage = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, damage) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok((template_ref, frame_rng, health, state)) =
					queries.1.get_mut(&mut world, target)
				{
					if health.current <= 0.0 {
						break;
					}

					health.current -= damage.amount;

					let template = asset_storage.get(&template_ref.0).unwrap();

					if health.current <= 0.0 {
						if health.current < -health.max && template.states.contains_key("xdeath") {
							let new = (StateName::from("xdeath").unwrap(), 0);
							state.action = StateAction::Set(new);
						} else if template.states.contains_key("death") {
							let new = (StateName::from("death").unwrap(), 0);
							state.action = StateAction::Set(new);
						} else {
							state.action = StateAction::None;
							command_buffer.remove(entity);
						}
					} else {
						if template.states.contains_key("pain")
							&& frame_rng.gen_bool(health.pain_chance as f64)
						{
							let new = (StateName::from("pain").unwrap(), 0);
							state.action = StateAction::Set(new);
						}
					}
				}
			}
		})
}
