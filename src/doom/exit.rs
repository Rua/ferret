use crate::{
	common::{
		assets::AssetStorage,
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom, SpawnMergerHandlerSet},
		time::GameTime,
	},
	doom::{
		client::{Usable, UseEvent},
		map::{LinedefRef, MapDynamic},
		switch::{SwitchActive, SwitchParams},
	},
};
use crossbeam_channel::Sender;
use legion::{
	component,
	systems::{ResourceSet, Runnable},
	IntoQuery, Read, Registry, Resources, SystemBuilder, Write,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct ExitSwitchUse {
	pub switch_params: SwitchParams,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NextMap(pub String);

#[derive(Clone, Copy, Debug, Default)]
pub struct NextMapDef;

impl SpawnFrom<NextMapDef> for NextMap {
	fn spawn(_component: &NextMapDef, _accessor: ComponentAccessor, resources: &Resources) -> Self {
		<Read<SpawnContext<NextMap>>>::fetch(resources).0.clone()
	}
}

pub fn exit_switch_use(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<NextMap>("NextMap".into());
	handler_set.register_spawn::<NextMapDef, NextMap>();
	handler_set.register_clone::<ExitSwitchUse>();

	SystemBuilder::new("exit_switch_use")
		.read_resource::<AssetStorage>()
		.read_resource::<Sender<String>>()
		.read_resource::<GameTime>()
		.with_query(<(&UseEvent, &ExitSwitchUse)>::query())
		.with_query(<(&LinedefRef, &NextMap)>::query().filter(!component::<SwitchActive>()))
		.with_query(<&mut MapDynamic>::query())
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, command_sender, game_time) = resources;
			let (mut world2, world) = world.split_for_query(&queries.2);

			for (event, exit_switch_use) in queries.0.iter(&world) {
				if let Ok((linedef_ref, NextMap(next_map))) = queries.1.get(&world, event.entity) {
					let map_dynamic = queries
						.2
						.get_mut(&mut world2, linedef_ref.map_entity)
						.unwrap();
					let map = asset_storage.get(&map_dynamic.map).unwrap();

					command_sender.send(format!("change {}", next_map)).ok();

					crate::doom::switch::activate(
						&exit_switch_use.switch_params,
						command_buffer,
						**game_time,
						linedef_ref.index,
						map,
						map_dynamic,
					);

					if exit_switch_use.switch_params.retrigger_time.is_none() {
						command_buffer.remove_component::<Usable>(event.entity);
					}
				}
			}
		})
}
