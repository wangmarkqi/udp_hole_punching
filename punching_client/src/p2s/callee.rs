use async_std::net::{SocketAddr,UdpSocket};
use async_std::io;
use punching_server::{CALLEE_SLEEP, PAC_SIZE, Packet, CMD};
use crate::p2s::p2s::P2S;
use std::time::Duration;
use std::{thread, time};
use super::caller::CONN;
use once_cell::sync::OnceCell;

pub static SOC: OnceCell<UdpSocket> = OnceCell::new();

async fn init_callee() -> anyhow::Result<()> {
    // 远程连接必须0.0.0.0:0
    let mut soc = UdpSocket::bind("0.0.0.0:0").await?;
    SOC.set(soc).unwrap();
    Ok(())
}

pub async fn _listen(host: &str) -> anyhow::Result<()> {
    init_callee().await?;
    let socket = SOC.get().unwrap();

    // 获取注册的uuid
    let mut registry = Packet::default();
    registry.callee_report()?;
    let registry_vec = registry.pack();
    loop {
        socket.send_to(&registry_vec, host).await?;

        let res = rec_pac("callee").await;
        if let Err(e) = res {
            continue;
        }
        let res2 = res.unwrap();
        let mut income=res2.0;
        let address=res2.1;
        match income.cmd {
            CMD::Open => {
                let caller = &income.caller_address;
                // 先caller开门，打洞关键
                let mut pac = Packet::default();
                pac.cmd = CMD::P2P;
                pac.msg = "open from callee".to_string();
                socket.send_to(&pac.pack(), caller).await?;
            }
            CMD::P2P => {
                dbg!("from caller",&income);
                income.msg = "p2p from callee".to_string();
                let msg=&income.msg;
                // 把这个做成api
                
                let back=handler(&msg);

                income.msg=back;
                let echo = &income.pack();
                socket.send_to(echo,address).await?;
            }
            _ => {
                dbg!("no cmd match");
            }
        }
    }
}


pub fn handler(msg:&str)->String{
    msg
}
pub async fn rec_pac(which: &str) -> anyhow::Result<(Packet,SocketAddr)> {
    let socket = {
        if which == "caller" {
            CONN.get().unwrap()
        } else {
            SOC.get().unwrap()
        }
    };

    let mut buf = vec![0u8; PAC_SIZE];


    let (n, peer) = io::timeout(Duration::from_secs(4), async {
        socket.recv_from(&mut buf).await
    }).await?;

    if n == 0 {
        return Err(anyhow!("receive no data from server"));
    }
    if n > PAC_SIZE {
        return Err(anyhow!("max pack size:{},actual rec{}",PAC_SIZE,n));
    }
    let data = String::from_utf8_lossy(&buf[0..n]);
    let mut income: Packet = serde_json::from_str(&data)?;

    Ok((income,peer))
}
