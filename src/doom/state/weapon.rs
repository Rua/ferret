use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		frame::{FrameRng, FrameState},
		geometry::{Angle, Line3, AABB3},
		quadtree::Quadtree,
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom, SpawnMergerHandlerSet},
		time::Timer,
	},
	doom::{
		camera::{Camera, MovementBob},
		client::Client,
		components::Transform,
		draw::{sprite::SpriteRender, wsprite::WeaponSpriteRender},
		health::Damage,
		map::{LinedefRef, MapDynamic, SectorRef},
		physics::{BoxCollider, DamageParticle, SolidType},
		sound::{Sound, StartSound},
		spawn::{spawn_entity, spawn_helper},
		state::{State, StateAction, StateName, StateSystemsRun},
		template::WeaponTemplate,
		trace::EntityTracer,
	},
};
use legion::{
	component,
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Read, Resources, SystemBuilder, Write,
};
use nalgebra::{Vector2, Vector3};
use num_traits::Zero;
use rand::{distributions::Uniform, Rng};
use std::{sync::atomic::Ordering, time::Duration};

#[derive(Clone, Copy, Debug, Default)]
pub struct WeaponSpriteSlotDef;

impl SpawnFrom<WeaponSpriteSlotDef> for WeaponSpriteSlot {
	fn spawn(
		_component: &WeaponSpriteSlotDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		<Read<SpawnContext<WeaponSpriteSlot>>>::fetch(resources).0
	}
}

#[derive(Clone, Debug)]
pub struct WeaponState {
	pub slots: [State; 2],
	pub current: AssetHandle<WeaponTemplate>,
	pub switch_to: Option<AssetHandle<WeaponTemplate>>,
	pub inaccurate: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum WeaponSpriteSlot {
	Weapon = 0,
	Flash = 1,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WeaponStateDef;

impl SpawnFrom<WeaponStateDef> for WeaponState {
	fn spawn(
		_component: &WeaponStateDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> WeaponState {
		let (asset_storage, frame_state) =
			<(Read<AssetStorage>, Read<FrameState>)>::fetch(resources);

		let current = asset_storage
			.handle_for::<WeaponTemplate>("pistol")
			.unwrap();

		WeaponState {
			slots: [
				State {
					timer: Timer::new_elapsed(frame_state.time, Duration::default()),
					action: StateAction::Set((StateName::from("up").unwrap(), 0)),
				},
				State {
					timer: Timer::new_elapsed(frame_state.time, Duration::default()),
					action: StateAction::None,
				},
			],
			current,
			switch_to: None,
			inaccurate: false,
		}
	}
}

pub fn weapon_state(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_spawn::<WeaponStateDef, WeaponState>();
	handler_set.register_clone::<WeaponSpriteSlot>();
	handler_set.register_spawn::<WeaponSpriteSlotDef, WeaponSpriteSlot>();

	const SLOTS: [WeaponSpriteSlot; 2] = [WeaponSpriteSlot::Weapon, WeaponSpriteSlot::Flash];

	SystemBuilder::new("set_weapon_state")
		.read_resource::<FrameState>()
		.read_resource::<StateSystemsRun>()
		.with_query(<(Entity, &mut WeaponState)>::query())
		.build(move |command_buffer, world, resources, query| {
			let (frame_state, state_systems_run) = resources;

			for (&entity, weapon_state) in query.iter_mut(world) {
				for slot in SLOTS.iter().copied() {
					let state = &mut weapon_state.slots[slot as usize];

					if let StateAction::Wait(state_name) = state.action {
						if state.timer.is_elapsed(frame_state.time) {
							state.action = StateAction::Set(state_name);
						}
					}

					if let StateAction::Set(state_name) = state.action {
						state_systems_run.0.store(true, Ordering::Relaxed);
						state.action = StateAction::None;
						let handle = weapon_state.current.clone();

						command_buffer.exec_mut(move |world, resources| {
							resources.insert(SpawnContext(entity));
							resources.insert(SpawnContext(slot));
							let asset_storage = <Read<AssetStorage>>::fetch(resources);
							let state_world = &asset_storage
								.get(&handle)
								.unwrap()
								.states
								.get(&state_name.0)
								.and_then(|x| x.get(state_name.1))
								.unwrap_or_else(|| panic!("Invalid state {:?}", state_name));

							spawn_helper(&state_world, world, resources);
						});
					}
				}
			}

			command_buffer.exec_mut(move |_world, resources| {
				resources.remove::<SpawnContext<Entity>>();
				resources.remove::<SpawnContext<WeaponSpriteSlot>>();
			});
		})
}

#[derive(Clone, Copy, Debug)]
pub struct NextWeaponState {
	pub time: Duration,
	pub state: (StateName, usize),
}

pub fn next_weapon_state(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<NextWeaponState>();

	SystemBuilder::new("next_weapon_state")
		.read_resource::<FrameState>()
		.with_query(<(Entity, &Entity, &WeaponSpriteSlot, &NextWeaponState)>::query())
		.with_query(<&mut WeaponState>::query())
		.build(move |command_buffer, world, resources, queries| {
			let frame_state = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, &slot, next_state) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(weapon_state) = queries.1.get_mut(&mut world, target) {
					let state = &mut weapon_state.slots[slot as usize];

					if let StateAction::None = state.action {
						state.timer.restart_with(frame_state.time, next_state.time);
						state.action = StateAction::Wait(next_state.state);
					}
				}
			}
		})
}

#[derive(Clone, Debug)]
pub struct LineAttack {
	pub count: usize,
	pub damage_range: Uniform<u32>,
	pub damage_multiplier: f32,
	pub distance: f32,
	pub spread: Vector2<Angle>,
	pub accurate_until_refire: bool,
	pub sparks: bool,
	pub hit_sound: Option<AssetHandle<Sound>>,
	pub miss_sound: Option<AssetHandle<Sound>>,
}

pub fn line_attack(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<LineAttack>();

	SystemBuilder::new("line_attack")
		.read_resource::<AssetStorage>()
		.read_resource::<Quadtree>()
		.with_query(<(Entity, &Entity, &LineAttack)>::query())
		.with_query(<(Option<&BoxCollider>, &Transform, &WeaponState)>::query())
		.with_query(<&mut FrameRng>::query())
		.with_query(<&MapDynamic>::query())
		.with_query(<(
			Option<&BoxCollider>,
			Option<&LinedefRef>,
			Option<&SectorRef>,
		)>::query())
		.read_component::<BoxCollider>() // used by EntityTracer
		.read_component::<Transform>() // used by EntityTracer
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, quadtree) = resources;
			let (mut world1, world) = world.split_for_query(&queries.2);

			for (&entity, &target, line_attack) in queries.0.iter(&world) {
				command_buffer.remove(entity);

				if let (Ok((box_collider, transform, weapon_state)), Ok(frame_rng)) = (
					queries.1.get(&world, target),
					queries.2.get_mut(&mut world1, target),
				) {
					let map_dynamic = queries.3.iter(&world).next().unwrap();
					let map = asset_storage.get(&map_dynamic.map).unwrap();

					let tracer = EntityTracer {
						map,
						map_dynamic,
						quadtree: &quadtree,
						world: &world,
					};

					let mut position = transform.position;

					if let Some(box_collider) = box_collider {
						position[2] += box_collider.height * 0.5 + 8.0;
					}

					for _ in 0..line_attack.count {
						let mut rotation = transform.rotation;

						// Apply spread if the weapon is shooting inaccurately.
						// Subtracting two uniform random numbers results in a triangle distribution.
						if !line_attack.accurate_until_refire || weapon_state.inaccurate {
							if !line_attack.spread[0].is_zero() {
								rotation[2] +=
									frame_rng.gen_range(0, line_attack.spread[0].0) -
									frame_rng.gen_range(0, line_attack.spread[0].0);
							}

							if !line_attack.spread[1].is_zero() {
								rotation[1] +=
									frame_rng.gen_range(0, line_attack.spread[1].0) -
									frame_rng.gen_range(0, line_attack.spread[1].0);
							}
						}

						let direction = crate::common::geometry::angles_to_axes(rotation)[0];
						let trace = tracer.trace(
							&AABB3::from_point(position),
							direction * line_attack.distance,
							SolidType::PROJECTILE,
						);

						// Hit something!
						if let Some(collision) = trace.collision {
							// Apply the damage
							let damage = line_attack.damage_multiplier
								* frame_rng.sample(line_attack.damage_range) as f32;

							command_buffer.push((
								collision.entity,
								Damage {
									amount: damage,
									source_entity: target,
									line: Line3::new(position, trace.move_step),
								},
							));

							// Spawn particles
							let mut particle_transform = Transform {
								position: position + trace.move_step,
								rotation: Vector3::zeros(),
							};

							let particle = match queries.4.get(&world, collision.entity) {
								Ok((Some(box_collider), None, None)) => {
									// Hit a mobj
									particle_transform.position -= direction * 10.0;
									box_collider.damage_particle
								}
								Ok((None, Some(_linedef_ref), None)) => {
									// Hit a linedef
									// TODO test for sky
									particle_transform.position -= direction * 4.0;
									DamageParticle::Puff
								}
								Ok((None, None, Some(_sector_ref))) => {
									// Hit a sector
									// TODO test for sky
									particle_transform.position -= direction * 4.0;
									DamageParticle::Puff
								}
								_ => {
									log::warn!("Collision entity {:?} does not have exactly one of BoxCollider, LinedefRef, SectorRef", collision.entity);
									continue
								}
							};
							let template_name = match particle {
								DamageParticle::Blood => {
									if damage <= 9.0 {
										"blood3"
									} else if damage <= 12.0 {
										"blood2"
									} else {
										"blood1"
									}
								}
								DamageParticle::Puff => {
									if line_attack.sparks {
										"puff1"
									} else {
										"puff3"
									}
								}
							};
							let handle = asset_storage
								.handle_for(template_name)
								.expect("Damage particle template is not present");

							command_buffer.exec_mut(move |world, resources| {
								spawn_entity(world, resources, &handle, particle_transform);
							});

							// Play hit sound if present
							if let Some(sound) = line_attack.hit_sound.as_ref() {
								command_buffer.push((target, StartSound(sound.clone())));
							}
						} else {
							// Play miss sound if present
							if let Some(sound) = line_attack.miss_sound.as_ref() {
								command_buffer.push((target, StartSound(sound.clone())));
							}
						}
					}
				}
			}
		})
}

#[derive(Clone, Debug)]
pub struct SetWeaponSprite(pub Option<SpriteRender>);

pub fn set_weapon_sprite(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<SetWeaponSprite>();

	SystemBuilder::new("set_weapon_sprite")
		.with_query(<(Entity, &Entity, &WeaponSpriteSlot, &SetWeaponSprite)>::query())
		.with_query(<&mut WeaponSpriteRender>::query().filter(component::<WeaponState>()))
		.build(move |command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, &slot, SetWeaponSprite(sprite)) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(weapon_sprite_render) = queries.1.get_mut(&mut world, target) {
					weapon_sprite_render.slots[slot as usize] = sprite.clone();
				}
			}
		})
}

#[derive(Clone, Copy, Debug)]
pub struct SetWeaponState(pub (StateName, usize));

pub fn set_weapon_state(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<SetWeaponState>();

	SystemBuilder::new("set_weapon_state")
		.with_query(<(Entity, &Entity, &WeaponSpriteSlot, &SetWeaponState)>::query())
		.with_query(<&mut WeaponState>::query())
		.build(move |command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, &slot, &SetWeaponState(next_state)) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(weapon_state) = queries.1.get_mut(&mut world, target) {
					let state = &mut weapon_state.slots[slot as usize];
					state.action = StateAction::Set(next_state);
				}
			}
		})
}

#[derive(Clone, Copy, Debug)]
pub enum WeaponPosition {
	Bob,
	Down,
	Up,
}

pub fn weapon_position(resources: &mut Resources) -> impl Runnable {
	const DOWN_SPEED: f32 = 6.0;
	const UP_SPEED: f32 = -6.0;

	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<WeaponPosition>();

	SystemBuilder::new("weapon_position")
		.read_resource::<FrameState>()
		.with_query(<(Entity, &Entity, &WeaponSpriteSlot, &WeaponPosition)>::query())
		.with_query(
			<(
				&Camera,
				&MovementBob,
				&mut WeaponState,
				&mut WeaponSpriteRender,
			)>::query()
			.filter(component::<WeaponState>()),
		)
		.build(move |command_buffer, world, resources, queries| {
			let frame_state = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, &slot, weapon_position) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok((camera, movement_bob, weapon_state, weapon_sprite_render)) =
					queries.1.get_mut(&mut world, target)
				{
					let state = &mut weapon_state.slots[slot as usize];

					match weapon_position {
						WeaponPosition::Bob => {
							let mut angle = Angle::from_units(
								frame_state.time.as_secs_f64()
									/ camera.weapon_bob_period.as_secs_f64(),
							);
							weapon_sprite_render.position[0] =
								movement_bob.amplitude * angle.cos() as f32;

							angle.0 &= 0x7FFF_FFFF;
							weapon_sprite_render.position[1] =
								movement_bob.amplitude * angle.sin() as f32;
						}
						WeaponPosition::Down => {
							weapon_sprite_render.position[1] += DOWN_SPEED;

							if weapon_sprite_render.position[1] >= 96.0 {
								weapon_sprite_render.position[1] = 96.0;

								if let Some(switch_to) = weapon_state.switch_to.take() {
									let state_name = (StateName::from("up").unwrap(), 0);
									weapon_state.slots[slot as usize].action =
										StateAction::Set(state_name);
									weapon_state.current = switch_to;
								}
							}
						}
						WeaponPosition::Up => {
							weapon_sprite_render.position[1] += UP_SPEED;

							if weapon_sprite_render.position[1] <= 0.0 {
								weapon_sprite_render.position[1] = 0.0;
								let state_name = (StateName::from("ready").unwrap(), 0);
								state.action = StateAction::Set(state_name);
							}
						}
					}
				}
			}
		})
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WeaponReady;

pub fn weapon_ready(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<WeaponReady>();

	SystemBuilder::new("weapon_ready")
		.read_resource::<Client>()
		.with_query(<(Entity, &Entity, &WeaponSpriteSlot, &WeaponReady)>::query())
		.with_query(<&mut WeaponState>::query().filter(component::<WeaponState>()))
		.build(move |command_buffer, world, resources, queries| {
			let client = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, &slot, WeaponReady) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(weapon_state) = queries.1.get_mut(&mut world, target) {
					let state = &mut weapon_state.slots[slot as usize];

					if weapon_state.switch_to.is_some() {
						let state_name = (StateName::from("down").unwrap(), 0);
						state.action = StateAction::Set(state_name);
					} else if client.command.attack {
						let state_name = (StateName::from("attack").unwrap(), 0);
						state.action = StateAction::Set(state_name);
					}
				}
			}
		})
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WeaponReFire;

pub fn weapon_refire(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<WeaponReFire>();

	SystemBuilder::new("weapon_refire")
		.read_resource::<Client>()
		.with_query(<(Entity, &Entity, &WeaponSpriteSlot, &WeaponReFire)>::query())
		.with_query(<&mut WeaponState>::query().filter(component::<WeaponState>()))
		.build(move |command_buffer, world, resources, queries| {
			let client = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&entity, &target, &slot, WeaponReFire) in queries.0.iter(&world0) {
				command_buffer.remove(entity);

				if let Ok(weapon_state) = queries.1.get_mut(&mut world, target) {
					let state = &mut weapon_state.slots[slot as usize];

					if client.command.attack {
						let state_name = (StateName::from("attack").unwrap(), 0);
						state.action = StateAction::Set(state_name);
						weapon_state.inaccurate = true;
					} else {
						weapon_state.inaccurate = false;
					}
				}
			}
		})
}
