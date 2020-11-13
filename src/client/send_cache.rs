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


pub fn new_sess(addr: SocketAddr) -> u8 {
    let store = SCache.lock().unwrap();
    for i in 0..=255 {
        if !store.contains_key(&(addr, i)) {
            return i;
        }
    }
    255
}

pub fn save_send_cache(data:&Vec<Packet>,addr:SocketAddr,sess:u8){
    let mut store = SCache.lock().unwrap();
    let k = (addr, sess);
    store.insert(k,data.to_owned());
}
pub fn get_send_cache(addr: SocketAddr, sess: u8,order:u32) -> anyhow::Result<Packet> {
    let store = SCache.lock().unwrap();
    let k = (addr, sess);
    if store.contains_key(&k) {
        let pacs=&store[&k];
        for pac in pacs.iter(){
            if pac.order==order{
                return Ok(pac.to_owned());
            }
        }
    }
    Err(anyhow!("can not get send cache"))
}

pub fn clear_send_cache(addr: SocketAddr, sess: u8) {
    let mut store = SCache.lock().unwrap();
    let k = (addr, sess);
    store.remove(&k);
}

