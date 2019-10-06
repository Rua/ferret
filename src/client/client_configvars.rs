use crate::configvars::{ConfigVariable, ConfigVariableT};


pub struct ClientConfigVars {
	pub cl_timeout: ConfigVariable<u64>,
}

impl ClientConfigVars {
	pub fn new() -> ClientConfigVars {
		ClientConfigVars {
			cl_timeout: ConfigVariable::new("cl_timeout", 10, None),
		}
	}

	pub fn refs(&self) -> Vec<&dyn ConfigVariableT> {
		vec![
			&self.cl_timeout,
		]
	}
}
