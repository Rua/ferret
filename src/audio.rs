use rodio::{source::Done, Device, Sample, Source};
use std::{
	sync::{
		atomic::{AtomicBool, AtomicUsize, Ordering},
		Arc,
	},
	time::Duration,
};

/*pub struct Audio {}

impl Audio {
	pub fn new() -> Result<Audio, Box<dyn Error + Send + Sync>> {
		Ok(Audio {})
	}
}

impl Drop for Audio {
	fn drop(&mut self) {}
}*/

pub struct Sound {
	pub data: Arc<[u8]>,
	pub sample_rate: u32,
}

pub struct SoundSource {
	current: usize,
	data: Arc<[u8]>,
	duration: Duration,
	sample_rate: u32,
}

impl SoundSource {
	pub fn new(sound: &Sound) -> Self {
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
	type Item = u16;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		let item = self.data.get(self.current);
		self.current += 1;
		item.map(|x| ((*x as u16) << 8 | *x as u16))
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.data.len();
		(len, Some(len))
	}
}

pub struct Sink {
	controls: Arc<Controls>,
	is_playing: Arc<AtomicUsize>,
}

struct Controls {
	stopped: AtomicBool,
}

impl Sink {
	#[inline]
	pub fn play<S>(device: &Device, source: S) -> Sink
	where
		S: Source + Send + 'static,
		S::Item: Sample,
		S::Item: Send,
	{
		let sink = Sink {
			controls: Arc::new(Controls {
				stopped: AtomicBool::new(false),
			}),
			is_playing: Arc::new(AtomicUsize::new(1)),
		};

		let controls = sink.controls.clone();
		let source = source
			.stoppable()
			.periodic_access(Duration::from_millis(5), move |src| {
				if controls.stopped.load(Ordering::SeqCst) {
					src.stop();
				}
			})
			.convert_samples();
		let source = Done::new(source, sink.is_playing.clone());
		rodio::play_raw(device, source);
		sink
	}

	#[inline]
	pub fn stop(&self) {
		self.controls.stopped.store(true, Ordering::SeqCst);
	}

	#[inline]
	pub fn is_done(&self) -> bool {
		self.is_playing.load(Ordering::Relaxed) == 0
	}
}
