use async_std::net::UdpSocket;
use std::net::SocketAddr;
use once_cell::sync::OnceCell;
use crate::server::swap_cmd::SwapCmd;
use crate::server::swap_protocal::Swap;
use std::time::{Duration, Instant};
use super::conf::Conf;
use super::packet::Packet;
use super::rec_cache::RecCache;
use super::packets::Packets;
use super::api_peer::*;
use crate::client::cmd_packet::Cmd2Pac;
use super::api_server::*;

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
pub async fn listen() -> anyhow::Result<()> {
    // 远程连接必须0.0.0.0:0
    dbg!(" begin listen");
    let conf = Conf::get();
    let soc = UdpSocket::bind("0.0.0.0:0").await?;

    let mut last_hb = Instant::now();
    loop {
        // 定时发送hb
        let elapse = last_hb.elapsed().as_secs() as i32;
        if elapse > conf.heart_beat {
            dbg!("send hb");
            let hb = SwapCmd::save(&conf.id);
            soc.send_to(&hb, &conf.swap_server).await?;
            last_hb = Instant::now();
        }

// 收data
        let mut buf = vec![0u8; conf.size];

        let receive = async_std::io::timeout(Duration::from_micros(conf.rec_elapse as u64), async {
            soc.recv_from(&mut buf).await
        }).await;

        if let Err(e) = receive {
            continue;
        };
        let (n,address)=receive.unwrap();
        dbg!("rec data");
        dbg!(n,address);

        let cmd = buf[0];
        let cmd = SwapCmd::int2enum(cmd);

        let from_server = cmd.from_server();
        if from_server {
            let swap = Swap::new(&buf, address, n);
            match cmd {
                // open is server  init do not feed back,only send to peer
                SwapCmd::Open => {
                    // open is server init
                    dbg!("send hello to peer");
                    let peer_address = swap.id;
                    // 这种地方发送给peer!!!!!!!!!!!!!
                    let pac = SwapCmd::Hello.hello();
                    soc.send_to(&pac.to_bytes(), peer_address).await?;
                    continue;
                }
                _ => {
                    dbg!("heart beat success");
                    dbg!(swap);
                    // ask, save servererror
                    continue;
                }
            }
        }
        // below is from peer

        let pac = Packet::new_from_rec_bytes(n, &buf.to_vec());
        dbg!("from peer");
        dbg!(&pac);
        match cmd {
            SwapCmd::P2P => {
                // save to cache first and send finish to sender
                RecCache::add_pac(address, &pac);
                let order = &pac.order;
                let fini_pac = SwapCmd::Finish.finish(*order);
                dbg!("send peer finish order");
                soc.send_to(&fini_pac.to_bytes(), address).await?;
            }
            // peer tell me he open the door
            SwapCmd::Hello => {
                dbg!(format!("receive hello from {}", address));
                continue;
            }
            // peer tell me he got all and i del send cache
            // peer tell me he  did not  get all and i resend

            _ => {}
        }
    }
}


pub async fn test_callee_listen() -> anyhow::Result<()> {
    let mut conf = Conf::default();
    conf.swap_server = "39.96.40.177:4222".to_string();
    conf.heart_beat=40;
    conf.set();
    listen().await;
    Ok(())
}