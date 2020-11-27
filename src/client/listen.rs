use super::conf::Conf;
use super::listen_utils::*;
use std::net::SocketAddr;
use async_std::task::block_on;
use std::time::Duration;
use super::sled_db::DB;
use super::timer::{HeartBeat,Timer};
use crate::server::swap_cmd::SwapCmd;
pub fn listen() {
    let res = block_on(async {
        init_udp().await?;
        _listen().await
    });
    match res {
        Ok(()) => dbg!("everything ok"),
        Err(e) => dbg!(&e.to_string()),
    };
}

async fn _listen() -> anyhow::Result<()> {
    // 远程连接必须0.0.0.0:0
    println!("udp begin to listen");
    let mut tim_hb = Timer::start();
    loop {
        // 定时发送hb
        tim_hb.heart_beat();
        // send one from send or task
        process_send_task().await;
        //************************ rec data until err
        loop {
            match rec_with_timeout().await {
                Ok(res) => {
                    let (n, address, buf)=res;
                    let cmd = SwapCmd::int2enum(buf[0]);
                    if cmd.from_server() {
                        process_from_server(n, address, buf).await?;
                    } else {
                        process_from_peer(n, address, buf).await?;
                    }
                }
                Err(e) => {
                    dbg!(e);
                    break;
                }
            }
        }
    }
}
