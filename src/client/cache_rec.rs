use std::net::SocketAddr;
use super::packet::Packet;
use super::packets::Packets;
use super::conf::Conf;
use crate::client::cache::Cache;
use async_trait::async_trait;
use std::collections::VecDeque;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use super::utils::*;
pub static Msg: Lazy<Mutex<VecDeque<(SocketAddr, Vec<u8>)>>> = Lazy::new(|| {
    let conf = Conf::get();
    let m: VecDeque<(SocketAddr, Vec<u8>)> = VecDeque::with_capacity(conf.msg_queue_len);
    Mutex::new(m)
});


// ask resend ,clear cache by rec timeout and move to msg
#[async_trait]
pub trait RecCacheTask {
    fn move_msg(&self);
    async fn ask_resend(&self) -> anyhow::Result<()>;
}

#[async_trait]
impl RecCacheTask for Cache {
    // 完整移交
    fn move_msg(&self) {
        let mut msg = Msg.lock().unwrap();
        for k in self.keys().iter() {
            if self.is_complete(k) {
                let info = self.get(k);
                let mut pacs = info.pacs;
                let data = pacs.assembly();
                msg.push_back((k.address, data));
                self.clear(k);
            }
        }
    }
    async fn ask_resend(&self) -> anyhow::Result<()> {
        let soc = SOC.get().unwrap();
        let conf = Conf::get();
        for k in self.keys().iter() {
            let info = self.get(k);
            let save_time = info.time;
            let save_elapse = save_time.elapsed().as_micros() as i32;
            if save_elapse < conf.ask_resend_elapse {
                continue;
            }

            let mut pacs = info.pacs;
            let lack_order = pacs.lacks();
            if lack_order.len() == 0 {
                continue;
            }
            for ord in lack_order.iter() {
                let resend = Packet::resend(k.session, *ord);
                soc.send_to(&resend, k.address).await?;
            }
        }
        Ok(())
    }
}
