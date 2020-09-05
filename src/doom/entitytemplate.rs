use crate::common::{
	assets::{Asset, DataSource},
	component::EntityComponents,
};

pub struct EntityTemplate {
	pub name: Option<&'static str>,
	pub type_id: Option<EntityTypeId>,
	pub components: EntityComponents,
}

impl Asset for EntityTemplate {
	type Data = Self;
	type Intermediate = Self;
	const NAME: &'static str = "EntityTemplate";

	fn import(_name: &str, _source: &dyn DataSource) -> anyhow::Result<Self::Intermediate> {
		unimplemented!();
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityTypeId {
	Linedef(u16),
	Sector(u16),
	Thing(u16),
}
