use async_std::net::UdpSocket;

pub mod action;

pub use crate::action::packet::{Packet, CMD};

use crate::action::process::*;

#[macro_use]
extern crate lazy_static;

pub const CALLEE_SLEEP: u64 = 100;
pub const SUCCESS: &'static [u8] = "success".as_bytes();
pub const FAIL: &'static [u8] = "fail".as_bytes();
pub const PAC_SIZE: usize = 4096;


pub async fn make_match(host: &str) -> anyhow::Result<()> {
    let socket = UdpSocket::bind(host).await?;
    let mut buf = vec![0u8; PAC_SIZE];
    loop {
        let (n, me) = socket.recv_from(&mut buf).await?;
        if n == 0 {
            continue;
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
            _ =>(),
        }
    }
}