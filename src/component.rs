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

#[derive(Default)]
pub struct EntityComponents(FnvHashMap<TypeId, Box<dyn DynComponent>>);

impl EntityComponents {
	pub fn new() -> EntityComponents {
		EntityComponents(FnvHashMap::default())
	}

	pub fn with_component<T: Component + Clone>(mut self, component: T) -> Self {
		self.0.insert(TypeId::of::<T>(), Box::from(component));
		self
	}

	pub fn add_to_entity(&self, entity: Entity, command_buffer: &mut CommandBuffer) {
		for dyn_component in self.0.values() {
			(*dyn_component).add_to_entity(entity, command_buffer);
		}
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}
}
