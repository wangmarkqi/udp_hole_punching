use async_std::net::{UdpSocket, SocketAddr};
use once_cell::sync::OnceCell;
use crate::server::swap_cmd::SwapCmd;
use std::time::{Duration, Instant};
use super::conf::Conf;
use super::packet::Packet;
use super::send_cache::save_send_cache;
use super::rec_cache::RecCache;
use super::packets::Packets;
use crate::client::server_cache::ServerCache;

pub static SOC: OnceCell<UdpSocket> = OnceCell::new();

pub async fn init_socket()->anyhow::Result<()> {
    // 远程连接必须0.0.0.0:0
    dbg!("callee listen");
    let soc = UdpSocket::bind("0.0.0.0:0").await?;
    SOC.set(soc).unwrap();
    Ok(())
}

// ask cmd get feed back peer address, and server will send open to peer
pub async fn get_peer_address(peer_id: &str) -> anyhow::Result<SocketAddr> {
    let res = send_server(peer_id, SwapCmd::Ask).await?;
    // let addr_str = String::from_utf8_lossy(&res);
    let addr: SocketAddr = res.parse()?;
    Ok(addr)
}
// save cmd server record me address and feed back successs
pub async fn heart_beat() -> anyhow::Result<()> {
    let conf = Conf::get();
    let res = send_server(&conf.id, SwapCmd::Save).await?;
    if res == "success".to_string() {
        return Ok(());
    }
    Err(anyhow!("can not heart beat to server"))
}



async fn send_server(id: &str, command: SwapCmd) -> anyhow::Result<String> {
    let conf = Conf::get();
    let server = conf.swap_server;
    let soc = SOC.get().unwrap();
    // 给服务器的走原始接口
    let cmd = {
        match command {
            SwapCmd::Save => SwapCmd::save(id),
            SwapCmd::Ask => SwapCmd::ask(id),
            SwapCmd::Open => SwapCmd::open(id),
            _ => panic!("command not match to server"),
        }
    };
    // 发之前清缓存,两个缓存，错误和正确，拿不到正确，拿错误
    SwapCmd::ServerErr.clear_cache();
    command.clear_cache();

    soc.send_to(&cmd, &server).await?;
    // let mut buf = vec![0u8; conf.size];
    let feedback = command.get_cache();
    if feedback.len() > 0 {
        return Ok(feedback);
    }
    let err = SwapCmd::ServerErr.get_cache();
    Ok(err)
}
