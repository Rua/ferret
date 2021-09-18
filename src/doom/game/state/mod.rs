use crate::{
	common::time::Timer,
	doom::game::{combat::weapon::weapon_state, state::entity::entity_state},
};
use arrayvec::ArrayString;
use legion::{systems::ResourceSet, Read, Resources, Schedule, World};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};

pub type StateName = ArrayString<16>;

pub mod entity;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
	pub timer: Timer,
	pub action: StateAction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StateAction {
	Set((StateName, usize)),
	Wait((StateName, usize)),
	None,
}

#[derive(Default, Debug)]
pub struct StateSystemsRun(pub AtomicBool);

pub fn state(
	resources: &mut Resources,
	mut actions: Schedule,
) -> impl FnMut(&mut World, &mut Resources) {
	let mut schedule = Schedule::builder()
		.add_system(entity_state(resources))
		.add_system(weapon_state(resources))
		.build();

	move |world, resources| loop {
		resources.insert(StateSystemsRun::default());
		schedule.execute(world, resources);

		{
			let systems_run = <Read<StateSystemsRun>>::fetch(resources)
				.0
				.swap(false, Ordering::Relaxed);

			if !systems_run {
				resources.remove::<StateSystemsRun>();
				break;
			}
		}

		actions.execute(world, resources);
	}
}
