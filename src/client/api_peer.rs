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
use super::api_server::SOC;

pub async fn send(msg: &Vec<u8>, cmd: SwapCmd, address: SocketAddr) -> anyhow::Result<u8> {
    let pac = Packet::new_pacs_from_send_bytes(msg, sess, cmd);
    let sess = save_send_cache(&pac, address);
    Ok(sess)
}
pub async fn rec(session: u8, address: SocketAddr, elapse: u128) -> Vec<u8> {
    let conf = Conf::get();
    let now = std::time::Instant::now();
    loop {
        let pacs = RecCache::get_pacs(address, session);
        if &pacs.has_begin() && &pacs.has_end() && pacs.is_continue() {
            let data = pacs.assembly();
            // 表示拿出
            RecCache::set_taken(address, session);
            return data;
        }
        let differ = now.elapsed().as_micros() as i32;
        if differ > conf.rec_elapse{ break; };
    }
    vec![]
}
