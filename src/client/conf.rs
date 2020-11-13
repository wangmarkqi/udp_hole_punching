use async_std::net::{SocketAddr};
use std::{sync::{Arc, Mutex, MutexGuard}, collections::HashMap};
use once_cell::sync::Lazy;

#[derive(PartialEq, Debug, Clone)]
pub struct Conf {
    pub id: String,
    pub size: usize,
    // micro secs
    pub rec_elapse: i32,
    // micro secs
    pub send_timeout: i32,
    pub swap_server: String,
    // 秒
    pub heart_beat:i32,
}

pub static CONF: Lazy<Mutex<Conf>> = Lazy::new(|| {
    let mut m = Conf::default();
    Mutex::new(m)
});

impl Conf {
    pub fn default() -> Self {
        let _id = "test";
        let conf = Conf {
            id: _id.to_string(),
            size: 1024,
            rec_elapse: 800,
            send_timeout: 4000,
            swap_server: String::from("127.0.0.1:4222"),
            heart_beat:8,
        };
        conf
    }
    pub fn set(&self) {
        let mut m = CONF.lock().unwrap();
        *m = self.clone();
    }
    pub fn get() -> Self {
        let m = &*CONF.lock().unwrap();
        m.clone()
    }
}