mod audio;
mod commands;
mod input;
mod video;

use sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use std::error::Error;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};

use crate::client::audio::Audio;
pub use crate::client::commands::COMMANDS;
use crate::client::input::Input;
use crate::client::video::Video;
use crate::commands::CommandDispatcher;
use crate::net::Socket;
use crate::protocol::{ClientConnectionlessPacket, ClientPacket, ServerPacket};
//use crate::wad::WadLoader;

pub fn client_main(dispatcher: CommandDispatcher) {
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
	
	let mut client = Client::new(dispatcher).unwrap();
	client.run();
	
	//local_server.quit().unwrap();
	//local_server.quit_and_wait().unwrap();
}

pub struct Client {
	audio: Audio,
	dispatcher: CommandDispatcher,
	event_pump: EventPump,
	input: Input,
	socket: Socket<ClientPacket, ServerPacket>,
	video: Video,
	
	should_quit: bool,
}

impl Client {
	pub fn new(dispatcher: CommandDispatcher) -> Result<Client, Box<dyn Error>> {
		let socket = match Socket::new(Ipv4Addr::UNSPECIFIED, Ipv6Addr::UNSPECIFIED, 0) {
			Ok(val) => val,
			Err(err) => {
				return Err(Box::from(format!("Could not create client socket: {}", err)));
			}
		};
		
		info!("Client socket mode: {}", socket.mode());
		
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
			dispatcher,
			event_pump,
			input,
			socket,
			video,
			
			should_quit: false,
		})
	}
	
	pub fn run(&mut self) {
		loop {
			for event in self.event_pump.poll_iter() {
				match event {
					Event::Quit {..} => self.dispatcher.push("quit"),
					_ => {},
				}
			}
			
			// Execute console commands
			while let Some(args) = self.dispatcher.next(false) {
				COMMANDS.execute(args, self);
			}
			
			// Receive network packets
			for (packet, addr) in &mut self.socket {
				println!("Client: {:?}, {}", packet, addr);
			}
			
			if self.should_quit {
				return;
			}
			
			//console.process();
			
			//window.gl_swap_window();
		}
	}
	
	pub fn quit(&mut self) {
		self.should_quit = true;
	}
	
	pub fn dispatcher(&self) -> &CommandDispatcher {
		&self.dispatcher
	}
}
