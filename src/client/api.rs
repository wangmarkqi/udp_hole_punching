use std::net::SocketAddr;
use crate::server::swap_cmd::SwapCmd;
use super::conf::Conf;
use super::packet::Packet;
use std::time::Duration;
use super::sled_db::DB;
use super::peer_address::{update_peer_address, get_peer_address};
use super::cache_send::BatchSave;
use crate::client::cache_rec::MergeMsg;
use crate::client::cache_task::TaskSaveAsk;


pub fn ask_peer_address(peer_id: &str) -> anyhow::Result<()> {
    let conf = Conf::get();
    update_peer_address("".to_string());
    let send_data = SwapCmd::ask(peer_id);
    let s=&conf.swap_server;
    let address: SocketAddr= s.parse()?;
    DB::Task.task_save_ask(address, &send_data);
    Ok(())
}

pub fn read_peer_address() -> anyhow::Result<SocketAddr> {
    let res = get_peer_address();
    let addr: SocketAddr = res.parse()?;
    Ok(addr)
}

pub fn send(msg: &Vec<u8>, address: SocketAddr) -> u32 {
    let (sess, pacs) = Packet::new_pacs_from_send_bytes(msg);
    DB::Send.batch_save(address, &pacs.to_owned());
    sess
}

pub fn rec_from(address: SocketAddr, sess: u32) -> Vec<u8> {
    match DB::Rec.merge_msg(address,sess){
        Ok(res)=>res,
        Err(e)=>{
            dbg!(e);
            vec![]
        }
    }
}
pub fn rec_all() -> Vec<u8> {
    match DB::Rec.merge_msg(address,sess){
        Ok(res)=>res,
        Err(e)=>{
            dbg!(e);
            vec![]
        }
    }
}
