use std::net::SocketAddr;
use super::conf::Conf;
use crate::client::cache_key::Key;
use super::packet::Packet;
use super::packets::Packets;
use super::listen_utils::*;
use super::sled_db::DB;
use async_trait::async_trait;


// for task
pub trait TaskSaveAsk {
    fn task_save_ask(&self, address: SocketAddr, data: &Vec<u8>);
}

impl TaskSaveAsk for DB {
    fn task_save_ask(&self, address: SocketAddr, data: &Vec<u8>) {
        let k = address.to_string().as_bytes().to_vec();
        self.insert(&k, data);
    }
}

// for task and send
#[async_trait]
pub trait DoTask {
    async fn do_task(&self) -> anyhow::Result<()>;
}

#[async_trait]
impl DoTask for DB {
    async fn do_task(&self) -> anyhow::Result<()> {
        let (k, v) = self.next();
        self.remove(&k);
        let address = String::from_utf8_lossy(&v);
        let s=address.to_string();
        let address:SocketAddr=s.parse()?;
        let soc = SOC.get().unwrap();
        soc.send_to(&v, &address).await?;
        Ok(())
    }
}