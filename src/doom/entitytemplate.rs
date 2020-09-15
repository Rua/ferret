use crate::common::{assets::Asset, component::EntityComponents};

pub struct EntityTemplate {
	pub name: Option<&'static str>,
	pub type_id: Option<EntityTypeId>,
	pub components: EntityComponents,
}

impl Asset for EntityTemplate {
	const NAME: &'static str = "EntityTemplate";
	const NEEDS_PROCESSING: bool = false;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityTypeId {
	Linedef(u16),
	Sector(u16),
	Thing(u16),
}
