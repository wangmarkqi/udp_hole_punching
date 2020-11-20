use async_std::net::UdpSocket;
use std::net::SocketAddr;
use once_cell::sync::OnceCell;
use crate::server::swap_cmd::SwapCmd;
use crate::server::swap_protocal::Swap;

use once_cell::sync::Lazy;
use super::conf::Conf;
use super::packet::Packet;
use std::sync::Mutex;
use super::timer::Timer;
use crate::client::timer::{HeartBeat, AskResend};
use super::cache::Cache;
use crate::client::cache_rec::RecCacheTask;
use crate::client::cache_send::SendCacheTask;
use std::collections::HashMap;

/// # Examples
/// A simple peer-to-peer echo callee
/// ```
///use async_std::task::block_on;
///  fn echo(msg:&Vec<u8>)->Vec<u8>{
///     msg.to_vec()

/// fn main() {
///      let host = "0.0.0.0:4222";
///      block_on(punching_server::make_match(host)).unwrap();
///
///     let remote = "xx.xx.xx.xx:xxxxx";
///     block_on(punching_client::listen(remote ,&echo)).unwrap_or(());

/// ```


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
                    send_hello(&swap.id).await?;
                    let peer_address = swap.id;
                    update_peer_address(peer_address);
                }
                _ => {}
            }
            continue;
        }


        dbg!("below is from peer");
        let pac = Packet::new_from_rec_bytes(n, &buf);
        dbg!(&pac);
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
pub static SOC: OnceCell<UdpSocket> = OnceCell::new();
pub static PeerAddress: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new("".to_string())
});
fn update_peer_address(address:String){
    let mut store=PeerAddress.lock().unwrap();
    *store=address;
}
async fn send_hello(peer: &str) -> anyhow::Result<()> {
    let soc = SOC.get().unwrap();
    let hello = Packet::hello();
    soc.send_to(&hello, peer).await?;
    Ok(())
}

async fn rec_with_timeout() -> (usize, SocketAddr, Vec<u8>) {
    let conf = Conf::get();
    let soc = SOC.get().unwrap();
    let mut buf = vec![0u8; conf.size];

    let res = async_std::io::timeout(std::time::Duration::from_micros(conf.single_rec_timeout as u64), async {
        soc.recv_from(&mut buf).await
    }).await;
    if let Err(e) = res {
        let default: SocketAddr = "127.0.0.1:0000".parse().unwrap();
        return (0, default, buf);
    }
    let (n, address) = res.unwrap();
    (n, address, buf)
}

