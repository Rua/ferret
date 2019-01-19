use crate::server::Server;
use crate::commands::CommandList;

lazy_static! {
	pub static ref COMMANDS: CommandList<Server> = { CommandList::new()
		//.add("killserver", |server: &mut Server, _args: Vec<String>| server.shutdown())
		.add("quit", |server: &mut Server, _args: Vec<String>| server.quit())
	};
}
