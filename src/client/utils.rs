use once_cell::sync::OnceCell;
use once_cell::sync::Lazy;
use super::conf::Conf;
use async_std::net::UdpSocket;
use std::net::SocketAddr;
use std::sync::Mutex;
pub static SOC: OnceCell<UdpSocket> = OnceCell::new();
pub static PeerAddress: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new("".to_string())
});
pub fn update_peer_address(address:String){
    let mut store=PeerAddress.lock().unwrap();
    *store=address;
}


pub async fn rec_with_timeout() -> (usize, SocketAddr, Vec<u8>) {
    let conf = Conf::get();
    let mut buf = vec![0u8; conf.size];

    let res = async_std::io::timeout(std::time::Duration::from_micros(conf.single_rec_timeout as u64), async {
        let soc = SOC.get().unwrap();
        soc.recv_from(&mut buf).await
    }).await;
    if let Err(e) = res {
        let default: SocketAddr = "127.0.0.1:0000".parse().unwrap();
        return (0, default, buf);
    }
    let (n, address) = res.unwrap();
    (n, address, buf)
}
