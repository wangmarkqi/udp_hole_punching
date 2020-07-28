use punching_server::{Packet, CMD};
use async_std::net::UdpSocket;
use std::time::Duration;
use once_cell::sync::OnceCell;
use crate::p2s::p2s::P2S;
use crate::p2s::callee::rec_pac;

pub static CONN: OnceCell<UdpSocket> = OnceCell::new();

pub async fn _init_caller(localhost: &str) -> anyhow::Result<()> {
    let mut  soc = UdpSocket::bind("0.0.0.0:0").await?;
    CONN.set(soc).unwrap();
    Ok(())
}


pub async fn ask_connect(host: &str, uuid: &str) -> anyhow::Result<()> {
    let conn = CONN.get().unwrap();
    let mut ask_open = Packet::default();
    ask_open.caller_ask_open(uuid)?;
    conn.send_to(&ask_open.pack(), host).await?;

    let  (ask ,address)= rec_pac("callee").await?;
    dbg!(&ask);
    let suc = ask.success;
    if suc {
        loop {
            let callee = ask.caller_address;
            let mut pac = Packet::default();
            pac.cmd = CMD::P2P;
            pac.msg = "from caller".to_string();
            conn.send_to(&pac.pack(), callee).await?;
            let  rec = rec_pac("callee").await?;
            dbg!(rec);
        }
    }
    Ok(())
}

