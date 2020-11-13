use async_std::net::UdpSocket;
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
use crate::client::send_cache::new_sess;
use crate::client::cmd_packet::Cmd2Pac;

pub async fn send(msg: &Vec<u8>, address: SocketAddr) -> anyhow::Result<u8> {
    let sess = new_sess(address);
    let pacs = Packet::new_pacs_from_send_bytes(msg, sess);
    save_send_cache(&pacs, address, sess);

    let soc = SOC.get().unwrap();
    // 发之前不需要清缓存,因为sess是根据没有的k，取的时候清理
    for pac in pacs.iter() {
        let data = pac.to_bytes();
        soc.send_to(&data, address).await?;
    }
    Ok(sess)
}


pub async fn rec(session: u8, address: SocketAddr) -> anyhow::Result<Vec<u8>> {
    let conf = Conf::get();
    let soc = SOC.get().unwrap();
    let now = std::time::Instant::now();
    loop {
        let pacs = RecCache::get_pacs(address, session);
        if pacs.has_begin() && pacs.has_end() && pacs.is_continue() {
            let data = pacs.assembly();
            // take and clear and send finish to peer
            RecCache::clear(address, session);
            let finish_pac = SwapCmd::Finish.finish(session);
            soc.send_to(&finish_pac.to_bytes(), address).await?;
            return Ok(data);
        }
        let rec_from_0 = RecCache::time_differ(address, session);
        // ask peer to resend
        if rec_from_0 > conf.resend_elapse {
            let lacks = pacs.lack();
            for order in lacks.iter(){
                let resend_pac=SwapCmd::Resend.resend(session,*order);
                soc.send_to(&resend_pac.to_bytes(), address).await?;
            }
        }
        let differ = now.elapsed().as_micros() as i32;
        if differ > conf.rec_elapse { break; };
    }
    Ok(vec![])
}

