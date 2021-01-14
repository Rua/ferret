use legion::{systems::Runnable, SystemBuilder};
use std::time::Duration;

#[derive(Debug)]
pub struct FrameState {
	pub delta_time: Duration,
	pub time: Duration,
}

pub fn frame_state_system(frame_time: Duration) -> impl Runnable {
	SystemBuilder::new("frame_rng_system")
		.write_resource::<FrameState>()
		.build(move |_command_buffer, _world, resources, _query| {
			let frame_state = resources;

			frame_state.delta_time = frame_time;
			frame_state.time += frame_time;
		})
}
