use crate::common::spawn::{ComponentAccessor, SpawnFrom};
use legion::{
	systems::{ResourceSet, Runnable},
	IntoQuery, Read, Resources, SystemBuilder,
};
use rand::{RngCore, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::{sync::Mutex, time::Duration};

pub type FrameRng = Pcg64Mcg;

#[derive(Debug)]
pub struct FrameState {
	pub delta_time: Duration,
	pub time: Duration,
	pub rng: Mutex<FrameRng>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FrameRngDef;

impl SpawnFrom<FrameRngDef> for FrameRng {
	fn spawn(
		_component: &FrameRngDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> Self {
		let frame_state = <Read<FrameState>>::fetch(resources);
		let mut rng = frame_state.rng.lock().unwrap();
		let mut seed = <FrameRng as SeedableRng>::Seed::default();
		rng.fill_bytes(&mut seed);
		FrameRng::from_seed(seed)
	}
}

pub fn frame_state_system(frame_time: Duration) -> impl Runnable {
	SystemBuilder::new("frame_rng_system")
		.write_resource::<FrameState>()
		.with_query(<&mut FrameRng>::query())
		.build(move |_, world, frame_state, query| {
			frame_state.delta_time = frame_time;
			frame_state.time += frame_time;

			// Make the RNG state time dependent
			// Since we have write access to FrameState, this Mutex will never be contended
			let mut rng = frame_state.rng.lock().unwrap();
			rng.next_u64();

			for rng in query.iter_mut(world) {
				rng.next_u64();
			}
		})
}
