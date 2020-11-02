use serde::{Deserialize, Serialize};
use async_std::net::{SocketAddr};
use bincode::{deserialize, serialize};
use super::tools::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Packet {
    pub address: SocketAddr,
    pub callee: String,
    pub cmd: CMD,
    pub success: bool,
    pub err: String,
    pub session: u16,
    pub order: u16,
    pub max: u16,
    pub body_len: u16,
}


impl Packet {
    pub fn default() -> Self {
        let localhost: SocketAddr = "0.0.0.0:0".parse().unwrap();
        let empty = "".to_string();
        Packet {
            address: localhost,
            callee: empty.clone(),
            cmd: CMD::None,
            success: true,
            err: empty,
            session: rand_u16(),
            order: 0,
            max: 0,
            body_len: 0,
        }
    }

    pub fn callee_save_default() -> anyhow::Result<Self> {
        let mut def = Packet::default();
        let uuid = get_uuid()?;
        def.callee = uuid;
        def.cmd = CMD::Save;
        Ok(def)
    }

    pub fn caller_open_default(id: &str) -> Self {
        let mut def = Packet::default();
        def.cmd = CMD::Open;
        def.callee = id.to_string();
        def
    }
    pub fn p2p_default(address: SocketAddr) -> Self {
        let mut def = Packet::default();
        def.cmd = CMD::P2P;
        def.address = address;
        def
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut encoded: Vec<u8> = serialize(&self).unwrap();
        let n = encoded.len() as u16;
        // u16::from_be_bytes([0x12, 0x34]);
        let header_bytes = n.to_be_bytes();
        let mut header_vec = header_bytes.to_vec();
        header_vec.append(&mut encoded);
        header_vec
    }
    pub fn unpack(enc: &Vec<u8>) -> anyhow::Result<Self> {
        let header = [enc[0], enc[1]];
        let n = u16::from_be_bytes(header) as usize;
        let mut body = vec![0; n];
        for i in 0..n {
            body[i] = enc[i + 2];
        }
        let dec = deserialize(&body)?;
        Ok(dec)
    }
}

