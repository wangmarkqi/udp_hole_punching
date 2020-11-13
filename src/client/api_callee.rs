use async_std::net::UdpSocket;
use std::net::SocketAddr;
use once_cell::sync::OnceCell;
use crate::server::swap_cmd::SwapCmd;
use crate::server::swap_protocal::Swap;
use std::time::{Duration, Instant};
use super::conf::Conf;
use super::packet::Packet;
use super::send_cache::save_send_cache;
use super::rec_cache::RecCache;
use super::packets::Packets;
use super::api_server::SOC;
use super::server_cache::ServerCache;
use super::api_peer::*;
use crate::client::cmd_packet::Cmd2Pac;
use crate::client::send_cache::{clear_send_cache, get_send_cache};

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
pub async fn listen(handler: &dyn Fn(&Vec<u8>) -> Vec<u8>) -> anyhow::Result<()> {
    // 远程连接必须0.0.0.0:0
    dbg!(" begin listen");
    let conf = Conf::get();
    let soc = SOC.get().unwrap();

    loop {
        let mut buf = vec![0u8; conf.size];
        let (n, address) = soc.recv_from(&mut buf).await?;
        let cmd = buf[0];
        let cmd = SwapCmd::int2enum(cmd);


        let from_server = cmd.from_server();
        if from_server {
            match cmd {
                // open is server  init do not feed back,only send to peer
                SwapCmd::Open => {
                    // open is server init
                    let swap = Swap::new(&buf, address, n);
                    let peer_address = swap.id;
                    // let peer: SocketAddr = peer_address.parse()?;
                    // 这种歌地方发送给peer!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                    let pac=SwapCmd::Hello.hello();
                    soc.send_to(&pac.to_bytes(), peer_address).await?;
                    continue;
                }
                _ => {
                    // ask, save servererror
                    cmd.save_cache(&buf.to_vec(), n);
                    continue;
                }
            }
        }
        // below is from peer

        let pac=Packet::new_from_rec_bytes(n,&buf.to_vec());
        match cmd {
            SwapCmd::P2P => {
                RecCache::add_pac(address,pac);
            }
            // peer tell me he open the door
            SwapCmd::Hello => {
                dbg!(format!("receive hello from {}",address));
                continue;
            }
            // peer tell me he got all and i del send cache
            SwapCmd::Finish => {
                let sess=pac.session;
                clear_send_cache(address,sess);
            }
            // peer tell me he  did not  get all and i resend
            SwapCmd::Resend=>{
                let send_pac=get_send_cache(address,pac.session,pac.order)?;
                soc.send_to(&send_pac.to_bytes(), address).await?;
            }
            _=>{}

        }
    }
}


pub async fn test() -> anyhow::Result<()> {
    let uuid = "b997dbac-e919-4e44-a8b5-9f7017381e30";
    let remote = "39.96.40.177:4222";
    let mut msg = vec![];
    msg.push(1);
    msg.push(2);
    for _ in 0..1099 {
        for u in 0..10 {
            msg.push(u);
        }
    }

    Ok(())
}