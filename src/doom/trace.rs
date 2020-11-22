use crate::{
	common::{
		geometry::{Interval, Plane3, AABB2, AABB3},
		quadtree::Quadtree,
	},
	doom::{
		components::Transform,
		map::{Map, MapDynamic, NodeChild, Subsector},
		physics::{BoxCollider, SolidBits, SolidType, DISTANCE_EPSILON},
	},
};
use arrayvec::ArrayVec;

use lazy_static::lazy_static;
use legion::{Entity, EntityStore, IntoQuery};
use nalgebra::Vector3;
use smallvec::SmallVec;

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

const EXTRA_HEADROOM: f32 = 0.1;

impl<'a, W: EntityStore> EntityTracer<'a, W> {
	pub fn trace(
		&self,
		entity_bbox: &AABB3,
		move_step: Vector3<f32>,
		solid_type: SolidType,
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
								SolidBits::all(),
								true,
							),
							(
								Interval::new(intersection.min, intersection.max + EXTRA_HEADROOM),
								linedef.blocks_types,
								false,
							),
							(
								Interval::new(intersection.max + EXTRA_HEADROOM, union.max),
								SolidBits::all(),
								false,
							),
						]);

						for (interval, blocks_types, step) in intervals.into_iter() {
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
							let bbox = if blocks_types.blocks(solid_type) {
								entity_bbox
							} else {
								&zero_bbox
							};

							if let Some((fraction, normal)) = trace_planes(bbox, move_step, iter) {
								if blocks_types.blocks(solid_type) {
									if fraction < trace_fraction
										// Wall takes priority over other vertical surfaces
										|| fraction == trace_fraction && normal[2] == 0.0
									{
										trace_fraction = fraction;
										trace_collision = Some(EntityTraceCollision {
											entity: linedef_dynamic.entity,
											normal,
											step_z: if step
												&& !linedef.blocks_types.blocks(solid_type)
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
								Plane3::new(
									front_interval.max + EXTRA_HEADROOM,
									Vector3::new(0.0, 0.0, 1.0),
								),
								false,
							),
						];
						let iter = linedef.collision_planes.iter().chain(z_planes.iter());

						if let Some((fraction, normal)) =
							trace_planes(&entity_bbox, move_step, iter)
						{
							if SolidBits::all().blocks(solid_type) {
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
						(
							-(sector_dynamic.interval.max + EXTRA_HEADROOM),
							Vector3::new(0.0, 0.0, -1.0),
						),
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
							if SolidBits::all().blocks(solid_type) {
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
					let (transform, box_collider) =
						match <(&Transform, &BoxCollider)>::query().get(self.world, entity) {
							Ok(x) => x,
							_ => continue,
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
						if box_collider.blocks_types.blocks(solid_type) {
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
		mut distance: f32,
		normal: f32,
		move_step: f32,
		subsectors: impl Iterator<Item = &'b Subsector> + Clone,
	) -> SectorTrace {
		if normal < 0.0 {
			distance -= EXTRA_HEADROOM;
		}

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

		for (&entity, transform, box_collider) in
			<(Entity, &Transform, &BoxCollider)>::query().iter(self.world)
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

						let trace = entity_tracer.trace(
							&entity_bbox,
							entity_move_step,
							box_collider.solid_type,
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
