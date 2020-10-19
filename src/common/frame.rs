use crate::common::resources_merger::FromWithResources;
use legion::{
	systems::{ResourceSet, Runnable},
	IntoQuery, Read, Resources, SystemBuilder,
};
use rand::{RngCore, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::time::Duration;

pub type FrameRng = Pcg64Mcg;

#[derive(Clone, Debug)]
pub struct FrameState {
	pub delta_time: Duration,
	pub total_time: Duration,
	pub rng: FrameRng,
	pub seed: <FrameRng as SeedableRng>::Seed,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FrameRngDef;

impl FromWithResources<FrameRngDef> for FrameRng {
	fn from_with_resources(_src_component: &FrameRngDef, resources: &Resources) -> Self {
		let frame_state = <Read<FrameState>>::fetch(resources);
		FrameRng::from_seed(frame_state.seed)
	}
}

pub fn frame_rng_system() -> impl Runnable {
	SystemBuilder::new("frame_rng_system")
		.with_query(<&mut FrameRng>::query())
		.build(move |_, world, _, query| {
			for rng in query.iter_mut(world) {
				rng.next_u64();
			}
		})
}
