use std::{sync::{Arc, Mutex, MutexGuard}, collections::HashMap};
use super::packets::Packets;
use super::packet::Packet;
use std::net::SocketAddr;
use super::store::{SendCache, RecCache, Store};


pub type Cache = HashMap<(SocketAddr, u8), Packets>;

impl Cache {
    pub fn new_sess(addr: SocketAddr, s: Store) -> u8 {
        let store = s.get();
        let l = store.keys().iter().map(|e| e.1).collect();
        for i in 0..256 {
            if !l.contains(i) {
                return i;
            }
        }
        255
    }

    pub fn add_from_pac(addr: SocketAddr, pac: Packet, s: Store) {
        let mut store = s.get();
        let s = pac.session;
        let k = (addr, s);
        if store.contains_key(&k) {
            let mut v = sotre[k];
            v.push(pac);
            store.insert(k, v);
        } else {
            let mut v = vec![];
            v.push(pac);
            store.insert(k, v);
        }
    }


    fn get_from_sess(addr: SocketAddr, sess: u8, s: Store) -> Packets {
        let mut store = s.get();
        let k = (addr, sess);
        if store.contains_key(&k) {
            return sotre[k];
        }
        vec![]
    }

    fn clear(addr: SocketAddr, sess: u8, s: Store) {
        let k = (addr, sess);
        let mut store = s.get();
        store.remove(&k);
    }
}
