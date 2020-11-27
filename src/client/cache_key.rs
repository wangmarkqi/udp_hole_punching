use std::{sync::{Mutex, MutexGuard}, collections::HashMap};
use once_cell::sync::Lazy;
use super::packets::Packets;
use std::net::SocketAddr;
use super::packet::Packet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
pub struct Key {
    pub address: SocketAddr,
    pub session: u32,
    pub retry: usize,
    pub max: u32,
    pub order: u32,
}

impl Key {
    pub fn new(pac: &Packet, addr: SocketAddr) -> Self {
        Self {
            address: addr,
            session: pac.sess,
            retry: 0,
            max: pac.max,
            order: pac.order,
        }
    }
    pub fn enc(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    pub fn dec(v: &Vec<u8>) -> anyhow::Result<Self> {
        let res = bincode::deserialize(v)?;
        Ok(res)
    }
    pub fn group_by_key(dic: &Haskmap<Vec<u8>, Vec<u8>>) -> HashMap<(SocketAddr,u32), Vec<Packet>> {
        let mut m = HashMap::new();
        for (k, v) in dic.iter() {
            let key = Key::dec(k)?;
            let mk=(key.address,key.session);
            let pac = Packet::new_from_save_db(v);
            if m.contains_key(&mk){
                m[mk].push(pac);
            }else{
                let mut  l=vec![];
                l.push(pac);
                m[mk]=l;
            }
        }
        m
    }
    pub fn get_complete_keys(dic: &Haskmap<Vec<u8>, Vec<u8>>) -> Vec<(SocketAddr,u32)> {
        let m=Key::group_by_key(dic);
        let mut l=vec![];
        for (k,v) in m.iter(){
            if v.is_complete(){
                l.push(k.to_owned());
            }
        }
        l
    }
}

