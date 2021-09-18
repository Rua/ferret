use crate::{
	common::{
		assets::{AssetHandle, AssetStorage},
		geometry::Angle,
		sound::{SoundController, SoundSource},
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom, SpawnMergerHandlerSet},
	},
	doom::{
		assets::sound::Sound,
		game::{client::Client, Transform},
	},
};
use crossbeam_channel::Sender;
use legion::{
	systems::{ResourceSet, Runnable},
	Entity, IntoQuery, Read, Resources, SystemBuilder, Write,
};
use nalgebra::Vector2;
use rand::{thread_rng, Rng};
use rodio::Source;

#[derive(Clone, Debug)]
pub struct StartSoundEvent {
	pub handle: AssetHandle<Sound>,
	pub entity: Option<Entity>,
}

#[derive(Clone, Debug)]
pub struct StartSoundEventDef {
	pub handle: AssetHandle<Sound>,
}

#[derive(Clone, Copy, Debug)]
pub struct StartSoundEventEntity(pub Option<Entity>);

impl SpawnFrom<StartSoundEventDef> for StartSoundEvent {
	fn spawn(
		component: &StartSoundEventDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		StartSoundEvent {
			handle: component.handle.clone(),
			entity: <Read<SpawnContext<StartSoundEventEntity>>>::fetch(resources)
				.0
				 .0,
		}
	}
}

#[derive(Clone, Debug)]
pub struct SoundPlaying {
	pub controller: SoundController,
	pub entity: Option<Entity>,
}

type SoundSender = Sender<Box<dyn Source<Item = f32> + Send>>;

pub fn start_sound(resources: &mut Resources) -> impl Runnable {
	let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
	handler_set.register_clone::<StartSoundEvent>();
	handler_set.register_spawn::<StartSoundEventDef, StartSoundEvent>();

	SystemBuilder::new("start_sound")
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.read_resource::<SoundSender>()
		.write_resource::<Vec<SoundPlaying>>()
		.with_query(<&Transform>::query())
		.with_query(<&StartSoundEvent>::query())
		.build(move |_command_buffer, world, resources, queries| {
			let (asset_storage, client, sound_sender, sounds_playing) = resources;
			let client_transform = *queries.0.get(world, client.entity.unwrap()).unwrap();
			let (world1, mut world) = world.split_for_query(&queries.1);

			for event in queries.1.iter(&world1) {
				// Create new sound controller
				let sound = asset_storage.get(&event.handle).unwrap();
				let index = match sound.sounds.len() {
					0 => continue,
					1 => 0,
					len => thread_rng().gen_range(0..len),
				};
				let raw_sound = asset_storage.get(&sound.sounds[index]).unwrap();
				let (controller, source) = SoundController::new(SoundSource::new(&raw_sound));
				let sound_playing = SoundPlaying {
					controller,
					entity: event.entity,
				};

				if let Some(entity) = sound_playing.entity {
					// Stop old sound on this entity, if any
					if let Some(i) = sounds_playing
						.iter()
						.position(|old| old.entity == Some(entity))
					{
						let old = sounds_playing.swap_remove(i);
						old.controller.stop();
					}

					// Set distance falloff and stereo panning
					if let Ok(transform) = queries.0.get_mut(&mut world, entity) {
						let volumes = calculate_volumes(&client_transform, transform);
						sound_playing.controller.set_volumes(volumes.into());
					}
				}

				sounds_playing.push(sound_playing);
				sound_sender.send(Box::from(source.convert_samples())).ok();
			}
		})
}

pub fn update_sound(resources: &mut Resources) -> impl Runnable {
	let sounds_playing: Vec<SoundPlaying> = Vec::new();
	resources.insert(sounds_playing);

	SystemBuilder::new("update_sound")
		.read_resource::<Client>()
		.write_resource::<Vec<SoundPlaying>>()
		.with_query(<&Transform>::query())
		.build(move |_command_buffer, world, resources, query| {
			let (client, sounds_playing) = resources;
			let client_transform = *query.get(world, client.entity.unwrap()).unwrap();

			sounds_playing.retain(|sound_playing| {
				if sound_playing.controller.is_done() {
					return false;
				}

				// Set distance falloff and stereo panning, if attached to an entity
				if let Some(transform) = sound_playing
					.entity
					.and_then(|entity| query.get(world, entity).ok())
				{
					let volumes = calculate_volumes(&client_transform, transform);
					sound_playing.controller.set_volumes(volumes.into());
				}

				true
			});
		})
}

fn calculate_volumes(client_transform: &Transform, entity_transform: &Transform) -> Vector2<f32> {
	let to_entity_vec = entity_transform.position - client_transform.position;
	let distance = to_entity_vec.norm();

	// Calculate distance falloff
	const MIN_DIST: f32 = 160.0;
	const MAX_DIST: f32 = 1200.0;

	let distance_factor = if distance < MIN_DIST {
		1.0
	} else if distance > MAX_DIST {
		0.0
	} else {
		(MAX_DIST - distance) / (MAX_DIST - MIN_DIST)
	};

	// Calculate stereo panning
	const MAX_PAN: f32 = 0.75;

	let pan = if distance < 1.0 {
		0.0
	} else {
		let angle = client_transform.rotation[2]
			- Angle::from_radians(f64::atan2(to_entity_vec[1] as f64, to_entity_vec[0] as f64));
		MAX_PAN * angle.sin() as f32
	};

	let volumes = Vector2::new(
		1.0 - 0.25 * (pan + 1.0).powi(2),
		1.0 - 0.25 * (pan - 1.0).powi(2),
	);

	// Final result
	volumes * distance_factor
}
