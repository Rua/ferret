mod commands;

use std::collections::hash_map::{Entry, HashMap};
use std::convert::TryFrom;
use std::error::Error;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::rc::Rc;
use std::sync::{mpsc, mpsc::Receiver};
use std::time::{Duration, Instant};

use crate::commands::CommandSender;
use crate::net::{SequencedChannel, Socket};
use crate::protocol::{ClientPacket, ClientConnectionlessPacket, ClientSequencedPacket, SequencedPacket, ServerConnectionlessPacket, ServerSequencedPacket};
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
	socket: Rc<Socket>,
	
	clients: HashMap<SocketAddr, ServerClient>,
	real_time: Instant,
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
			
			clients: HashMap::new(),
			real_time: Instant::now(),
			session: None,
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
			
			if (self.real_time - client.last_packet_received_time).as_secs() >= 10 {
				self.drop_client(addr, "timed out");
			}
		}
		
		self.send_updates();
	}
	
	pub fn quit(&mut self) {
		self.should_quit = true;
	}
	
	pub fn shutdown(&mut self) {
		self.session = None;
	}
	
	pub fn drop_client(&mut self, addr: SocketAddr, reason: &str) {
		self.clients.remove(&addr);
		info!("Client {} {}", addr, reason);
	}
	
	fn handle_packet(&mut self, data: Vec<u8>, addr: SocketAddr) {
		match ClientPacket::try_from(data) {
			Ok(packet) => match packet {
				ClientPacket::Connectionless(packet) => {
					self.handle_connectionless_packet(packet, addr);
				},
				ClientPacket::Sequenced(packet) => {
					self.handle_sequenced_packet(packet, addr);
				},
			},
			Err(err) => {
				warn!(
					"Server received a malformed packet from {}: {}",
					addr,
					err,
				);
			},
		}
	}
	
	fn handle_connectionless_packet(&mut self, packet: ClientConnectionlessPacket, addr: SocketAddr) {
		println!("Server received from {}: {:?}", addr, packet);
		
		match packet {
			ClientConnectionlessPacket::Connect(_) => {
				let client = match self.clients.entry(addr) {
					Entry::Occupied(item) => item.into_mut(),
					Entry::Vacant(item) => item.insert(ServerClient::new(self.socket.clone(), addr)),
				};
				
				client.last_packet_received_time = self.real_time;
				client.next_update_time = self.real_time;
				
				let packet = ServerConnectionlessPacket::ConnectResponse;
				self.socket.send_to(packet.into(), addr);
			},
			ClientConnectionlessPacket::GetInfo => unimplemented!(),
			ClientConnectionlessPacket::GetStatus => unimplemented!(),
			ClientConnectionlessPacket::RCon(args) => {
				COMMANDS.execute(args, self);
			},
		}
	}
	
	fn handle_sequenced_packet(&mut self, packet: SequencedPacket, addr: SocketAddr) {
		if let Some(client) = self.clients.get_mut(&addr) {
			if let Some(data) = client.channel.process(packet) {
				client.last_packet_received_time = self.real_time;
				
				match ClientSequencedPacket::try_from(data) {
					Ok(packet) => {
						println!("Server received from {}: {:?}", addr, packet);
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
	}
	
	fn send_updates(&mut self) {
		for (_, client) in &mut self.clients {
			if self.real_time < client.next_update_time {
				continue;
			}
			
			let packet = ServerSequencedPacket {};
			client.channel.send(packet.into());
			client.next_update_time = self.real_time + Duration::from_millis(50);
		}
	}
}

struct ServerSession {
	
}

pub struct ServerClient {
	channel: SequencedChannel,
	last_packet_received_time: Instant,
	next_update_time: Instant,
}

impl ServerClient {
	fn new(socket: Rc<Socket>, addr: SocketAddr) -> ServerClient {
		ServerClient {
			channel: SequencedChannel::new(socket, addr),
			last_packet_received_time: Instant::now(),
			next_update_time: Instant::now(),
		}
	}
}
