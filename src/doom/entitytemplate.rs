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
	pub spawn_state: Option<StateName>,
	pub see_state: Option<StateName>,
	pub pain_state: Option<StateName>,
	pub melee_state: Option<StateName>,
	pub missile_state: Option<StateName>,
	pub death_state: Option<StateName>,
	pub xdeath_state: Option<StateName>,
	pub raise_state: Option<StateName>,
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
