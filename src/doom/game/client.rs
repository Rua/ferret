use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		geometry::{Line2, Line3, AABB3},
		input::InputState,
		quadtree::Quadtree,
		spawn::{spawn_helper, ComponentAccessor, SpawnContext, SpawnFrom, SpawnMergerHandlerSet},
		time::DeltaTime,
	},
	doom::{
		assets::{
			sound::Sound,
			template::{EntityTemplateRef, WeaponTemplate},
		},
		data::{FORWARD_ACCEL, FRAME_RATE, STRAFE_ACCEL},
		game::{
			camera::Camera,
			combat::{weapon::WeaponState, Owner},
			map::MapDynamic,
			physics::{BoxCollider, Physics, TouchEvent},
			trace::EntityTracer,
			Transform,
		},
		input::UserCommand,
		sound::StartSoundEvent,
	},
};
use legion::{
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Read, Registry, Resources, SystemBuilder, Write,
};
use nalgebra::{Vector2, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Client {
	pub entity: Option<Entity>,
	pub command: UserCommand,
	pub previous_command: UserCommand,
}

fn select_weapon<'a>(
	possible: &'a [&str],
	asset_storage: &AssetStorage,
	weapon_state: &WeaponState,
) -> Option<&'a str> {
	// Find the first weapon that is in the inventory, if any
	let first: Option<&str> = possible.iter().copied().find(|name| {
		let asset_name = format!("{}.weapon", name);

		let handle = match asset_storage.handle_for::<WeaponTemplate>(&asset_name) {
			Some(handle) => handle,
			None => return false,
		};

		weapon_state.inventory.contains(&handle)
	});

	// Find the weapon after the current one, if any
	let next: Option<&str> = possible
		.iter()
		.copied()
		.skip_while(|name| {
			let asset_name = format!("{}.weapon", name);

			let handle = match asset_storage.handle_for::<WeaponTemplate>(&asset_name) {
				Some(handle) => handle,
				None => return false,
			};

			weapon_state.current != handle
		})
		.nth(1);

	next.or(first)
}

pub fn player_command(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("player_command")
		.read_resource::<AssetStorage>()
		.read_resource::<InputState>()
		.write_resource::<Client>()
		.with_query(<&WeaponState>::query())
		.build(move |_command_buffer, world, resources, query| {
			let (asset_storage, input_state, client) = resources;

			let weapon: Option<&str> = client.entity.and_then(|entity| {
				let weapon_keys = [
					input_state.bool_value("weapon1"),
					input_state.bool_value("weapon2"),
					input_state.bool_value("weapon3"),
					input_state.bool_value("weapon4"),
					input_state.bool_value("weapon5"),
					input_state.bool_value("weapon6"),
					input_state.bool_value("weapon7"),
				];
				let mut iter =
					weapon_keys
						.iter()
						.enumerate()
						.filter_map(|(i, x)| if *x { Some(i + 1) } else { None });

				// Do not register a button press if more than one weapon key is pressed at a time
				let weapon_index: Option<usize> = match (iter.next(), iter.next()) {
					(Some(i), None) => Some(i),
					_ => None,
				};

				weapon_index
					.map(|i| match i {
						1 => &["chainsaw", "fist"] as &[&str],
						2 => &["pistol"],
						3 => &["supershotgun", "shotgun"],
						4 => &["chaingun"],
						5 => &["missile"],
						6 => &["plasma"],
						7 => &["bfg"],
						_ => unreachable!(),
					})
					.and_then(|possible| {
						let weapon_state = query
							.get(world, entity)
							.expect("Client entity does not have WeaponState");

						select_weapon(possible, asset_storage, weapon_state)
					})
			});

			let mut command = UserCommand {
				attack: input_state.bool_value("attack"),
				weapon: weapon.map(|x| x.to_owned()),
				r#use: input_state.bool_value("use"),
				forward: input_state.float_value("forward") as f32,
				pitch: input_state.float_value("pitch") as f32,
				strafe: input_state.float_value("strafe") as f32,
				yaw: input_state.float_value("yaw") as f32,
			};

			if input_state.bool_value("walk") {
				command.forward *= 0.5;
				command.strafe *= 0.6;
			}

			let client: &mut Client = &mut *client; // This prevents borrow errors
			std::mem::swap(&mut client.previous_command, &mut client.command);
			client.command = command;
		})
}

pub fn player_move(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("player_move")
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.read_resource::<DeltaTime>()
		.read_resource::<Quadtree>()
		.with_query(<&mut Transform>::query())
		.with_query(<&MapDynamic>::query())
		.with_query(<(&BoxCollider, Option<&Owner>, &Transform)>::query())
		.with_query(<(&Transform, &mut Physics)>::query())
		.read_component::<BoxCollider>() // used by EntityTracer
		.read_component::<Owner>() // used by EntityTracer
		.read_component::<Transform>() // used by EntityTracer
		.build(move |_command_buffer, world, resources, queries| {
			let (asset_storage, client, delta_time, quadtree) = resources;

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

				let (bbox, ignore, solid_type, position) = {
					let (box_collider, owner, transform) =
						queries.2.get(world, client_entity).unwrap();
					(
						AABB3::from_radius_height(box_collider.radius, box_collider.height),
						Some(owner.map_or(client_entity, |&Owner(owner)| owner)),
						box_collider.solid_type,
						transform.position,
					)
				};

				let tracer = EntityTracer {
					map,
					map_dynamic,
					quadtree: &quadtree,
					world,
				};

				let trace = tracer.trace(
					&bbox,
					solid_type,
					ignore,
					Line3::new(position, Vector3::new(0.0, 0.0, -0.25)),
				);

				if trace.collision.is_none() {
					// Player is not on ground
					return;
				}

				let move_dir = Vector2::new(
					client.command.forward.max(-1.0).min(1.0) * FORWARD_ACCEL,
					client.command.strafe.max(-1.0).min(1.0) * STRAFE_ACCEL,
				);

				let (transform, physics) = queries.3.get_mut(world, client_entity).unwrap();

				let angles = Vector3::new(0.into(), 0.into(), transform.rotation[2]);
				let axes = crate::common::geometry::angles_to_axes(angles);
				let accel =
					(axes[0] * move_dir[0] + axes[1] * move_dir[1]) * delta_time.0.as_secs_f32();

				physics.velocity += accel;
			}
		})
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
	pub error_sound: AssetHandle<Sound>,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Usable;

#[derive(Clone, Copy, Debug)]
pub struct UseEvent {
	pub entity: Entity,
	pub other: Entity,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct UseEventDef;

impl SpawnFrom<UseEventDef> for UseEvent {
	fn spawn(
		_component: &UseEventDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		<Read<SpawnContext<UseEvent>>>::fetch(resources).0
	}
}

pub fn player_use(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<User>("User".into());
	handler_set.register_clone::<User>();

	registry.register::<Usable>("Usable".into());
	handler_set.register_clone::<Usable>();
	handler_set.register_spawn::<UseEventDef, UseEvent>();

	SystemBuilder::new("player_use")
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.with_query(<(&Transform, &User)>::query())
		.with_query(<&MapDynamic>::query())
		.with_query(<(&EntityTemplateRef, &Usable)>::query())
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, client) = resources;

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
					// TODO use a trace here
					let mut pmax = 1.0;
					let mut closest_linedef = None;

					for (i, linedef) in map.linedefs.iter().enumerate() {
						if let Some((linedef_p, use_p)) = linedef.line.intersect(&use_line) {
							if linedef_p >= 0.0 && linedef_p <= 1.0 && use_p >= 0.0 && use_p < pmax
							{
								// Always hit a usable linedef
								if let Ok((_, Usable)) =
									queries.2.get(world, map_dynamic.linedefs[i].entity)
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

						if let Ok((template_ref, Usable)) = queries.2.get(world, linedef_entity) {
							let event = UseEvent {
								entity: linedef_entity,
								other: entity,
							};
							let handle = template_ref.0.clone();
							command_buffer.exec_mut(move |world, resources| {
								resources.insert(SpawnContext(event));
								let asset_storage = <Read<AssetStorage>>::fetch(resources);
								let use_world = &asset_storage.get(&handle).unwrap().r#use;
								spawn_helper(&use_world, world, resources);
							});
						} else {
							command_buffer.push((StartSoundEvent {
								handle: user.error_sound.clone(),
								entity: Some(entity),
							},));
						}
					}
				}
			}
		})
}

pub fn player_weapon(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("player_weapon")
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.with_query(<&mut WeaponState>::query())
		.build(move |_command_buffer, world, resources, query| {
			let (asset_storage, client) = resources;

			if let Some(weapon_state) = client
				.entity
				.and_then(|entity| query.get_mut(world, entity).ok())
			{
				if let Some(switch_to) = client
					.command
					.weapon
					.as_ref()
					.and_then(|name| {
						let asset_name = format!("{}.weapon", name);
						asset_storage.handle_for::<WeaponTemplate>(&asset_name)
					})
					.as_ref()
					.filter(|&handle| *handle != weapon_state.current)
					.map(Clone::clone)
				{
					weapon_state.switch_to = Some(switch_to);
				}
			}
		})
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerTouch;

pub fn player_touch(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<PlayerTouch>();

	SystemBuilder::new("player_touch")
		.with_query(<(&TouchEvent, &PlayerTouch)>::query())
		.with_query(<&mut Camera>::query())
		.build(move |command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, PlayerTouch) in queries.0.iter(&world0) {
				// Shift the camera downwards if hitting the ground
				if let (Ok(camera), Some(collision)) =
					(queries.1.get_mut(&mut world, event.entity), event.collision)
				{
					let speed = -collision.velocity.dot(&collision.normal);
					let down_speed = collision.normal[2] * speed;

					if down_speed >= 8.0 * FRAME_RATE {
						camera.deviation_velocity = -down_speed / 8.0;
						command_buffer.push((StartSoundEvent {
							handle: camera.impact_sound.clone(),
							entity: Some(event.entity),
						},));
					}
				}
			}
		})
}
