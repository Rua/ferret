use crate::{
	common::{
		assets::AssetStorage,
		frame::FrameRng,
		spawn::{ComponentAccessor, SpawnFrom},
	},
	doom::{
		state::{State, StateAction, StateName},
		template::EntityTemplateRef,
	},
};
use legion::{systems::Runnable, Entity, IntoQuery, Resources, SystemBuilder};
use rand::Rng;
use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub struct Health {
	pub current: f32,
	pub max: f32,
	pub pain_chance: f32,
	pub damage: SmallVec<[(Entity, f32); 8]>,
}

#[derive(Clone, Copy, Debug)]
pub struct HealthDef {
	pub max: f32,
	pub pain_chance: f32,
}

impl SpawnFrom<HealthDef> for Health {
	fn spawn(component: &HealthDef, _accessor: ComponentAccessor, _resources: &Resources) -> Self {
		Health {
			current: component.max,
			max: component.max,
			pain_chance: component.pain_chance,
			damage: SmallVec::new(),
		}
	}
}

pub fn damage_system(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("damage_system")
		.read_resource::<AssetStorage>()
		.with_query(<(
			Entity,
			&EntityTemplateRef,
			&mut FrameRng,
			&mut Health,
			&mut State,
		)>::query())
		.build(move |command_buffer, world, resources, query| {
			let asset_storage = resources;

			for (entity, template_ref, frame_rng, health, state) in query.iter_mut(world) {
				for (_source, damage) in health.damage.drain(..) {
					if health.current <= 0.0 {
						break;
					}

					health.current -= damage;

					let template = asset_storage.get(&template_ref.0).unwrap();

					if health.current <= 0.0 {
						if health.current < -health.max && template.states.contains_key("xdeath") {
							let new = (StateName::from("xdeath").unwrap(), 0);
							state.action = StateAction::Set(new);
						} else if template.states.contains_key("death") {
							let new = (StateName::from("death").unwrap(), 0);
							state.action = StateAction::Set(new);
						} else {
							state.action = StateAction::None;
							command_buffer.remove(*entity);
						}
					} else {
						if template.states.contains_key("pain")
							&& frame_rng.gen_bool(health.pain_chance as f64)
						{
							let new = (StateName::from("pain").unwrap(), 0);
							state.action = StateAction::Set(new);
						}
					}
				}
			}
		})
}
