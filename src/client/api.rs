use std::net::SocketAddr;
use crate::server::swap_cmd::SwapCmd;
use super::conf::Conf;
use super::packet::Packet;
use super::sled_db::DB;
use super::peer_address::{update_peer_address, get_peer_address};
use super::cache_send::BatchSave;
use crate::client::cache_rec::RecMsg;
use crate::client::cache_task::TaskSave;


pub fn ask_peer_address(peer_id: &str) -> anyhow::Result<()> {
    let conf = Conf::get();
    update_peer_address("".to_string());
    let send_data = SwapCmd::ask(peer_id);
    let s = &conf.swap_server;
    let address: SocketAddr = s.parse()?;
    DB::Task.task_save(address, &send_data);
    Ok(())
}

pub fn read_peer_address() -> anyhow::Result<SocketAddr> {
    let res = get_peer_address();
    dbg!(&res);
    let addr: SocketAddr = res.parse()?;
    Ok(addr)
}

pub fn send(msg: &Vec<u8>, address: SocketAddr) -> u32 {
    let conf = Conf::get();
    let (sess, pacs) = Packet::new_pacs_from_send_bytes(msg);
    let total = &pacs.len();
    dbg!(total);
    let lower = conf.min_retry_len as usize;
    if total > &lower {
        dbg!("save send cache");
        DB::Send.batch_save(address, &pacs.to_owned());
    }
    for pac in pacs.iter() {
        DB::Task.task_save(address, &pac.to_bytes());
    }
    sess
}

pub fn rec_one(address: SocketAddr, sess: u32) ->  Vec<u8> {
    DB::Rec.rec_one(address, sess)
}

pub fn rec_from() -> (SocketAddr, Vec<u8>) {
    DB::Rec.rec_from()
}
