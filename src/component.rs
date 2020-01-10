use specs::{Component, Entity, World, WorldExt};

pub trait DynComponent: Send + Sync {
    fn add_to_entity(&self, entity: Entity, world: &World) -> Result<(), specs::error::Error>;
}

impl<T: Component + Clone + Send + Sync> DynComponent for T {
    fn add_to_entity(&self, entity: Entity, world: &World) -> Result<(), specs::error::Error> {
        world.write_component().insert(entity, self.clone())?;
        Ok(())
    }
}
