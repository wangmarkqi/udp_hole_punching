use async_std::net::UdpSocket;

pub mod action;

pub use crate::action::packet::{Packet, CMD, PAC_SIZE,HEADER_SIZE,MTU_SIZE};

use crate::action::process::*;

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;

/// # Examples
/// A server to make match
/// ```
///use async_std::task::block_on;

/// fn main() {
///  let host = "0.0.0.0:9292";
///     block_on(punching_server::make_match(host)).unwrap();
/// ```
pub async fn make_match(host: &str) -> anyhow::Result<()> {
    dbg!("server=====",host);
    let socket = UdpSocket::bind(host).await?;
    let mut buf = vec![0u8; MTU_SIZE];
    loop {
        let (n, me) = socket.recv_from(&mut buf).await?;
        if n == 0 {
            continue;
        }
        if n>HEADER_SIZE{
            return Err(anyhow!("header size beyond limits"));
        }
        let data = String::from_utf8_lossy(&buf[0..n]);
        let mut income: Packet = serde_json::from_str(&data)?;

        match &income.cmd {
            // callee sent to registry
            CMD::Save => {
                income.callee_address = me;
                income.callee_registry();
                dbg!("save callee",me);
            }

            CMD::Open => {
                income.caller_address = me;
                let open = income.make_pair();
                let res = &income.pack();
                if open {
                    dbg!("open to callee and caller",me);
                    socket.send_to(res, &income.callee_address).await?;
                }
                socket.send_to(res, me).await?;
            }
            _ => (),
        }
    }
}