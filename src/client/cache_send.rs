use std::net::SocketAddr;
use super::conf::Conf;
use crate::client::cache::Cache;
use super::packet::Packet;
use async_trait::async_trait;
use super::utils::*;

pub trait GenSession {
    fn gen_session(&self, addr: SocketAddr) -> u8;
}

impl GenSession for Cache {
    fn gen_session(&self, addr: SocketAddr) -> u8 {
        let  store = self.store();
        let mut sess_list = vec![];
        for k in store.keys() {
            if k.address == addr {
                sess_list.push(k.session);
            }
        }
        for i in 1..255 {
            let ii = i as u8;
            if !sess_list.contains(&ii) {
                return ii;
            }
        }
        0
    }
}

#[async_trait]
pub trait SendCacheTask {
    fn clear_timeout(&self);
    async fn resend(&self, addr: SocketAddr, pac: &Packet) -> anyhow::Result<()>;
}

#[async_trait]
impl SendCacheTask for Cache {
    fn clear_timeout(&self) {
        let conf = Conf::get();
        let timeout = {
            match self {
                Cache::Rec => conf.rec_cache_timeout,
                Cache::Send => conf.send_cache_timeout,
            }
        };
        for k in self.keys().iter() {
            let info = self.get(k);
            let how_long = info.time.elapsed().as_secs() as i32;
            // 超时删除
            if how_long > timeout {
                self.clear(k);
            }
        }
    }
    async fn resend(&self, addr: SocketAddr, pac: &Packet) -> anyhow::Result<()> {
        let sess = pac.sess;
        let order = pac.order;
        let k = self.key(addr, sess);
        let info = self.get(&k);
        let soc = SOC.get().unwrap();
        for p in info.pacs.iter() {
            if p.order == order {
                let data = p.to_bytes();
                soc.send_to(&data, k.address).await?;
            }
        }
        Ok(())
    }
}