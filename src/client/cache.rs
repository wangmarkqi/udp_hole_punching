use std::{sync::{Mutex, MutexGuard}, collections::HashMap};
use once_cell::sync::Lazy;
use super::packets::Packets;
use std::net::SocketAddr;
use super::packet::Packet;
use std::time::Instant;

#[derive(PartialEq, Debug, Clone)]
pub struct Info {
    pub time: Instant,
    pub pacs: Vec<Packet>,
}

impl Info {
    pub fn default() -> Self {
        Info {
            time: Instant::now(),
            pacs: vec![],
        }
    }
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub struct Key {
    pub address: SocketAddr,
    pub session: u8,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Cache {
    Rec,
    Send,
}

pub static RCache: Lazy<Mutex<HashMap<Key, Info>>> = Lazy::new(|| {
    let m: HashMap<Key, Info> = HashMap::new();
    Mutex::new(m)
});
pub static SCache: Lazy<Mutex<HashMap<Key, Info>>> = Lazy::new(|| {
    let m: HashMap<Key, Info> = HashMap::new();
    Mutex::new(m)
});

impl Cache {
    pub fn store(&self) -> MutexGuard<HashMap<Key, Info>> {
        match self {
            Cache::Rec => RCache.lock().unwrap(),
            Cache::Send => SCache.lock().unwrap(),
        }
    }
    pub fn key(&self, addr: SocketAddr, sess: u8) -> Key {
        Key {
            address: addr,
            session: sess,
        }
    }
    pub fn keys(&self) -> Vec<Key> {
        let store = self.store();
        let l = store.keys().into_iter().map(|e| e.to_owned()).collect();
        l
    }
    pub fn is_empty(&self, k: &Key) -> bool {
        let store = self.store();
        if !&store.contains_key(&k) {
            return true;
        }
        let info = store[&k].clone();
        if info.pacs.len() == 0 {
            return true;
        }
        false
    }
    pub fn get(&self, k: &Key) -> Info {
        if self.is_empty(k) {
            return Info::default();
        }
        let store = self.store();
        let info = store[&k].clone();
        info
    }
    pub fn is_complete(&self, k: &Key) -> bool {
        let info = self.get(k);
        let mut pacs = info.pacs;
        let lack=pacs.lacks();
        lack.len()==0
    }
    // use func below by callee
    pub fn add_pac(&self, addr: SocketAddr, pac: &Packet) -> u8 {
        let sess = pac.sess;
        let k = self.key(addr, sess);
        self.add_one(&k, pac);
        sess
    }
    // with out duplicate and sort
    pub fn add_one(&self, k: &Key, pac: &Packet) {
        let mut info = self.get(k);
        // store 在info后面，否则死锁
        let mut store = self.store();
        let orders: Vec<u32> = info.pacs.iter().map(|e| e.order).collect();
        if orders.contains(&pac.order) {
            return;
        }
        info.pacs.push(pac.to_owned());
        info.pacs.sort();
        store.insert(k.to_owned(), info);
    }
    pub fn save_all(&self, k: &Key, packs: Vec<Packet>) {
        let mut store = self.store();
        let info = Info {
            time: Instant::now(),
            pacs: packs,
        };
        store.insert(k.to_owned(), info);
    }

    pub fn clear(&self, k: &Key) {
        let mut store = self.store();
        store.remove(&k);
    }
}
