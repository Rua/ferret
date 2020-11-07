use crate::{
	common::{
		assets::AssetStorage,
		frame::FrameRng,
		geometry::Line3,
		spawn::{ComponentAccessor, SpawnFrom, SpawnMergerHandlerSet},
	},
	doom::{
		components::{Transform, Velocity},
		map::spawn::spawn_entity,
		state::{State, StateAction, StateName},
		template::EntityTemplateRef,
		FRAME_TIME,
	},
};
use legion::{
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Resources, SystemBuilder, Write,
};
use nalgebra::Vector3;
use rand::Rng;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Health {
	pub current: f32,
	pub max: f32,
	pub pain_chance: f32,
	pub blood: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct HealthDef {
	pub max: f32,
	pub pain_chance: f32,
	pub blood: bool,
}

impl SpawnFrom<HealthDef> for Health {
	fn spawn(component: &HealthDef, _accessor: ComponentAccessor, _resources: &Resources) -> Self {
		Health {
			current: component.max,
			max: component.max,
			pain_chance: component.pain_chance,
			blood: component.blood,
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Damage {
	pub amount: f32,
	pub source_entity: Entity,
	pub line: Line3,
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

			for (&entity, &target, &damage) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok((template_ref, frame_rng, health, state)) =
					queries.1.get_mut(&mut world, target)
				{
					let transform = Transform {
						position: damage.line.point + damage.line.dir,
						rotation: Vector3::zeros(),
					};

					// Spawn particles
					// TODO make this a part of entity templates
					let template_name = if health.blood { "blood" } else { "puff" };
					let handle = asset_storage
						.handle_for(template_name)
						.expect("Damage particle template is not present");

					command_buffer.exec_mut(move |world, resources| {
						let entity = spawn_entity(world, resources, &handle, transform);
						if let Ok((frame_rng, state, transform, velocity)) =
							<(&mut FrameRng, &mut State, &mut Transform, &mut Velocity)>::query()
								.get_mut(world, entity)
						{
							transform.position[2] += frame_rng.gen_range(-4.0, 4.0);

							let diff = frame_rng.gen_range(0, 8) * FRAME_TIME;
							let new_time = state
								.timer
								.target_time()
								.checked_sub(diff)
								.unwrap_or(Duration::default());
							state.timer.set_target(new_time);

							if template_name == "blood" {
								velocity.velocity[2] += 2.0;

								if damage.amount < 9.0 {
									let new = (StateName::from("spawn").unwrap(), 2);
									state.action = StateAction::Set(new);
								} else if damage.amount <= 12.0 {
									let new = (StateName::from("spawn").unwrap(), 1);
									state.action = StateAction::Set(new);
								}
							} else {
								velocity.velocity[2] += 1.0;
							}
						}
					});

					// Apply damage
					if health.current <= 0.0 {
						break;
					}

					health.current -= damage.amount;

					// Trigger states
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
