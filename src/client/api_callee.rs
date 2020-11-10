use async_std::net::{UdpSocket, SocketAddr};
use std::net::SocketAddr;
use once_cell::sync::OnceCell;
use crate::server::swap_cmd::SwapCmd;
use std::time::{Duration, Instant};
use super::conf::Conf;
use super::packet::Packet;
use super::send_cache::save_send_cache;
use super::rec_cache::RecCache;
use super::packets::Packets;
use super::api_server::SOC;

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
        let cmd=buf[0];
        let cmd=SwapCmd::int2enum(cmd);


        match income.cmd {
            CMD::Open => {
                // 先caller开门，打洞关键,address从服务器make match过来的
                let pac = Packet::p2p_default(income.address);
                pac.send_pac(Who::Callee, &vec![]).await?;
            }
            CMD::P2P => {
                dbg!("callee rec p2p");
                // 把这个做成api
                if income.body_len as i32 > 0 && income.is_done(Who::Callee) {
                    // 拿到成功后删除了数据
                    let msg = income.assembly(Who::Callee)?;
                    let back = handler(&msg);
                    dbg!(&income);
                    income.send_pac(Who::Callee, &back).await?;
                }
            }
            _ => {
                dbg!("no cmd match");
            }
        }
    }
}


pub async fn test() -> anyhow::Result<()> {
    let uuid = "b997dbac-e919-4e44-a8b5-9f7017381e30";
    let remote = "39.96.40.177:4222";
    let address = connect(remote, uuid).await?;
    dbg!(&address);
    let mut msg = vec![];
    msg.push(1);
    msg.push(2);
    for _ in 0..1099 {
        for u in 0..10 {
            msg.push(u);
        }
    }
    // let msg="wolxie了几个中卫你看看".as_bytes().to_vec();
    dbg!(&msg.len());
    let session = send(&msg, address).await?;
    dbg!(session);
    let res = rec(session, 1000).await?;
    let back = res.1;
    // let back= std::str::from_utf8(&res.1).unwrap();
    dbg!(back.len());
    Ok(())
}