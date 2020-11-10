use std::sync::Mutex;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use super::packets::Packets;
use std::net::SocketAddr;
use super::packet::Packet;


pub static SCache: Lazy<Mutex<HashMap<(SocketAddr, u8), Vec<Packet>>>> = Lazy::new(|| {
    let m: HashMap<(SocketAddr, u8), Vec<Packet>> = HashMap::new();
    Mutex::new(m)
});


fn new_sess(addr: SocketAddr) -> u8 {
    let store = SCache.lock().unwrap();
    for i in 0..=255 {
        if !store.contains_key(&(addr, i)) {
            return i;
        }
    }
    255
}

pub fn save_send_cache(data:&Vec<Packet>,addr:SocketAddr)->u8{
    let sess=new_sess(addr);
    let mut store = SCache.lock().unwrap();
    let k = (addr, sess);
    store.insert(k,data.to_owned());
    sess
}
pub fn get_send_cache(addr: SocketAddr, sess: u8) -> Vec<Packet> {
    let store = SCache.lock().unwrap();
    let k = (addr, sess);
    if store.contains_key(&k) {
        return store[&k].clone();
    }
    vec![]
}

pub fn clear_send_cache(addr: SocketAddr, sess: u8) {
    let mut store = SCache.lock().unwrap();
    let k = (addr, sess);
    store.remove(&k);
}

