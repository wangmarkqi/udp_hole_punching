use std::time:: Instant;
use super::conf::Conf;
use async_trait::async_trait;
use super::utils::*;
use crate::server::swap_cmd::SwapCmd;
use super::cache_rec::RecCacheTask;
use super::cache::Cache;

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
        let id=conf.id;
        if id=="".to_string(){
            return Ok(());
        }

        let soc = SOC.get().unwrap();
        // let mut last_hb = Instant::now();
        // 定时发送hb
        let elapse = self.time.elapsed().as_secs() as i32;
        if elapse > conf.heart_beat_interval {
            dbg!("send hb");
            let hb = SwapCmd::save(&id);
            soc.send_to(&hb, &conf.swap_server).await?;
            self.time = Instant::now();
        }
        Ok(())
    }
}

#[async_trait]
pub trait AskResend {
    async fn ask_resend(&mut self) -> anyhow::Result<()>;
}

#[async_trait]
impl AskResend for Timer {
    async fn ask_resend(&mut self) -> anyhow::Result<()> {
        let conf = Conf::get();
        let elapse = self.time.elapsed().as_micros() as i32;
        if elapse > conf.ask_resend_interval{
            Cache::Rec.ask_resend().await?;
            self.time = Instant::now();
        }
        Ok(())
    }
}
