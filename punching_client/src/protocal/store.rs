use std::{sync::{Arc, Mutex, MutexGuard}, collections::HashMap};
use once_cell::sync::Lazy;
use super::packets::Packets;
use super::cache::CachePacs;
use std::net::SocketAddr;

pub enum Store {
    Send,
    Rec,
}

pub static SendCache: Lazy<Mutex<CachePacs>> = Lazy::new(|| {
    let m: HashMap<(SocketAddr,u8), Packets> = HashMap::new();
    Mutex::new(m)
});

pub static RecCache: Lazy<Mutex<CachePacs>> = Lazy::new(|| {
    let m: HashMap<(SocketAddr,u8), Packets> = HashMap::new();
    Mutex::new(m)
});

impl Store {
    pub fn get(&self)->MutexGuard<CachePacs> {
        match self {
            Store::Send => SendCache.lock().unwrap(),
            Store::Rec => RecCache.lock().unwrap(),
        }
    }
}
