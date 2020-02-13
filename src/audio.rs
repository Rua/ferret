use rodio::{Device, queue::{SourcesQueueInput, SourcesQueueOutput}, Sample, Source, source::Done};
use std::{sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}, mpsc::Receiver, Mutex}, time::Duration};

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
    queue_tx: Arc<SourcesQueueInput<f32>>,
    sleep_until_end: Mutex<Option<Receiver<()>>>,

    controls: Arc<Controls>,
    sound_count: Arc<AtomicUsize>,

    detached: bool,
}

struct Controls {
    pause: AtomicBool,
    volume: Mutex<f32>,
    stopped: AtomicBool,
}

impl Sink {
    /// Builds a new `Sink`, beginning playback on a Device.
    #[inline]
    pub fn new(device: &Device) -> Sink {
        let (sink, queue_rx) = Sink::new_idle();
        rodio::play_raw(device, queue_rx);
        sink
    }

    /// Builds a new `Sink`.
    #[inline]
    pub fn new_idle() -> (Sink, SourcesQueueOutput<f32>) {
        let (queue_tx, queue_rx) = rodio::queue::queue(true);

        let sink = Sink {
            queue_tx: queue_tx,
            sleep_until_end: Mutex::new(None),
            controls: Arc::new(Controls {
                pause: AtomicBool::new(false),
                volume: Mutex::new(1.0),
                stopped: AtomicBool::new(false),
            }),
            sound_count: Arc::new(AtomicUsize::new(0)),
            detached: false,
        };
        (sink, queue_rx)
    }

    /// Appends a sound to the queue of sounds to play.
    #[inline]
    pub fn append<S>(&self, source: S)
    where
        S: Source + Send + 'static,
        S::Item: Sample,
        S::Item: Send,
    {
        let controls = self.controls.clone();

        let source = source
            .pausable(false)
            .amplify(1.0)
            .stoppable()
            .periodic_access(Duration::from_millis(5), move |src| {
                if controls.stopped.load(Ordering::SeqCst) {
                    src.stop();
                } else {
                    src.inner_mut().set_factor(*controls.volume.lock().unwrap());
                    src.inner_mut()
                        .inner_mut()
                        .set_paused(controls.pause.load(Ordering::SeqCst));
                }
            })
            .convert_samples();
        self.sound_count.fetch_add(1, Ordering::Relaxed);
        let source = Done::new(source, self.sound_count.clone());
        *self.sleep_until_end.lock().unwrap() = Some(self.queue_tx.append_with_signal(source));
    }

    /// Gets the volume of the sound.
    ///
    /// The value `1.0` is the "normal" volume (unfiltered input). Any value other than 1.0 will
    /// multiply each sample by this value.
    #[inline]
    pub fn volume(&self) -> f32 {
        *self.controls.volume.lock().unwrap()
    }

    /// Changes the volume of the sound.
    ///
    /// The value `1.0` is the "normal" volume (unfiltered input). Any value other than `1.0` will
    /// multiply each sample by this value.
    #[inline]
    pub fn set_volume(&self, value: f32) {
        *self.controls.volume.lock().unwrap() = value;
    }

    /// Resumes playback of a paused sink.
    ///
    /// No effect if not paused.
    #[inline]
    pub fn play(&self) {
        self.controls.pause.store(false, Ordering::SeqCst);
    }

    /// Pauses playback of this sink.
    ///
    /// No effect if already paused.
    ///
    /// A paused sink can be resumed with `play()`.
    pub fn pause(&self) {
        self.controls.pause.store(true, Ordering::SeqCst);
    }

    /// Gets if a sink is paused
    ///
    /// Sinks can be paused and resumed using `pause()` and `play()`. This returns `true` if the
    /// sink is paused.
    pub fn is_paused(&self) -> bool {
        self.controls.pause.load(Ordering::SeqCst)
    }

    /// Stops the sink by emptying the queue.
    #[inline]
    pub fn stop(&self) {
        self.controls.stopped.store(true, Ordering::SeqCst);
    }

    /// Destroys the sink without stopping the sounds that are still playing.
    #[inline]
    pub fn detach(mut self) {
        self.detached = true;
    }

    /// Sleeps the current thread until the sound ends.
    #[inline]
    pub fn sleep_until_end(&self) {
        if let Some(sleep_until_end) = self.sleep_until_end.lock().unwrap().take() {
            let _ = sleep_until_end.recv();
        }
    }

    /// Returns true if this sink has no more sounds to play.
    #[inline]
    pub fn empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of sounds currently in the queue.
    #[inline]
    pub fn len(&self) -> usize {
        self.sound_count.load(Ordering::Relaxed)
    }
}

impl Drop for Sink {
    #[inline]
    fn drop(&mut self) {
        self.queue_tx.set_keep_alive_if_empty(false);

        if !self.detached {
            self.controls.stopped.store(true, Ordering::Relaxed);
        }
    }
}
