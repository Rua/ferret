mod audio;
mod client_commands;
mod input;
mod video;
mod vulkan;

use sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use std::convert::TryFrom;
use std::error::Error;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::panic;
use std::panic::AssertUnwindSafe;
use std::sync::{mpsc, mpsc::Receiver};
use std::thread::Builder;
use std::time::{Duration, Instant};

use crate::client::audio::Audio;
pub use crate::client::client_commands::COMMANDS;
use crate::client::input::Input;
use crate::client::video::Video;
use crate::commands;
use crate::commands::CommandSender;
use crate::net::Socket;
use crate::protocol::{ClientConnectionlessPacket, ClientPacket, ServerConnectionlessPacket, ServerPacket};
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
	
	let server_thread = Builder::new()
		.name("server".to_owned())
		.spawn(move || {
			match panic::catch_unwind(AssertUnwindSafe(|| {
				server::server_main()
			})) {
				Ok(()) => debug!("Server thread terminated."),
				Err(_) => (),
			}
		});
	
	let server_thread = match server_thread {
		Ok(val) => val,
		Err(err) => {
			error!("Could not start server thread: {}", err);
			return
		}
	};
	
	let mut client = Client::new().unwrap();
	
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
	event_pump: EventPump,
	input: Input,
	socket: Socket,
	video: Video,
	
	should_quit: bool,
}

impl Client {
	pub fn new() -> Result<Client, Box<dyn Error>> {
		let socket = match Socket::new(Ipv4Addr::UNSPECIFIED, Ipv6Addr::UNSPECIFIED, 0) {
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
			event_pump,
			input,
			socket,
			video,
			
			should_quit: false,
		})
	}
	
	pub fn frame(&mut self, delta: Duration) {
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
		while let Some((packet, addr)) = self.socket.next() {
			match ServerPacket::try_from(packet) {
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
		
		if self.should_quit {
			return;
		}
		
		//console.process();
		
		self.video.draw_frame().unwrap();
	}
	
	pub fn quit(&mut self) {
		self.should_quit = true;
		
		let packet = ClientPacket::Connectionless(ClientConnectionlessPacket::RCon(vec!["quit".to_owned()]));
		let addr = SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 40011);
		self.socket.send_to(packet.into(), addr);
	}
	
	/*pub fn dispatcher(&self) -> &CommandDispatcher {
		&self.dispatcher
	}*/
	
	fn process_packet(&mut self, packet: ServerPacket, addr: SocketAddr) {
		println!("Client: {:?}, {}", packet, addr);
		
		match packet {
			ServerPacket::Connectionless(packet) => match packet {
				ServerConnectionlessPacket::ChallengeResponse(_) => unimplemented!(),
				ServerConnectionlessPacket::ConnectResponse => unimplemented!(),
				ServerConnectionlessPacket::Disconnect => unimplemented!(),
				ServerConnectionlessPacket::InfoResponse(_) => unimplemented!(),
				ServerConnectionlessPacket::Print(message) => {
					info!("message");
				},
				ServerConnectionlessPacket::StatusResponse(_, _) => unimplemented!(),
			},
			_ => unimplemented!(),
		}
	}
}
