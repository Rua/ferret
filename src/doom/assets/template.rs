use crate::{
	common::{
		assets::{AssetHandle, AssetStorage, ImportData},
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom},
	},
	doom::{
		data::{AMMO, LINEDEFS, MOBJS, SECTORS, UI, WEAPONS},
		game::state::StateName,
	},
};
use anyhow::Context;
use legion::{systems::ResourceSet, Read, Resources, World};
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct EntityTemplate {
	pub name: Option<&'static str>,
	pub world: World,
	pub touch: World,
	pub r#use: World,
	pub states: HashMap<StateName, Vec<World>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityTemplateRef(pub AssetHandle<EntityTemplate>);

#[derive(Clone, Copy, Debug, Default)]
pub struct EntityTemplateRefDef;

impl SpawnFrom<EntityTemplateRefDef> for EntityTemplateRef {
	fn spawn(
		_component: &EntityTemplateRefDef,
		_accessor: ComponentAccessor,
		resources: &Resources,
	) -> EntityTemplateRef {
		let template_handle = <Read<SpawnContext<AssetHandle<EntityTemplate>>>>::fetch(resources);
		EntityTemplateRef(template_handle.0.clone())
	}
}

pub fn import_entity(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let func = MOBJS
		.get(path.as_str())
		.or_else(|| LINEDEFS.get(path.as_str()))
		.or_else(|| SECTORS.get(path.as_str()))
		.or_else(|| UI.get(path.as_str()))
		.with_context(|| format!("EntityTemplate \"{}\" not found", path))?;
	let template = func(asset_storage);
	Ok(Box::new(template))
}

#[derive(Debug, Default)]
pub struct WeaponTemplate {
	pub name: &'static str,
	pub ammo: Option<WeaponAmmo>,
	pub states: HashMap<StateName, Vec<World>>,
}

#[derive(Clone, Debug)]
pub struct WeaponAmmo {
	pub handle: AssetHandle<AmmoTemplate>,
	pub count: i32,
}

pub fn import_weapon(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let func = WEAPONS
		.get(path.as_str())
		.with_context(|| format!("WeaponTemplate \"{}\" not found", path))?;
	let template = func(asset_storage);
	Ok(Box::new(template))
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AmmoTemplate {
	pub name: &'static str,
}

pub fn import_ammo(
	path: &RelativePath,
	asset_storage: &mut AssetStorage,
) -> anyhow::Result<Box<dyn ImportData>> {
	let func = AMMO
		.get(path.as_str())
		.with_context(|| format!("AmmoTemplate \"{}\" not found", path))?;
	let template = func(asset_storage);
	Ok(Box::new(template))
}
