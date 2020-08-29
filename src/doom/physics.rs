use crate::{
	assets::AssetStorage,
	doom::{
		components::{Transform, Velocity},
		data::{FRICTION, GRAVITY},
		door::DoorTouch,
		floor::FloorTouch,
		map::{Map, MapDynamic, NodeChild, Subsector},
		plat::PlatTouch,
	},
	geometry::{Interval, Plane3, AABB2, AABB3},
	quadtree::Quadtree,
};
use arrayvec::ArrayVec;
use bitflags::bitflags;
use lazy_static::lazy_static;
use legion::prelude::{
	component, Entity, EntityStore, IntoQuery, Read, Resources, Runnable, SystemBuilder,
};
use nalgebra::Vector3;
use shrev::EventChannel;
use smallvec::SmallVec;
use std::time::Duration;

#[derive(Default)]
pub struct PhysicsSystem;

pub fn physics_system(resources: &mut Resources) -> Box<dyn Runnable> {
	resources.insert(EventChannel::<StepEvent>::new());
	resources.insert(EventChannel::<TouchEvent>::new());

	SystemBuilder::new("physics_system")
		.read_resource::<AssetStorage>()
		.read_resource::<Duration>()
		.write_resource::<Quadtree>()
		.write_resource::<EventChannel<StepEvent>>()
		.write_resource::<EventChannel<TouchEvent>>()
		.read_component::<BoxCollider>()
		.read_component::<MapDynamic>()
		.write_component::<Transform>()
		.write_component::<Velocity>()
		.build_thread_local(move |_, world, resources, _| {
			let (asset_storage, delta, quadtree, step_event_channel, touch_event_channel) =
				resources;
			let (map_dynamic_world, mut world) = world.split::<Read<MapDynamic>>();
			let map_dynamic = <Read<MapDynamic>>::query()
				.iter(&map_dynamic_world)
				.next()
				.unwrap();
			let map = asset_storage.get(&map_dynamic.map).unwrap();

			// Clone the mask so that transform_component is free to be borrowed during the loop
			let entities: Vec<Entity> = <Read<Transform>>::query()
				.filter(component::<BoxCollider>() & component::<Velocity>())
				.iter_entities(&world)
				.map(|(e, _)| e)
				.collect();

			for entity in entities {
				let mut new_position = world.get_component::<Transform>(entity).unwrap().position;
				let mut new_velocity = world.get_component::<Velocity>(entity).unwrap().velocity;
				let entity_bbox = {
					let box_collider = world.get_component::<BoxCollider>(entity).unwrap();
					AABB3::from_radius_height(box_collider.radius, box_collider.height)
				};

				let mut step_events: SmallVec<[StepEvent; 8]> = SmallVec::new();
				let mut touch_events: SmallVec<[TouchEvent; 8]> = SmallVec::new();

				if new_velocity == Vector3::zeros() {
					continue;
				}

				quadtree.remove(entity);

				let tracer = EntityTracer {
					map,
					map_dynamic: map_dynamic.as_ref(),
					quadtree: &quadtree,
					world: &world,
				};

				// Check for ground
				let trace = tracer.trace(
					&entity_bbox.offset(new_position),
					Vector3::new(0.0, 0.0, -0.25),
					SolidMask::NON_MONSTER, // TODO solid mask
				);

				if let Some(collision) = trace.collision {
					// Entity is on ground, apply friction
					// TODO make this work with any ground normal
					let factor = FRICTION.powf(delta.as_secs_f32());
					new_velocity[0] *= factor;
					new_velocity[1] *= factor;

					// Send touch event
					touch_events.push(TouchEvent {
						toucher: entity,
						touched: collision.entity,
						collision: None,
					});
				} else {
					// Entity isn't on ground, apply gravity
					new_velocity[2] -= GRAVITY * delta.as_secs_f32();
				}

				// Apply the move
				step_slide_move(
					&tracer,
					&mut new_position,
					&mut new_velocity,
					&mut step_events,
					&mut touch_events,
					entity,
					&entity_bbox,
					SolidMask::NON_MONSTER, // TODO solid mask
					**delta,
				);

				// Set new position and velocity
				world
					.get_component_mut::<Transform>(entity)
					.unwrap()
					.position = new_position;
				world
					.get_component_mut::<Velocity>(entity)
					.unwrap()
					.velocity = new_velocity;
				quadtree.insert(entity, &AABB2::from(&entity_bbox.offset(new_position)));

				// Send events
				step_event_channel.iter_write(step_events);
				touch_event_channel.iter_write(touch_events);
			}
		})
}

fn step_slide_move<W: EntityStore>(
	tracer: &EntityTracer<W>,
	position: &mut Vector3<f32>,
	velocity: &mut Vector3<f32>,
	step_events: &mut SmallVec<[StepEvent; 8]>,
	touch_events: &mut SmallVec<[TouchEvent; 8]>,
	entity: Entity,
	entity_bbox: &AABB3,
	solid_mask: SolidMask,
	mut time_left: Duration,
) {
	let original_velocity = *velocity;

	// Limit the number of move-steps to avoid bumping back and forth between things forever
	let mut range = 0..4;

	while range.next().is_some() && time_left != Duration::default() {
		let trace = tracer.trace(
			&entity_bbox.offset(*position),
			*velocity * time_left.as_secs_f32(),
			solid_mask,
		);

		// Commit to the move
		*position += trace.move_step;
		time_left = time_left
			.checked_sub(time_left.mul_f32(trace.fraction))
			.unwrap_or_default();

		for touched in trace.touched.iter().copied() {
			if touch_events.iter().find(|t| t.touched == touched).is_none() {
				touch_events.push(TouchEvent {
					toucher: entity,
					touched,
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
					solid_mask,
				);

				if trace.collision.is_none() {
					*position += trace.move_step;
					step_events.push(StepEvent { entity, height });

					for touched in trace.touched.iter().copied() {
						if touch_events.iter().find(|t| t.touched == touched).is_none() {
							touch_events.push(TouchEvent {
								toucher: entity,
								touched,
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
			.find(|t| t.touched == collision.entity)
		{
			event.collision = touch_collision;
		} else {
			touch_events.push(TouchEvent {
				toucher: entity,
				touched: collision.entity,
				collision: touch_collision,
			});
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct BoxCollider {
	pub height: f32,
	pub radius: f32,
	pub solid_mask: SolidMask,
}

#[derive(Clone, Copy, Debug)]
pub struct TouchEvent {
	pub toucher: Entity,
	pub touched: Entity,
	pub collision: Option<TouchEventCollision>,
}

#[derive(Clone, Copy, Debug)]
pub struct TouchEventCollision {
	pub normal: Vector3<f32>,
	pub speed: f32,
}

#[derive(Clone, Debug)]
pub enum TouchAction {
	DoorTouch(DoorTouch),
	FloorTouch(FloorTouch),
	PlatTouch(PlatTouch),
}

#[derive(Clone, Copy, Debug)]
pub struct StepEvent {
	pub entity: Entity,
	pub height: f32,
}

bitflags! {
	pub struct SolidMask: u16 {
		const NON_MONSTER = 0b01;
		const MONSTER = 0b10;
	}
}

pub struct EntityTracer<'a, W: EntityStore> {
	pub map: &'a Map,
	pub map_dynamic: &'a MapDynamic,
	pub quadtree: &'a Quadtree,
	pub world: &'a W,
}

#[derive(Clone, Debug)]
pub struct EntityTrace {
	pub fraction: f32,
	pub move_step: Vector3<f32>,
	pub collision: Option<EntityTraceCollision>,
	pub touched: SmallVec<[Entity; 4]>,
}

#[derive(Clone, Debug)]
pub struct EntityTraceCollision {
	pub entity: Entity,
	pub normal: Vector3<f32>,
	pub step_z: Option<f32>,
}

const DISTANCE_EPSILON: f32 = 0.03125;

impl<'a, W: EntityStore> EntityTracer<'a, W> {
	pub fn trace(
		&self,
		entity_bbox: &AABB3,
		move_step: Vector3<f32>,
		entity_solid_mask: SolidMask,
	) -> EntityTrace {
		let mut trace_fraction = 1.0;
		let mut trace_collision = None;
		let mut trace_touched: SmallVec<[(f32, Entity); 8]> = SmallVec::new();

		let zero_bbox = AABB3::from_point(entity_bbox.middle());
		let move_bbox = entity_bbox.union(&entity_bbox.offset(move_step));
		let move_bbox2 = AABB2::from(&move_bbox);

		self.map
			.traverse_nodes(NodeChild::Node(0), &move_bbox2, &mut |node: NodeChild| {
				let linedefs = match node {
					NodeChild::Subsector(index) => &self.map.subsectors[index].linedefs,
					NodeChild::Node(index) => &self.map.nodes[index].linedefs,
				};

				for linedef_index in linedefs.iter().copied() {
					let linedef = &self.map.linedefs[linedef_index];

					if !move_bbox2.overlaps(&linedef.bbox) {
						continue;
					}

					let linedef_dynamic = &self.map_dynamic.linedefs[linedef_index];

					if let [Some(front_sidedef), Some(back_sidedef)] = &linedef.sidedefs {
						let front_interval =
							&self.map_dynamic.sectors[front_sidedef.sector_index].interval;
						let back_interval =
							&self.map_dynamic.sectors[back_sidedef.sector_index].interval;

						let intersection = front_interval.intersection(*back_interval);
						let union = front_interval.union(*back_interval);
						let intervals = ArrayVec::from([
							(
								Interval::new(union.min, intersection.min),
								SolidMask::all(),
								true,
							),
							(
								Interval::new(intersection.min, intersection.max),
								linedef.solid_mask,
								false,
							),
							(
								Interval::new(intersection.max, union.max),
								SolidMask::all(),
								false,
							),
						]);

						for (interval, solid_mask, step) in intervals.into_iter() {
							if interval.is_empty() {
								continue;
							}

							let z_planes = [
								CollisionPlane(
									Plane3::new(-interval.min, Vector3::new(0.0, 0.0, -1.0)),
									false,
								),
								CollisionPlane(
									Plane3::new(interval.max, Vector3::new(0.0, 0.0, 1.0)),
									false,
								),
							];
							let iter = linedef.collision_planes.iter().chain(z_planes.iter());

							// Non-solid linedefs are only touched
							// if the midpoint of the entity touches
							let bbox = if entity_solid_mask.intersects(solid_mask) {
								entity_bbox
							} else {
								&zero_bbox
							};

							if let Some((fraction, normal)) = trace_planes(bbox, move_step, iter) {
								if entity_solid_mask.intersects(solid_mask) {
									if fraction < trace_fraction
										// Wall takes priority over other vertical surfaces
										|| fraction == trace_fraction && normal[2] == 0.0
									{
										trace_fraction = fraction;
										trace_collision = Some(EntityTraceCollision {
											entity: linedef_dynamic.entity,
											normal,
											step_z: if step
												&& !entity_solid_mask.intersects(linedef.solid_mask)
											{
												Some(interval.max + DISTANCE_EPSILON)
											} else {
												None
											},
										});
										trace_touched.retain(|(f, _)| *f <= fraction);
									}
								} else if fraction <= trace_fraction {
									trace_touched.push((fraction, linedef_dynamic.entity));
								}
							}
						}
					} else if let [Some(front_sidedef), None] = &linedef.sidedefs {
						let front_interval =
							&self.map_dynamic.sectors[front_sidedef.sector_index].interval;
						let z_planes = [
							CollisionPlane(
								Plane3::new(-front_interval.min, Vector3::new(0.0, 0.0, -1.0)),
								false,
							),
							CollisionPlane(
								Plane3::new(front_interval.max, Vector3::new(0.0, 0.0, 1.0)),
								false,
							),
						];
						let iter = linedef.collision_planes.iter().chain(z_planes.iter());

						if let Some((fraction, normal)) =
							trace_planes(&entity_bbox, move_step, iter)
						{
							if entity_solid_mask.intersects(SolidMask::all()) {
								if fraction < trace_fraction
									// Wall takes priority over other vertical surfaces
									|| fraction == trace_fraction && normal[2] == 0.0
								{
									trace_fraction = fraction;
									trace_collision = Some(EntityTraceCollision {
										entity: linedef_dynamic.entity,
										normal,
										step_z: None,
									});
									trace_touched.retain(|(f, _)| *f <= fraction);
								}
							} else if fraction <= trace_fraction {
								trace_touched.push((fraction, linedef_dynamic.entity));
							}
						}
					}
				}

				if let NodeChild::Subsector(subsector_index) = node {
					let subsector = &self.map.subsectors[subsector_index];

					if !move_bbox2.overlaps(&subsector.bbox) {
						return;
					}

					let sector_dynamic = &self.map_dynamic.sectors[subsector.sector_index];

					for (distance, normal) in ArrayVec::from([
						(-sector_dynamic.interval.max, Vector3::new(0.0, 0.0, -1.0)),
						(sector_dynamic.interval.min, Vector3::new(0.0, 0.0, 1.0)),
					])
					.into_iter()
					{
						let z_planes = [
							CollisionPlane(Plane3::new(distance, normal), true),
							CollisionPlane(Plane3::new(-distance, -normal), false),
						];
						let iter = subsector.collision_planes.iter().chain(z_planes.iter());

						if let Some((fraction, normal)) =
							trace_planes(&entity_bbox, move_step, iter)
						{
							if entity_solid_mask.intersects(SolidMask::all()) {
								if fraction < trace_fraction
									// Flat takes priority over other horizontal surfaces
									|| fraction == trace_fraction
										&& normal[0] == 0.0 && normal[1] == 0.0
								{
									trace_fraction = fraction;
									trace_collision = Some(EntityTraceCollision {
										entity: sector_dynamic.entity,
										normal,
										step_z: None,
									});
									trace_touched.retain(|(f, _)| *f <= fraction);
								}
							} else if fraction <= trace_fraction {
								trace_touched.push((fraction, sector_dynamic.entity));
							}
						}
					}
				}
			});

		self.quadtree
			.traverse_nodes(&move_bbox2, &mut |entities: &[Entity]| {
				for &entity in entities {
					let transform = if let Some(val) = self.world.get_component::<Transform>(entity)
					{
						val
					} else {
						continue;
					};

					let box_collider =
						if let Some(val) = self.world.get_component::<BoxCollider>(entity) {
							val
						} else {
							continue;
						};

					let other_bbox =
						AABB3::from_radius_height(box_collider.radius, box_collider.height)
							.offset(transform.position);

					// Don't collide against self
					if entity_bbox == &other_bbox {
						continue;
					}

					if !move_bbox.overlaps(&other_bbox) {
						continue;
					}

					let other_planes = other_bbox
						.planes()
						.iter()
						.map(|p| CollisionPlane(*p, true))
						.collect::<Vec<_>>(); // TODO make this not allocate

					if let Some((fraction, normal)) =
						trace_planes(&entity_bbox, move_step, other_planes.iter())
					{
						if entity_solid_mask.intersects(box_collider.solid_mask) {
							if fraction < trace_fraction {
								trace_fraction = fraction;
								trace_collision = Some(EntityTraceCollision {
									entity,
									normal,
									step_z: Some(other_bbox[2].max + DISTANCE_EPSILON),
								});
								trace_touched.retain(|(f, _)| *f <= fraction);
							}
						} else if fraction <= trace_fraction {
							trace_touched.push((fraction, entity));
						}
					}
				}
			});

		EntityTrace {
			fraction: trace_fraction,
			move_step: move_step * trace_fraction,
			collision: trace_collision,
			touched: trace_touched.into_iter().map(|(_, e)| e).collect(),
		}
	}
}

pub struct SectorTracer<'a, W: EntityStore> {
	pub map: &'a Map,
	pub map_dynamic: &'a MapDynamic,
	pub quadtree: &'a Quadtree,
	pub world: &'a W,
}

#[derive(Clone, Debug)]
pub struct SectorTrace {
	pub fraction: f32,
	pub move_step: f32,
	pub pushed_entities: SmallVec<[SectorTraceEntity; 8]>,
}

#[derive(Clone, Copy, Debug)]
pub struct SectorTraceEntity {
	pub entity: Entity,
	pub move_step: Vector3<f32>,
}

impl<'a, W: EntityStore> SectorTracer<'a, W> {
	pub fn trace<'b>(
		&self,
		distance: f32,
		normal: f32,
		move_step: f32,
		subsectors: impl Iterator<Item = &'b Subsector> + Clone,
	) -> SectorTrace {
		let normal3 = Vector3::new(0.0, 0.0, normal);
		let move_step3 = Vector3::new(0.0, 0.0, move_step);

		let mut trace_fraction = 1.0;
		let mut trace_touched = SmallVec::<[(f32, Entity); 8]>::new();

		let z_planes = [
			CollisionPlane(Plane3::new(distance * normal, normal3), true),
			CollisionPlane(Plane3::new(-distance * normal, -normal3), false),
		];

		let entity_tracer = EntityTracer {
			map: self.map,
			map_dynamic: self.map_dynamic,
			quadtree: self.quadtree,
			world: self.world,
		};

		for (entity, (transform, box_collider)) in
			<(Read<Transform>, Read<BoxCollider>)>::query().iter_entities(self.world)
		{
			let entity_bbox = AABB3::from_radius_height(box_collider.radius, box_collider.height)
				.offset(transform.position);
			let entity_bbox2 = AABB2::from(&entity_bbox);

			for subsector in subsectors
				.clone()
				.filter(|s| entity_bbox2.overlaps(&s.bbox))
			{
				let iter = subsector.collision_planes.iter().chain(z_planes.iter());

				if let Some((hit_fraction, _)) = trace_planes(&entity_bbox, -move_step3, iter) {
					if hit_fraction < 1.0 {
						let remainder = 1.0 - hit_fraction;
						let entity_move_step = remainder * move_step3;

						// TODO solid mask
						let trace = entity_tracer.trace(
							&entity_bbox,
							entity_move_step,
							SolidMask::NON_MONSTER,
						);
						let total_fraction = hit_fraction + remainder * trace.fraction;

						if total_fraction < trace_fraction {
							trace_fraction = total_fraction;
							trace_touched.retain(|(f, _)| *f <= total_fraction);
						}

						if hit_fraction <= total_fraction {
							trace_touched.push((hit_fraction, entity));
						}

						break;
					}
				}
			}
		}

		SectorTrace {
			fraction: trace_fraction,
			move_step: move_step * trace_fraction,
			pushed_entities: trace_touched
				.into_iter()
				.map(|(hit_fraction, entity)| SectorTraceEntity {
					entity,
					move_step: move_step3 * (trace_fraction - hit_fraction),
				})
				.collect(),
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct CollisionPlane(pub Plane3, pub bool);

fn trace_planes<'a>(
	entity_bbox: &AABB3,
	move_step: Vector3<f32>,
	collision_planes: impl IntoIterator<Item = &'a CollisionPlane>,
) -> Option<(f32, Vector3<f32>)> {
	let mut interval = Interval::new(f32::NEG_INFINITY, 1.0);
	let mut ret = None;

	for CollisionPlane(plane, collides) in collision_planes.into_iter() {
		let closest_point =
			entity_bbox
				.vector()
				.zip_map(&plane.normal, |b, n| if n < 0.0 { b.max } else { b.min });
		let start_dist = closest_point.dot(&plane.normal) - plane.distance;
		let move_dist = move_step.dot(&plane.normal);

		if start_dist < 0.0 && start_dist + move_dist < 0.0 {
			continue;
		}

		if move_dist < 0.0 {
			let fraction = (start_dist - DISTANCE_EPSILON) / -move_dist;

			if fraction > interval.min {
				interval.min = fraction;

				if *collides {
					ret = Some((f32::max(0.0, interval.min), plane.normal));
				}
			}
		} else {
			if start_dist > 0.0 {
				return None;
			}

			let fraction = (start_dist + DISTANCE_EPSILON) / -move_dist;

			if fraction < interval.max {
				interval.max = fraction;
			}
		}
	}

	if !interval.is_empty() {
		ret
	} else {
		None
	}
}

lazy_static! {
	static ref BBOX_NORMALS: [Vector3<f32>; 4] = [
		Vector3::new(1.0, 0.0, 0.0),   // right
		Vector3::new(0.0, 1.0, 0.0),   // up
		Vector3::new(-1.0, 0.0, 0.0),  // left
		Vector3::new(0.0, -1.0, 0.0),  // down
	];
}
