use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		frame::FrameState,
		quadtree::Quadtree,
		time::Timer,
	},
	doom::{
		components::Transform,
		map::{MapDynamic, SectorRef},
		physics::{BoxCollider, SectorTracer},
		sound::{Sound, StartSound},
	},
};
use legion::{systems::Runnable, world::SubWorld, Entity, IntoQuery, Resources, SystemBuilder};
use shrev::EventChannel;

#[derive(Clone, Debug)]
pub struct FloorMove(pub SectorMove);

#[derive(Clone, Debug)]
pub struct CeilingMove(pub SectorMove);

#[derive(Clone, Debug)]
pub struct SectorMove {
	pub velocity: f32,
	pub target: f32,
	pub sound: Option<AssetHandle<Sound>>,
	pub sound_timer: Timer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectorMoveEvent {
	pub event_type: SectorMoveEventType,
	pub entity: Entity,
	pub normal: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectorMoveEventType {
	Collided,
	TargetReached,
}

pub fn sector_move_system(resources: &mut Resources) -> impl Runnable {
	resources.insert(EventChannel::<SectorMoveEvent>::new());

	SystemBuilder::new("sector_move_system")
		.read_resource::<AssetStorage>()
		.read_resource::<FrameState>()
		.read_resource::<Quadtree>()
		.write_resource::<EventChannel<SectorMoveEvent>>()
		.with_query(<&mut MapDynamic>::query())
		.with_query(<&mut Transform>::query())
		.with_query(<(Entity, &SectorRef, &mut FloorMove)>::query())
		.with_query(<(Entity, &SectorRef, &mut CeilingMove)>::query())
		.read_component::<BoxCollider>() // used by SectorTracer
		.read_component::<Transform>() // used by SectorTracer
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, frame_state, quadtree, sector_move_event_channel) = resources;

			// TODO check if this is still needed with new Rust versions
			let query0 = &mut queries.0;
			let query1 = &mut queries.1;
			let (mut world0, mut world) = world.split_for_query(query0);

			let mut do_move = |entity: Entity,
			                   sector_ref: &SectorRef,
			                   sector_move: &mut SectorMove,
			                   normal: f32,
			                   world: &mut SubWorld| {
				debug_assert!(normal == 1.0 || normal == -1.0);

				let map_dynamic = query0.get_mut(&mut world0, sector_ref.map_entity).unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();
				let sector = &map.sectors[sector_ref.index];
				let mut event_type = None;

				if sector_move.sound_timer.is_elapsed(frame_state.time)
					&& sector_move.sound.is_some()
				{
					sector_move.sound_timer.restart(frame_state.time);
					command_buffer.push((StartSound {
						entity,
						sound: sector_move.sound.as_ref().unwrap().clone(),
					},));
				}

				let mut move_step = sector_move.velocity * frame_state.delta_time.as_secs_f32();

				let current_height = if normal == 1.0 {
					map_dynamic.sectors[sector_ref.index].interval.min
				} else {
					map_dynamic.sectors[sector_ref.index].interval.max
				};

				let distance_left = sector_move.target - current_height;

				if move_step < 0.0 {
					if move_step <= distance_left {
						move_step = distance_left;
						event_type = Some(SectorMoveEventType::TargetReached);
					}
				} else if move_step > 0.0 {
					if move_step >= distance_left {
						move_step = distance_left;
						event_type = Some(SectorMoveEventType::TargetReached);
					}
				}

				let tracer = SectorTracer {
					map,
					map_dynamic: &map_dynamic,
					quadtree,
					world,
				};

				let trace = tracer.trace(
					current_height,
					normal,
					move_step,
					sector.subsectors.iter().map(|i| &map.subsectors[*i]),
				);

				// Push the entities out of the way
				for pushed_entity in trace.pushed_entities.iter() {
					let transform = query1.get_mut(world, pushed_entity.entity).unwrap();
					transform.position += pushed_entity.move_step;
				}

				// Move the plat into place
				let current_height = if normal == 1.0 {
					&mut map_dynamic.sectors[sector_ref.index].interval.min
				} else {
					&mut map_dynamic.sectors[sector_ref.index].interval.max
				};

				*current_height += trace.move_step;

				if trace.fraction < 1.0 {
					event_type = Some(SectorMoveEventType::Collided);
				} else if event_type == Some(SectorMoveEventType::TargetReached) {
					// Set this explicitly to the exact value
					*current_height = sector_move.target;
				}

				if let Some(event_type) = event_type {
					sector_move_event_channel.single_write(SectorMoveEvent {
						entity,
						event_type,
						normal,
					});
				}
			};

			{
				let (mut world2, mut world) = world.split_for_query(&queries.2);

				for (entity, sector_ref, floor_move) in queries.2.iter_mut(&mut world2) {
					let sector_move = &mut floor_move.0;

					if sector_move.velocity == 0.0 {
						continue;
					}

					do_move(*entity, &sector_ref, sector_move, 1.0, &mut world);
				}
			}

			{
				let (mut world3, mut world) = world.split_for_query(&queries.3);

				for (entity, sector_ref, ceiling_move) in queries.3.iter_mut(&mut world3) {
					let sector_move = &mut ceiling_move.0;

					if sector_move.velocity == 0.0 {
						continue;
					}

					do_move(*entity, &sector_ref, sector_move, -1.0, &mut world);
				}
			}
		})
}
