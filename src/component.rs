use crate::assets::{Asset, DataSource};
use specs::{Component, Entity, World, WorldExt};
use std::{any::TypeId, collections::HashMap};

pub trait DynComponent: Send + Sync {
	fn add_to_entity(&self, entity: Entity, world: &World) -> Result<(), specs::error::Error>;
}

impl<T: Component + Clone + Send + Sync> DynComponent for T {
	fn add_to_entity(&self, entity: Entity, world: &World) -> Result<(), specs::error::Error> {
		world.write_component().insert(entity, self.clone())?;
		Ok(())
	}
}

pub struct EntityTemplate {
	components: HashMap<TypeId, Box<dyn DynComponent>>,
}

impl EntityTemplate {
	pub fn new() -> EntityTemplate {
		EntityTemplate {
			components: HashMap::new(),
		}
	}

	pub fn add_component<T: Component + Clone + Send + Sync>(&mut self, component: T) {
		self.components
			.insert(TypeId::of::<T>(), Box::from(component));
	}

	pub fn with_component<T: Component + Clone + Send + Sync>(mut self, component: T) -> Self {
		self.add_component(component);
		self
	}

	pub fn add_to_entity(&self, entity: Entity, world: &World) -> Result<(), specs::error::Error> {
		for dyn_component in self.components.values() {
			dyn_component.add_to_entity(entity, world)?;
		}

		Ok(())
	}
}

impl Asset for EntityTemplate {
	type Data = Self;
	type Intermediate = Self;
	const NAME: &'static str = "EntityTemplate";

	fn import(
		_name: &str,
		_source: &impl DataSource,
	) -> anyhow::Result<Self::Intermediate> {
		unimplemented!();
	}
}
