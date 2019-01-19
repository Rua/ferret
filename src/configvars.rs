use std::borrow::BorrowMut;
use std::cell::{Cell, Ref, RefCell};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;


pub struct ConfigVariable<T> {
	name: &'static str,
	value: RefCell<T>,
	validator: Option<Box<dyn Fn(&T) -> bool + Sync>>,
	modified: Cell<bool>,
}

impl<T: PartialEq> ConfigVariable<T> {
	pub fn new(name: &'static str, default: T, mut validator: Option<Box<dyn Fn(&T) -> bool + Sync>>) -> ConfigVariable<T> {
		assert!(validator.is_none() || validator.as_mut().unwrap()(&default));
		
		ConfigVariable {
			name: name,
			value: RefCell::new(default),
			validator,
			modified: Cell::new(false),
		}
	}

	pub fn get(&self) -> Ref<T> {
		self.value.borrow()
	}

	fn set(&self, newvalue: T) {
		if *self.value.borrow() != newvalue && (self.validator.is_none() || self.validator.as_ref().unwrap()(&newvalue)) {
			self.value.replace(newvalue);
			self.modified.set(true);
		}
	}
}

impl<T: fmt::Display> fmt::Display for ConfigVariable<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.borrow().fmt(f)
    }
}

pub trait ConfigVariableT: fmt::Display {
	fn name(&self) -> &'static str;
	fn set_string(&self, value: &str);
}

impl<T: PartialEq + FromStr + fmt::Display> ConfigVariableT for ConfigVariable<T>
where <T as FromStr>::Err: std::fmt::Debug {
	fn name(&self) -> &'static str {
		self.name
	}
	
	fn set_string(&self, value: &str) {
		self.set(value.parse().unwrap())
	}
}

/*pub struct ConfigVariables {
	variables: HashMap<String, ConfigVariable>,
}

impl ConfigVariables {
	pub fn new<I>(iter: I) -> ConfigVariables
	where I: IntoIterator<Item = ConfigVariable> {
		let mut variables = HashMap::new();
		
		for item in iter.into_iter() {
			if let Some(item) = variables.insert(item.name.clone(), item) {
				panic!("Duplicate variable name: {}", item.name);
			}
		}
		
		ConfigVariables {
			variables,
		}
	}
	
	pub fn get<T: Clone>(&self, key: &str) -> Option<&T>
	where ConfigVariable: ValueAccess<T> {
		self.variables.get(key).map(ValueAccess::get)
	}

	fn set<T: Clone>(&mut self, key: &str, newvalue: T)
	where ConfigVariable: ValueAccess<T> {
		match self.variables.get_mut(key) {
			Some(variable) => variable.set(newvalue),
			None => (),
		}
	}

	fn set_string(&mut self, key: &str, string: &str) -> Result<(), Box<dyn Error>> {
		match self.variables.get_mut(key) {
			Some(variable) => variable.set_string(string),
			None => Ok(()),
		}
	}
}*/


/*
impl<T: FromStr + ToString> ConsoleVariableT for ConsoleVariable<T> {
	fn print_value_str(&self) {
		info!("\"{}\" = \"{}\"", self.name, self.value.borrow().to_string());
		//if let Some(var) = self.upgrade() {
	}
	
	fn set_value_str(&self, newvalue: &str) {
		if let Ok(value) = newvalue.parse::<T>() {
			self.set_value(value);
		}
		//if let Some(var) = self.upgrade() {
		// TODO: print message if parse fails
	}
}
*/
