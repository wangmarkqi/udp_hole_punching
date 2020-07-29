use serde::{Deserialize, Serialize};
use async_std::net::{SocketAddr};
use rand::prelude::*;
#[derive(Serialize,PartialEq, Eq, Deserialize, Copy,Debug, Clone)]
pub enum CMD {
    Save,
    Open,
    P2P,
    None,
}


pub const PAC_SIZE: usize = 1472;
pub const MTU_SIZE: usize = 548;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Packet {
    #[serde(default = "localhost")]
    pub caller_address: SocketAddr,
    #[serde(default = "localhost")]
    pub callee_address: SocketAddr,
    #[serde(default = "empty")]
    pub callee_uuid: String,
    #[serde(default = "default_cmd")]
    pub cmd: CMD,
    #[serde(default = "default_true")]
    pub success: bool,
    #[serde(default = "empty")]
    pub err: String,

    #[serde(default = "default_vec")]
    pub msg: Vec<u8>,
    #[serde(default = "rand_i32")]
    pub session: i32,
    #[serde(default = "default0")]
    pub order:usize,
    #[serde(default = "default0")]
    pub max:usize,
}

fn localhost() -> SocketAddr {
    let localhost: SocketAddr = "0.0.0.0:0".parse().unwrap();
    localhost
}

fn default0()->usize{0}
pub fn rand_i32()->i32{
    let mut rng = rand::thread_rng();
    let res=rng.gen::<i32>();
    res
}
fn default_cmd() -> CMD { CMD::None }

fn empty() -> String {
    "".to_string()
}

fn default_true() -> bool {
    true
}

fn default_vec() -> Vec<u8> {
    let mut v = vec![];
    v.push(0);
    v
}

impl Packet {
    pub fn default() -> Self {
        let localhost: SocketAddr = "0.0.0.0:0".parse().unwrap();
        let empty = "".to_string();
        let mut v = vec![];
        v.push(0);
        Packet {
            caller_address: localhost,
            callee_address: localhost,
            callee_uuid: empty.clone(),
            cmd: CMD::None,
            msg: v,
            success: true,
            err: empty,
            session:rand_i32(),
            order:0,
            max:0,
        }
    }
    pub fn localhost() -> SocketAddr {
        localhost()
    }
    pub fn pack(&self) -> Vec<u8> {
        if let Ok(str) = serde_json::to_string(&self) {
            return str.as_bytes().to_vec();
        }
        let mut p = Packet::default();
        p.success = false;
        p.err = "serde to str err".to_string();
        let str2 = serde_json::to_string(&p).unwrap();
        str2.as_bytes().to_vec()
    }

}
