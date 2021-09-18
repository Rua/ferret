use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		geometry::{angles_to_axes, Angle, Line2, Line3, AABB2, AABB3},
		quadtree::Quadtree,
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom, SpawnMergerHandlerSet},
	},
	doom::{
		assets::template::{EntityTemplate, EntityTemplateRef},
		data::FRAME_RATE,
		game::{
			camera::Camera,
			map::MapDynamic,
			physics::{BoxCollider, Physics, SolidType, TouchEvent},
			spawn::spawn_entity,
			state::{entity::EntityStateEvent, State, StateAction, StateName},
			trace::EntityTracer,
			Transform,
		},
	},
};
use legion::{
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Read, Registry, Resources, SystemBuilder, Write,
};
use nalgebra::{Vector2, Vector3};
use num_traits::Zero;
use rand::{distributions::Uniform, thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub mod weapon;

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
pub struct DamageEvent {
	pub entity: Entity,
	pub damage: i32,
	pub source_entity: Entity,
	pub direction: Vector3<f32>,
}

pub fn apply_damage(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<Health>("Health".into());
	handler_set.register_spawn::<HealthDef, Health>();

	SystemBuilder::new("apply_damage")
		.read_resource::<AssetStorage>()
		.with_query(<&DamageEvent>::query())
		.with_query(<(
			&EntityTemplateRef,
			&mut Health,
			Option<&mut Physics>,
			Option<&mut State>,
		)>::query())
		.build(move |command_buffer, world, resources, queries| {
			let asset_storage = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for &event in queries.0.iter(&world0) {
				if let Ok((template_ref, health, physics, state)) =
					queries.1.get_mut(&mut world, event.entity)
				{
					// Apply damage
					if health.current <= 0 {
						continue;
					}

					health.current -= event.damage;

					// Push the entity away from the damage source
					if let Some(physics) = physics {
						let mut direction =
							Vector3::new(event.direction[0], event.direction[1], 0.0);

						// Avoid dividing by zero
						if !direction.is_zero() {
							direction.normalize_mut();
							let mut thrust = event.damage as f32 * 12.5 * FRAME_RATE / physics.mass;

							// Sometimes push the other direction for low damage
							if health.current < 0
								&& event.damage < 40 && event.direction[2] > 64.0
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
								command_buffer.remove(event.entity);
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

#[derive(Clone, Copy, Debug, Default)]
pub struct ExtraLight(pub f32);

pub fn extra_light(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<ExtraLight>();

	SystemBuilder::new("extra_light")
		.with_query(<(&EntityStateEvent, &ExtraLight)>::query())
		.with_query(<&mut Camera>::query())
		.build(move |_command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, &ExtraLight(extra_light)) in queries.0.iter(&world0) {
				if let Ok(camera) = queries.1.get_mut(&mut world, event.entity) {
					camera.extra_light = extra_light;
				}
			}
		})
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Owner(pub Entity);

#[derive(Clone, Copy, Debug, Default)]
pub struct OwnerDef;

impl SpawnFrom<OwnerDef> for Owner {
	fn spawn(_component: &OwnerDef, _accessor: ComponentAccessor, resources: &Resources) -> Self {
		<Read<SpawnContext<Owner>>>::fetch(resources).0
	}
}

#[derive(Clone, Debug)]
pub struct ProjectileTouch {
	pub damage_range: Uniform<i32>,
	pub damage_multiplier: i32,
}

pub fn projectile_touch(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<ProjectileTouch>();

	SystemBuilder::new("projectile_touch")
		.with_query(<(&TouchEvent, &ProjectileTouch)>::query())
		.with_query(<(&Owner, &mut State)>::query())
		.build(move |command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (event, projectile_touch) in queries.0.iter(&world0) {
				if let Some(collision) = event.collision {
					if let Ok((&Owner(source_entity), state)) =
						queries.1.get_mut(&mut world, event.entity)
					{
						// Kill the projectile entity
						let new = (StateName::from("death").unwrap(), 0);
						state.action = StateAction::Set(new);

						// Apply the damage to the other entity
						let damage = projectile_touch.damage_multiplier
							* thread_rng().sample(projectile_touch.damage_range);

						command_buffer.push((DamageEvent {
							entity: event.other,
							damage,
							source_entity,
							direction: collision.velocity,
						},));
					}
				}
			}
		})
}

#[derive(Clone, Copy, Debug)]
pub struct RadiusAttack {
	pub damage: i32,
	pub radius: f32,
}

pub fn radius_attack(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<RadiusAttack>();

	SystemBuilder::new("radius_attack")
		.read_resource::<AssetStorage>()
		.read_resource::<Quadtree>()
		.with_query(<&MapDynamic>::query())
		.with_query(<(&EntityStateEvent, &RadiusAttack)>::query())
		.with_query(<(Option<&BoxCollider>, Option<&Owner>, &Transform)>::query())
		.with_query(<(&BoxCollider, &Transform)>::query())
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, quadtree) = resources;
			let map_dynamic = queries.0.iter(world).next().unwrap();
			let map = asset_storage.get(&map_dynamic.map).unwrap();

			for (&event, radius_attack) in queries.1.iter(world) {
				let (source_entity, midpoint) = match queries.2.get(world, event.entity) {
					Ok((box_collider, owner, transform)) => {
						let mut midpoint = transform.position;

						if let Some(box_collider) = box_collider {
							midpoint[2] += box_collider.height * 0.75;
						}

						(owner.map_or(event.entity, |o| o.0), midpoint)
					}
					Err(_) => continue,
				};

				let query = &mut queries.3;

				quadtree.traverse_nodes(
					AABB2::from_radius(radius_attack.radius),
					Line2::new(midpoint.fixed_resize(0.0), Vector2::zeros()),
					&mut |entities: &[Entity]| {
						for &entity in entities {
							let (box_collider, transform) = match query.get(world, entity) {
								Ok(x) => x,
								_ => continue,
							};

							if !box_collider.blocks_types.blocks(SolidType::PROJECTILE) {
								continue;
							}

							let bbox =
								AABB3::from_radius_height(box_collider.radius, box_collider.height)
									.offset(transform.position);
							let dist_sq = bbox.direction_from(midpoint).norm_squared();

							if dist_sq >= radius_attack.radius * radius_attack.radius {
								continue;
							}

							if map
								.visible_interval(
									map_dynamic,
									Line3::new(midpoint, transform.position - midpoint),
									box_collider.height,
								)
								.is_empty()
							{
								continue;
							}

							// Apply the damage
							let scale = 1.0 - dist_sq.sqrt() / radius_attack.radius;

							command_buffer.push((DamageEvent {
								entity,
								damage: (radius_attack.damage as f32 * scale) as i32,
								source_entity,
								direction: transform.position - midpoint,
							},));
						}

						Vector2::zeros()
					},
				);
			}
		})
}

#[derive(Clone, Debug)]
pub struct SpawnProjectile(pub AssetHandle<EntityTemplate>);

pub fn spawn_projectile(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<Owner>("Owner".into());
	handler_set.register_spawn::<OwnerDef, Owner>();

	handler_set.register_clone::<SpawnProjectile>();

	SystemBuilder::new("spawn_projectile")
		.read_resource::<AssetStorage>()
		.with_query(<(&EntityStateEvent, &SpawnProjectile)>::query())
		.with_query(<&Transform>::query())
		.build(move |command_buffer, world, _resources, queries| {
			let (world0, world) = world.split_for_query(&queries.0);

			for (&event, SpawnProjectile(projectile_handle)) in queries.0.iter(&world0) {
				if let Ok(&(mut transform)) = queries.1.get(&world, event.entity) {
					let handle = projectile_handle.clone();
					let direction = angles_to_axes(transform.rotation)[0];
					transform.position += direction; // Start a little forward from the spawner
					transform.position[2] += 32.0;

					command_buffer.exec_mut(move |world, resources| {
						resources.insert(SpawnContext(Owner(event.entity)));
						spawn_entity(world, resources, &handle, transform);
					});
				}
			}

			command_buffer.exec_mut(move |_world, resources| {
				resources.remove::<SpawnContext<Owner>>();
			})
		})
}

#[derive(Clone, Debug)]
pub struct SprayAttack {
	pub count: usize,
	pub damage_range: Uniform<i32>,
	pub damage_multiplier: i32,
	pub distance: f32,
	pub particle: AssetHandle<EntityTemplate>,
	pub spread: Vector2<Angle>,
}

pub fn spray_attack(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<SprayAttack>();

	SystemBuilder::new("spray_attack")
		.read_resource::<AssetStorage>()
		.read_resource::<Quadtree>()
		.with_query(<&MapDynamic>::query())
		.with_query(<(&EntityStateEvent, &SprayAttack)>::query())
		.with_query(<&Owner>::query())
		.with_query(<(Option<&BoxCollider>, &Transform)>::query())
		.read_component::<BoxCollider>() // used by EntityTracer
		.read_component::<Health>() // used by EntityTracer
		.read_component::<Owner>() // used by EntityTracer
		.read_component::<Transform>() // used by EntityTracer
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, quadtree) = resources;

			let map_dynamic = queries.0.iter(world).next().unwrap();
			let map = asset_storage.get(&map_dynamic.map).unwrap();

			for (&event, spray_attack) in queries.1.iter(world) {
				let owner = queries
					.2
					.get(world, event.entity)
					.map_or(event.entity, |o| o.0);

				let (midpoint, angle) = match queries.3.get(world, owner) {
					Ok((box_collider, transform)) => {
						let mut midpoint = transform.position;

						if let Some(box_collider) = box_collider {
							midpoint[2] += box_collider.height + 8.0;
						}

						(midpoint, transform.rotation[2])
					}
					_ => continue,
				};

				assert!(spray_attack.count >= 2);
				let step = 2.0 / (spray_attack.count - 1) as f64;

				for i in 0..spray_attack.count {
					let angle = angle + spray_attack.spread[0] * (i as f64 * step - 1.0);
					let move_step = Line3::new(
						midpoint,
						spray_attack.distance
							* Vector3::new(angle.cos() as f32, angle.sin() as f32, 0.0),
					);

					let tracer = EntityTracer {
						map,
						map_dynamic,
						quadtree: &quadtree,
						world,
					};

					let trace = tracer.closest_visible_target(Some(owner), move_step);

					// Hit something!
					if let Some(collision) = trace.collision {
						// Apply the damage
						let damage = spray_attack.damage_multiplier
							* thread_rng().sample(spray_attack.damage_range);

						command_buffer.push((DamageEvent {
							entity: collision.entity,
							damage,
							source_entity: event.entity,
							direction: trace.move_step.dir,
						},));

						if let Ok((box_collider, transform)) =
							queries.3.get(world, collision.entity)
						{
							// Spawn particles
							let mut particle_transform = Transform {
								position: transform.position,
								rotation: Vector3::zeros(),
							};

							if let Some(box_collider) = box_collider {
								particle_transform.position[2] += box_collider.height / 4.0;
							}

							let handle = spray_attack.particle.clone();

							command_buffer.exec_mut(move |world, resources| {
								spawn_entity(world, resources, &handle, particle_transform);
							});
						}
					}
				}
			}
		})
}
