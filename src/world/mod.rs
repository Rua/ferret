mod component;

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;

use self::component::Component;


pub struct World {
	entities: Vec<Rc<Entity>>,
}

impl World {
	pub fn new() -> World {
		World {
			entities: Vec::new(),
		}
	}
	
	pub fn spawn_entity(&mut self) -> Rc<Entity> {
		let entity = Rc::new(Entity::new());
		self.entities.push(entity.clone());
		entity
	}
	
	pub fn entities<'a>(&'a mut self) -> impl Iterator<Item = Rc<Entity>> + 'a {
		self.entities.iter().cloned()
	}
}

#[derive(Debug)]
pub struct Entity {
	components: Vec<Box<dyn Component>>,
	component_types: HashMap<TypeId, usize>,
}

impl Entity {
    fn new() -> Entity {
        Entity {
            components: Vec::new(),
            component_types: HashMap::new(),
        }
    }
    
    fn add_component<T: Component>(&mut self, component: T) {
        self.component_types.insert(TypeId::of::<T>(), self.components.len());
        self.components.push(Box::from(component));
    }
    
    fn get_component<T: Component>(&self) -> Option<&T> {
        match self.component_types.get(&TypeId::of::<T>()) {
            Some(index) => self.components[*index].downcast_ref::<T>(),
            None => None,
        }
    }
}
