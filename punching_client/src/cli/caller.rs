use async_std::net::{UdpSocket, SocketAddr};
use super::define::*;
use crate::cli::rec_p2p::rec_single_pac;

pub async fn connect(server: &str, uuid: &str) -> anyhow::Result<SocketAddr> {
    let soc = UdpSocket::bind("0.0.0.0:0").await?;
    CONN.set(soc).unwrap();

    let conn = CONN.get().unwrap();
    // 给服务器的走原始接口
    let ask_open = Packet::caller_open_default(uuid);
    conn.send_to(&ask_open.pack(), server).await?;
// 所有的收都是封装过得
    let ask = rec_single_pac(Who::Caller).await?;
    if ask.success && ask.cmd==CMD::Open {
        // 服务器make match发过来的
        return Ok(ask.address);
    }
    Err(anyhow!(ask.err))
}

// send rec caller call共用
pub async fn send(msg: &Vec<u8>, address: SocketAddr) -> anyhow::Result<u16> {
    let  pac = Packet::p2p_default(address);
    let session = pac.send_pac(Who::Caller, msg).await?;
    Ok(session)
}

async fn _rec(session:u16) -> anyhow::Result<(u16, Vec<u8>)> {
    let rec = rec_single_pac(Who::Caller).await?;
    if rec.body_len == 0 && rec.session==session{
        return Ok((rec.session, vec![]));
    }
    if rec.session==session && rec.is_done(Who::Caller) {
        // 拿到成功后删除了数据
        let msg = rec.assembly(Who::Caller)?;
        return Ok((rec.session, msg));
    }
    Err(anyhow!("the pac receive has not been done"))
}

// 传入毫秒的单位（千分之一秒）
pub async fn rec(session:u16,elapse: u128) -> anyhow::Result<(u16, Vec<u8>)> {
    let now = std::time::Instant::now();
    loop {
        let res=_rec(session).await;
        if let Ok(e)=res{
            return Ok(e);
        }
        let differ = now.elapsed().as_millis();
        if differ > elapse { break; };
    }
    Err(anyhow!("the pac receive has not been done"))
}

pub async fn test() -> anyhow::Result<()> {
    let uuid = "b997dbac-e919-4e44-a8b5-9f7017381e30";
    let remote = "192.168.40.177:4222";
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
    // dbg!(&msg.len());
    let session = send(&msg, address).await?;
    dbg!(session);
    let res = rec(session,1000).await?;
    let back=res.1;
   // let back= std::str::from_utf8(&res.1).unwrap();
    dbg!(back.len());
    Ok(())
}