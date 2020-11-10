use async_std::net::{UdpSocket, SocketAddr};
use std::net::SocketAddr;
use once_cell::sync::OnceCell;
use crate::server::swap_cmd::SwapCmd;
use std::time::{Duration, Instant};
use super::conf::Conf;
use super::packet::Packet;
use super::send_cache::save_send_cache;
use super::rec_cache::RecCache;
use super::packets::Packets;

pub static SOC: OnceCell<UdpSocket> = OnceCell::new();

pub async fn init_socket() {
    // 远程连接必须0.0.0.0:0
    dbg!("callee listen");
    let soc = UdpSocket::bind("0.0.0.0:0").await?;
    SOC.set(soc).unwrap();
}

pub async fn get_peer_address( peer_id: &str) -> anyhow::Result<SocketAddr> {
    let res = send_server( peer_id, SwapCmd::Ask).await?;
    let addr_str = String::from_utf8_lossy(&res);
    let addr: SocketAddr = addr_str.parse()?;
    Ok(addr)
}

pub async fn heart_beat() -> anyhow::Result<()> {
    let conf = Conf::get();
    let res = send_server( &conf.id, SwapCmd::Save).await?;
    let s = *String::from_utf8_lossy(&res);
    if s == "success" {
        return Ok(());
    }
    Err(anyhow!("can not heart beat to server"))
}

pub async fn ask_peer_open( peer_id: &str) -> anyhow::Result<()> {
    let res = send_server( peer_id, SwapCmd::Open).await?;
    let s = *String::from_utf8_lossy(&res);
    if s == "success" {
        return Ok(());
    }
    Err(anyhow!("can not ask open through server"))
}

async fn send_server( id: &str, command: SwapCmd) -> anyhow::Result<Vec<u8>> {
    let conf = Conf::get();
    let server=conf.swap_server;
    let soc = CONN.get().unwrap();
    // 给服务器的走原始接口
    let cmd = {
        match command {
            SwapCmd::Save => SwapCmd::save(id),
            SwapCmd::Ask => SwapCmd::ask(id),
            SwapCmd::Open => SwapCmd::open(id),
            _ => panic!("command not match to server"),
        }
    };
    soc.send_to(&cmd, &server).await?;
    let mut buf = vec![0u8; conf.size];
    let (n, peer) = async_std::io::timeout(Duration::from_micros(conf.rec_elapse as u64), async {
        soc.recv_from(&mut buf).await
    }).await?;
    let cmd = buf[0];

    if cmd == SwapCmd::ServerErr.enum2int() {
        let err = String::from_utf8_lossy(&buf[1..n]);
        return Err(anyhow!(err));
    }
    let s = buf[1..n].to_vec();
    Ok(s)
}
