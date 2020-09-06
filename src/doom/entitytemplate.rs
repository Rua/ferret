use crate::common::{
	assets::{Asset, AssetStorage, ImportData},
	component::EntityComponents,
};

pub struct EntityTemplate {
	pub name: Option<&'static str>,
	pub type_id: Option<EntityTypeId>,
	pub components: EntityComponents,
}

impl Asset for EntityTemplate {
	type Data = Self;
	const NAME: &'static str = "EntityTemplate";

	fn import(
		_name: &str,
		_asset_storage: &mut AssetStorage,
	) -> anyhow::Result<Box<dyn ImportData>> {
		unimplemented!();
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityTypeId {
	Linedef(u16),
	Sector(u16),
	Thing(u16),
}
