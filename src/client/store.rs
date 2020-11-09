use std::{sync::{Arc, Mutex, MutexGuard}, collections::HashMap};
use once_cell::sync::Lazy;
use super::packets::Packets;
use std::net::SocketAddr;
use super::packet::Packet;

pub enum Store {
    Send,
    Rec,
}

pub static SendCache: Lazy<Mutex<HashMap<(SocketAddr, u8),Vec<Packet>>>> = Lazy::new(|| {
    let m: HashMap<(SocketAddr, u8),Vec<Packet>> = HashMap::new();
    Mutex::new(m)
});

pub static RecCache: Lazy<Mutex<HashMap<(SocketAddr, u8),Vec<Packet>>>> = Lazy::new(|| {
    let m: HashMap<(SocketAddr, u8),Vec<Packet>> = HashMap::new();
    Mutex::new(m)
});

impl Store {
    pub fn get(&self) -> MutexGuard<HashMap<(SocketAddr, u8),Vec<Packet>>> {
        match self {
            Store::Send => SendCache.lock().unwrap(),
            Store::Rec => RecCache.lock().unwrap(),
        }
    }
    pub fn new_sess(&self,addr: SocketAddr) -> u8 {
        let store = self.get();
        for i in 0..=255 {
            if !store.contains_key(&(addr, i)) {
                return i;
            }
        }
        255
    }

    pub fn add_from_pac(&self,addr: SocketAddr, pac: Packet) {
        let mut store = self.get();
        let s = pac.session;
        let k = (addr, s);
        if store.contains_key(&k) {
            let mut v = store[&k].clone();
            v.push(pac);
            store.insert(k, v);
        } else {
            let mut v = vec![];
            v.push(pac);
            store.insert(k, v);
        }
    }


    fn get_from_sess(&self,addr: SocketAddr, sess: u8 ) -> Vec<Packet>{
        let mut store = self.get();
        let k = (addr, sess);
        if store.contains_key(&k) {
            return store[&k].clone();
        }
        vec![]
    }

    fn clear(&self,addr: SocketAddr, sess: u8, ) {
        let k = (addr, sess);
        let mut store = self.get();
        store.remove(&k);
    }
}
