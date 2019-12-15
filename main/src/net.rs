use byteorder::{NetworkEndian as NE, ReadBytesExt, WriteBytesExt};
use net2::UdpBuilder;
use std::{
	error::Error,
	fmt,
	io::{self, Cursor, ErrorKind, Read, Write},
	marker::PhantomData,
	net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket},
	rc::Rc,
	sync::mpsc::{Receiver, Sender, TryRecvError},
};
use crate::protocol::{SequencedPacket, TryRead};


pub struct Socket {
	v4: Option<UdpSocket>,
	v6: Option<UdpSocket>,
	sender: Sender<Vec<u8>>,
	receiver: Receiver<Vec<u8>>,
}

impl Socket {
	pub fn new(ipv4_addr: Ipv4Addr, ipv6_addr: Ipv6Addr, port: u16, sender: Sender<Vec<u8>>, receiver: Receiver<Vec<u8>>) -> Result<Rc<Socket>, Box<dyn Error>> {
		let ipv4_addr_port = SocketAddrV4::new(ipv4_addr, port);
		let v4 = bind_v4(ipv4_addr_port);

		if let Err(ref err) = v4 {
			debug!("could not bind IPv4 socket to {}: {}", ipv4_addr_port, err);
		};

		let ipv6_addr_port = SocketAddrV6::new(ipv6_addr, port, 0, 0);
		let v6 = bind_v6(ipv6_addr_port);

		if let Err(ref err) = v6 {
			debug!("could not bind IPv6 socket to {}: {}", ipv6_addr_port, err);
		};

		if v4.is_err() && v6.is_err() {
			Err(Box::from("both IPv4 and IPv6 bindings failed"))
		} else {
			Ok(Rc::new(Socket {
				v4: v4.ok(),
				v6: v6.ok(),
				sender,
				receiver,
			}))
		}
	}

	pub fn mode(&self) -> SocketMode {
		let v4_addr = self.v4.as_ref().map(|s| s.local_addr().unwrap());
		let v6_addr = self.v6.as_ref().map(|s| s.local_addr().unwrap());

		if v4_addr.is_some() && v6_addr.is_some() {
			SocketMode::DualStack(
				if let SocketAddr::V4(s) = v4_addr.unwrap() { s } else { unreachable!() },
				if let SocketAddr::V6(s) = v6_addr.unwrap() { s } else { unreachable!() },
			)
		} else if v4_addr.is_some() {
			SocketMode::IPv4(
				if let SocketAddr::V4(s) = v4_addr.unwrap() { s } else { unreachable!() },
			)
		} else if v6_addr.is_some() {
			SocketMode::IPv6(
				if let SocketAddr::V6(s) = v6_addr.unwrap() { s } else { unreachable!() },
			)
		} else {
			unreachable!()
		}
	}

	pub fn filter_supported(&self) -> impl FnMut(&SocketAddr) -> bool {
		let v4 = self.v4.is_some();
		let v6 = self.v6.is_some();

		return move |&addr| -> bool {
			addr.is_ipv4() && v4 || addr.is_ipv6() && v6
		}
	}

	pub fn send_to(&self, packet: Vec<u8>, addr: Addr) {
		if let Addr::Local = addr {
			self.sender.send(packet).ok();
			return;
		}

		let addr: SocketAddr = match addr {
			Addr::V4(addr) => addr.into(),
			Addr::V6(addr) => addr.into(),
			_ => unreachable!(),
		};

		let socket = match addr {
			SocketAddr::V4(_) => &self.v4,
			SocketAddr::V6(_) => &self.v6,
		};

		if let Some(socket) = socket {
			if let Err(err) = socket.send_to(packet.as_slice(), addr) {
				error!("could not send packet to {}: {}", addr, err);
			}
		} else {
			panic!(
				"socket does not support {} addresses",
				match addr {
					SocketAddr::V4(_) => "IPv4",
					SocketAddr::V6(_) => "IPv6",
				}
			);
		}
	}

	pub fn next(&self) -> Option<(Vec<u8>, Addr)> {
		// Try the local mpsc channel first
		match self.receiver.try_recv() {
			Ok(buf) => return Some((buf, Addr::Local)),
			Err(TryRecvError::Empty) => (),
			Err(TryRecvError::Disconnected) => panic!("Socket mpsc receiver disconnected"),
		}

		let mut buf = vec![0u8; 8192];

		// Try reading from available sockets, first from the IPv6 socket,
		// then from the IPv4 socket.
		for socket in [self.v6.as_ref(), self.v4.as_ref()].iter().filter_map(|s| *s) {
			match socket.recv_from(&mut buf) {
				Ok((bytes_read, addr)) => {
					if bytes_read == buf.len() {
						// Oversized packet, ignore it.
						warn!("received an oversized packet from {}", addr);
					} else {
						// We got a packet, parse it, and return if valid.
						buf.truncate(bytes_read);
						return Some((buf, addr.into()));
					}
				},
				Err(err) => {
					if err.kind() != ErrorKind::WouldBlock {
						// Got an error.
						error!(
							"could not receive {} packet: {}",
							match socket.local_addr().unwrap() {
								SocketAddr::V4(_) => "IPv4",
								SocketAddr::V6(_) => "IPv6",
							},
							err,
						);
					}
				},
			}
		}

		None
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Addr {
	Local,
	V4(SocketAddrV4),
	V6(SocketAddrV6),
}

impl From<SocketAddr> for Addr {
	fn from(addr: SocketAddr) -> Addr {
		match addr {
			SocketAddr::V4(addr) => Addr::V4(addr),
			SocketAddr::V6(addr) => Addr::V6(addr),
		}
	}
}

impl fmt::Display for Addr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Addr::Local => write!(f, "."),
			Addr::V4(addr) => addr.fmt(f),
			Addr::V6(addr) => addr.fmt(f),
		}
	}
}

pub enum SocketMode {
	DualStack(SocketAddrV4, SocketAddrV6),
	IPv4(SocketAddrV4),
	IPv6(SocketAddrV6),
}

impl fmt::Display for SocketMode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			SocketMode::DualStack(v4, v6) => write!(f, "dual-stack ({}, {})", v4, v6),
			SocketMode::IPv4(v4) => write!(f, "IPv4-only ({})", v4),
			SocketMode::IPv6(v6) => write!(f, "IPv6-only ({})", v6),
		}
	}
}

fn bind_v4(addr_port: SocketAddrV4) -> Result<UdpSocket, io::Error> {
	let builder = UdpBuilder::new_v4()?;
	let socket = builder.bind(addr_port)?;
	socket.set_nonblocking(true)?;

	Ok(socket)
}

fn bind_v6(addr_port: SocketAddrV6) -> Result<UdpSocket, io::Error> {
	let builder = UdpBuilder::new_v6()?;
	builder.only_v6(true)?;
	let socket = builder.bind(addr_port)?;
	socket.set_nonblocking(true)?;

	Ok(socket)
}

pub struct SequencedChannel<S, R> {
	addr: Addr,
	socket: Rc<Socket>,
	in_sequence: u32,
	out_sequence: u32,
	_phantom1: PhantomData<S>,
	_phantom2: PhantomData<R>,
}

impl<S: Into<Vec<u8>>, R: TryRead<R>> SequencedChannel<S, R> {
	pub fn new(socket: Rc<Socket>, addr: Addr) -> SequencedChannel<S, R> {
		SequencedChannel {
			addr,
			socket,
			in_sequence: 0,
			out_sequence: 1,
			_phantom1: PhantomData,
			_phantom2: PhantomData,
		}
	}

	pub fn addr(&self) -> Addr {
		self.addr
	}

	pub fn send(&mut self, messages: Vec<S>) {
		let data = messages.into_iter().map(|x| x.into()).collect::<Vec<Vec<u8>>>().concat();

		let packet = SequencedPacket {
			sequence: self.out_sequence,
			data,
		};

		self.socket.send_to(packet.into(), self.addr);
		self.out_sequence += 1;
	}

	pub fn process(&mut self, packet: SequencedPacket) -> Option<Vec<R>> {
		if packet.sequence < self.in_sequence {
			return None;
		}

		let mut reader = Cursor::new(packet.data);
		let mut messages = Vec::new();

		while reader.position() < reader.get_ref().len() as u64 {
			let message = match R::try_read(&mut reader) {
				Ok(message) => message,
				Err(err) => {
					warn!("received a malformed packet from {}", self.addr);
					return None;
				},
			};

			messages.push(message);
		}

		self.in_sequence = packet.sequence;
		Some(messages)
	}

	pub fn in_sequence(&self) -> u32 {
		self.in_sequence
	}

	pub fn out_sequence(&self) -> u32 {
		self.out_sequence
	}
}
