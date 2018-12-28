use net2::UdpBuilder;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::ErrorKind;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket};

pub struct Socket {
	v4: Option<UdpSocket>,
	v6: Option<UdpSocket>,
}

impl Socket {
	pub fn new(ipv4_addr: Ipv4Addr, ipv6_addr: Ipv6Addr, port: u16) -> Result<Socket, Box<dyn Error>> {
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
			Ok(Socket {
				v4: v4.ok(),
				v6: v6.ok(),
			})
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
	
	pub fn send_to(&self, packet: Vec<u8>, addr: SocketAddr) {
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
}

impl Iterator for Socket {
	type Item = (Vec<u8>, SocketAddr);
	
	fn next(&mut self) -> Option<(Vec<u8>, SocketAddr)> {
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
						return Some((buf, addr));
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
