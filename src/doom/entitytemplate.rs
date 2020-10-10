use crate::{
	common::{assets::AssetHandle, component::EntityComponents},
	doom::state::{StateDef, StateName},
};
use std::collections::HashMap;

#[derive(Default)]
pub struct EntityTemplate {
	pub name: Option<&'static str>,
	pub type_id: Option<EntityTypeId>,
	pub states: HashMap<StateName, StateDef>,
	pub components: EntityComponents,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityTypeId {
	Linedef(u16),
	Sector(u16),
	Thing(u16),
}

#[derive(Clone, Debug)]
pub struct EntityTemplateRef(pub AssetHandle<EntityTemplate>);
