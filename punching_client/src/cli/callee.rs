use async_std::net::{UdpSocket};
use super::define::*;
use crate::cli::rec_p2p::rec_single_pac;


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
    let registry = Packet::callee_save_default()?;
    loop {
        // 给服务器的走原始接口
        socket.send_to(&registry.pack(), host).await?;

        let res = rec_single_pac(Who::Callee).await;
        if let Err(_) = res {
            continue;
        }
        let  income = res.unwrap();

        match income.cmd {
            CMD::Open => {
                // 先caller开门，打洞关键,address从服务器make match过来的
                let  pac = Packet::p2p_default(income.address);
                pac.send_pac(Who::Callee,&vec![]).await?;
            }
            CMD::P2P => {
                dbg!("callee rec p2p");
                // 把这个做成api
                if income.body_len as i32 > 0  && income.is_done(Who::Callee) {
                    // 拿到成功后删除了数据
                    let msg = income.assembly(Who::Callee)?;
                    let back = handler(&msg);
                    dbg!(&income);
                    income.send_pac(Who::Callee,&back).await?;
                }
            }
            _ => {
                dbg!("no cmd match");
            }
        }
    }
}



