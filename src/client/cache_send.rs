use std::net::SocketAddr;
use super::conf::Conf;
use crate::client::cache_key::Key;
use super::packet::Packet;
use super::packets::Packets;
use super::listen_utils::*;
use super::sled_db::DB;
use async_trait::async_trait;

// for send
pub trait BatchSave {
    fn batch_save(&self, address: SocketAddr, pacs: &Vec<Packet>);
}

impl BatchSave for DB {
    fn batch_save(&self, address: SocketAddr, pacs: &Vec<Packet>) {
        if pacs.len() == 0 {
            return;
        }
        for pac in pacs.iter() {
            let k = Key::new(pac, address);
            self.insert(&k.enc(), &pac.to_bytes());
        }
    }
}

// for send
pub trait SendDelOne {
    fn send_del_one(&self, address: SocketAddr, pacs: &Packet);
}

impl SendDelOne for DB {
    fn send_del_one(&self, address: SocketAddr, pac: &Packet) {
        let k = Key::new(pac, address);
        self.remove(&k.enc());
    }
}


// for send
#[async_trait]
pub trait DoSend {
    async fn do_send(&self) -> anyhow::Result<()>;
}

#[async_trait]
impl DoSend for DB {
    async fn do_send(&self) -> anyhow::Result<()> {
        let (k, v) = self.next();
        self.remove(&k);

        if let Ok(key) = Key::dec(&k) {
            let conf = Conf::get();
            let retry = key.retry;
            if retry < conf.retry_send_times as usize {
                let mut newk = key.clone();
                newk.retry = key.retry + 1;
                self.insert(&newk.enc(), &v);
            }


            let address = key.address;
            let soc = SOC.get().unwrap();
            soc.send_to(&v, &address).await?;
        }
        Ok(())
    }
}