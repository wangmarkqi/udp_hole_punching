use async_std::net::{SocketAddr};
use std::{sync::{Arc, Mutex, MutexGuard}, collections::HashMap};
use once_cell::sync::Lazy;

#[derive(PartialEq, Debug, Clone)]
pub struct Conf {
    pub id: String,
    pub size: usize,
    pub rec_elapse: i32,
    pub resend_elapse: i32,
    pub swap_server: String,
}

pub static CONF: Lazy<Mutex<Conf>> = Lazy::new(|| {
    let m = Conf::default();
    Mutex::new(m)
});

impl Conf {
    pub fn default() -> Self {
        let _id = "test";
        let conf=Conf {
            id: _id.to_string(),
            size: 1024 ,
            rec_elapse:4000,
            resend_elapse: 400,
            swap_server: String::from("127.0.0.1:4222"),
        };
        let mut m = CONF.lock().unwrap();
        *m = conf.clone();
        conf
    }
    pub fn set(&self) {
        let mut m = CONF.lock().unwrap();
        *m = self.clone();
    }
    pub fn get()->Self {
        let  m = &*CONF.lock().unwrap();
        m.clone()
    }
}