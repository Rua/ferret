use crate::doom::{client::Client, health::Health, ui::UiText};
use legion::{component, systems::Runnable, IntoQuery, Resources, SystemBuilder};
use std::fmt::Write as _;

#[derive(Clone, Copy, Debug, Default)]
pub struct HealthStat;

pub fn health_stat(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("health_stat")
		.read_resource::<Client>()
		.with_query(<&Health>::query())
		.with_query(<&mut UiText>::query().filter(component::<HealthStat>()))
		.build(move |_command_buffer, world, resources, queries| {
			let client = resources;
			let client_entity = match client.entity {
				Some(e) => e,
				None => return,
			};

			let health = queries
				.0
				.get(world, client_entity)
				.ok()
				.map(|health| health.current);

			for ui_text in queries.1.iter_mut(world) {
				ui_text.text.clear();

				if let Some(health) = health {
					write!(ui_text.text, "{:3}%", health).ok();
				}
			}
		})
}
