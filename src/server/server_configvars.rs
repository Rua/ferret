use crate::configvars::{ConfigVariable, ConfigVariableT};


pub struct ServerConfigVars {
	pub sv_port: ConfigVariable<u16>,
	pub sv_timeout: ConfigVariable<u64>,
}

impl ServerConfigVars {
	pub fn new() -> ServerConfigVars {
		ServerConfigVars {
			sv_port: ConfigVariable::new("sv_port", 40011, None),
			sv_timeout: ConfigVariable::new("sv_timeout", 10, None),
		}
	}

	pub fn refs(&self) -> Vec<&dyn ConfigVariableT> {
		vec![
			&self.sv_port,
			&self.sv_timeout,
		]
	}
}
