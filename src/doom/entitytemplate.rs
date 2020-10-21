use crate::{
	common::{assets::AssetHandle, resources_merger::FromWithResources},
	doom::{
		map::spawn::SpawnContext,
		state::{StateInfo, StateName},
	},
};
use legion::{systems::ResourceSet, Read, Resources, World};
use std::collections::HashMap;

#[derive(Default)]
pub struct EntityTemplate {
	pub name: Option<&'static str>,
	pub type_id: Option<EntityTypeId>,
	pub states: HashMap<StateName, Vec<StateInfo>>,
	pub world: World,
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

impl FromWithResources<EntityTemplateRefDef> for EntityTemplateRef {
	fn from_with_resources(
		_src_component: &EntityTemplateRefDef,
		resources: &Resources,
	) -> EntityTemplateRef {
		let spawn_context = <Read<SpawnContext>>::fetch(resources);
		EntityTemplateRef(spawn_context.template_handle.clone())
	}
}
