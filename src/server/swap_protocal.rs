use async_std::net::{SocketAddr};
use super::swap_cmd::SwapCmd;

#[derive(PartialEq, Debug, Clone)]
pub struct Swap {
    pub cmd: u8,
    pub id: String,
    pub address: SocketAddr,
}

// 交换协议格式
// [cmd1,id...]
impl Swap {
    pub fn new(buf: &Vec<u8>, me: SocketAddr, total: usize) -> Self {
        let ids = {
            if let Ok(i) = std::str::from_utf8(&buf[1..total - 1]) {
                i
            } else {
                ""
            }
        };
        Self {
            cmd: buf[0],
            id: ids.to_string(),
            address: me,
        }
    }

    pub fn pack_err(err:&str) -> Vec<u8> {
        let err_code=SwapCmd::ServerErr;
        let err_u=err_code.enum2int();
        let mut v = vec![];
        v.push(err_u);
        let ss = err.as_bytes();
        for i in ss.iter() {
            v.push(*i);
        }
        v
    }
    pub fn pack(&self, b: &[u8]) -> Vec<u8> {
        let mut v = vec![];
        v.push(self.cmd);
        // let ss = s.as_bytes();
        for i in b.iter() {
            v.push(*i);
        }
        v
    }
}

