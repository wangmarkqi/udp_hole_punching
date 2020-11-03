use serde::{Serialize,Deserialize};
use async_std::net::{SocketAddr};
use rand::prelude::*;
use std::io::prelude::*;
use std::fs::File;
#[derive(Serialize, PartialEq, Eq, Deserialize, Copy, Debug, Clone)]
pub enum CMD {
    Save,
    Open,
    P2P,
    None,
}

pub const PAC_SIZE: usize = 1472;

pub fn get_uuid() -> anyhow::Result<String> {
    let path = "./uuid";
    let p = std::path::Path::new(path);
    if p.exists() {
        let res = std::fs::read_to_string(p)?;
        if res != "" {
            return Ok(res);
        }
    }
    let my_uuid = uuid::Uuid::new_v4();
    let mut output: File = File::create(p)?;
    write!(output, "{}", my_uuid)?;
    let content = format!("{}", my_uuid);
    Ok(content)
}
pub fn localhost() -> SocketAddr {
    let localhost: SocketAddr = "0.0.0.0:0".parse().unwrap();
    localhost
}

pub fn default0() -> usize { 0 }

pub fn rand_u16() -> u16 {
    let mut rng = rand::thread_rng();
    let res = rng.gen::<u16>();
    res
}

pub fn default_cmd() -> CMD { CMD::None }

pub fn empty() -> String {
    "".to_string()
}

pub fn default_true() -> bool {
    true
}

pub fn default_vec() -> Vec<u8> {
    let mut v = vec![];
    v.push(0);
    v
}