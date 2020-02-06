use crate::doom::input::UserCommand;
use specs::Entity;

#[derive(Default)]
pub struct Client {
	pub entity: Option<Entity>,
	pub command: UserCommand,
	pub previous_command: UserCommand,
}
