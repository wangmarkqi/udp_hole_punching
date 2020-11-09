use async_std::net::UdpSocket;
use once_cell::sync::OnceCell;
pub static SOC: OnceCell<UdpSocket> = OnceCell::new();

pub async fn listen(host: &str, handler: &dyn Fn(&Vec<u8>) -> Vec<u8>) -> anyhow::Result<()> {
    // 远程连接必须0.0.0.0:0
    dbg!("callee listen");
    let soc = UdpSocket::bind("0.0.0.0:0").await?;
    SOC.set(soc).unwrap();
    Ok(())
}