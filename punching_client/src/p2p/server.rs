use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::{SocketAddr, UdpSocket};


///
/// # Errors
/// This function returns immediately with any [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) that
/// comes up.
pub fn run(port: u16) -> Result<(), std::io::Error> {
	let mut matchmap: HashMap<Vec<u8>, SocketAddr> = Default::default();

	let socket = UdpSocket::bind(("0.0.0.0", port))?;

	let mut packet = [0u8; 512];
	loop {
		let (size, sender) = socket.recv_from(&mut packet)?;
		let packet = &packet[0..size];

		matchmap.retain(|_, addr| sender.ip() != addr.ip());

		println!(
			"{}:{} {}",
			sender.ip(),
			sender.port(),
			String::from_utf8_lossy(packet)
		);
		if let Some(other) = matchmap.remove(packet) {
			make_match(&socket, sender, other)?;
			println!("Matching {} and {}", sender, other);
		} else {
			matchmap.insert(packet.to_vec(), sender);
		}
	}
}

/// Sends `addr1`'s IP address and external port to `addr2` using `socket` and vice versa.
/// # Errors
/// Returns any error produced by `socket.send_to(...)`, or an
/// [`InvalidInput`](https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.InvalidInput)
/// if one of the supplied addresses is ipv4 and the other is ipv6
pub fn make_match(socket: &UdpSocket, addr1: SocketAddr, addr2: SocketAddr) -> Result<(), Error> {
	if addr1.is_ipv4() != addr2.is_ipv4() {
		socket.send_to(&[0u8], addr1)?;
		socket.send_to(&[0u8], addr2)?;
		return Err(Error::new(ErrorKind::InvalidInput, "addr1 and addr2 be of the same type (ipv4, ipv6)"));
	} else {
		send_info(socket, addr1, addr2)?;
		send_info(socket, addr2, addr1)?;
	}
	Ok(())
}

fn send_info(socket: &UdpSocket, to: SocketAddr, about: SocketAddr) -> Result<(), Error> {
	let mut ip_raw = match about {
		SocketAddr::V4(s) => s.ip().octets().to_vec(),
		SocketAddr::V6(s) => s.ip().octets().to_vec(),
	};
	let mut packet = vec![];
	packet.push(if about.is_ipv4() { 4u8 } else { 6u8 });
	packet.append(&mut ip_raw);
	packet.push((about.port() >> 8) as u8);
	packet.push((about.port() % 0x100) as u8);

	socket.send_to(&packet, to).map(|_| ())
}
