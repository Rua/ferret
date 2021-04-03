use crate::{
	common::{
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom, SpawnMergerHandlerSet},
		time::Timer,
	},
	doom::state::{entity::entity_state, weapon::weapon_state},
};
use arrayvec::ArrayString;
use legion::{systems::ResourceSet, Entity, Read, Resources, Schedule, World, Write};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};

pub type StateName = ArrayString<16>;

pub mod entity;
pub mod weapon;

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

#[derive(Clone, Copy, Debug, Default)]
pub struct EntityDef;

impl SpawnFrom<EntityDef> for Entity {
	fn spawn(_component: &EntityDef, _accessor: ComponentAccessor, resources: &Resources) -> Self {
		<Read<SpawnContext<Entity>>>::fetch(resources).0
	}
}

#[derive(Default, Debug)]
pub struct StateSystemsRun(AtomicBool);

pub fn state(
	resources: &mut Resources,
	mut actions: Schedule,
) -> impl FnMut(&mut World, &mut Resources) {
	{
		let mut handler_set = <Write<SpawnMergerHandlerSet>>::fetch_mut(resources);
		handler_set.register_spawn::<EntityDef, Entity>();
	}

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
