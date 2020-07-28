use std::net::{SocketAddr};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize,Debug,Clone)]
pub enum CMD {
    Save,
    Open,
    P2P,
    None,
}


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
    #[serde(default = "empty")]
    pub msg: String,
    #[serde(default = "default_true")]
    pub success: bool,
    #[serde(default = "empty")]
    pub err: String,
}

fn localhost() -> SocketAddr {
    let localhost: SocketAddr = "0.0.0.0:0".parse().unwrap();
    localhost
}

fn default_cmd() -> CMD { CMD::None }

fn empty() -> String {
    "".to_string()
}

fn default_true() -> bool {
    true
}

fn default_vec() -> Vec<u8> { vec![] }

impl Packet {
    pub fn default() -> Self {
        let localhost: SocketAddr = "0.0.0.0:0".parse().unwrap();
        let empty = "".to_string();
        Packet {
            caller_address: localhost,
            callee_address: localhost,
            callee_uuid: empty.clone(),
            cmd: CMD::None,
            msg: "".to_string(),
            success: true,
            err: empty,
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
