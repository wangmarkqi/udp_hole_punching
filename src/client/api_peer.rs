use async_std::net::UdpSocket;
use std::net::SocketAddr;
use once_cell::sync::OnceCell;
use crate::server::swap_cmd::SwapCmd;
use std::time::{Duration, Instant};
use super::conf::Conf;
use super::packet::Packet;
use super::rec_cache::RecCache;
use super::packets::Packets;

pub async fn send(msg: &Vec<u8>, address: SocketAddr) -> anyhow::Result<bool> {
    let soc = UdpSocket::bind("0.0.0.0:0").await?;
    let conf = Conf::get();

    let pacs = Packet::new_pacs_from_send_bytes(msg );

    // 发之前不需要清缓存,因为sess是根据没有的k，取的时候清理

    let now = std::time::Instant::now();
    let mut task = pacs;
    loop {
        let mut rest = vec![];
        for pac in task.iter() {
            let data = pac.to_bytes();
            soc.send_to(&data, address).await?;
            let mut buf = vec![0u8; conf.size];
            let (n, address) = soc.recv_from(&mut buf).await?;
            let back = Packet::new_from_rec_bytes(n, &buf.to_vec());
            if  back.order == pac.order {
                dbg!("send success");
                continue;
            }
            rest.push(pac.to_owned());
        }
        task=rest;
        let differ = now.elapsed().as_micros() as i32;
        if differ > conf.send_timeout {
            dbg!("time out");
            return Ok(false);
        };
    }
    Ok(true)
}

pub async fn rec( address: SocketAddr) -> Vec<u8> {
    let pacs=RecCache::get_pacs(address);
    if pacs.is_complete(){
        let data=pacs.assembly();
        return data;
    }
    vec![]
}

