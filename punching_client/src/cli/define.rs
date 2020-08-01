use async_trait::async_trait;
pub use punching_server::{PAC_SIZE, Packet, CMD};
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;
use async_std::net::UdpSocket;
use once_cell::sync::OnceCell;
pub static SOC: OnceCell<UdpSocket> = OnceCell::new();
pub static CONN: OnceCell<UdpSocket> = OnceCell::new();


#[async_trait]
pub trait Sender {
    fn segmentation(&self, msg: &Vec<u8>) -> Vec<Vec<u8>>;
    async fn send_pac(&self, me: Who, msg: &Vec<u8>) -> anyhow::Result<u16>;
}

#[async_trait]
pub trait Receiver {
    fn get_cached(&self, me: Who) -> Vec<(u16, Vec<u8>)>;
    fn is_done(&self, me: Who) -> bool;
    fn clear_cached(&self, me: Who) ;
    fn assembly(&self, me: Who) -> anyhow::Result<Vec<u8>>;
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Who {
    Callee,
    Caller,
}

pub static REC_CALLEE: Lazy<Mutex<HashMap<(u16, u16), Vec<(u16, Vec<u8>)>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let mut v = vec![];
    let vv: Vec<u8> = vec![];
    v.push((0 as u16, vv));
    map.insert((0 as u16, 0 as u16), v);
    Mutex::new(map)
});
pub static REC_CALLER: Lazy<Mutex<HashMap<(u16, u16), Vec<(u16, Vec<u8>)>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let mut v = vec![];
    let vv: Vec<u8> = vec![];
    v.push((0 as u16, vv));
    map.insert((0 as u16, 0 as u16), v);
    Mutex::new(map)
});
