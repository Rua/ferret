use byteorder::{NetworkEndian as NE, ReadBytesExt, WriteBytesExt};
use std::convert::TryFrom;
use std::error::Error;
use std::io::{Cursor, Read, Write};
use std::str;

use crate::commands;


#[derive(Debug)]
pub enum Packet<T> {
	Unsequenced(Vec<T>),
	Sequenced(SequencedPacket),
}

impl<T> From<SequencedPacket> for Packet<T> {
	fn from(packet: SequencedPacket) -> Packet<T> {
		Packet::Sequenced(packet)
	}
}

impl<T: TryRead<T>> TryFrom<Vec<u8>> for Packet<T> {
	type Error = Box<dyn Error>;
	
	fn try_from(data: Vec<u8>) -> Result<Packet<T>, Box<dyn Error>> {
		let mut reader = Cursor::new(data);
		let sequence = reader.read_u32::<NE>()?;
		
		if sequence == 0xFFFFFFFF {
			let mut messages = Vec::new();
			
			while reader.position() < reader.get_ref().len() as u64 {
				messages.push(T::try_read(&mut reader)?)
			}
			
			Ok(Packet::Unsequenced(messages))
		} else {
			Ok(SequencedPacket::try_from(reader.into_inner())?.into())
		}
	}
}

impl<T: Into<Vec<u8>>> From<Packet<T>> for Vec<u8> {
	fn from(packet: Packet<T>) -> Vec<u8> {
		match packet {
			Packet::Unsequenced(messages) => {
				let mut writer = Cursor::new(Vec::new());
				writer.write_u32::<NE>(0xFFFFFFFF).unwrap();
				
				for message in messages {
					writer.write(&message.into()).unwrap();
				}
				
				writer.into_inner()
			},
			Packet::Sequenced(p) => p.into(),
		}
	}
}

#[derive(Debug)]
pub struct SequencedPacket {
	pub sequence: u32,
	pub data: Vec<u8>,
}

impl TryFrom<Vec<u8>> for SequencedPacket {
	type Error = Box<dyn Error>;
	
	fn try_from(buf: Vec<u8>) -> Result<SequencedPacket, Box<dyn Error>> {
		let mut reader = Cursor::new(buf);
		let sequence = reader.read_u32::<NE>()?;
		
		if sequence == 0xFFFFFFFF {
			return Err(Box::from("not a sequenced packet"))
		}
		
		Ok(SequencedPacket {
			sequence,
			data: reader.into_inner()[4..].to_owned(),
		})
	}
}

impl From<SequencedPacket> for Vec<u8> {
	fn from(packet: SequencedPacket) -> Vec<u8> {
		let mut writer = Cursor::new(Vec::new());
		writer.write_u32::<NE>(packet.sequence).unwrap();
		writer.write(&packet.data).unwrap();
		writer.into_inner()
	}
}

pub trait TryRead<T> {
	fn try_read(reader: &mut Cursor<Vec<u8>>) -> Result<T, Box<dyn Error>>;
}


/*
 * Client-to-server protocol
 */

#[derive(Debug)]
pub enum ClientMessage {
	Connect,
	RCon(String),
}

impl TryRead<ClientMessage> for ClientMessage {
	fn try_read(reader: &mut Cursor<Vec<u8>>) -> Result<ClientMessage, Box<dyn Error>> {
		let message_type = reader.read_u8()?;
		
		Ok(match message_type {
			1 => {
				ClientMessage::Connect
			},
			2 => {
				let length = reader.read_u32::<NE>()?;
				let mut data = vec![0u8; length as usize];
				reader.read_exact(data.as_mut_slice())?;
				ClientMessage::RCon(String::from_utf8(data)?)
			},
			_ => unreachable!(),
		})
	}
}

impl From<ClientMessage> for Vec<u8> {
	fn from(message: ClientMessage) -> Vec<u8> {
		let mut writer = Cursor::new(Vec::new());
		
		match message {
			ClientMessage::Connect => {
				writer.write_u8(1).unwrap();
			}
			ClientMessage::RCon(text) => {
				writer.write_u8(2).unwrap();
				writer.write_u32::<NE>(text.len() as u32).unwrap();
				writer.write(text.as_bytes()).unwrap();
			}
		}
		
		writer.into_inner()
	}
}


/*
 * Server-to-client protocol
 */

#[derive(Debug)]
pub enum ServerMessage {
	ConnectResponse,
	Disconnect,
}

impl TryRead<ServerMessage> for ServerMessage {
	fn try_read(reader: &mut Cursor<Vec<u8>>) -> Result<ServerMessage, Box<dyn Error>> {
		let message_type = reader.read_u8()?;
		
		Ok(match message_type {
			1 => {
				ServerMessage::ConnectResponse
			},
			2 => {
				ServerMessage::Disconnect
			},
			_ => unreachable!(),
		})
	}
}

impl From<ServerMessage> for Vec<u8> {
	fn from(message: ServerMessage) -> Vec<u8> {
		let mut writer = Cursor::new(Vec::new());
		
		match message {
			ServerMessage::ConnectResponse => {
				writer.write_u8(1).unwrap();
			},
			ServerMessage::Disconnect => {
				writer.write_u8(2).unwrap();
			}
		}
		
		writer.into_inner()
	}
}
