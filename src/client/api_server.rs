use async_std::net::{UdpSocket, SocketAddr};
use crate::server::swap_cmd::SwapCmd;
use std::time::{Duration, Instant};
use super::conf::Conf;
use super::packet::Packet;
use super::rec_cache::RecCache;
use super::packets::Packets;


// ask cmd get feed back peer address, and server will send open to peer
pub async fn get_peer_address(peer_id: &str) -> anyhow::Result<SocketAddr> {
    let conf = Conf::get();
    let soc = UdpSocket::bind("0.0.0.0:0").await?;

    let send_data = SwapCmd::ask(peer_id);
    soc.send_to(&send_data, &conf.swap_server).await?;

    let mut buf = vec![0u8; conf.size];
    let (n, address) = soc.recv_from(&mut buf).await?;
    let cmd = buf[0];
    let res = {
        if let Ok(i) = std::str::from_utf8(&buf[1..n ]) {
            i.to_string()
        } else {
            "".to_string()
        }
    };
    dbg!(&res);
    // let addr_str = String::from_utf8_lossy(&res);
    let addr: SocketAddr = res.parse()?;
    Ok(addr)
}



pub async fn test_server_api() -> anyhow::Result<()> {
    let mut conf = Conf::default();
    conf.swap_server = "39.96.40.177:4222".to_string();
    conf.set();
    let res2 = get_peer_address(&conf.id).await?;
    dbg!(res2);
    Ok(())
}