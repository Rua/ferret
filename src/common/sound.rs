use crate::common::assets::AssetHandle;
use anyhow::Context;
use crossbeam_channel::Sender;
use rodio::{
	source::{ChannelVolume, Done},
	OutputStream, Sample, Source,
};
use smallvec::SmallVec;
use std::{
	sync::{
		atomic::{AtomicBool, AtomicUsize, Ordering},
		Arc, Mutex,
	},
	thread::Builder,
	time::Duration,
};

pub fn init() -> anyhow::Result<Sender<Box<dyn Source<Item = f32> + Send>>> {
	log::debug!("Spawning audio thread");
	let (sender, receiver) = crossbeam_channel::unbounded();

	Builder::new()
		.name("audio".to_owned())
		.spawn(move || {
			let (_stream, stream_handle) = OutputStream::try_default().unwrap();

			// Play a dummy sound to force the sound engine to initialise itself
			stream_handle.play_raw(rodio::source::Empty::new()).unwrap();

			for source in receiver {
				stream_handle.play_raw(source).unwrap();
			}
		})
		.context("Couldn't spawn audio thread")?;

	Ok(sender)
}

#[derive(Clone, Debug)]
pub struct Sound {
	pub sounds: SmallVec<[AssetHandle<RawSound>; 4]>,
}

#[derive(Clone, Debug)]
pub struct RawSound {
	pub data: Arc<[i16]>,
	pub sample_rate: u32,
}

#[derive(Clone, Debug)]
pub struct SoundSource {
	current: usize,
	data: Arc<[i16]>,
	duration: Duration,
	sample_rate: u32,
}

impl SoundSource {
	pub fn new(sound: &RawSound) -> Self {
		let duration_ns = 1_000_000_000u64
			.checked_mul(sound.data.len() as u64)
			.unwrap() / sound.sample_rate as u64;
		let duration = Duration::new(
			duration_ns / 1_000_000_000,
			(duration_ns % 1_000_000_000) as u32,
		);

		let sample_rate = sound.sample_rate;

		SoundSource {
			current: 0,
			data: sound.data.clone(),
			duration,
			sample_rate,
		}
	}
}

impl Source for SoundSource {
	#[inline]
	fn current_frame_len(&self) -> Option<usize> {
		None
	}

	#[inline]
	fn channels(&self) -> u16 {
		1
	}

	#[inline]
	fn sample_rate(&self) -> u32 {
		self.sample_rate
	}

	#[inline]
	fn total_duration(&self) -> Option<Duration> {
		Some(self.duration)
	}
}

impl Iterator for SoundSource {
	type Item = i16;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		let item = self.data.get(self.current);
		self.current += 1;
		item.copied()
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.data.len();
		(len, Some(len))
	}
}

#[derive(Clone, Debug)]
pub struct SoundController {
	controls: Arc<Controls>,
	is_playing: Arc<AtomicUsize>,
}

#[derive(Debug)]
struct Controls {
	stopped: AtomicBool,
	volumes: Mutex<[f32; 2]>,
}

impl SoundController {
	#[inline]
	pub fn new<S>(source: S) -> (SoundController, impl Source<Item = S::Item>)
	where
		S: Source + Send + 'static,
		S::Item: Sample,
		S::Item: Send,
	{
		let controller = SoundController {
			controls: Arc::new(Controls {
				stopped: AtomicBool::new(false),
				volumes: Mutex::new([1.0, 1.0]),
			}),
			is_playing: Arc::new(AtomicUsize::new(1)),
		};

		let controls = controller.controls.clone();
		let source = ChannelVolume::new(source.stoppable(), vec![1.0, 1.0]).periodic_access(
			Duration::from_millis(5),
			move |src| {
				if controls.stopped.load(Ordering::SeqCst) {
					src.inner_mut().stop();
				} else {
					let volumes = controls.volumes.lock().unwrap();
					src.set_volume(0, volumes[0]);
					src.set_volume(1, volumes[1]);
				}
			},
		);
		let source = Done::new(source, controller.is_playing.clone());
		(controller, source)
	}

	#[inline]
	pub fn stop(&self) {
		self.controls.stopped.store(true, Ordering::SeqCst);
	}

	#[inline]
	pub fn is_done(&self) -> bool {
		self.is_playing.load(Ordering::Relaxed) == 0
	}

	#[inline]
	pub fn set_volumes(&self, volumes: [f32; 2]) {
		*self.controls.volumes.lock().unwrap() = volumes;
	}
}
