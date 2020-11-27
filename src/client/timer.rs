use std::time::Instant;
use super::conf::Conf;
use super::listen_utils::*;
use crate::server::swap_cmd::SwapCmd;
use std::net::SocketAddr;
use super::sled_db::DB;
use async_trait::async_trait;


pub struct Timer {
    time: Instant,
}

impl Timer {
    pub fn start() -> Self {
        Timer {
            time: Instant::now(),
        }
    }
}
#[async_trait]
pub trait HeartBeat {
    async fn heart_beat(&mut self) -> anyhow::Result<()>;
}

#[async_trait]
impl HeartBeat for Timer {
    async fn heart_beat(&mut self) -> anyhow::Result<()> {
        let conf = Conf::get();
        let id = conf.id;
        if id == "".to_string() {
            dbg!("null id not send to save");
            return Ok(());
        }

        // let mut last_hb = Instant::now();
        // 定时发送hb
        let elapse = self.time.elapsed().as_secs() as i32;
        if elapse < conf.heart_beat_interval {
            return Ok(());
        }
        self.time = Instant::now();

        let hb = SwapCmd::save(&id);
        let s = &conf.swap_server;
        let address: SocketAddr = s.parse()?;
        let soc = SOC.get().unwrap();
        soc.send_to(&hb, address).await?;
        Ok(())
    }
}

