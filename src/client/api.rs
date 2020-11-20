use async_std::net::UdpSocket;
use std::net::SocketAddr;
use crate::server::swap_cmd::SwapCmd;
use super::cache_rec::Msg;
use super::conf::Conf;
use super::packet::Packet;
use super::listen::SOC;
use super::cache::Cache;
use crate::client::cache_send::GenSession;
use std::time::Duration;
use super::listen::PeerAddress;
// ask cmd get feed back peer address, and server will send open to peer
pub async fn init_udp() -> anyhow::Result<()> {
    let soc = UdpSocket::bind("0.0.0.0:0").await?;
    SOC.set(soc).unwrap();
    Ok(())
}

pub async fn get_peer_address(peer_id: &str) -> anyhow::Result<SocketAddr> {
    let conf = Conf::get();

    let send_data = SwapCmd::ask(peer_id);

    let soc = SOC.get().unwrap();
    soc.send_to(&send_data, &conf.swap_server).await?;
    async_std::task::sleep(Duration::from_micros(conf.hello_elapse as u64)).await;

    let mut  peer_cache=PeerAddress.lock().unwrap();
    let res=&*peer_cache.clone();
    dbg!(res);
    *peer_cache="".to_string();
    // let addr_str = String::from_utf8_lossy(&res);
    let addr: SocketAddr = res.parse()?;

    Ok(addr)
}

pub async fn send(msg: &Vec<u8>, address: SocketAddr) -> anyhow::Result<()> {

    let cs = Cache::Send;
    let sess = cs.gen_session(address);
    let pacs = Packet::new_pacs_from_send_bytes(msg, sess);
    let k = cs.key(address, sess);
    cs.save_all(&k, pacs.to_owned());

    let soc = SOC.get().unwrap();
    for pac in pacs.iter() {
        let data = pac.to_bytes();
        soc.send_to(&data, address).await?;
    }
    Ok(())
}

pub fn rec_from() -> (SocketAddr, Vec<u8>) {
    let mut msg = Msg.lock().unwrap();
    let default: SocketAddr = "0.0.0.1:0000".parse().expect("Cannot handle the socket port");
    match msg.pop_front() {
        None => (default, vec![]),
        Some(res) => res,
    }
}

