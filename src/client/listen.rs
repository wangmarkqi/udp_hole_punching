use crate::server::swap_cmd::SwapCmd;
use crate::server::swap_protocal::Swap;
use super::conf::Conf;
use super::packet::Packet;
use super::timer::Timer;
use crate::client::timer::{HeartBeat, AskResend};
use super::cache::Cache;
use crate::client::cache_rec::RecCacheTask;
use crate::client::cache_send::SendCacheTask;
use std::collections::HashMap;
use super::utils::*;



pub async fn listen() {
    let res = _listen().await;
    match res {
        Ok(()) => dbg!("everything ok"),
        Err(e) => dbg!(&e.to_string()),
    };
}

async fn _listen() -> anyhow::Result<()> {
    // 远程连接必须0.0.0.0:0
    dbg!(" begin listen");
    let mut tim_hb = Timer::start();
    let mut tim_resend = Timer::start();
    loop {
        // task for rec cache****************
        Cache::Rec.clear_timeout();
        Cache::Rec.move_msg();
        // task for send cache****************
        Cache::Send.clear_timeout();
        // 定时发送hb
        tim_hb.heart_beat().await?;
        tim_resend.ask_resend().await?;
// 收data
        let (n, address, buf) = rec_with_timeout().await;
        if n == 0 { continue; }
        let cmd = SwapCmd::int2enum(buf[0]);

        if cmd.from_server() {
            dbg!("from server");
            let swap = Swap::new(&buf, address, n);
            match cmd {
                SwapCmd::Open => {
                    dbg!("send hello to another peer");
                    send_hello(&swap.id).await?;
                }

                SwapCmd::Ask => {
                    dbg!("update peer address");
                    let peer_address = swap.id;
                    update_peer_address(peer_address);
                }
                _ => {}
            }
            continue;
        }

        dbg!("below is from peer");
        let pac = Packet::new_from_rec_bytes(n, &buf);
        match cmd {
            SwapCmd::P2P => {
                Cache::Rec.add_pac(address, &pac);
            }
            SwapCmd::Resend => {
                Cache::Send.resend(address, &pac).await?;
            }
            _ => {}
        }
    }
}


async fn send_hello(peer: &str) -> anyhow::Result<()> {
    let hello = Packet::hello();
    let soc = SOC.get().unwrap();
    soc.send_to(&hello, peer).await?;
    Ok(())
}