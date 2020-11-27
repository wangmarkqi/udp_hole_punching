use std::net::SocketAddr;
use super::conf::Conf;
use crate::client::cache_key::Key;
use super::packet::Packet;
use super::packets::Packets;
use super::listen_utils::*;
use super::sled_db::DB;
use async_trait::async_trait;
use std::collections::HashMap;

pub trait SingleSave {
    fn single_save(&self, address: SocketAddr, pac: &Packet);
}

impl SingleSave for DB {
    fn single_save(&self, address: SocketAddr, pac: &Packet) {
        let key = Key::new(pac, address);
        self.insert(&key.enc(), &pac.to_bytes());
    }
}

// for rec
pub trait RecMsg {
    fn rec_one(&self, address: SocketAddr, sess: u32) -> Vec<u8>;
    fn rec_many(&self) -> Vec<Vec<u8>>;
    fn del_by_session_address(&self);
}

impl RecMsg for DB {
    fn rec_one(&self, address: SocketAddr, sess: u32) -> Vec<u8> {
        let dic = self.dic();
        let mut m = Key::group_by_key(&dic);
        for (key, mut pacs) in m.iter() {
            if key.address == address && key.session == sess && pacs.is_complete() {
                let res = pacs.assembly();


                return res;
            }
        }
        vec![]
    }

    fn rec_many(&self) -> Vec<Vec<u8>> {
        let mut res = vec![];
        let dic = self.dic();
        let m = Key::group_by_key(&dic);
        let complete = Key::get_complete_keys(&dic);
        for k in complete.iter() {
            let mut pacs = m[k];
            let data = pacs.assembly();
            res.push(data);
            self.remove(&k.enc());
        }
        res
    }
}

