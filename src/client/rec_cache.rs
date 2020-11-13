use std::{sync::{Arc, Mutex, MutexGuard}, collections::HashMap};
use once_cell::sync::Lazy;
use super::packets::Packets;
use std::net::SocketAddr;
use super::packet::Packet;
use std::time::{Duration, Instant};

#[derive(PartialEq, Debug, Clone)]
pub struct RecCache {
    timer: Instant,
    pacs: Vec<Packet>,
}

pub static RCache: Lazy<Mutex<HashMap<SocketAddr, RecCache>>> = Lazy::new(|| {
    let m: HashMap<SocketAddr, RecCache> = HashMap::new();
    Mutex::new(m)
});

impl RecCache {
    pub fn get_pacs(addr: SocketAddr) -> Vec<Packet> {
        let mut store = RCache.lock().unwrap();
        if store.contains_key(&addr) {
            let pacs = store[&addr].pacs.clone();
            return pacs;
        }
        vec![]
    }
    pub fn add_pac(addr: SocketAddr, pac: &Packet) {
        let mut store = RCache.lock().unwrap();
        if !&store.contains_key(&addr) {
            let mut v = vec![];
            v.push(pac.to_owned());
            let rc = RecCache {
                timer: Instant::now(),
                pacs: v,
            };
            store.insert(addr, rc);
            return;
        }
        let mut rec: RecCache = store[&addr].clone();
        rec.pacs.add_no_duplicate(pac);
        store.insert(addr, rec);
    }

    pub fn clear(addr: SocketAddr) {
        let mut store = RCache.lock().unwrap();
        store.remove(&addr);
    }
}
