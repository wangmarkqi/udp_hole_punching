use std::{sync::{Arc, Mutex, MutexGuard}, collections::HashMap};
use once_cell::sync::Lazy;
use super::packets::Packets;
use std::net::SocketAddr;
use super::packet::Packet;
use std::time::{Duration, Instant};

pub struct RecCache {
    taken:bool,
    timer: Instant,
    pacs: Vec<Packet>,
}

pub static RCache: Lazy<Mutex<HashMap<(SocketAddr, u8), RecCache>>> = Lazy::new(|| {
    let m: HashMap<(SocketAddr, u8), Vec<Packet>> = HashMap::new();
    Mutex::new(m)
});

impl RecCache {
    pub fn add_pac( addr: SocketAddr, pac: Packet) {
        let mut store = RCache.lock().unwrap();
        let s = pac.session;
        let k = (addr, s);
        if store.contains_key(&k) {
            let mut v = store[&k].clone();
            v.pacs.push(pac);
            store.insert(k, v);
        } else {
            let mut v = vec![];
            v.push(pac);
            let rc = RecCache {
                timer: Instant::now(),
                taken:false,
                pacs: v,
            }
            store.insert(k, rc);
        }
    }
    pub fn time_differ( addr: SocketAddr, sess: u8) -> i64 {
        let mut store = RCache.lock().unwrap();
        let k = (addr, sess);
        if store.contains_key(&k) {
            let start=store[&k].timer;
            let duration = start.elapsed();
            return duration.as_micros();
        }
        0
    }

    pub fn set_taken(addr: SocketAddr, sess: u8){
        let mut store = RCache.lock().unwrap();
        let k = (addr, sess);
        if store.contains_key(&k) {
            let mut rec = store[&k].clone();
            rec.taken=true;
            store.insert(k, rc);
        }
    }
    pub fn get_pacs(addr: SocketAddr, sess: u8) -> Vec<Packet> {
        let mut store = RCache.lock().unwrap();
        let k = (addr, sess);
        if store.contains_key(&k) {
            let pacs = store[&k].pacs.clone();
            return pacs;
        }
        vec![]
    }

    pub fn clear(addr: SocketAddr, sess: u8) {
        let k = (addr, sess);
        let mut store = RCache.lock().unwrap();
        store.remove(&k);
    }
}
