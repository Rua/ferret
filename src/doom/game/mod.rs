pub mod camera;
pub mod cheats;
pub mod client;
pub mod combat;
pub mod map;
pub mod physics;
pub mod spawn;
pub mod state;
pub mod trace;

use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		dirs::config_dir,
		geometry::{Angle, Interval, AABB2},
		quadtree::Quadtree,
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom, SpawnMergerHandlerSet},
		time::GameTime,
	},
	doom::{
		assets::{
			map::{load::build_things, Map},
			process_assets,
		},
		clear_event,
		draw::sprite::SpriteRender,
		game::{
			camera::{camera_move, movement_bob},
			client::{
				player_command, player_move, player_touch, player_use, player_weapon, Client,
				UseEvent,
			},
			combat::{
				apply_damage, extra_light, projectile_touch, radius_attack, spawn_projectile,
				spray_attack,
				weapon::{
					change_ammo_count, line_attack, next_weapon_state, set_weapon_sprite,
					set_weapon_state, weapon_position, weapon_ready, weapon_refire,
					WeaponStateEvent,
				},
				DamageEvent,
			},
			map::{
				anim::{light_flash, light_glow, texture_animation, texture_scroll},
				door::{door_active, door_linedef_touch, door_switch_use, door_use},
				exit::exit_switch_use,
				floor::{floor_active, floor_linedef_touch, floor_switch_use},
				plat::{plat_active, plat_linedef_touch, plat_switch_use},
				sector_move::{sector_move, SectorMoveEvent},
				switch::switch_active,
				LinedefRef, MapDynamic, SectorRef,
			},
			physics::{
				physics, set_blocks_types, set_solid_type, BoxCollider, StepEvent, TouchEvent,
				DISTANCE_EPSILON,
			},
			spawn::{spawn_map_entities, spawn_things},
			state::{
				entity::{next_entity_state, remove_entity, EntityStateEvent},
				state,
			},
		},
		iwad::IWADInfo,
		ui::hud::{ammo_stat, arms_stat, health_stat},
		ASSET_SERIALIZER,
	},
};
use anyhow::{bail, Context};
use legion::{
	component,
	serialize::{set_entity_serializer, Canon},
	systems::{Builder, CommandBuffer, ResourceSet, Runnable},
	Entity, IntoQuery, Read, Registry, Resources, Schedule, SystemBuilder, World, Write,
};
use nalgebra::Vector3;
use rand::{distributions::Uniform, thread_rng, Rng};
use relative_path::RelativePathBuf;
use serde::{de::DeserializeSeed, Deserialize, Serialize};
use std::{
	fs::{create_dir_all, File},
	io::{BufReader, BufWriter, Write as _},
	path::PathBuf,
};

pub fn add_update_systems(builder: &mut Builder, resources: &mut Resources) -> anyhow::Result<()> {
	#[rustfmt::skip]
	builder
		.add_system(player_command(resources))
		.add_system(player_move(resources))
		.add_system(player_weapon(resources))

		.add_system(player_use(resources))
		.flush()
		.add_system(door_use(resources))
		.add_system(door_switch_use(resources))
		.add_system(exit_switch_use(resources))
		.add_system(floor_switch_use(resources))
		.add_system(plat_switch_use(resources))
		.add_system(clear_event::<UseEvent>())

		.add_system(physics(resources))
		.flush()
		.add_system(door_linedef_touch(resources))
		.add_system(floor_linedef_touch(resources))
		.add_system(plat_linedef_touch(resources))
		.add_system(player_touch(resources))
		.add_system(projectile_touch(resources))
		.add_system(movement_bob(resources))
		.add_system(camera_move(resources))
		.add_system(clear_event::<StepEvent>())
		.add_system(clear_event::<TouchEvent>())

		.add_system(sector_move(resources))
		.flush()
		.add_system(door_active(resources))
		.add_system(floor_active(resources))
		.add_system(plat_active(resources))
		.add_system(clear_event::<SectorMoveEvent>())

		.add_system(light_flash(resources))
		.add_system(light_glow(resources))
		.add_system(switch_active(resources))
		.add_system(texture_animation(resources))
		.add_system(texture_scroll(resources))
		
		.add_system(apply_damage(resources))
		.add_system(clear_event::<DamageEvent>())
		.flush()

		.add_thread_local_fn({
			let actions = Schedule::builder()
				.add_system(change_ammo_count(resources))
				.add_system(extra_light(resources))
				.add_system(next_entity_state(resources))
				.add_system(remove_entity(resources))
				.add_system(set_blocks_types(resources))
				.add_system(set_entity_sprite(resources))
				.add_system(set_solid_type(resources))
				.add_system(next_weapon_state(resources))
				.add_system(line_attack(resources))
				.add_system(radius_attack(resources))
				.add_system(set_weapon_sprite(resources))
				.add_system(set_weapon_state(resources))
				.add_system(spawn_projectile(resources))
				.add_system(spray_attack(resources))
				.add_system(weapon_position(resources))
				.add_system(weapon_ready(resources))
				.add_system(weapon_refire(resources))
				.add_system(clear_event::<EntityStateEvent>())
				.add_system(clear_event::<WeaponStateEvent>())
				.build();

			state(resources, actions)
		})
		.add_system(ammo_stat(resources))
		.add_system(health_stat(resources))
		.add_system(arms_stat(resources));

	Ok(())
}

pub fn new_game(map: &str, world: &mut World, resources: &mut Resources) {
	let mut map = RelativePathBuf::from(map.to_ascii_lowercase());
	map.set_extension("map");

	log::info!("Starting new game on {}...", map);

	clear_game(world, resources);

	let result = || -> anyhow::Result<()> {
		resources.insert(GameTime::default());

		log::info!("Loading map...");
		let map_handle: AssetHandle<Map> = {
			let mut asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
			asset_storage.load(map.as_str())
		};
		process_assets(resources);
		spawn_map_entities(world, resources, &map_handle)?;

		let quadtree = create_quadtree(world, resources);
		resources.insert(quadtree);

		log::info!("Spawning entities...");
		let things = {
			let asset_storage = <Write<AssetStorage>>::fetch_mut(resources);
			build_things(&asset_storage.source().load(&map.with_extension("things"))?)?
		};
		spawn_things(things, world, resources)?;

		// Spawn player
		let entity = spawn::spawn_player(world, resources, 1)?;
		resources.insert(Client {
			entity: Some(entity),
			..Client::default()
		});

		process_assets(resources);

		Ok(())
	}();

	match result {
		Ok(_) => log::info!("Game started."),
		Err(err) => log::error!("{:?}", err),
	}
}

pub fn change_map(map: &str, world: &mut World, resources: &mut Resources) {
	if !resources.contains::<GameTime>() {
		log::error!("Can't change map, not currently in a game.");
		return;
	}

	// TODO continue existing game
	new_game(map, world, resources);
}

#[derive(Serialize, Deserialize)]
struct SavedResources {
	client: Client,
	game_time: GameTime,
}

#[inline]
fn save_path(name: &str, resources: &Resources) -> anyhow::Result<PathBuf> {
	if name.contains("/") || name.contains("\\") {
		bail!("Save names cannot contain \"/\" or \"\\\"");
	}

	let mut path = config_dir();
	path.push(<Read<IWADInfo>>::fetch(resources).files[0]);
	path.push(name);
	path.set_extension("sav");
	Ok(path)
}

macro_rules! game_entities {
	() => {
		component::<Transform>()
			| component::<MapDynamic>()
			| component::<LinedefRef>()
			| component::<SectorRef>()
	};
}

pub fn save_game(name: &str, world: &mut World, resources: &mut Resources) {
	if !resources.contains::<GameTime>() {
		log::error!("Can't save game, not currently in a game.");
		return;
	}

	let path = match save_path(name, resources) {
		Ok(x) => x,
		Err(err) => {
			log::error!("{:?}", err);
			return;
		}
	};
	log::info!("Saving game to \"{}\"...", path.display());

	let result = Ok(())
		.and_then(|_| {
			if let Some(dir) = path.parent() {
				if !dir.is_dir() {
					create_dir_all(dir).with_context(|| {
						format!("Couldn't create directory \"{}\"", dir.display())
					})?;
				}
			}
			Ok(())
		})
		.and_then(|_| {
			File::create(&path)
				.with_context(|| format!("Couldn't open \"{}\" for writing", path.display()))
		})
		.and_then(|file| {
			let mut file = BufWriter::new(file);
			let (canon, client, game_time, registry, mut asset_storage) = <(
				Read<Canon>,
				Read<Client>,
				Read<GameTime>,
				Read<Registry<String>>,
				Write<AssetStorage>,
			)>::fetch_mut(resources);

			let saved_resources = SavedResources {
				client: client.clone(),
				game_time: *game_time,
			};

			ASSET_SERIALIZER.set(&mut asset_storage, || -> anyhow::Result<()> {
				let mut serializer = rmp_serde::encode::Serializer::new(&mut file);

				set_entity_serializer(&*canon, || saved_resources.serialize(&mut serializer))
					.context("Couldn't serialize resources")?;
				world
					.as_serializable(game_entities!(), &*registry, &*canon)
					.serialize(&mut serializer)
					.context("Couldn't serialize world")?;

				file.flush().context("Couldn't flush file")?;
				log::info!("Game saved.");
				Ok(())
			})
		})
		.context("Couldn't save game");

	if let Err(err) = result {
		log::error!("{:?}", err);
	}
}

pub fn load_game(name: &str, world: &mut World, resources: &mut Resources) {
	let path = match save_path(name, resources) {
		Ok(x) => x,
		Err(err) => {
			log::error!("{:?}", err);
			return;
		}
	};
	log::info!("Loading game from \"{}\"...", path.display());

	let result = Ok(())
		.and_then(|_| {
			File::open(&path)
				.with_context(|| format!("Couldn't open \"{}\" for reading", path.display()))
		})
		.and_then(|file| {
			clear_game(world, resources);
			let mut file = BufReader::new(file);
			let (canon, registry, mut asset_storage) =
				<(Read<Canon>, Read<Registry<String>>, Write<AssetStorage>)>::fetch_mut(resources);

			ASSET_SERIALIZER.set(&mut asset_storage, || -> anyhow::Result<_> {
				let mut deserializer = rmp_serde::decode::Deserializer::new(&mut file);
				let saved_resources = set_entity_serializer(&*canon, || {
					SavedResources::deserialize(&mut deserializer)
				})
				.context("Couldn't deserialize resources")?;
				registry
					.as_deserialize_into_world(world, &*canon)
					.deserialize(&mut deserializer)
					.context("Couldn't deserialize world")?;

				Ok(saved_resources)
			})
		})
		.context("Couldn't load game");

	match result {
		Ok(saved_resources) => {
			resources.insert(saved_resources.client);
			resources.insert(saved_resources.game_time);

			process_assets(resources);

			let quadtree = create_quadtree(world, resources);
			resources.insert(quadtree);

			log::info!("Game loaded.");
		}
		Err(err) => log::error!("{:?}", err),
	}
}

pub fn clear_game(world: &mut World, resources: &mut Resources) {
	log::debug!("Clearing game...");
	let mut command_buffer = CommandBuffer::new(world);
	command_buffer.exec_mut(|_, resources| {
		resources.remove::<Client>();
		resources.remove::<GameTime>();
		resources.remove::<Quadtree>();
	});
	for &entity in <Entity>::query().filter(game_entities!()).iter(world) {
		command_buffer.remove(entity);
	}
	command_buffer.flush(world, resources);
}

pub fn create_quadtree(world: &World, resources: &Resources) -> Quadtree {
	let asset_storage = <Read<AssetStorage>>::fetch(resources);
	let map_dynamic = <&MapDynamic>::query()
		.iter(world)
		.next()
		.expect("No MapDynamic entity found");
	let map = asset_storage.get(&map_dynamic.map).unwrap();
	let mut quadtree = Quadtree::new(map.bbox);

	for (&entity, box_collider, transform) in
		<(Entity, &BoxCollider, &Transform)>::query().iter(world)
	{
		quadtree.insert(
			entity,
			&AABB2::from_radius(box_collider.radius).offset(transform.position.fixed_resize(0.0)),
		);
	}

	quadtree
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Transform {
	pub position: Vector3<f32>,
	pub rotation: Vector3<Angle>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransformDef {
	pub spawn_on_ceiling: bool,
}

impl SpawnFrom<TransformDef> for Transform {
	fn spawn(component: &TransformDef, accessor: ComponentAccessor, resources: &Resources) -> Self {
		let transform = <Read<SpawnContext<Transform>>>::fetch(resources);
		let mut transform = transform.0;

		if transform.position[2].is_nan() {
			let sector_interval = <Read<SpawnContext<Interval>>>::fetch(resources);

			if component.spawn_on_ceiling {
				transform.position[2] = sector_interval.0.max - DISTANCE_EPSILON;

				if let Some(box_collider) = accessor.get::<BoxCollider>() {
					transform.position[2] -= box_collider.height;
				}
			} else {
				transform.position[2] = sector_interval.0.min + DISTANCE_EPSILON;
			}
		}

		transform
	}
}

#[derive(Clone, Copy, Debug)]
pub struct RandomTransformDef(pub [Uniform<f32>; 3]);

impl SpawnFrom<RandomTransformDef> for Transform {
	fn spawn(
		component: &RandomTransformDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		let transform = <Read<SpawnContext<Transform>>>::fetch(resources);
		let mut transform = transform.0;
		let offset = Vector3::from_iterator(component.0.iter().map(|u| thread_rng().sample(u)));
		transform.position += offset;
		transform
	}
}

#[derive(Clone, Debug)]
pub struct SetEntitySprite(pub SpriteRender);

pub fn set_entity_sprite(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<SetEntitySprite>();

	SystemBuilder::new("set_entity_sprite")
		.with_query(<(&EntityStateEvent, &SetEntitySprite)>::query())
		.with_query(<&mut SpriteRender>::query())
		.build(move |_command_buffer, world, _resources, queries| {
			let (world0, mut world) = world.split_for_query(&queries.0);

			for (&event, SetEntitySprite(sprite)) in queries.0.iter(&world0) {
				if let Ok(sprite_render) = queries.1.get_mut(&mut world, event.entity) {
					*sprite_render = sprite.clone();
				}
			}
		})
}
