use crate::{
	common::{
		assets::AssetStorage,
		geometry::{angles_to_axes, Line3, AABB2, AABB3},
		quadtree::Quadtree,
		spawn::{spawn_helper, ComponentAccessor, SpawnContext, SpawnFrom, SpawnMergerHandlerSet},
		time::DeltaTime,
	},
	doom::{
		assets::template::EntityTemplateRef,
		data::{FRICTION, GRAVITY},
		game::{
			combat::Owner, map::MapDynamic, state::entity::EntityStateEvent, trace::EntityTracer,
			Transform,
		},
	},
};
use bitflags::bitflags;
use legion::{
	component,
	systems::{ResourceSet, Runnable},
	Entity, EntityStore, IntoQuery, Read, Registry, Resources, SystemBuilder, Write,
};
use nalgebra::Vector3;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::time::Duration;

bitflags! {
	/// What solid types an entity will block movement for.
	#[derive(Serialize, Deserialize)]
	pub struct SolidBits: u8 {
		const PLAYER = 0b1;
		const MONSTER = 0b10;
		const PROJECTILE = 0b100;
		const PARTICLE = 0b1000;
	}
}

/// What type of solid an entity is.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum SolidType {
	PLAYER = SolidBits::PLAYER.bits(),
	MONSTER = SolidBits::MONSTER.bits(),
	PROJECTILE = SolidBits::PROJECTILE.bits(),
	PARTICLE = SolidBits::PARTICLE.bits(),
}

impl SolidBits {
	/// Returns whether the current entity will block movement of a certain solid type.
	#[inline]
	pub fn blocks(&self, solid_type: SolidType) -> bool {
		self.intersects(SolidBits::from_bits_truncate(solid_type as u8))
	}
}

/// Component for entities that can be collided with, optionally blocking movement.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct BoxCollider {
	pub height: f32,
	pub radius: f32,
	pub solid_type: SolidType,
	pub blocks_types: SolidBits,
	pub damage_particle: DamageParticle,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DamageParticle {
	Blood,
	Puff,
}

/// Component for entities that can move and be pushed around.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Physics {
	pub collision_response: CollisionResponse,
	pub gravity: bool,
	pub mass: f32,
	pub velocity: Vector3<f32>,
}

/// How the entity responds to colliding with something.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CollisionResponse {
	Stop,
	StepSlide,
}

/// Spawns a Physics component using the specified initial speed,
// in the direction specified by the spawn Transform angle.
#[derive(Clone, Copy, Debug)]
pub struct PhysicsDef {
	pub collision_response: CollisionResponse,
	pub gravity: bool,
	pub mass: f32,
	pub speed: f32,
}

impl SpawnFrom<PhysicsDef> for Physics {
	fn spawn(component: &PhysicsDef, _accessor: ComponentAccessor, resources: &Resources) -> Self {
		assert_ne!(component.mass, 0.0);

		let velocity = if component.speed > 0.0 {
			let transform = <Read<SpawnContext<Transform>>>::fetch(resources);
			angles_to_axes(transform.0.rotation)[0] * component.speed
		} else {
			Vector3::zeros()
		};

		Physics {
			collision_response: component.collision_response,
			gravity: component.gravity,
			mass: component.mass,
			velocity,
		}
	}
}

pub fn physics(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<BoxCollider>("BoxCollider".into());
	handler_set.register_clone::<BoxCollider>();

	registry.register::<Physics>("Physics".into());
	handler_set.register_clone::<Physics>();
	handler_set.register_spawn::<PhysicsDef, Physics>();

	registry.register::<Touchable>("Touchable".into());
	handler_set.register_clone::<Touchable>();
	handler_set.register_spawn::<TouchEventDef, TouchEvent>();

	SystemBuilder::new("physics")
		.read_resource::<AssetStorage>()
		.read_resource::<DeltaTime>()
		.write_resource::<Quadtree>()
		.with_query(<&MapDynamic>::query())
		.with_query(
			<(Entity, &Transform)>::query()
				.filter(component::<BoxCollider>() & component::<Physics>()),
		)
		.with_query(<(&BoxCollider, Option<&Owner>, &Physics, &Transform)>::query())
		.with_query(<(&mut Transform, &mut Physics)>::query())
		.with_query(<(&EntityTemplateRef, &Touchable)>::query())
		.read_component::<BoxCollider>() // used by EntityTracer
		.read_component::<Owner>() // used by EntityTracer
		.read_component::<Transform>() // used by EntityTracer
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, delta_time, quadtree) = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);
			let map_dynamic = queries.0.iter(&world0).next().unwrap();
			let map = asset_storage.get(&map_dynamic.map).unwrap();

			// Clone the mask so that transform_component is free to be borrowed during the loop
			let entities: Vec<Entity> = queries.1.iter(&world).map(|(e, _)| *e).collect();

			for entity in entities {
				let (box_collider, owner, &(mut physics), &(mut transform)) =
					queries.2.get_mut(&mut world, entity).unwrap();

				let ignore = Some(owner.map_or(entity, |&Owner(owner)| owner));
				let bbox = { AABB3::from_radius_height(box_collider.radius, box_collider.height) };
				let solid_type = box_collider.solid_type;

				let mut step_events: SmallVec<[StepEvent; 8]> = SmallVec::new();
				let mut touch_events: SmallVec<[TouchEvent; 8]> = SmallVec::new();

				{
					let tracer = EntityTracer {
						map,
						map_dynamic,
						quadtree: &quadtree,
						world: &world,
					};

					// Check for ground
					let trace = tracer.trace(
						&bbox,
						solid_type,
						ignore,
						Line3::new(transform.position, Vector3::new(0.0, 0.0, -0.25)),
					);

					// Only things that get pulled to the ground can experience friction
					if physics.gravity {
						if let Some(collision) = trace.collision {
							// Entity is on ground, apply friction
							// TODO make this work with any ground normal
							let factor = FRICTION.powf(delta_time.0.as_secs_f32());
							physics.velocity[0] *= factor;
							physics.velocity[1] *= factor;

							const STOP_EPSILON: f32 = 0.001;

							if physics.velocity.fixed_rows::<2>(0).norm_squared() < STOP_EPSILON {
								physics.velocity[0] = 0.0;
								physics.velocity[1] = 0.0;
							}

							// Send touch event
							touch_events.push(TouchEvent {
								entity,
								other: collision.entity,
								collision: None,
							});
						} else {
							// Entity isn't on ground, apply gravity
							physics.velocity[2] -= GRAVITY * delta_time.0.as_secs_f32();
						}
					}
				}

				if !physics.velocity.is_zero() {
					quadtree.remove(entity);

					let tracer = EntityTracer {
						map,
						map_dynamic,
						quadtree: &quadtree,
						world: &world,
					};

					// Apply the move
					match physics.collision_response {
						CollisionResponse::Stop => simple_move(
							&tracer,
							&mut transform.position,
							&mut physics.velocity,
							&mut touch_events,
							entity,
							&bbox,
							solid_type,
							ignore,
							delta_time.0,
						),
						CollisionResponse::StepSlide => step_slide_move(
							&tracer,
							&mut transform.position,
							&mut physics.velocity,
							&mut step_events,
							&mut touch_events,
							entity,
							&bbox,
							solid_type,
							ignore,
							delta_time.0,
						),
					}

					// Set new position and velocity
					quadtree.insert(entity, &AABB2::from(bbox.offset(transform.position)));
				}

				let (transform_mut, physics_mut) = queries.3.get_mut(&mut world, entity).unwrap();
				*transform_mut = transform;
				*physics_mut = physics;

				// Send events
				for event in step_events {
					command_buffer.push((event,));
				}

				for event in touch_events {
					if let Ok((template_ref, Touchable)) = queries.4.get(&world, event.entity) {
						let handle = template_ref.0.clone();
						command_buffer.exec_mut(move |world, resources| {
							resources.insert(SpawnContext(event));
							let asset_storage = <Read<AssetStorage>>::fetch(resources);
							let touch_world = &asset_storage.get(&handle).unwrap().touch;
							spawn_helper(&touch_world, world, resources);
						});
					}

					if let Ok((template_ref, Touchable)) = queries.4.get(&world, event.other) {
						let event = TouchEvent {
							entity: event.other,
							other: event.entity,
							collision: event.collision.map(|c| TouchEventCollision {
								velocity: -c.velocity,
								normal: -c.normal,
							}),
						};
						let handle = template_ref.0.clone();
						command_buffer.exec_mut(move |world, resources| {
							resources.insert(SpawnContext(event));
							let asset_storage = <Read<AssetStorage>>::fetch(resources);
							let touch_world = &asset_storage.get(&handle).unwrap().touch;
							spawn_helper(&touch_world, world, resources);
						});
					}
				}
			}

			command_buffer.exec_mut(move |_world, resources| {
				resources.remove::<SpawnContext<TouchEvent>>();
			});
		})
}

fn simple_move<W: EntityStore>(
	tracer: &EntityTracer<W>,
	position: &mut Vector3<f32>,
	velocity: &mut Vector3<f32>,
	touch_events: &mut SmallVec<[TouchEvent; 8]>,
	entity: Entity,
	bbox: &AABB3,
	solid_type: SolidType,
	ignore: Option<Entity>,
	time_left: Duration,
) {
	let trace = tracer.trace(
		&bbox,
		solid_type,
		ignore,
		Line3::new(*position, *velocity * time_left.as_secs_f32()),
	);

	// Commit to the move
	*position = trace.move_step.end_point();

	// Touch nonsolids
	for other in tracer.trace_nonsolid(&bbox, solid_type, trace.move_step) {
		if touch_events.iter().find(|t| t.other == other).is_none() {
			touch_events.push(TouchEvent {
				entity,
				other,
				collision: None,
			});
		}
	}

	let collision = match trace.collision {
		Some(x) => x,
		None => return,
	};

	// Add TouchEvent
	let touch_collision = Some(TouchEventCollision {
		velocity: *velocity,
		normal: collision.normal,
	});

	if let Some(event) = touch_events
		.iter_mut()
		.find(|t| t.other == collision.entity)
	{
		event.collision = touch_collision;
	} else {
		touch_events.push(TouchEvent {
			entity,
			other: collision.entity,
			collision: touch_collision,
		});
	}

	// Stop the entity
	*velocity = Vector3::zeros();
}

fn step_slide_move<W: EntityStore>(
	tracer: &EntityTracer<W>,
	position: &mut Vector3<f32>,
	velocity: &mut Vector3<f32>,
	step_events: &mut SmallVec<[StepEvent; 8]>,
	touch_events: &mut SmallVec<[TouchEvent; 8]>,
	entity: Entity,
	bbox: &AABB3,
	solid_type: SolidType,
	ignore: Option<Entity>,
	mut time_left: Duration,
) {
	let original_velocity = *velocity;

	// Limit the number of move-steps to avoid bumping back and forth between things forever
	let mut range = 0..4;

	while range.next().is_some() && !time_left.is_zero() {
		let trace = tracer.trace(
			&bbox,
			solid_type,
			ignore,
			Line3::new(*position, *velocity * time_left.as_secs_f32()),
		);

		// Commit to the move
		*position = trace.move_step.end_point();
		time_left = time_left
			.checked_sub(time_left.mul_f32(trace.fraction))
			.unwrap_or_default();

		// Touch nonsolids
		for other in tracer.trace_nonsolid(&bbox, solid_type, trace.move_step) {
			if touch_events.iter().find(|t| t.other == other).is_none() {
				touch_events.push(TouchEvent {
					entity,
					other,
					collision: None,
				});
			}
		}

		let collision = match trace.collision {
			Some(x) => x,
			None => continue,
		};

		// If entity collided with a step, try to step up first
		if let Some(step_z) = collision.step_z {
			let height = step_z - position[2];
			const MAX_STEP: f32 = 24.5;

			// See if it can move up by the step height
			if height > 0.0 && height < MAX_STEP {
				let trace = tracer.trace(
					&bbox,
					solid_type,
					ignore,
					Line3::new(*position, Vector3::new(0.0, 0.0, height)),
				);

				if trace.collision.is_none() {
					*position = trace.move_step.end_point();
					velocity[2] = velocity[2].max(0.0); // Do not fall back down
					step_events.push(StepEvent { entity, height });

					// Touch nonsolids
					for other in tracer.trace_nonsolid(&bbox, solid_type, trace.move_step) {
						if touch_events.iter().find(|t| t.other == other).is_none() {
							touch_events.push(TouchEvent {
								entity,
								other,
								collision: None,
							});
						}
					}

					// Stepped up, do not collide
					continue;
				}
			}
		}

		// Add TouchEvent
		let touch_collision = Some(TouchEventCollision {
			normal: collision.normal,
			velocity: *velocity,
		});

		if let Some(event) = touch_events
			.iter_mut()
			.find(|t| t.other == collision.entity)
		{
			event.collision = touch_collision;
		} else {
			touch_events.push(TouchEvent {
				entity,
				other: collision.entity,
				collision: touch_collision,
			});
		}

		// Push back along surface normal
		let speed = -velocity.dot(&collision.normal);
		*velocity += collision.normal * speed;

		// Do not bounce back
		if velocity.dot(&original_velocity) <= 0.0 {
			*velocity = Vector3::zeros();
			break;
		}
	}
}

pub const DISTANCE_EPSILON: f32 = 0.03125;

#[derive(Clone, Copy, Debug)]
pub struct StepEvent {
	pub entity: Entity,
	pub height: f32,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Touchable;

#[derive(Clone, Copy, Debug)]
pub struct TouchEvent {
	pub entity: Entity,
	pub other: Entity,
	pub collision: Option<TouchEventCollision>,
}

#[derive(Clone, Copy, Debug)]
pub struct TouchEventCollision {
	pub velocity: Vector3<f32>,
	pub normal: Vector3<f32>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TouchEventDef;

impl SpawnFrom<TouchEventDef> for TouchEvent {
	fn spawn(
		_component: &TouchEventDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		<Read<SpawnContext<TouchEvent>>>::fetch(resources).0
	}
}

#[derive(Clone, Copy, Debug)]
pub struct SetBlocksTypes(pub SolidBits);

pub fn set_blocks_types(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<SetBlocksTypes>();

	SystemBuilder::new("set_blocks_types")
		.with_query(<(&EntityStateEvent, &SetBlocksTypes)>::query())
		.with_query(<&mut BoxCollider>::query())
		.build(move |_command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, &SetBlocksTypes(blocks_types)) in queries.0.iter(&world0) {
				if let Ok(box_collider) = queries.1.get_mut(&mut world, event.entity) {
					box_collider.blocks_types = blocks_types;
				}
			}
		})
}

#[derive(Clone, Copy, Debug)]
pub struct SetSolidType(pub SolidType);

pub fn set_solid_type(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<SetSolidType>();

	SystemBuilder::new("set_solid_type")
		.with_query(<(&EntityStateEvent, &SetSolidType)>::query())
		.with_query(<&mut BoxCollider>::query())
		.build(move |_command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, &SetSolidType(solid_type)) in queries.0.iter(&world0) {
				if let Ok(box_collider) = queries.1.get_mut(&mut world, event.entity) {
					box_collider.solid_type = solid_type;
				}
			}
		})
}
