use std::{sync::{Mutex, MutexGuard}, collections::HashMap};
use once_cell::sync::Lazy;
use super::packets::Packets;
use std::net::SocketAddr;
use super::packet::Packet;
use std::time::{Duration, Instant};
use super::conf::Conf;
use crate::server::swap_cmd::SwapCmd;

pub trait ServerCache {
    fn save_cache(&self, buf: &Vec<u8>, total: usize);
    fn get_cache(&self) -> String;
    fn clear_cache(&self);
}

pub static SVCache: Lazy<Mutex<HashMap<SwapCmd, String>>> = Lazy::new(|| {
    let m: HashMap<SwapCmd, String> = HashMap::new();
    Mutex::new(m)
});

impl ServerCache for SwapCmd {
    fn save_cache(&self, buf: &Vec<u8>, total: usize) {
        let mut store = SVCache.lock().unwrap();
        let info = {
            if let Ok(i) = std::str::from_utf8(&buf[1..total - 1]) {
                i.to_string()
            } else {
                "".to_string()
            }
        };
        store.insert(self.to_owned(), info);
    }


    fn get_cache(&self) -> String {
        let store = SVCache.lock().unwrap();
        let conf = Conf::get();
        let start = Instant::now();
        loop {
            if store.contains_key(self) {
                let feedback = store[self].clone();
                if feedback.len() > 0 {
                    return feedback;
                }
            }
            let duration = start.elapsed();
            let differ = duration.as_micros() as i32;
            if differ > conf.rec_elapse {
                break;
            }
        }
        "".to_string()
    }

    fn clear_cache(&self) {
        let mut store = SVCache.lock().unwrap();
        if store.contains_key(self) {
            store.remove(self);
        }
    }
}
