use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		audio::Sound,
		frame::FrameState,
		geometry::{Line2, AABB3},
		input::{Bindings, InputState},
		quadtree::Quadtree,
	},
	doom::{
		camera::Camera,
		components::{Transform, Velocity},
		data::{FORWARD_ACCEL, STRAFE_ACCEL},
		door::{DoorSwitchUse, DoorUse},
		floor::FloorSwitchUse,
		input::{BoolInput, FloatInput, UserCommand},
		map::MapDynamic,
		physics::{BoxCollider, EntityTracer, SolidMask},
		plat::PlatSwitchUse,
		state::{State, StateAction, StateName},
	},
};
use legion::{
	component, systems::Runnable, Entity, EntityStore, IntoQuery, Resources, SystemBuilder,
};
use nalgebra::{Vector2, Vector3};
use shrev::EventChannel;

#[derive(Default)]
pub struct Client {
	pub entity: Option<Entity>,
	pub command: UserCommand,
	pub previous_command: UserCommand,
}

pub fn player_command_system() -> impl Runnable {
	SystemBuilder::new("player_command_system")
		.read_resource::<Bindings<BoolInput, FloatInput>>()
		.read_resource::<InputState>()
		.write_resource::<Client>()
		.build(move |_, _, resources, _| {
			let (bindings, input_state, client) = resources;

			let mut command = UserCommand {
				attack: bindings.bool_value(&BoolInput::Attack, &input_state),
				r#use: bindings.bool_value(&BoolInput::Use, &input_state),
				forward: bindings.float_value(&FloatInput::Forward, &input_state) as f32,
				pitch: bindings.float_value(&FloatInput::Pitch, &input_state) as f32,
				strafe: bindings.float_value(&FloatInput::Strafe, &input_state) as f32,
				yaw: bindings.float_value(&FloatInput::Yaw, &input_state) as f32,
			};

			if bindings.bool_value(&BoolInput::Walk, &input_state) {
				command.forward *= 0.5;
				command.strafe *= 0.6;
			}

			client.previous_command = client.command;
			client.command = command;
		})
}

pub fn player_move_system() -> impl Runnable {
	SystemBuilder::new("player_move_system")
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.read_resource::<FrameState>()
		.read_resource::<Quadtree>()
		.with_query(<&mut Transform>::query())
		.with_query(<&MapDynamic>::query())
		.with_query(<(&Transform, &BoxCollider)>::query())
		.with_query(<(&Transform, &mut Velocity)>::query())
		.read_component::<BoxCollider>() // used by EntityTracer
		.read_component::<Transform>() // used by EntityTracer
		.build(move |_, world, resources, queries| {
			let (asset_storage, client, frame_state, quadtree) = resources;

			let client_entity = match client.entity {
				Some(e) => e,
				None => return,
			};

			// Apply rotation
			{
				let transform = queries.0.get_mut(world, client_entity).unwrap();

				transform.rotation[1] += (client.command.pitch * 1e6) as i32;
				transform.rotation[1].0 =
					num_traits::clamp(transform.rotation[1].0, -0x4000_0000, 0x4000_0000);

				transform.rotation[2] -= (client.command.yaw * 1e6) as i32;
			}

			// Apply acceleration
			{
				if client.command.forward == 0.0 && client.command.strafe == 0.0 {
					return;
				}

				let map_dynamic = queries.1.iter(world).next().unwrap();
				let map = asset_storage.get(&map_dynamic.map).unwrap();

				let entity_bbox = {
					let (transform, box_collider) = queries.2.get(world, client_entity).unwrap();
					AABB3::from_radius_height(box_collider.radius, box_collider.height)
						.offset(transform.position)
				};

				let tracer = EntityTracer {
					map,
					map_dynamic,
					quadtree: &quadtree,
					world,
				};

				let trace = tracer.trace(
					&entity_bbox,
					Vector3::new(0.0, 0.0, -0.25),
					SolidMask::NON_MONSTER, // TODO solid mask
				);

				if trace.collision.is_none() {
					// Player is not on ground
					return;
				}

				let move_dir = Vector2::new(
					client.command.forward.max(-1.0).min(1.0) * FORWARD_ACCEL,
					client.command.strafe.max(-1.0).min(1.0) * STRAFE_ACCEL,
				);

				let (transform, velocity) = queries.3.get_mut(world, client_entity).unwrap();

				let angles = Vector3::new(0.into(), 0.into(), transform.rotation[2]);
				let axes = crate::common::geometry::angles_to_axes(angles);
				let accel = (axes[0] * move_dir[0] + axes[1] * move_dir[1])
					* frame_state.delta_time.as_secs_f32();

				velocity.velocity += accel;
			}
		})
}

pub fn player_use_system(resources: &mut Resources) -> impl Runnable {
	resources.insert(EventChannel::<UseEvent>::new());

	SystemBuilder::new("player_use_system")
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.write_resource::<EventChannel<UseEvent>>()
		.write_resource::<Vec<(AssetHandle<Sound>, Entity)>>()
		.with_query(<(&Transform, &User)>::query())
		.with_query(<&MapDynamic>::query())
		.read_component::<UseAction>()
		.build(move |_, world, resources, queries| {
			let (asset_storage, client, use_event_channel, sound_queue) = resources;

			if let Some(entity) = client.entity {
				if client.command.r#use && !client.previous_command.r#use {
					let (transform, user) = queries.0.get(world, entity).unwrap();
					let map_dynamic = queries.1.iter(world).next().unwrap();
					let map = asset_storage.get(&map_dynamic.map).unwrap();

					const USERANGE: f32 = 64.0;
					let yaw = transform.rotation[2].to_radians() as f32;
					let use_line = Line2::new(
						Vector2::new(transform.position[0], transform.position[1]),
						Vector2::new(yaw.cos(), yaw.sin()) * USERANGE,
					);

					// Find the closest linedef hit
					let mut pmax = 1.0;
					let mut closest_linedef = None;

					for (i, linedef) in map.linedefs.iter().enumerate() {
						if let Some((linedef_p, use_p)) = linedef.line.intersect(&use_line) {
							if linedef_p >= 0.0 && linedef_p <= 1.0 && use_p >= 0.0 && use_p < pmax
							{
								// Always hit a usable linedef
								if world
									.entry_ref(map_dynamic.linedefs[i].entity)
									.unwrap()
									.get_component::<UseAction>()
									.is_ok()
								{
									pmax = use_p;
									closest_linedef = Some(i);
								} else if let [Some(_front_sidedef), Some(_back_sidedef)] =
									&linedef.sidedefs
								{
									// Skip two-sided linedefs
								} else {
									pmax = use_p;
									closest_linedef = Some(i);
								}
							}
						}
					}

					// We hit a linedef, use it
					if let Some(linedef_index) = closest_linedef {
						let linedef = &map.linedefs[linedef_index];

						// Used from the back, ignore
						if (use_line.point - linedef.line.point).dot(&linedef.normal) <= 0.0 {
							return;
						}

						let linedef_entity = map_dynamic.linedefs[linedef_index].entity;

						if world
							.entry_ref(linedef_entity)
							.unwrap()
							.get_component::<UseAction>()
							.is_ok()
						{
							use_event_channel.single_write(UseEvent { linedef_entity });
						} else {
							sound_queue.push((user.error_sound.clone(), entity));
						}
					}
				}
			}
		})
}

pub fn player_attack_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("player_attack_system")
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.write_resource::<Quadtree>()
		.with_query(<(&Transform, Option<&Camera>)>::query())
		.with_query(<&MapDynamic>::query())
		.with_query(<&mut State>::query().filter(component::<BoxCollider>()))
		.read_component::<BoxCollider>() // used by EntityTracer
		.read_component::<Transform>() // used by EntityTracer
		.build(move |_command_buffer, world, resources, queries| {
			let (asset_storage, client, quadtree) = resources;

			if let Some(client_entity) = client.entity {
				if client.command.attack && !client.previous_command.attack {
					let (transform, camera) = queries.0.get(world, client_entity).unwrap();
					let map_dynamic = queries.1.iter(world).next().unwrap();
					let map = asset_storage.get(&map_dynamic.map).unwrap();

					let tracer = EntityTracer {
						map,
						map_dynamic,
						quadtree: &quadtree,
						world,
					};

					const ATTACKRANGE: f32 = 2000.0;
					let axes = crate::common::geometry::angles_to_axes(transform.rotation);
					let mut position = transform.position;

					if let Some(camera) = camera {
						position += camera.base + camera.offset;
					}

					let trace = tracer.trace(
						&AABB3::from_point(position),
						axes[0] * ATTACKRANGE,
						SolidMask::all(),
					);

					if let Some(collision) = trace.collision {
						if let Ok(state) = queries.2.get_mut(world, collision.entity) {
							state.action = StateAction::Set((StateName::from("death").unwrap(), 0))
						}
					}
				}
			}
		})
}

#[derive(Clone, Debug)]
pub struct User {
	pub error_sound: AssetHandle<Sound>,
}

#[derive(Clone, Debug)]
pub enum UseAction {
	DoorUse(DoorUse),
	DoorSwitchUse(DoorSwitchUse),
	FloorSwitchUse(FloorSwitchUse),
	PlatSwitchUse(PlatSwitchUse),
}

#[derive(Clone, Copy, Debug)]
pub struct UseEvent {
	pub linedef_entity: Entity,
}
