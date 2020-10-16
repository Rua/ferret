use crate::{
	common::assets::AssetHandle,
	doom::state::{StateDef, StateName},
};
use legion::World;
use std::collections::HashMap;

#[derive(Default)]
pub struct EntityTemplate {
	pub name: Option<&'static str>,
	pub type_id: Option<EntityTypeId>,
	pub states: HashMap<StateName, Vec<StateDef>>,
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
