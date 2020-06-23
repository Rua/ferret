use crate::assets::{Asset, DataSource};
use fnv::FnvHashMap;
use legion::{
	prelude::{CommandBuffer, Entity},
	storage::Component,
};
use std::any::TypeId;

pub trait DynComponent: Send + Sync {
	fn add_to_entity(&self, entity: Entity, command_buffer: &mut CommandBuffer);
}

impl<T: Component + Clone> DynComponent for T {
	fn add_to_entity(&self, entity: Entity, command_buffer: &mut CommandBuffer) {
		command_buffer.add_component(entity, self.clone());
	}
}

pub struct EntityTemplate {
	components: FnvHashMap<TypeId, Box<dyn DynComponent>>,
}

impl EntityTemplate {
	pub fn new() -> EntityTemplate {
		EntityTemplate {
			components: FnvHashMap::default(),
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

	pub fn add_to_entity(&self, entity: Entity, command_buffer: &mut CommandBuffer) {
		for dyn_component in self.components.values() {
			(*dyn_component).add_to_entity(entity, command_buffer);
		}
	}

	pub fn len(&self) -> usize {
		self.components.len()
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
