use crate::{
	common::{
		assets::AssetStorage,
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom, SpawnMergerHandlerSet},
		time::GameTime,
	},
	doom::game::{
		client::{Usable, UseEvent},
		map::{
			switch::{self, SwitchActive, SwitchParams},
			LinedefRef, MapDynamic,
		},
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
pub struct ExitMap(String);

#[derive(Clone, Copy, Debug, Default)]
pub struct ExitMapDef {
	pub secret: bool,
}

impl SpawnFrom<ExitMapDef> for ExitMap {
	fn spawn(component: &ExitMapDef, _accessor: ComponentAccessor, resources: &Resources) -> Self {
		let exits = <Read<SpawnContext<MapExits>>>::fetch(resources);

		ExitMap(if component.secret {
			exits.0.secret_exit.clone()
		} else {
			exits.0.exit.clone()
		})
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapExits {
	pub exit: String,
	pub secret_exit: String,
}

pub fn exit_switch_use(resources: &mut Resources) -> impl Runnable {
	let (mut handler_set, mut registry) =
		<(Write<SpawnMergerHandlerSet>, Write<Registry<String>>)>::fetch_mut(resources);

	registry.register::<ExitMap>("ExitMap".into());
	handler_set.register_spawn::<ExitMapDef, ExitMap>();
	handler_set.register_clone::<ExitSwitchUse>();

	SystemBuilder::new("exit_switch_use")
		.read_resource::<AssetStorage>()
		.read_resource::<Sender<String>>()
		.read_resource::<GameTime>()
		.with_query(<(&UseEvent, &ExitSwitchUse)>::query())
		.with_query(<(&LinedefRef, &ExitMap)>::query().filter(!component::<SwitchActive>()))
		.with_query(<&mut MapDynamic>::query())
		.build(move |command_buffer, world, resources, queries| {
			let (asset_storage, command_sender, game_time) = resources;
			let (mut world2, world) = world.split_for_query(&queries.2);

			for (event, exit_switch_use) in queries.0.iter(&world) {
				if let Ok((linedef_ref, ExitMap(next_map))) = queries.1.get(&world, event.entity) {
					let map_dynamic = queries
						.2
						.get_mut(&mut world2, linedef_ref.map_entity)
						.unwrap();
					let map = asset_storage.get(&map_dynamic.map).unwrap();

					command_sender.send(format!("change {}", next_map)).ok();

					switch::activate(
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
