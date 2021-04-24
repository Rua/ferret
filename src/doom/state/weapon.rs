use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		geometry::{angles_to_axes, Angle, Line2, Line3, AABB2, AABB3},
		quadtree::Quadtree,
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom, SpawnMergerHandlerSet},
		time::{GameTime, Timer},
	},
	doom::{
		camera::{Camera, MovementBob},
		client::Client,
		components::Transform,
		draw::{sprite::SpriteRender, wsprite::WeaponSpriteRender},
		health::{DamageEvent, Health},
		map::{LinedefRef, MapDynamic, SectorRef},
		physics::{BoxCollider, DamageParticle, SolidType, TouchEvent},
		sound::{Sound, StartSoundEvent, StartSoundEventEntity},
		spawn::{spawn_entity, spawn_helper},
		state::{entity::EntityStateEvent, State, StateAction, StateName, StateSystemsRun},
		template::{AmmoTemplate, EntityTemplate, WeaponTemplate},
		trace::EntityTracer,
	},
};
use legion::{
	component,
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Read, Registry, Resources, SystemBuilder, Write,
};
use nalgebra::{Vector2, Vector3};
use num_traits::Zero;
use rand::{distributions::Uniform, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{
	collections::{HashMap, HashSet},
	sync::atomic::Ordering,
	time::Duration,
};

#[derive(Clone, Copy, Debug)]
pub struct WeaponStateEvent {
	entity: Entity,
	slot: WeaponSpriteSlot,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WeaponStateEventDef;

impl SpawnFrom<WeaponStateEventDef> for WeaponStateEvent {
	fn spawn(
		_component: &WeaponStateEventDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		<Read<SpawnContext<WeaponStateEvent>>>::fetch(resources).0
	}
}

#[derive(Clone, Copy, Debug)]
pub struct WeaponStateEventDefSlot(pub WeaponSpriteSlot);

impl SpawnFrom<WeaponStateEventDefSlot> for WeaponStateEvent {
	fn spawn(
		component: &WeaponStateEventDefSlot,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		WeaponStateEvent {
			slot: component.0,
			..<Read<SpawnContext<WeaponStateEvent>>>::fetch(resources).0
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeaponState {
	pub slots: [State; 2],
	pub current: AssetHandle<WeaponTemplate>,
	pub switch_to: Option<AssetHandle<WeaponTemplate>>,
	pub inventory: HashSet<AssetHandle<WeaponTemplate>>,
	pub ammo: HashMap<AssetHandle<AmmoTemplate>, AmmoState>,
	pub inaccurate: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum WeaponSpriteSlot {
	Weapon = 0,
	Flash = 1,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct AmmoState {
	pub current: i32,
	pub max: i32,
}

impl WeaponState {
	fn can_fire(&self, asset_storage: &AssetStorage) -> bool {
		asset_storage
			.get(&self.current)
			.unwrap()
			.ammo
			.as_ref()
			.map_or(true, |weapon_ammo| {
				self.ammo
					.get(&weapon_ammo.handle)
					.map_or(false, |ammo_state| ammo_state.current >= weapon_ammo.count)
			})
	}
}

#[derive(Clone, Debug)]
pub struct WeaponStateDef {
	pub current: AssetHandle<WeaponTemplate>,
	pub inventory: HashSet<AssetHandle<WeaponTemplate>>,
	pub ammo: HashMap<AssetHandle<AmmoTemplate>, AmmoState>,
}

impl SpawnFrom<WeaponStateDef> for WeaponState {
	fn spawn(
		component: &WeaponStateDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> WeaponState {
		let game_time = <Read<GameTime>>::fetch(resources);

		WeaponState {
			slots: [
				State {
					timer: Timer::new_elapsed(*game_time, Duration::default()),
					action: StateAction::Set((StateName::from("up").unwrap(), 0)),
				},
				State {
					timer: Timer::new_elapsed(*game_time, Duration::default()),
					action: StateAction::None,
				},
			],
			current: component.current.clone(),
			switch_to: None,
			inventory: component.inventory.clone(),
			ammo: component.ammo.clone(),
			inaccurate: false,
		}
	}
}

pub fn weapon_state(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<WeaponState>("WeaponState".into());
	handler_set.register_spawn::<WeaponStateDef, WeaponState>();

	handler_set.register_spawn::<WeaponStateEventDef, WeaponStateEvent>();
	handler_set.register_spawn::<WeaponStateEventDefSlot, WeaponStateEvent>();

	const SLOTS: [WeaponSpriteSlot; 2] = [WeaponSpriteSlot::Weapon, WeaponSpriteSlot::Flash];

	SystemBuilder::new("weapon_state")
		.read_resource::<GameTime>()
		.read_resource::<StateSystemsRun>()
		.with_query(<(Entity, &mut WeaponState)>::query())
		.build(move |command_buffer, world, resources, query| {
			let (game_time, state_systems_run) = resources;

			for (&entity, weapon_state) in query.iter_mut(world) {
				for slot in SLOTS.iter().copied() {
					let state = &mut weapon_state.slots[slot as usize];

					if let StateAction::Wait(state_name) = state.action {
						if state.timer.is_elapsed(**game_time) {
							state.action = StateAction::Set(state_name);
						}
					}

					if let StateAction::Set(state_name) = state.action {
						state_systems_run.0.store(true, Ordering::Relaxed);
						state.action = StateAction::None;
						let handle = weapon_state.current.clone();

						command_buffer.exec_mut(move |world, resources| {
							resources.insert(SpawnContext(WeaponStateEvent { entity, slot }));
							resources.insert(SpawnContext(StartSoundEventEntity(Some(entity))));
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
				resources.remove::<SpawnContext<WeaponStateEvent>>();
				resources.remove::<SpawnContext<StartSoundEventEntity>>();
			});
		})
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ChangeAmmoCount;

pub fn change_ammo_count(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<ChangeAmmoCount>();

	SystemBuilder::new("change_ammo_count")
		.read_resource::<AssetStorage>()
		.with_query(<(&WeaponStateEvent, &ChangeAmmoCount)>::query())
		.with_query(<&mut WeaponState>::query())
		.build(move |_command_buffer, world, resources, queries| {
			let asset_storage = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, &ChangeAmmoCount) in queries.0.iter(&world0) {
				if let Ok(weapon_state) = queries.1.get_mut(&mut world, event.entity) {
					let weapon_template = asset_storage.get(&weapon_state.current).unwrap();

					if let Some(weapon_ammo) = &weapon_template.ammo {
						if let Some(ammo) = weapon_state.ammo.get_mut(&weapon_ammo.handle) {
							ammo.current -= weapon_ammo.count;

							if ammo.current < 0 {
								log::warn!("Negative ammo count");
							}
						} else {
							log::warn!("ChangeAmmoCount on entity without that ammo type");
						}
					} else {
						log::warn!("ChangeAmmoCount on weapon with no ammo consumption");
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
		.with_query(<(&WeaponStateEvent, &ExtraLight)>::query())
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

#[derive(Clone, Copy, Debug)]
pub struct NextWeaponState {
	pub time: Duration,
	pub state: (StateName, usize),
}

pub fn next_weapon_state(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<NextWeaponState>();

	SystemBuilder::new("next_weapon_state")
		.read_resource::<GameTime>()
		.with_query(<(&WeaponStateEvent, &NextWeaponState)>::query())
		.with_query(<&mut WeaponState>::query())
		.build(move |_command_buffer, world, resources, queries| {
			let game_time = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, next_state) in queries.0.iter(&world0) {
				if let Ok(weapon_state) = queries.1.get_mut(&mut world, event.entity) {
					let state = &mut weapon_state.slots[event.slot as usize];

					if let StateAction::None = state.action {
						state.timer.restart_with(**game_time, next_state.time);
						state.action = StateAction::Wait(next_state.state);
					}
				}
			}
		})
}

#[derive(Clone, Debug)]
pub struct LineAttack {
	pub count: usize,
	pub damage_range: Uniform<i32>,
	pub damage_multiplier: i32,
	pub distance: f32,
	pub spread: Vector2<Angle>,
	pub accurate_until_refire: bool,
	pub sparks: bool,
	pub hit_sound: Option<AssetHandle<Sound>>,
	pub miss_sound: Option<AssetHandle<Sound>>,
}

pub fn line_attack(resources: &mut Resources) -> impl Runnable {
	let (mut asset_storage, mut handler_set) =
		<(Write<AssetStorage>, Write<SpawnMergerHandlerSet>)>::fetch_mut(resources);
	handler_set.register_clone::<LineAttack>();

	let blood1 = asset_storage.load::<EntityTemplate>("blood1.entity");
	let blood2 = asset_storage.load::<EntityTemplate>("blood2.entity");
	let blood3 = asset_storage.load::<EntityTemplate>("blood3.entity");
	let puff1 = asset_storage.load::<EntityTemplate>("puff1.entity");
	let puff3 = asset_storage.load::<EntityTemplate>("puff3.entity");

	SystemBuilder::new("line_attack")
		.read_resource::<AssetStorage>()
		.read_resource::<Quadtree>()
		.with_query(<(&WeaponStateEvent, &LineAttack)>::query())
		.with_query(<(Option<&BoxCollider>, Option<&Owner>, &Transform, &WeaponState)>::query())
		.with_query(<&MapDynamic>::query())
		.with_query(<(
			Option<&BoxCollider>,
			Option<&LinedefRef>,
			Option<&SectorRef>,
		)>::query())
		.read_component::<BoxCollider>() // used by EntityTracer
		.read_component::<Owner>() // used by EntityTracer
		.read_component::<Transform>() // used by EntityTracer
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, quadtree) = resources;

			for (&event, line_attack) in queries.0.iter(world) {
				if let Ok((box_collider, owner, transform, weapon_state)) =
					queries.1.get(world, event.entity)
				 {
					let map_dynamic = queries.2.iter(world).next().unwrap();
					let map = asset_storage.get(&map_dynamic.map).unwrap();

					let tracer = EntityTracer {
						map,
						map_dynamic,
						quadtree: &quadtree,
						world,
					};

					let mut position = transform.position;

					if let Some(box_collider) = box_collider {
						position[2] += box_collider.height * 0.5 + 8.0;
					}

					let ignore = Some(owner.map_or(event.entity, |&Owner(owner)| owner));

					for _ in 0..line_attack.count {
						let mut rotation = transform.rotation;

						// Apply spread if the weapon is shooting inaccurately.
						// Subtracting two uniform random numbers results in a triangle distribution.
						if !line_attack.accurate_until_refire || weapon_state.inaccurate {
							if !line_attack.spread[0].is_zero() {
								rotation[2] +=
									thread_rng().gen_range(0..line_attack.spread[0].0) -
									thread_rng().gen_range(0..line_attack.spread[0].0);
							}

							if !line_attack.spread[1].is_zero() {
								rotation[1] +=
									thread_rng().gen_range(0..line_attack.spread[1].0) -
									thread_rng().gen_range(0..line_attack.spread[1].0);
							}
						}

						let direction = angles_to_axes(rotation)[0];
						let trace = tracer.trace(
							&AABB3::from_point(Vector3::zeros()),
							SolidType::PROJECTILE,
							ignore,
							Line3::new(position, direction * line_attack.distance),
						);

						// Hit something!
						if let Some(collision) = trace.collision {
							// Apply the damage
							let damage = line_attack.damage_multiplier
								* thread_rng().sample(line_attack.damage_range);

							command_buffer.push((
								DamageEvent {
									entity: collision.entity,
									damage,
									source_entity: event.entity,
									direction: trace.move_step.dir,
								},
							));

							// Spawn particles
							let mut particle_transform = Transform {
								position: trace.move_step.end_point(),
								rotation: Vector3::zeros(),
							};

							let particle = match queries.3.get(world, collision.entity) {
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
							let handle = match particle {
								DamageParticle::Blood => {
									if damage <= 9 {
										&blood3
									} else if damage <= 12 {
										&blood2
									} else {
										&blood1
									}
								}
								DamageParticle::Puff => {
									if line_attack.sparks {
										&puff1
									} else {
										&puff3
									}
								}
							}.clone();

							command_buffer.exec_mut(move |world, resources| {
								spawn_entity(world, resources, &handle, particle_transform);
							});

							// Play hit sound if present
							if let Some(sound) = line_attack.hit_sound.as_ref() {
								command_buffer.push((StartSoundEvent {
									handle: sound.clone(),
									entity: Some(event.entity),
								},));
							}
						} else {
							// Play miss sound if present
							if let Some(sound) = line_attack.miss_sound.as_ref() {
								command_buffer.push((StartSoundEvent {
									handle: sound.clone(),
									entity: Some(event.entity),
								},));
							}
						}
					}
				}
			}
		})
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
pub struct SetWeaponSprite(pub Option<SpriteRender>);

pub fn set_weapon_sprite(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<SetWeaponSprite>();

	SystemBuilder::new("set_weapon_sprite")
		.with_query(<(&WeaponStateEvent, &SetWeaponSprite)>::query())
		.with_query(<&mut WeaponSpriteRender>::query().filter(component::<WeaponState>()))
		.build(move |_command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, SetWeaponSprite(sprite)) in queries.0.iter(&world0) {
				if let Ok(weapon_sprite_render) = queries.1.get_mut(&mut world, event.entity) {
					weapon_sprite_render.slots[event.slot as usize] = sprite.clone();
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
		.with_query(<(&WeaponStateEvent, &SetWeaponState)>::query())
		.with_query(<&mut WeaponState>::query())
		.build(move |_command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, &SetWeaponState(next_state)) in queries.0.iter(&world0) {
				if let Ok(weapon_state) = queries.1.get_mut(&mut world, event.entity) {
					let state = &mut weapon_state.slots[event.slot as usize];
					state.action = StateAction::Set(next_state);
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
pub struct SpawnProjectile(pub AssetHandle<EntityTemplate>);

pub fn spawn_projectile(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<Owner>("Owner".into());
	handler_set.register_spawn::<OwnerDef, Owner>();

	handler_set.register_clone::<SpawnProjectile>();

	SystemBuilder::new("spawn_projectile")
		.read_resource::<AssetStorage>()
		.with_query(<(&WeaponStateEvent, &SpawnProjectile)>::query())
		.with_query(<&Transform>::query().filter(component::<WeaponState>()))
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
		.read_resource::<GameTime>()
		.with_query(<(&WeaponStateEvent, &WeaponPosition)>::query())
		.with_query(
			<(
				&Camera,
				&MovementBob,
				&mut WeaponState,
				&mut WeaponSpriteRender,
			)>::query()
			.filter(component::<WeaponState>()),
		)
		.build(move |_command_buffer, world, resources, queries| {
			let game_time = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, weapon_position) in queries.0.iter(&world0) {
				if let Ok((camera, movement_bob, weapon_state, weapon_sprite_render)) =
					queries.1.get_mut(&mut world, event.entity)
				{
					let state = &mut weapon_state.slots[event.slot as usize];

					match weapon_position {
						WeaponPosition::Bob => {
							let mut angle = Angle::from_units(
								game_time.0.as_secs_f64() / camera.weapon_bob_period.as_secs_f64(),
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
									weapon_state.slots[event.slot as usize].action =
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
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.with_query(<(&WeaponStateEvent, &WeaponReady)>::query())
		.with_query(<&mut WeaponState>::query().filter(component::<WeaponState>()))
		.build(move |_command_buffer, world, resources, queries| {
			let (asset_storage, client) = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, WeaponReady) in queries.0.iter(&world0) {
				if let Ok(weapon_state) = queries.1.get_mut(&mut world, event.entity) {
					if weapon_state.switch_to.is_some() {
						let state_name = (StateName::from("down").unwrap(), 0);
						weapon_state.slots[event.slot as usize].action =
							StateAction::Set(state_name);
					} else if client.command.attack {
						if weapon_state.can_fire(&asset_storage) {
							let state_name = (StateName::from("attack").unwrap(), 0);
							weapon_state.slots[event.slot as usize].action =
								StateAction::Set(state_name);
						}
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
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.with_query(<(&WeaponStateEvent, &WeaponReFire)>::query())
		.with_query(<&mut WeaponState>::query().filter(component::<WeaponState>()))
		.build(move |_command_buffer, world, resources, queries| {
			let (asset_storage, client) = resources;
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, WeaponReFire) in queries.0.iter(&world0) {
				if let Ok(weapon_state) = queries.1.get_mut(&mut world, event.entity) {
					if client.command.attack {
						if weapon_state.can_fire(&asset_storage) {
							let state_name = (StateName::from("attack").unwrap(), 0);
							weapon_state.slots[event.slot as usize].action =
								StateAction::Set(state_name);
							weapon_state.inaccurate = true;
						} else {
							weapon_state.inaccurate = false;
						}
					} else {
						weapon_state.inaccurate = false;
					}
				}
			}
		})
}
