use super::conf::Conf;
use std::net::SocketAddr;
use super::packet::Packet;
use once_cell::sync::OnceCell;
use super::sled_db::DB;
use crate::server::swap_cmd::SwapCmd;
use crate::server::swap_protocal::Swap;
use super::peer_address::update_peer_address;
use async_std::net::UdpSocket;
use crate::client::cache_task::DoSend;
use crate::client::cache_send::SendDelOne;
use crate::client::cache_rec::SingleSave;
use async_std::task::block_on;

pub static SOC: OnceCell<UdpSocket> = OnceCell::new();

pub fn init_udp() {
    block_on(async {
        DB::init();
        DB::clear_db();
        let soc = UdpSocket::bind("0.0.0.0:0").await.expect("udp can not open");
        SOC.set(soc).expect("udp can not set");
    });
}


pub async fn rec_with_timeout() -> anyhow::Result<(usize, SocketAddr, Vec<u8>)> {
    let conf = Conf::get();
    let mut buf = vec![0u8; conf.size];
    let (n, address) = async_std::io::timeout(std::time::Duration::from_micros(conf.single_rec_timeout as u64), async {
        let soc = SOC.get().unwrap();
        soc.recv_from(&mut buf).await
    }).await?;
    Ok((n, address, buf))
}


pub async fn process_from_server(n: usize, address: SocketAddr, buf: Vec<u8>) -> anyhow::Result<()> {
    let cmd = SwapCmd::int2enum(buf[0]);
    let swap = Swap::new(&buf, address, n);
    match cmd {
        SwapCmd::Open => {
            dbg!("rec open and send hello to another peer");
            let hello = Packet::hello();
            let soc = SOC.get().unwrap();
            soc.send_to(&hello, &swap.id).await?;
        }
        SwapCmd::Ask => {
            dbg!("rec ask and update peer address");
            let peer_address = swap.id;
            update_peer_address(peer_address);
        }
        _ => {}
    }
    Ok(())
}

pub async fn process_from_peer(n: usize, address: SocketAddr, buf: Vec<u8>) -> anyhow::Result<()> {
    let cmd = SwapCmd::int2enum(buf[0]);
    let pac = Packet::new_from_rec_bytes(n, &buf);
    match cmd {
        SwapCmd::P2P => {
            DB::Rec.single_save(address, &pac);
            let got = Packet::got(&pac);
            let soc = SOC.get().unwrap();
            soc.send_to(&got, address).await?;
        }
        // callee receive ask from caller,and answer over or lacks
        SwapCmd::Got => {
            DB::Send.send_del_one(address, &pac);
        }

        _ => {}
    }
    Ok(())
}

pub async fn process_send_task() {
    match DB::Task.do_send().await {
        Ok(()) => {}
        Err(e) => { dbg!(e); }
    }
}





