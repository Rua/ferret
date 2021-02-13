use crate::{
	common::{
		assets::AssetStorage,
		spawn::{ComponentAccessor, SpawnFrom, SpawnMergerHandlerSet},
	},
	doom::{
		data::FRAME_RATE,
		physics::Physics,
		state::{State, StateAction, StateName},
		template::EntityTemplateRef,
	},
};
use legion::{
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Registry, Resources, SystemBuilder, Write,
};
use nalgebra::Vector3;
use num_traits::Zero;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Health {
	pub current: i32,
	pub max: i32,
	pub pain_chance: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct HealthDef {
	pub max: i32,
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
	pub damage: i32,
	pub source_entity: Entity,
	pub direction: Vector3<f32>,
}

pub fn apply_damage(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	handler_set.register_clone::<Damage>();

	registry.register::<Health>("Health".into());
	handler_set.register_spawn::<HealthDef, Health>();

	SystemBuilder::new("apply_damage")
		.read_resource::<AssetStorage>()
		.with_query(<(Entity, &Entity, &Damage)>::query())
		.with_query(<(
			&EntityTemplateRef,
			&mut Health,
			Option<&mut Physics>,
			Option<&mut State>,
		)>::query())
		.build(move |command_buffer, world, resources, queries| {
			let asset_storage = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, &damage) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok((template_ref, health, physics, state)) =
					queries.1.get_mut(&mut world, target)
				{
					// Apply damage
					if health.current <= 0 {
						break;
					}

					health.current -= damage.damage;

					// Push the entity away from the damage source
					if let Some(physics) = physics {
						let mut direction =
							Vector3::new(damage.direction[0], damage.direction[1], 0.0);

						// Avoid dividing by zero
						if !direction.is_zero() {
							direction.normalize_mut();
							let mut thrust =
								damage.damage as f32 * 12.5 * FRAME_RATE / physics.mass;

							// Sometimes push the other direction for low damage
							if health.current < 0
								&& damage.damage < 40 && damage.direction[2] > 64.0
								&& thread_rng().gen_bool(0.5)
							{
								direction = -direction;
								thrust *= 4.0;
							}

							physics.velocity += direction * thrust;
						}
					}

					// Trigger states
					if let Some(state) = state {
						let template = asset_storage.get(&template_ref.0).unwrap();

						if health.current <= 0 {
							if health.current < -health.max
								&& template.states.contains_key("xdeath")
							{
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
								&& thread_rng().gen_bool(health.pain_chance as f64)
							{
								let new = (StateName::from("pain").unwrap(), 0);
								state.action = StateAction::Set(new);
							}
						}
					}
				}
			}
		})
}
