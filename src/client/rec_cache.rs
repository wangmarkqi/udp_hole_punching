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

pub static RCache: Lazy<Mutex<HashMap<(SocketAddr, u8), RecCache>>> = Lazy::new(|| {
    let m: HashMap<(SocketAddr, u8), RecCache> = HashMap::new();
    Mutex::new(m)
});

impl RecCache {
    pub fn get_pacs(addr: SocketAddr, sess: u8) -> Vec<Packet> {
        let mut store = RCache.lock().unwrap();
        let k = (addr, sess);
        if store.contains_key(&k) {
            let pacs = store[&k].pacs.clone();
            return pacs;
        }
        vec![]
    }
    pub fn add_pac(addr: SocketAddr, pac: Packet) {
        let mut store = RCache.lock().unwrap();
        let s = pac.session;
        let k = (addr, s);
        if !&store.contains_key(&k) {
            let mut v = vec![];
            v.push(pac);
            let rc = RecCache {
                timer: Instant::now(),
                pacs: v,
            };
            store.insert(k, rc);
            return;
        }
        let mut rec :RecCache= store[&k].clone();
        rec.pacs.add_no_duplicate(pac);
        store.insert(k, rec);
    }
    pub fn time_differ(addr: SocketAddr, sess: u8) -> i32 {
        let mut store = RCache.lock().unwrap();
        let k = (addr, sess);
        if store.contains_key(&k) {
            let start = store[&k].timer;
            let duration = start.elapsed();
            return duration.as_micros() as i32;
        }
        0
    }


    pub fn clear(addr: SocketAddr, sess: u8) {
        let k = (addr, sess);
        let mut store = RCache.lock().unwrap();
        store.remove(&k);
    }
}
