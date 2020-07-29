use async_std::net::{UdpSocket,SocketAddr};
use punching_server::{Packet, CMD};
use crate::cli::p2s::P2S;
use once_cell::sync::OnceCell;
use crate::cli::p2p::{P2P, Who};
use super::rec_pac::rec_pac;

pub static SOC: OnceCell<UdpSocket> = OnceCell::new();

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
pub async fn listen(host: &str, handler: &dyn Fn(&Vec<u8>) -> Vec<u8>) -> anyhow::Result<()> {
    // 远程连接必须0.0.0.0:0
    dbg!("callee listen");
    let soc = UdpSocket::bind("0.0.0.0:0").await?;
    SOC.set(soc).unwrap();


    let socket = SOC.get().unwrap();
    // 获取注册的uuid
    let mut registry = Packet::default();
    registry.callee_report()?;
    let registry_vec = registry.pack();
    loop {
        socket.send_to(&registry_vec, host).await?;

        let res = rec_pac(Who::Callee).await;
        if let Err(_) = res {
            continue;
        }
        let mut income = res.unwrap();

        match income.cmd {
            CMD::Open => {
                let caller = &income.caller_address;
                // 先caller开门，打洞关键
                let mut pac = Packet::default();
                pac.cmd = CMD::P2P;
                pac.send_pac(Who::Callee, *caller).await?;
            }
            CMD::P2P => {
                dbg!(&income);
                let msg = &income.msg;
                // 把这个做成api

                let back = handler(msg);

                income.msg = back;
                let addr = income.caller_address;
                income.send_pac(Who::Callee, addr).await?;
            }
            _ => {
                dbg!("no cmd match");
            }
        }
    }
}



