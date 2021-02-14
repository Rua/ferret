use crate::{
	common::assets::{AssetHandle, AssetStorage},
	doom::{
		client::Client,
		health::Health,
		image::Image,
		state::weapon::WeaponState,
		ui::{UiImage, UiText},
	},
};
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

#[derive(Clone, Debug)]
pub struct ArmsStat {
	pub weapons: Vec<String>,
	pub images: [AssetHandle<Image>; 2],
}

pub fn arms_stat(_resources: &mut Resources) -> impl Runnable {
	SystemBuilder::new("arms_stat")
		.read_resource::<AssetStorage>()
		.read_resource::<Client>()
		.with_query(<&WeaponState>::query())
		.with_query(<(&ArmsStat, &mut UiImage)>::query())
		.build(move |_command_buffer, world, resources, queries| {
			let (asset_storage, client) = resources;
			let (mut world1, world) = world.split_for_query(&queries.1);

			let client_entity = match client.entity {
				Some(e) => e,
				None => return,
			};

			let inventory = queries
				.0
				.get(&world, client_entity)
				.ok()
				.map(|weapon_state| &weapon_state.inventory);

			for (arms_stat, ui_image) in queries.1.iter_mut(&mut world1) {
				let is_present = inventory.map_or(false, |inventory| {
					arms_stat.weapons.iter().any(|weapon| {
						let asset_name = format!("{}.weapon", weapon);
						asset_storage
							.handle_for(&asset_name)
							.map_or(false, |weapon_handle| inventory.contains(&weapon_handle))
					})
				});
				ui_image.image = arms_stat.images[is_present as usize].clone();
			}
		})
}
