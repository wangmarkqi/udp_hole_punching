use punching_server::{Packet, CMD};
use async_std::net::{UdpSocket, SocketAddr};
use once_cell::sync::OnceCell;
use crate::cli::p2s::P2S;
use crate::cli::p2p::{P2P, Who};
use super::rec_pac::rec_pac;

pub static CONN: OnceCell<UdpSocket> = OnceCell::new();

pub async fn connect(server: &str, uuid: &str) -> anyhow::Result<SocketAddr> {
    let soc = UdpSocket::bind("0.0.0.0:0").await?;
    CONN.set(soc).unwrap();

    let conn = CONN.get().unwrap();
    let mut ask_open = Packet::default();
    ask_open.caller_ask_open(uuid)?;
    conn.send_to(&ask_open.pack(), server).await?;

    let ask = rec_pac(Who::Caller).await?;
    let suc = ask.success;
    if suc {
        let callee = ask.caller_address;
        // send first pac to open gate
        // let mut pac = Packet::default();
        // pac.cmd = CMD::P2P;
        // pac.send_pac(Who::Caller, callee).await?;
        return Ok(callee);
    }
    Err(anyhow!(ask.err))
}

pub async fn send(msg: &Vec<u8>, addr: SocketAddr) -> anyhow::Result<usize> {
    let mut pac = Packet::default();
    pac.cmd = CMD::P2P;
    pac.msg = msg.to_owned();
    let n = pac.send_pac(Who::Caller, addr).await?;
    Ok(n)
}

pub async fn rec() -> anyhow::Result<Vec<u8>> {
    let rec = rec_pac(Who::Caller).await?;
    Ok(rec.msg)
}

pub async fn test() -> anyhow::Result<()> {
    let uuid = "b997dbac-e919-4e44-a8b5-9f7017381e30";
    let remote = "39.96.40.177:4222";
    let address = connect(remote, uuid).await?;
    let mut v = vec![];
    v.push(1);
    v.push(2);
    for _ in 0..1024 {
        for u in 0..10{
            v.push(u);
        }
    }
    send(&v, address).await?;
    let res = rec().await?;
    dbg!(res);
    Ok(())
}