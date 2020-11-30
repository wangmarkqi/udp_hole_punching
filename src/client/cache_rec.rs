use std::net::SocketAddr;
use super::packet::Packet;
use super::packets::Packets;
use super::sled_db::DB;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
struct RecKey {
    pub address: SocketAddr,
    pub session: u32,
    pub max: u32,
    pub order: u32,
}

impl RecKey {
    fn new(pac: &Packet, addr: SocketAddr) -> Self {
        Self {
            address: addr,
            session: pac.sess,
            max: pac.max,
            order: pac.order,
        }
    }
    fn enc(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    fn dec(v: &Vec<u8>) -> anyhow::Result<Self> {
        let res = bincode::deserialize(v)?;
        Ok(res)
    }
    fn group_by_key(dic: &HashMap<Vec<u8>, Vec<u8>>) -> HashMap<(SocketAddr, u32), Vec<Packet>> {
        let mut m: HashMap<(SocketAddr, u32), Vec<Packet>> = HashMap::new();
        for (k, v) in dic.iter() {
            let key = RecKey::dec(k);
            if let Err(_) = key {
                continue;
            }
            let key = key.unwrap();
            let mk = (key.address, key.session);
            let pac = Packet::new_from_save_db(v);
            if m.contains_key(&mk) {
                let mut old = m[&mk].clone();
                old.push(pac);
                m.insert(mk, old);
            } else {
                let mut l = vec![];
                l.push(pac);
                m.insert(mk, l);
            }
        }
        m
    }
    fn get_complete_keys(dic: &HashMap<Vec<u8>, Vec<u8>>) -> Vec<(SocketAddr, u32)> {
        let m = RecKey::group_by_key(dic);
        let mut l = vec![];
        for (k, v) in m.iter() {
            if v.is_complete() {
                l.push(k.to_owned());
            }
        }
        l
    }
}

pub trait SingleSave {
    fn single_save(&self, address: SocketAddr, pac: &Packet);
}

impl SingleSave for DB {
    fn single_save(&self, address: SocketAddr, pac: &Packet) {
        if self != &DB::Rec {
            panic!("wrong db");
        }
        let key = RecKey::new(pac, address);
        self.insert(&key.enc(), &pac.to_bytes());
    }
}

// for rec
pub trait RecMsg {
    fn rec_one(&self, address: SocketAddr, sess: u32) -> Vec<u8>;
    fn rec_from(&self) -> (SocketAddr,Vec<u8>);
    fn del_by_session_address(&self, dic: &HashMap<Vec<u8>, Vec<u8>>, sese: u32, addr: SocketAddr);
}

impl RecMsg for DB {
    fn rec_one(&self, address: SocketAddr, sess: u32) -> Vec<u8> {
        if self != &DB::Rec {
            panic!("wrong db");
        }
        let dic = self.dic();
        if dic.len() == 0 { return vec![]; }
        let m = RecKey::group_by_key(&dic);
        for (key, pacs) in m.iter() {
            let is_complete = &pacs.is_complete();
            if key.0 == address && key.1 == sess && *is_complete {
                let mut data = pacs.clone();
                let res = data.assembly();
                self.del_by_session_address(&dic, sess, address);
                return res;
            }
        }
        vec![]
    }

    fn rec_from(&self) -> (SocketAddr,Vec<u8>) {
        if self != &DB::Rec {
            panic!("wrong db");
        }
        let dic = self.dic();
        let complete = RecKey::get_complete_keys(&dic);
        let default: SocketAddr = "0.0.0.0:8888".parse().unwrap();
        if complete.len() == 0 {
            return (default,vec![]);
        }
        let m = RecKey::group_by_key(&dic);
        for k in complete.iter() {
            let mut pacs = m[k].clone();
            let data = pacs.assembly();
            let addr=k.0;
            let sess=k.1;
            self.del_by_session_address(&dic, sess, addr);
            return (addr,data);
        }
        (default,vec![])
    }
    fn del_by_session_address(&self, dic: &HashMap<Vec<u8>, Vec<u8>>, sess: u32, addr: SocketAddr) {
        if self != &DB::Rec {
            panic!("wrong db");
        }
        for (k, _) in dic.iter() {
            let key = RecKey::dec(k);
            match key {
                Err(e) => {
                    dbg!(e);
                    self.remove(&k);
                }
                Ok(key) => {
                    if key.address == addr && key.session == sess {
                        self.remove(&k);
                    }
                }
            }
        }
    }
}

