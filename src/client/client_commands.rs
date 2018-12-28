use crate::client::Client;
use crate::commands::CommandList;

lazy_static! {
	pub static ref COMMANDS: CommandList<Client> = { CommandList::new()
		.add("cmdlist", |client: &mut Client, _args: Vec<String>| COMMANDS.print_commands())
		.add("connect", |client: &mut Client, args: Vec<String>| client.connect(&args[1]))
		//.add("cvarlist".to_owned(), Command::new(|client: &mut Client, args: Vec<String>| client.print_variables()));
		.add("quit", |client: &mut Client, _args: Vec<String>| client.quit())
	};
}
