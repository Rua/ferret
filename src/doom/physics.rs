use crate::{
	common::{
		assets::AssetStorage,
		frame::FrameState,
		geometry::{angles_to_axes, AABB2, AABB3},
		quadtree::Quadtree,
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom, SpawnMergerHandlerSet},
	},
	doom::{
		components::Transform,
		data::{FRICTION, GRAVITY},
		map::MapDynamic,
		spawn::spawn_helper,
		template::EntityTemplateRef,
		trace::EntityTracer,
	},
};
use bitflags::bitflags;
use legion::{
	component,
	systems::{ResourceSet, Runnable},
	Entity, EntityStore, IntoQuery, Read, Resources, SystemBuilder, Write,
};
use nalgebra::Vector3;
use shrev::EventChannel;
use smallvec::SmallVec;
use std::time::Duration;

bitflags! {
	/// What solid types an entity will block movement for.
	pub struct SolidBits: u8 {
		const PLAYER = 0b1;
		const MONSTER = 0b10;
		const PROJECTILE = 0b100;
		const PARTICLE = 0b1000;
	}
}

/// What type of solid an entity is.
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
pub struct BoxCollider {
	pub height: f32,
	pub radius: f32,
	pub solid_type: SolidType,
	pub blocks_types: SolidBits,
	pub damage_particle: DamageParticle,
}

#[derive(Clone, Copy, Debug)]
pub enum DamageParticle {
	Blood,
	Puff,
}

/// Component for entities that can move and be pushed around.
#[derive(Clone, Copy, Debug)]
pub struct Physics {
	pub collision_response: CollisionResponse,
	pub gravity: bool,
	pub mass: f32,
	pub velocity: Vector3<f32>,
}

/// How the entity responds to colliding with something.
#[derive(Clone, Copy, Debug)]
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
	resources.insert(EventChannel::<StepEvent>::new());

	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<BoxCollider>();
	handler_set.register_clone::<Touchable>();
	handler_set.register_spawn::<TouchEventDef, TouchEvent>();
	handler_set.register_clone::<Physics>();
	handler_set.register_spawn::<PhysicsDef, Physics>();

	SystemBuilder::new("physics")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameState>()
		.write_resource::<Quadtree>()
		.write_resource::<EventChannel<StepEvent>>()
		.with_query(<&MapDynamic>::query())
		.with_query(
			<(Entity, &Transform)>::query()
				.filter(component::<BoxCollider>() & component::<Physics>()),
		)
		.with_query(<(&Transform, &Physics, &BoxCollider)>::query())
		.with_query(<(&mut Transform, &mut Physics)>::query())
		.with_query(<(&EntityTemplateRef, &Touchable)>::query())
		.read_component::<BoxCollider>() // used by EntityTracer
		.read_component::<Transform>() // used by EntityTracer
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, frame_state, quadtree, step_event_channel) = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);
			let map_dynamic = queries.0.iter(&world0).next().unwrap();
			let map = asset_storage.get(&map_dynamic.map).unwrap();

			// Clone the mask so that transform_component is free to be borrowed during the loop
			let entities: Vec<Entity> = queries.1.iter(&world).map(|(e, _)| *e).collect();

			for entity in entities {
				let (&(mut transform), &(mut physics), box_collider) =
					queries.2.get_mut(&mut world, entity).unwrap();

				let entity_bbox =
					{ AABB3::from_radius_height(box_collider.radius, box_collider.height) };
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
						&entity_bbox.offset(transform.position),
						Vector3::new(0.0, 0.0, -0.25),
						solid_type,
					);

					// Only things that get pulled to the ground can experience friction
					if physics.gravity {
						if let Some(collision) = trace.collision {
							// Entity is on ground, apply friction
							// TODO make this work with any ground normal
							let factor = FRICTION.powf(frame_state.delta_time.as_secs_f32());
							physics.velocity[0] *= factor;
							physics.velocity[1] *= factor;

							// Send touch event
							touch_events.push(TouchEvent {
								entity,
								other: collision.entity,
								collision: None,
							});
						} else {
							// Entity isn't on ground, apply gravity
							physics.velocity[2] -= GRAVITY * frame_state.delta_time.as_secs_f32();
						}
					}
				}

				if physics.velocity != Vector3::zeros() {
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
							&entity_bbox,
							solid_type,
							frame_state.delta_time,
						),
						CollisionResponse::StepSlide => step_slide_move(
							&tracer,
							&mut transform.position,
							&mut physics.velocity,
							&mut step_events,
							&mut touch_events,
							entity,
							&entity_bbox,
							solid_type,
							frame_state.delta_time,
						),
					}

					// Set new position and velocity
					quadtree.insert(
						entity,
						&AABB2::from(&entity_bbox.offset(transform.position)),
					);
				}

				let (transform_mut, physics_mut) = queries.3.get_mut(&mut world, entity).unwrap();
				*transform_mut = transform;
				*physics_mut = physics;

				// Send events
				step_event_channel.iter_write(step_events);

				for touch_event in touch_events {
					if let Ok((template_ref, Touchable)) = queries.4.get(&world, touch_event.entity)
					{
						let handle = template_ref.0.clone();
						command_buffer.exec_mut(move |world, resources| {
							resources.insert(SpawnContext(touch_event));
							let asset_storage = <Read<AssetStorage>>::fetch(resources);
							let touch_world = &asset_storage.get(&handle).unwrap().touch;
							spawn_helper(&touch_world, world, resources);
						});
					}

					if let Ok((template_ref, Touchable)) = queries.4.get(&world, touch_event.other)
					{
						let touch_event = TouchEvent {
							entity: touch_event.other,
							other: touch_event.entity,
							collision: touch_event.collision.map(|c| TouchEventCollision {
								normal: -c.normal,
								speed: c.speed,
							}),
						};
						let handle = template_ref.0.clone();
						command_buffer.exec_mut(move |world, resources| {
							resources.insert(SpawnContext(touch_event));
							let asset_storage = <Read<AssetStorage>>::fetch(resources);
							let touch_world = &asset_storage.get(&handle).unwrap().touch;
							spawn_helper(&touch_world, world, resources);
						});
					}
				}
			}

			/*			command_buffer.exec_mut(move |_world, resources| {
				resources.remove::<SpawnContext<Entity>>();
				resources.remove::<SpawnContext<WeaponSpriteSlot>>();
			});*/
		})
}

fn simple_move<W: EntityStore>(
	tracer: &EntityTracer<W>,
	position: &mut Vector3<f32>,
	velocity: &mut Vector3<f32>,
	touch_events: &mut SmallVec<[TouchEvent; 8]>,
	entity: Entity,
	entity_bbox: &AABB3,
	solid_type: SolidType,
	time_left: Duration,
) {
	let trace = tracer.trace(
		&entity_bbox.offset(*position),
		*velocity * time_left.as_secs_f32(),
		solid_type,
	);

	// Commit to the move
	*position += trace.move_step;

	for other in trace.touched.iter().copied() {
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

	let speed = -velocity.dot(&collision.normal);
	*velocity = Vector3::zeros();

	let touch_collision = Some(TouchEventCollision {
		normal: collision.normal,
		speed,
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
}

fn step_slide_move<W: EntityStore>(
	tracer: &EntityTracer<W>,
	position: &mut Vector3<f32>,
	velocity: &mut Vector3<f32>,
	step_events: &mut SmallVec<[StepEvent; 8]>,
	touch_events: &mut SmallVec<[TouchEvent; 8]>,
	entity: Entity,
	entity_bbox: &AABB3,
	solid_type: SolidType,
	mut time_left: Duration,
) {
	let original_velocity = *velocity;

	// Limit the number of move-steps to avoid bumping back and forth between things forever
	let mut range = 0..4;

	while range.next().is_some() && time_left != Duration::default() {
		let trace = tracer.trace(
			&entity_bbox.offset(*position),
			*velocity * time_left.as_secs_f32(),
			solid_type,
		);

		// Commit to the move
		*position += trace.move_step;
		time_left = time_left
			.checked_sub(time_left.mul_f32(trace.fraction))
			.unwrap_or_default();

		for other in trace.touched.iter().copied() {
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
					&entity_bbox.offset(*position),
					Vector3::new(0.0, 0.0, height),
					solid_type,
				);

				if trace.collision.is_none() {
					*position += trace.move_step;
					step_events.push(StepEvent { entity, height });

					for other in trace.touched.iter().copied() {
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

		// Entity has collided, push back along surface normal
		let speed = -velocity.dot(&collision.normal);
		*velocity += collision.normal * speed;

		// Do not bounce back
		if velocity.dot(&original_velocity) <= 0.0 {
			*velocity = Vector3::zeros();
			break;
		}

		let touch_collision = Some(TouchEventCollision {
			normal: collision.normal,
			speed,
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
	}
}

pub const DISTANCE_EPSILON: f32 = 0.03125;

#[derive(Clone, Copy, Debug, Default)]
pub struct Touchable;

#[derive(Clone, Copy, Debug)]
pub struct TouchEvent {
	pub entity: Entity,
	pub other: Entity,
	pub collision: Option<TouchEventCollision>,
}

#[derive(Clone, Copy, Debug)]
pub struct TouchEventCollision {
	pub normal: Vector3<f32>,
	pub speed: f32,
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
pub struct StepEvent {
	pub entity: Entity,
	pub height: f32,
}
