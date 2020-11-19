use crate::{
	common::{
		assets::AssetHandle,
		spawn::{ComponentAccessor, SpawnContext, SpawnFrom},
	},
	doom::state::StateName,
};
use legion::{systems::ResourceSet, Read, Resources, World};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct EntityTemplate {
	pub name: Option<&'static str>,
	pub type_id: Option<EntityTypeId>,
	pub world: World,
	pub touch: World,
	pub states: HashMap<StateName, Vec<World>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityTypeId {
	Linedef(u16),
	Sector(u16),
	Thing(u16),
}

#[derive(Clone, Debug)]
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

#[derive(Debug, Default)]
pub struct WeaponTemplate {
	pub name: Option<&'static str>,
	pub states: HashMap<StateName, Vec<World>>,
}
