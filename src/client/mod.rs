mod audio;
mod client_commands;
mod client_configvars;
mod input;
mod video;
mod vulkan;

use sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use specs::{Entity, World, WorldExt};
use specs::world::Builder;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::io::{Cursor, Read, Write};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};
use std::ops::DerefMut;
use std::panic;
use std::panic::AssertUnwindSafe;
use std::process;
use std::rc::{Rc, Weak};
use std::sync::{Mutex, mpsc, mpsc::Receiver, mpsc::Sender};
use std::time::{Duration, Instant};

use crate::client::audio::Audio;
use crate::client::client_commands::COMMANDS;
use crate::client::client_configvars::ClientConfigVars;
use crate::client::input::Input;
use crate::client::video::Video;
use crate::commands;
use crate::commands::CommandSender;
//use crate::configvars::ConfigVariableT;
use crate::net::{Addr, SequencedChannel, Socket};
use crate::protocol::{ClientMessage, Packet, ServerMessage, TryRead};
use crate::server;
use crate::stdin;


pub fn client_main() {
	//let mut local_server = LocalServer::new().unwrap();
	//local_server.start().unwrap();

	//let mut console = Console::new();

	//let mut loader = WadLoader::new();
	//loader.add("doom.wad").unwrap();

	//let palette = doomtypes::palette::from_wad("PLAYPAL", &mut loader).unwrap();
	//let sprite = doomtypes::sprite::from_wad("TROO", &mut loader, &palette).unwrap();

	//let num = wadloader.num_for_name("STBAR").unwrap();
	//let mut data = wadloader.read_lump(num);
	//println!("{:?}", data);
	//let texture = video::Texture::from_patch(&mut data, &video.palette);

	let (server_sender, client_receiver) = mpsc::channel::<Vec<u8>>();
	let (client_sender, server_receiver) = mpsc::channel::<Vec<u8>>();

	let server_thread = std::thread::Builder::new()
		.name("server".to_owned())
		.spawn(move || {
			if let Err(_) = panic::catch_unwind(AssertUnwindSafe(|| {
				server::server_main(server_sender, server_receiver)
			})) {
				process::exit(1);
			}
		});

	let server_thread = match server_thread {
		Ok(val) => val,
		Err(err) => {
			error!("Could not start server thread: {}", err);
			return
		}
	};

	let mut client = Client::new(client_sender, client_receiver).unwrap();

	let mut old_time = Instant::now();
	let mut new_time = Instant::now();
	let mut delta = new_time - old_time;

	while !client.should_quit {
		// Busy-loop until there is at least a millisecond of delta
		while {
			new_time = Instant::now();
			delta = new_time - old_time;
			delta.as_millis() < 1
		} {}

		client.frame(delta);
		old_time = new_time;
	}

	debug!("Client thread terminated.");
	server_thread.join().ok();

	//local_server.quit().unwrap();
	//local_server.quit_and_wait().unwrap();
}

pub struct Client {
	audio: Audio,
	command_sender: CommandSender,
	command_receiver: Receiver<Vec<String>>,
	configvars: ClientConfigVars,
	//configvar_refs: Vec<&'static dyn ConfigVariableT>,
	event_pump: EventPump,
	input: Input,
	socket: Rc<Socket>,
	video: Video,

	connection: Option<ClientConnection>,
	real_time: Instant,
	should_quit: bool,
}

impl Client {
	pub fn new(sender: Sender<Vec<u8>>, receiver: Receiver<Vec<u8>>) -> Result<Client, Box<dyn Error>> {
		let configvars = ClientConfigVars::new();

		let socket = match Socket::new(Ipv4Addr::UNSPECIFIED, Ipv6Addr::UNSPECIFIED, 0, sender, receiver) {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!("Could not create client socket: {}", err)));
			}
		};

		info!("Client socket mode: {}", socket.mode());

		let (command_sender, command_receiver) = mpsc::channel();
		let command_sender = CommandSender::new(command_sender);

		match stdin::spawn(command_sender.clone()) {
			Ok(_) => (),
			Err(err) => {
				return Err(Box::from(format!("Could not start stdin thread: {}", err)));
			}
		};

		let sdl = match sdl2::init() {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!("Could not initialise SDL: {}", err)));
			}
		};

		let video = match Video::init(&sdl) {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!("Could not initialise video system: {}", err)));
			}
		};

		let audio = match Audio::init() {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!("Could not initialise audio system: {}", err)));
			}
		};

		let input = Input::init();

		let event_pump = match sdl.event_pump() {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!("Could not start event loop: {}", err)));
			}
		};

		Ok(Client {
			audio,
			command_sender,
			command_receiver,
			configvars,
			//configvar_refs,
			event_pump,
			input,
			socket,
			video,

			connection: None,
			real_time: Instant::now(),
			should_quit: false,
		})
	}

	pub fn frame(&mut self, delta: Duration) {
		self.real_time += delta;

		for event in self.event_pump.poll_iter() {
			match event {
				Event::Quit {..} => self.command_sender.send("quit"),
				_ => {},
			}
		}

		// Execute console commands
		while let Some(args) = self.command_receiver.try_iter().next() {
			COMMANDS.execute(args, self);
		}

		// Receive network packets
		while let Some((data, addr)) = self.socket.next() {
			self.handle_packet(data, addr);
		}

		if self.should_quit {
			return;
		}

		// Check for timeout
		if let Some(connection) = &mut self.connection {
			if let ConnectionState::Connected(_) = &mut connection.state {
				if (self.real_time - connection.last_packet_received_time).as_secs() >= *self.configvars.cl_timeout.get() {
					error!("Server connection timed out.");
					self.disconnect();
				}
			}
		}

		self.send_update();

		if let Some(connection) = &mut self.connection {
			match connection.state {
				ConnectionState::Connecting(ref mut last_packet_time) => {
					// If it has been longer than 3 seconds from the last, resend the packet
					if (self.real_time - *last_packet_time).as_secs() >= 3 {
						let packet = Packet::Unsequenced(vec![ClientMessage::Connect]);
						self.socket.send_to(packet.into(), connection.server_addr);
						*last_packet_time = self.real_time;
					}
				},
				_ => (),
			}
		}

		self.video.draw_frame().unwrap();
	}

	pub fn quit(&mut self) {
		self.should_quit = true;

		let packet = Packet::Unsequenced(vec![ClientMessage::RCon("quit".to_owned())]);
		self.socket.send_to(packet.into(), Addr::Local);
	}

	pub fn set_configvar(&mut self, name: &str, value: &str) {
		for cvar in self.configvars.refs() {
			if cvar.name() == name {
				cvar.set_string(value);
				break;
			}
		}
	}

	pub fn list_configvars(&mut self) {
		for cvar in self.configvars.refs() {
			info!("{} \"{}\"", cvar.name(), cvar);
		}
	}

	pub fn connect(&mut self, server_name: &str) {
		let connection = match ClientConnection::new(server_name, &self.socket) {
			Ok(val) => val,
			Err(err) => {
				error!("Could not connect: {}", err);
				return
			}
		};

		self.connection = Some(connection);
	}

	pub fn disconnect(&mut self) {
		if let Some(connection) = &mut self.connection {
			if let ConnectionState::Connected(channel) = &mut connection.state {
				// TODO: send disconnect
			}
		}

		self.connection = None;
	}

	fn handle_packet(&mut self, data: Vec<u8>, addr: Addr) {
		let packet: Packet<ServerMessage> = match Packet::try_from(data) {
			Ok(packet) => packet,
			Err(err) => {
				warn!(
					"Client received a malformed packet from {}: {}", addr, err);
				return;
			},
		};

		match packet {
			Packet::Unsequenced(messages) => {
				println!("Client received from {}: {:?}", addr, messages);

				for message in messages {
					self.handle_unsequenced_message(message, addr);
				}
			},
			Packet::Sequenced(packet) => {
				if let Some(connection) = &mut self.connection {
					if let ConnectionState::Connected(channel) = &mut connection.state {
						if addr == connection.server_addr {
							if let Some(messages) = channel.process(packet) {
								println!("Client received from server: {:?}", messages);
								connection.last_packet_received_time = self.real_time;

								for message in messages {
									self.handle_sequenced_message(message, addr);
								}
							}
						}
					}
				}
			},
		}
	}

	fn handle_unsequenced_message(&mut self, message: ServerMessage, addr: Addr) {
		match message {
			ServerMessage::ConnectResponse => {
				if let Some(connection) = &mut self.connection {
					match connection.state {
						ConnectionState::Connecting(_) => {
							if addr == connection.server_addr {
								let channel = SequencedChannel::new(self.socket.clone(), addr);
								connection.state = ConnectionState::Connected(channel);
								connection.last_packet_received_time = self.real_time;
							} else {
								info!("Received connection response from wrong address");
							}
						},
						ConnectionState::Connected(_) => {
							info!("Received duplicate connection response");
						}
					}
				}
			},
			ServerMessage::Disconnect => {
				self.disconnect();
			},
			_ => unimplemented!(),
		}
	}

	fn handle_sequenced_message(&mut self, message: ServerMessage, addr: Addr) {
		match message {
			ServerMessage::DeleteEntity(net_id) => {
			}
			ServerMessage::NewEntity(net_id) => {
				if let Some(connection) = &mut self.connection {
					let entity = connection.world.create_entity().build();

					if connection.lookup_id_entity.insert(net_id, entity).is_some() {
						// TODO: Received a duplicate id!
					}
				}
			},
			_ => unimplemented!(),
		}
	}

	fn send_update(&mut self) {
		if let Some(connection) = &mut self.connection {
			if (self.real_time - connection.last_packet_sent_time).as_secs() < 1 {
				return;
			}

			if let ConnectionState::Connected(channel) = &mut connection.state {
				channel.send(Vec::new());
				connection.last_packet_sent_time = self.real_time;
			}
		}
	}
}

struct ClientConnection {
	last_packet_sent_time: Instant,
	last_packet_received_time: Instant,
	lookup_id_entity: HashMap<u32, Entity>,
	server_name: String,
	server_addr: Addr,
	state: ConnectionState,
	world: World,
}

impl ClientConnection {
	fn new(server_name: &str, socket: &Socket) -> Result<ClientConnection, Box<dyn Error>> {
		let addr = if server_name == "." {
			Addr::Local
		} else {
			let mut socket_addrs = server_name.to_socket_addrs();

			if socket_addrs.is_err() {
				socket_addrs = (server_name, 40011).to_socket_addrs();
			}

			let socket_addrs: Vec<SocketAddr> = socket_addrs?.filter(socket.filter_supported()).collect();

			if socket_addrs.is_empty() {
				return Err(Box::from("Host name not found"));
			}

			info!("{} resolved to {}", server_name, socket_addrs[0].ip());
			socket_addrs[0].into()
		};

		let time = Instant::now() - Duration::new(9999, 0);

		Ok(ClientConnection {
			last_packet_sent_time: time,
			last_packet_received_time: time,
			lookup_id_entity: HashMap::new(),
			server_name: server_name.to_owned(),
			server_addr: addr,
			state: ConnectionState::Connecting(time),
			world: World::new(),
		})
	}
}

enum ConnectionState {
	Connected(SequencedChannel<ClientMessage, ServerMessage>),
	Connecting(Instant),
}
