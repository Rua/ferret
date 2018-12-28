mod client;
mod commands;

use std::convert::TryFrom;
use std::error::Error;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::{mpsc, mpsc::Receiver};
use std::time::{Duration, Instant};

use crate::commands::CommandSender;
use crate::net::Socket;
use crate::protocol::{ClientPacket, ServerPacket, ClientConnectionlessPacket, ServerConnectionlessPacket};
use crate::server::client::ServerClient;
pub use crate::server::commands::COMMANDS;

pub fn server_main() {
	let mut server = Server::new().unwrap();
	
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
	socket: Socket,
	
	clients: Vec<ServerClient>,
	session: Option<ServerSession>,
	should_quit: bool,
}

impl Server {
	fn new() -> Result<Server, Box<dyn Error>> {
		let socket = match Socket::new(Ipv4Addr::UNSPECIFIED, Ipv6Addr::UNSPECIFIED, 40011) {
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
			socket,
			
			clients: Vec::new(),
			session: None,
			should_quit: false,
		})
	}
	
	fn frame(&mut self, delta: Duration) {
		// Receive network packets
		while let Some((packet, addr)) = self.socket.next() {
			match ClientPacket::try_from(packet) {
				Ok(packet) => {
					self.process_packet(packet, addr)
				},
				Err(err) => {
					warn!(
						"received a malformed packet from {}: {}",
						addr,
						err,
					);
				},
			}
		}
		
		// Execute console commands
		while let Some(args) = self.command_receiver.try_iter().next() {
			COMMANDS.execute(args, self);
			
			if self.should_quit {
				return;
			}
		}
		
		if self.session.is_none() {
			return;
		}
	}
	
	pub fn quit(&mut self) {
		self.should_quit = true;
	}
	
	pub fn shutdown(&mut self) {
		self.session = None;
	}
	
	fn process_packet(&mut self, packet: ClientPacket, addr: SocketAddr) {
		println!("Server: {:?}, {}", packet, addr);
		
		match packet {
			ClientPacket::Connectionless(packet) => match packet {
				ClientConnectionlessPacket::Connect(_) => {
					// TODO: check if client is already in the list
					self.clients.push(ServerClient {} );
					
					let packet = ServerPacket::Connectionless(ServerConnectionlessPacket::ConnectResponse);
					self.socket.send_to(packet.into(), addr);
				},
				ClientConnectionlessPacket::GetInfo => unimplemented!(),
				ClientConnectionlessPacket::GetStatus => unimplemented!(),
				ClientConnectionlessPacket::RCon(args) => {
					COMMANDS.execute(args, self);
				},
			},
			_ => unimplemented!(),
		}
	}
}

struct ServerSession {
	
}
