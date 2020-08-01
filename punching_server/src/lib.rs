use async_std::net::UdpSocket;

pub mod action;

pub use crate::action::packet::{Packet};
pub use crate::action::tools::{ CMD, PAC_SIZE};

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
    let mut buf = vec![0u8; PAC_SIZE];
    loop {
        let (n, me) = socket.recv_from(&mut buf).await?;
        if n == 0 {
            continue;
        }
        if n>PAC_SIZE{
            return Err(anyhow!("pac size beyond limits"));
        }
        // let data = String::from_utf8_lossy(&buf[0..n]);
        let mut income: Packet = Packet::unpack(&buf[0..n].to_vec())?;

        match &income.cmd {
            // callee sent to registry
            CMD::Save => {
                income.address = me;
                income.callee_registry();
                dbg!("save callee",me);
            }

            CMD::Open => {
                income.address = me;
                let (pac2caller,pac2callee) = income.make_pair();
                if pac2callee.success && pac2caller.success {
                    socket.send_to(&pac2caller.pack(), me).await?;
                    socket.send_to(&pac2callee.pack(), pac2caller.address).await?;
                }else{
                    socket.send_to(&pac2caller.pack(), me).await?;
                }
            }
            _ => (),
        }
    }
}