use std::io::{Error, ErrorKind};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};



pub trait HolePunchConnect {
	/// Creates a new socket, calls [`hole_punch_connect`](#tymethod.hole_punch_connect) on it
	/// and returns it.
	///
	/// # Examples
	/// A simple peer-to-peer UDP chat client
	/// ```
	/// use hole_punch_connect::HolePunchConnect;
	/// use std::net::UdpSocket;
	///
	/// fn main() -> Result<(), std::io::Error> {
	///     let mut buf = String::new();
	///     std::io::stdin().read_line(&mut buf)?;
	///
	///     let socket = UdpSocket::new_hole_punched(
	///         "domain.name:420",
	///         buf.trim().as_bytes(),
	///     )?;
	///
	///     {
	///         let socket = socket.try_clone()?;
	///         std::thread::spawn(move || loop {
	///             buf.clear();
	///             if let Ok(_len) = std::io::stdin().read_line(&mut buf) {
	///                 socket.send(buf.trim().as_bytes()).unwrap();
	///             }
	///         });
	///     }
	///
	///     let mut buf = [0u8; 512];
	///     let _ = std::thread::spawn(move || loop {
	///         if let Ok(len) = socket.recv(&mut buf) {
	///             println!("{}", String::from_utf8_lossy(&buf[..len]));
	///         }
	///     })
	///     .join();
	///
	///     Ok(())
	/// }
	/// ```
	///
	/// # Errors
	/// Propagates any error from creating the socket or from connecting it with
	/// [`hole_punch_connect`](#tymethod.hole_punch_connect).
	fn new_hole_punched<A>(server_addr: A, ident: &[u8]) -> Result<UdpSocket, Error>
	where
		A: std::net::ToSocketAddrs;

	/// [`connect`](https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.connect)s
	/// the socket to another client.
	///
	/// The `server_addr` parameter is the address to the server
	/// that pairs the two clients together.
	///
	/// The `ident` parameter specifies an identifier that the server uses to know which two
	/// clients to pair together.
	///
	/// # Examples
	/// ```
	/// let socket = UdpSocket::bind("0.0.0.0:0")?;
	/// socket.hole_punch_connect("domain.name:420")?;
	/// socket.send(b"Hello there")?;
	///
	/// socket.hole_punch_connect("other.domain:6392")?;
	/// socket.send(b"General Kenobi")?;
	/// ```
	///
	/// # Errors
	/// Propagates any error from sending or receiving data via the socket or if the data
	/// from the server was incorrect.
	///
	/// It does not however return any errors if the other client can't be reached. That
	/// will produce an error at an attempt to send packets to it. (This will hopefully be
	/// fixed in the future.)
	fn hole_punch_connect<A>(&self, server_addr: A, ident: &[u8]) -> Result<(), Error>
	where
		A: std::net::ToSocketAddrs;
}

impl HolePunchConnect for UdpSocket {
	fn new_hole_punched<A>(server_addr: A, ident: &[u8]) -> Result<UdpSocket, Error>
	where
		A: std::net::ToSocketAddrs,
	{
		let socket = UdpSocket::bind("0.0.0.0:0")?;

		socket.hole_punch_connect(server_addr, ident)?;

		Ok(socket)
	}
	fn hole_punch_connect<A>(&self, server_addr: A, ident: &[u8]) -> Result<(), Error>
	where
		A: std::net::ToSocketAddrs,
	{
		self.connect(server_addr)?;

		self.send(ident)?;

		let mut b = [0u8; 512];
		let size = self.recv(&mut b)?;
		let ip = match b[0] {
			4 => {
				if size < 7 {
					return Err(Error::new(
						ErrorKind::InvalidData,
						"Server send a too short packet",
					));
				}
				IpAddr::V4(Ipv4Addr::new(b[1], b[2], b[3], b[4]))
			}
			6 => {
				if size < 19 {
					return Err(Error::new(
						ErrorKind::InvalidData,
						"Server send a too short packet",
					));
				}
				IpAddr::V6(Ipv6Addr::from([
					b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12],
					b[13], b[14], b[15], b[16],
				]))
			}
			_ => {
				return Err(Error::new(
					ErrorKind::InvalidData,
					"Server said other client used another ip version",
				))
			}
		};
		let port = match b[0] {
			4 => ((b[5] as u16) << 8) | b[6] as u16,
			6 => ((b[17] as u16) << 8) | b[18] as u16,
			_ => unreachable!(),
		};

		let addr = SocketAddr::new(ip, port);

		self.connect(addr)?;

		Ok(())
	}
}
