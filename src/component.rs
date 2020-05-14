use crate::assets::{Asset, DataSource};
use legion::{
	prelude::{Entity, World},
	storage::Component,
	world::EntityMutationError,
};
use std::{any::TypeId, collections::HashMap};

pub trait DynComponent: Send + Sync {
	fn add_to_entity<'a, 'b>(
		&'a self,
		entity: Entity,
		world: &'b mut World,
	) -> Result<(), EntityMutationError>;
}

impl<T: Component + Clone> DynComponent for T {
	fn add_to_entity<'a, 'b>(
		&'a self,
		entity: Entity,
		world: &'b mut World,
	) -> Result<(), EntityMutationError> {
		world.add_component(entity, self.clone())?;
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

	pub fn add_component<T: Component + Clone>(&mut self, component: T) {
		self.components
			.insert(TypeId::of::<T>(), Box::from(component));
	}

	pub fn with_component<T: Component + Clone>(mut self, component: T) -> Self {
		self.add_component(component);
		self
	}

	pub fn add_to_entity<'a, 'b>(
		&'a self,
		entity: Entity,
		world: &'b mut World,
	) -> Result<(), EntityMutationError> {
		for dyn_component in self.components.values() {
			//dyn_component.add_to_entity(entity, world)?;
		}

		Ok(())
	}
}

impl Asset for EntityTemplate {
	type Data = Self;
	type Intermediate = Self;
	const NAME: &'static str = "EntityTemplate";

	fn import(_name: &str, _source: &impl DataSource) -> anyhow::Result<Self::Intermediate> {
		unimplemented!();
	}
}
