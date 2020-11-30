use std::net::SocketAddr;
use super::conf::Conf;
use super::packet::Packet;
use super::sled_db::DB;
use crate::client::cache_task::TaskSave;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
struct SendKey {
    pub address: SocketAddr,
    pub session: u32,
    pub retry: i32,
    pub max: u32,
    pub order: u32,
}

impl SendKey {
    fn new(pac: &Packet, addr: SocketAddr) -> Self {
        Self {
            address: addr,
            session: pac.sess,
            retry: 0,
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
}


// for send
pub trait BatchSave {
    fn batch_save(&self, address: SocketAddr, pacs: &Vec<Packet>);
}

impl BatchSave for DB {
    fn batch_save(&self, address: SocketAddr, pacs: &Vec<Packet>) {
        if self != &DB::Send {
            panic!("wrong db");
        }
        if pacs.len() == 0 {
            return;
        }
        for pac in pacs.iter() {
            let k = SendKey::new(pac, address);
            self.insert(&k.enc(), &pac.to_bytes());
        }
    }
}

// for send
pub trait SendDelOne {
    fn send_del_one(&self, address: SocketAddr, pac: &Packet);
}

impl SendDelOne for DB {
    fn send_del_one(&self, address: SocketAddr, pac: &Packet) {
        if self != &DB::Send {
            panic!("wrong db");
        }
        let conf = Conf::get();
        let max = conf.retry_send_times;
        let lower = conf.min_retry_len ;
        let mut k = SendKey::new(pac, address);
        for i in 0..max + 1 {
            k.retry = i;
            self.remove(&k.enc());
        }
    }
}


// for send
pub trait Export2Task {
    fn export_task(&self);
}

impl Export2Task for DB {
    fn export_task(&self) {
        if self != &DB::Send {
            panic!("wrong db");
        }
        let conf = Conf::get();
        let dic = self.dic();
        if dic.len() == 0 { return; }

        DB::Send.clear_tree();

        for (k, v) in dic.iter() {
            let key = SendKey::dec(&k);
            if let Err(_) = key {
                continue;
            }
            let key = key.unwrap();

            let retry = key.retry;
            let limit = &conf.retry_send_times;
            if retry < *limit {
                let address = key.address;
                DB::Task.task_save(address, &v);
                let mut newk = key.clone();
                newk.retry = key.retry + 1;
                self.insert(&newk.enc(), &v);
            }
        }
    }
}