mod client;
mod commands;

use std::convert::TryFrom;
use std::error::Error;
use std::net::{Ipv4Addr, Ipv6Addr};

use crate::commands::CommandDispatcher;
use crate::net::Socket;
use crate::protocol::{ClientPacket, ServerPacket, ClientConnectionlessPacket, ServerConnectionlessPacket};
use crate::server::client::ServerClient;
pub use crate::server::commands::COMMANDS;

pub fn server_main(dispatcher: CommandDispatcher) {
	let mut server = Server::new(dispatcher).unwrap();
	server.run();
}

pub struct Server {
	dispatcher: CommandDispatcher,
	socket: Socket,
	
	session: Option<ServerSession>,
	should_quit: bool,
}

impl Server {
	fn new(dispatcher: CommandDispatcher) -> Result<Server, Box<dyn Error>> {
		let socket = match Socket::new(Ipv4Addr::UNSPECIFIED, Ipv6Addr::UNSPECIFIED, 40011) {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!("Could not create server socket: {}", err)));
			}
		};
		
		info!("Server socket mode: {}", socket.mode());
		
		Ok(Server {
			dispatcher,
			socket,
			
			session: None,
			should_quit: false,
		})
	}
	
	fn run(&mut self) {
		while !self.should_quit {
			// Receive network packets
			if self.session.is_some() {
				while let Some((packet, addr)) = self.socket.next() {
					match ClientPacket::try_from(packet) {
						Ok(packet) => {
							println!("Server: {:?}, {}", packet, addr);
							
							if let ClientPacket::Connectionless(ClientConnectionlessPacket::GetStatus) = packet {
								let packet = ServerPacket::Connectionless(ServerConnectionlessPacket::ConnectResponse);
								self.socket.send_to(packet.into(), addr);
							}
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
			}
			
			// Execute console commands
			while let Some(args) = self.dispatcher.next(self.session.is_none()) {
				COMMANDS.execute(args, self);
				
				if self.should_quit {
					return;
				}
			}
			
			if self.session.is_none() {
				continue;
			}
			

		}
	}
	
	pub fn quit(&mut self) {
		self.should_quit = true;
	}
	
	pub fn shutdown(&mut self) {
		self.session = None;
	}
}

struct ServerSession {
	
}
