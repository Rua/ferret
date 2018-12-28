use byteorder::{NetworkEndian as NE, ReadBytesExt, WriteBytesExt};
use std::convert::TryFrom;
use std::error::Error;
use std::io::{Cursor, Read, Write};
use std::str;

use crate::commands;


#[derive(Debug)]
pub struct SequencedPacket {
	pub sequence: u32,
	pub data: Vec<u8>,
	pub fragmentation: PacketFragmentation,
}

#[derive(Debug)]
pub enum PacketFragmentation {
	Complete,
	Fragmented(u16, u16),
}

impl TryFrom<Vec<u8>> for SequencedPacket {
	type Error = Box<dyn Error>;
	
	fn try_from(buf: Vec<u8>) -> Result<SequencedPacket, Box<dyn Error>> {
		let mut reader = Cursor::new(buf);
		let sequence = reader.read_u32::<NE>()?;
		
		if sequence == 0xFFFFFFFF {
			Err(Box::from("not a reliable packet"))
		} else if sequence & 1<<31 == 0 {
			Ok(SequencedPacket {
				sequence: sequence & !(1 << 31),
				data: reader.into_inner()[4..].to_owned(),
				fragmentation: PacketFragmentation::Complete,
			})
		} else {
			let frag_start = reader.read_u16::<NE>()?;
			let frag_len = reader.read_u16::<NE>()?;
			
			Ok(SequencedPacket {
				sequence: sequence & !(1 << 31),
				data: reader.into_inner()[4..].to_owned(),
				fragmentation: PacketFragmentation::Fragmented(frag_start, frag_len),
			})
		}
	}
}

impl From<SequencedPacket> for Vec<u8> {
	fn from(packet: SequencedPacket) -> Vec<u8> {
		let mut writer = Cursor::new(Vec::new());
		
		match packet.fragmentation {
			PacketFragmentation::Complete => {
				writer.write_u32::<NE>(packet.sequence).unwrap();
			},
			PacketFragmentation::Fragmented(frag_start, frag_len) => {
				writer.write_u32::<NE>(packet.sequence | (1 << 31)).unwrap();
				writer.write_u16::<NE>(frag_start).unwrap();
				writer.write_u16::<NE>(frag_len).unwrap();
			}
		}
		
		writer.write(&packet.data).unwrap();
		writer.into_inner()
	}
}



/*
 * Client-to-server protocol
 */

#[derive(Debug)]
pub enum ClientPacket {
	Connectionless(ClientConnectionlessPacket),
	Sequenced(SequencedPacket),
}

impl TryFrom<Vec<u8>> for ClientPacket {
	type Error = Box<dyn Error>;
	
	fn try_from(buf: Vec<u8>) -> Result<ClientPacket, Box<dyn Error>> {
		let mut reader = Cursor::new(buf);
		let sequence = reader.read_u32::<NE>()?;
		
		if sequence == 0xFFFFFFFF {
			Ok(ClientConnectionlessPacket::try_from(reader.into_inner())?.into())
		} else {
			Ok(SequencedPacket::try_from(reader.into_inner())?.into())
		}
	}
}

impl From<ClientPacket> for Vec<u8> {
	fn from(packet: ClientPacket) -> Vec<u8> {
		match packet {
			ClientPacket::Connectionless(p) => p.into(),
			ClientPacket::Sequenced(p) => p.into(),
		}
	}
}

#[derive(Debug)]
pub enum ClientConnectionlessPacket {
	Connect(String),
	GetInfo,
	GetStatus,
	RCon(Vec<String>),
}

impl From<ClientConnectionlessPacket> for ClientPacket {
	fn from(packet: ClientConnectionlessPacket) -> ClientPacket {
		ClientPacket::Connectionless(packet)
	}
}

impl From<SequencedPacket> for ClientPacket {
	fn from(packet: SequencedPacket) -> ClientPacket {
		ClientPacket::Sequenced(packet)
	}
}

impl TryFrom<Vec<u8>> for ClientConnectionlessPacket {
	type Error = Box<dyn Error>;
	
	fn try_from(buf: Vec<u8>) -> Result<ClientConnectionlessPacket, Box<dyn Error>> {
		let mut reader = Cursor::new(buf);
		let sequence = reader.read_u32::<NE>()?;
		
		if sequence != 0xFFFFFFFF {
			return Err(Box::from("not a connectionless packet"));
		}
		
		let mut buf = Vec::new();
		reader.read_to_end(&mut buf)?;
		let mut tokens = commands::tokenize(str::from_utf8(&buf)?)?;
		let mut tokens = tokens.drain(..);
		
		let cmd = match tokens.next() {
			Some(val) => val,
			None => return Err(Box::from("empty packet")),
		};
		
		let packet = match cmd.as_str() {
			"connect" => {
				let text = match tokens.next() {
					Some(val) => val,
					None => return Err(Box::from(format!("{}: argument 1 missing", cmd))),
				};
				
				ClientConnectionlessPacket::Connect(text)
			},
			"getinfo" => {
				ClientConnectionlessPacket::GetInfo
			},
			"getstatus" => {
				ClientConnectionlessPacket::GetStatus
			},
			"rcon" => {
				let mut args = Vec::new();
				
				while let Some(val) = tokens.next() {
					args.push(val);
				}
				
				ClientConnectionlessPacket::RCon(args)
			},
			_ => {
				return Err(Box::from(format!("invalid command: {}", cmd)))
			},
		};
		
		if tokens.count() != 0 {
			Err(Box::from(format!("{}: too many arguments", cmd)))
		} else {
			Ok(packet)
		}
	}
}

impl From<ClientConnectionlessPacket> for Vec<u8> {
	fn from(packet: ClientConnectionlessPacket) -> Vec<u8> {
		let mut writer = Cursor::new(Vec::new());
		writer.write_u32::<NE>(0xFFFFFFFF).unwrap();
		
		match packet {
			ClientConnectionlessPacket::Connect(text) => {
				write!(writer, "connect {}", commands::quote_escape(&text)).unwrap();
			},
			ClientConnectionlessPacket::GetInfo => {
				write!(writer, "getinfo").unwrap();
			},
			ClientConnectionlessPacket::GetStatus => {
				write!(writer, "getstatus").unwrap();
			},
			ClientConnectionlessPacket::RCon(mut args) => {
				write!(writer, "rcon").unwrap();
				
				for arg in args.drain(..) {
					write!(writer, " {}", commands::quote_escape(&arg)).unwrap();
				}
			},
		}
		
		writer.into_inner()
	}
}


/*
 * Server-to-client protocol
 */

#[derive(Debug)]
pub enum ServerPacket {
	Connectionless(ServerConnectionlessPacket),
	Sequenced(SequencedPacket),
}

impl TryFrom<Vec<u8>> for ServerPacket {
	type Error = Box<dyn Error>;
	
	fn try_from(buf: Vec<u8>) -> Result<ServerPacket, Box<dyn Error>> {
		let mut reader = Cursor::new(buf);
		let sequence = reader.read_u32::<NE>()?;
		
		if sequence == 0xFFFFFFFF {
			Ok(ServerConnectionlessPacket::try_from(reader.into_inner())?.into())
		} else {
			Ok(SequencedPacket::try_from(reader.into_inner())?.into())
		}
	}
}

impl From<ServerPacket> for Vec<u8> {
	fn from(packet: ServerPacket) -> Vec<u8> {
		match packet {
			ServerPacket::Connectionless(p) => p.into(),
			ServerPacket::Sequenced(p) => p.into(),
		}
	}
}

#[derive(Debug)]
pub enum ServerConnectionlessPacket {
	ConnectResponse,
	Disconnect,
	InfoResponse(String),
	Print(String),
	StatusResponse(String, String),
}

impl From<ServerConnectionlessPacket> for ServerPacket {
	fn from(packet: ServerConnectionlessPacket) -> ServerPacket {
		ServerPacket::Connectionless(packet)
	}
}

impl From<SequencedPacket> for ServerPacket {
	fn from(packet: SequencedPacket) -> ServerPacket {
		ServerPacket::Sequenced(packet)
	}
}

impl TryFrom<Vec<u8>> for ServerConnectionlessPacket {
	type Error = Box<dyn Error>;
	
	fn try_from(buf: Vec<u8>) -> Result<ServerConnectionlessPacket, Box<dyn Error>> {
		let mut reader = Cursor::new(buf);
		let sequence = reader.read_u32::<NE>()?;
		
		if sequence != 0xFFFFFFFF {
			return Err(Box::from("not a connectionless packet"));
		}
		
		let mut buf = Vec::new();
		reader.read_to_end(&mut buf)?;
		let mut tokens = commands::tokenize(str::from_utf8(&buf)?)?;
		let mut tokens = tokens.drain(..);
		
		let cmd = match tokens.next() {
			Some(val) => val,
			None => return Err(Box::from("empty packet")),
		};
		
		let packet = match cmd.as_str() {
			"connectResponse" => {
				ServerConnectionlessPacket::ConnectResponse
			},
			"disconnect" => {
				ServerConnectionlessPacket::Disconnect
			},
			"infoResponse" => {
				let text = match tokens.next() {
					Some(val) => val,
					None => return Err(Box::from(format!("{}: argument 1 missing", cmd))),
				};
				
				ServerConnectionlessPacket::InfoResponse(text)
			},
			"print" => {
				let text = match tokens.next() {
					Some(val) => val,
					None => return Err(Box::from(format!("{}: argument 1 missing", cmd))),
				};
				
				ServerConnectionlessPacket::Print(text)
			},
			"statusResponse" => {
				let info = match tokens.next() {
					Some(val) => val,
					None => return Err(Box::from(format!("{}: argument 1 missing", cmd))),
				};
				
				let status = match tokens.next() {
					Some(val) => val,
					None => return Err(Box::from(format!("{}: argument 2 missing", cmd))),
				};
				
				ServerConnectionlessPacket::StatusResponse(info, status)
			},
			_ => {
				return Err(Box::from(format!("invalid command: {}", cmd)))
			},
		};
		
		if tokens.count() != 0 {
			Err(Box::from(format!("{}: too many arguments", cmd)))
		} else {
			Ok(packet)
		}
	}
}

impl From<ServerConnectionlessPacket> for Vec<u8> {
	fn from(packet: ServerConnectionlessPacket) -> Vec<u8> {
		let mut writer = Cursor::new(Vec::new());
		writer.write_u32::<NE>(0xFFFFFFFF).unwrap();
		
		match packet {
			ServerConnectionlessPacket::ConnectResponse => {
				write!(writer, "connectResponse").unwrap();
			},
			ServerConnectionlessPacket::Disconnect => {
				write!(writer, "disconnect").unwrap();
			},
			ServerConnectionlessPacket::InfoResponse(text) => {
				write!(writer, "infoResponse {}",
					commands::quote_escape(&text),
				).unwrap();
			},
			ServerConnectionlessPacket::Print(text) => {
				write!(writer, "print {}",
					commands::quote_escape(&text),
				).unwrap();
			},
			ServerConnectionlessPacket::StatusResponse(info, status) => {
				write!(writer, "statusResponse {} {}",
					commands::quote_escape(&info),
					commands::quote_escape(&status),
				).unwrap();
			},
		}
		
		writer.into_inner()
	}
}
