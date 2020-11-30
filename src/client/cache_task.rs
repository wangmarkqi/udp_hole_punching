use std::net::SocketAddr;
use super::listen_utils::*;
use super::sled_db::DB;
use async_trait::async_trait;
use crate::client::cache_send::Export2Task;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
struct TaskV {
    address: SocketAddr,
    data: Vec<u8>,
}

impl TaskV {
    fn new(addr: SocketAddr, data: &Vec<u8>) -> Self {
        Self {
            address: addr,
            data: data.to_owned(),
        }
    }
    fn enc(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    fn dec(v: &Vec<u8>) -> anyhow::Result<Self> {
        let res = bincode::deserialize(v)?;
        Ok(res)
    }
}

// for task
pub trait TaskSave {
    fn task_save(&self, address: SocketAddr, data: &Vec<u8>);
}

impl TaskSave for DB {
    fn task_save(&self, address: SocketAddr, data: &Vec<u8>) {
        if self != &DB::Task {
            panic!("wrong db");
        }
        let taskv = TaskV::new(address, data);
        let v = taskv.enc();
        let n = DB::gen_id();
        let k = n.to_be_bytes().to_vec();
        self.insert(&k, &v);
    }
}

// for task and send
#[async_trait]
pub trait DoSend {
    async fn do_send(&self) -> anyhow::Result<()>;
}

#[async_trait]
impl DoSend for DB {
    async fn do_send(&self) -> anyhow::Result<()> {
        if self != &DB::Task {
            panic!("wrong db");
        }
        let (k, v) = self.pop();
        if k.len() > 0 && v.len() > 0 {
            let taskv = TaskV::dec(&v)?;
            dbg!("task do send");
            let address = taskv.address;
            let data = taskv.data;
            let soc = SOC.get().unwrap();
            soc.send_to(&data, &address).await?;
        } else {
            DB::Send.export_task();
        }

        Ok(())
    }
}

#[test]
fn test_taskv() {
    DB::init();
    let data = b"ad".to_vec();
    let addr: SocketAddr = "127.0.0.1:0000".parse().unwrap();
    let v = TaskV::new(addr, &data);
    let b = v.enc();
    let k = b"1";
    let k = k.to_vec();
    DB::Send.insert(&k, &b);
    let bb = DB::Send.get_or_empty(&k);
    let c = TaskV::dec(&bb);
    dbg!(c);
}