mod server_commands;
mod server_configvars;

use nalgebra::Vector3;
use specs::{BitSet, Builder, Entity, Entities, Join, ReaderId, SystemData, World, WorldExt};
use specs::storage::{ComponentEvent, ReadStorage, WriteStorage};
use std::collections::hash_map::{Entry, HashMap};
use std::convert::TryFrom;
use std::error::Error;
use std::io::{Cursor, Read, Write};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::rc::{Rc, Weak};
use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender};
use std::time::{Duration, Instant};

use crate::commands;
use crate::commands::CommandSender;
use crate::components::{NetworkComponent, TransformComponent};
use crate::doom;
//use crate::configvars::ConfigVariables;
use crate::net::{Addr, SequencedChannel, Socket};
use crate::protocol::{ClientMessage, Packet, ServerMessage, TryRead};
use crate::server::server_commands::COMMANDS;
use crate::server::server_configvars::ServerConfigVars;


pub fn server_main(sender: Sender<Vec<u8>>, receiver: Receiver<Vec<u8>>) {
	let mut server = Server::new(sender, receiver).unwrap();

	let mut old_time = Instant::now();
	let mut new_time = Instant::now();
	let mut delta = new_time - old_time;

	while !server.should_quit {
		// Busy-loop until there is at least a millisecond of delta
		while {
			new_time = Instant::now();
			delta = new_time - old_time;
			delta.as_millis() < 1
		} {}

		server.frame(delta);
		old_time = new_time;
	}

	debug!("Server thread terminated.");
}

pub struct Server {
	command_sender: CommandSender,
	command_receiver: Receiver<Vec<String>>,
	configvars: ServerConfigVars,
	socket: Rc<Socket>,

	clients: HashMap<Addr, ServerClient>,
	real_time: Instant,
	session: ServerSession,
	should_quit: bool,
}

impl Server {
	fn new(sender: Sender<Vec<u8>>, receiver: Receiver<Vec<u8>>) -> Result<Server, Box<dyn Error>> {
		let configvars = ServerConfigVars::new();

		let socket = match Socket::new(Ipv4Addr::UNSPECIFIED, Ipv6Addr::UNSPECIFIED, *configvars.sv_port.get(), sender, receiver) {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!("Could not create server socket: {}", err)));
			}
		};

		info!("Server socket mode: {}", socket.mode());

		let (command_sender, command_receiver) = mpsc::channel();
		let command_sender = CommandSender::new(command_sender);

		Ok(Server {
			command_sender,
			command_receiver,
			configvars,
			socket,

			clients: HashMap::new(),
			real_time: Instant::now(),
			session: ServerSession::new("E1M1")?,
			should_quit: false,
		})
	}

	fn frame(&mut self, delta: Duration) {
		self.real_time += delta;

		// Receive network packets
		while let Some((data, addr)) = self.socket.next() {
			self.handle_packet(data, addr);
		}

		// Execute console commands
		while let Some(args) = self.command_receiver.try_iter().next() {
			COMMANDS.execute(args, self);

			if self.should_quit {
				return;
			}
		}

		// Check for timeout
		// Need to avoid borrowing the hashmap because we're modifying it
		for addr in self.clients.keys().cloned().collect::<Vec<_>>() {
			let client = &self.clients[&addr];

			if (self.real_time - client.last_packet_received_time).as_secs() >= *self.configvars.sv_timeout.get() {
				self.drop_client(addr, "timed out");
			}
		}

		self.send_updates();
	}

	pub fn quit(&mut self) {
		self.should_quit = true;
	}

	pub fn drop_client(&mut self, addr: Addr, reason: &str) {
		self.clients.remove(&addr);
		info!("Client {} {}", addr, reason);
	}

	fn handle_packet(&mut self, data: Vec<u8>, addr: Addr) {
		let packet: Packet<ClientMessage> = match Packet::try_from(data) {
			Ok(packet) => packet,
			Err(err) => {
				warn!("Server received a malformed packet from {}: {}", addr, err);
				return;
			},
		};

		match packet {
			Packet::Unsequenced(messages) => {
				debug!("Server received from {}: {:?}", addr, messages);

				for message in messages {
					self.handle_unsequenced_message(message, addr);
				}
			},
			Packet::Sequenced(packet) => {
				if let Some(client) = self.clients.get_mut(&addr) {
					if let Some(messages) = client.channel.process(packet) {
						debug!("Server received from client {}: {:?}", addr, messages);
						client.last_packet_received_time = self.real_time;

						for message in messages {
							self.handle_sequenced_message(message, addr);
						}
					}
				}
			},
		}
	}

	fn handle_unsequenced_message(&mut self, message: ClientMessage, addr: Addr) {
		match message {
			ClientMessage::Connect => {
				let client = match self.clients.entry(addr) {
					Entry::Occupied(item) => item.into_mut(),
					Entry::Vacant(item) => item.insert(ServerClient::new(self.socket.clone(), addr)),
				};

				client.last_packet_received_time = self.real_time;
				client.next_update_time = self.real_time;

				let packet = Packet::Unsequenced(vec![ServerMessage::ConnectResponse]);
				self.socket.send_to(packet.into(), addr);
			},
			ClientMessage::RCon(text) => {
				match commands::tokenize(&text) {
					Ok(args) => COMMANDS.execute(args, self),
					Err(err) => warn!(
						"Malformed command string in rcon: {}",
						err
					),
				}
			},
		}
	}

	fn handle_sequenced_message(&mut self, message: ClientMessage, addr: Addr) {
	}

	fn send_updates(&mut self) {
		for (_, client) in &mut self.clients {
			if self.real_time < client.next_update_time {
				continue;
			}

			let mut messages = Vec::new();

			let ignore = if let Some(reader_id) = &mut client.network_reader_id {
				let mut inserted = BitSet::new();
				let mut removed = BitSet::new();

				for event in self.session.world.read_storage::<NetworkComponent>().channel().read(reader_id) {
					match event {
						ComponentEvent::Inserted(id) => {
							inserted.add(*id);
						},
						ComponentEvent::Modified(id) => {},
						ComponentEvent::Removed(id) => {
							if !inserted.remove(*id) {
								removed.add(*id);
							}
						}
					}
				}

				for id in &removed {
					messages.push(ServerMessage::EntityDelete(id));
				}

				for id in &inserted {
					messages.push(ServerMessage::EntityNew(id));
				}

				removed
			} else {
				// There is no reader yet, so notify the client about all entities
				for (entity, _) in (&self.session.world.entities(), &self.session.world.read_storage::<NetworkComponent>()).join() {
					messages.push(ServerMessage::EntityNew(entity.id()));
				}

				client.network_reader_id = Some(self.session.world.write_storage::<NetworkComponent>().register_reader());
				BitSet::new()
			};

			if let Some(reader_id) = &mut client.transform_reader_id {
				let mut inserted = BitSet::new();
				let mut modified = BitSet::new();
				let mut removed = BitSet::new();

				for event in self.session.world.read_storage::<TransformComponent>().channel().read(reader_id) {
					match event {
						ComponentEvent::Inserted(id) => {
							inserted.add(*id);
						},
						ComponentEvent::Modified(id) => {
							modified.add(*id);
						},
						ComponentEvent::Removed(id) => {
							if !inserted.remove(*id) && !ignore.contains(*id) {
								removed.add(*id);
							}
							modified.remove(*id);
						}
					}
				}

				for id in &removed {
					messages.push(ServerMessage::ComponentDelete(id, 1));
				}

				for id in &inserted {
					messages.push(ServerMessage::ComponentNew(id, 1));
				}
			} else {
				// There is no reader yet, so notify the client about all entities
				for (entity, component) in (&self.session.world.entities(), &self.session.world.read_storage::<TransformComponent>()).join() {
					messages.push(ServerMessage::ComponentNew(entity.id(), 1));

					let mut msg = Cursor::new(Vec::new());
					component.write_delta(&mut msg).ok();
					messages.push(ServerMessage::ComponentDelta(entity.id(), 1, msg.into_inner()));
				}

				client.transform_reader_id = Some(self.session.world.write_storage::<TransformComponent>().register_reader());
			}

			client.channel.send(messages);
			client.next_update_time = self.real_time + Duration::from_millis(50);
		}
	}
}

struct ServerSession {
	world: World,
}

impl ServerSession {
	fn new(mapname: &str) -> Result<ServerSession, Box<dyn Error>> {
		let mut world = World::new();
		world.register::<NetworkComponent>();
		world.register::<TransformComponent>();

		let mut loader = doom::wad::WadLoader::new();
		loader.add("doom.wad")?;
		loader.add("doom.gwa")?;
		doom::map::spawn_map_entities(&mut world, mapname, &mut loader)?;

		Ok(ServerSession {
			world,
		})
	}

	fn world(&mut self) -> &mut World {
		&mut self.world
	}
}

pub struct ServerClient {
	channel: SequencedChannel<ServerMessage, ClientMessage>,
	last_packet_received_time: Instant,
	lookup_entity_id: HashMap<Entity, u32>,
	next_entity_id: u32,
	next_update_time: Instant,

	network_reader_id: Option<ReaderId<ComponentEvent>>,
	transform_reader_id: Option<ReaderId<ComponentEvent>>,
}

impl ServerClient {
	fn new(socket: Rc<Socket>, addr: Addr) -> ServerClient {
		ServerClient {
			channel: SequencedChannel::new(socket, addr),
			last_packet_received_time: Instant::now(),
			lookup_entity_id: HashMap::new(),
			next_entity_id: 1,
			next_update_time: Instant::now(),

			network_reader_id: None,
			transform_reader_id: None,
		}
	}
}
