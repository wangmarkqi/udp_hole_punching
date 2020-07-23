use async_std::net::UdpSocket;
pub mod action;
pub async fn run(host:&str)->anyhow::Result<()> {
    let socket = UdpSocket::bind(host).await?;
    let mut buf = vec![0u8; 1024];
    loop {
        let (n, peer) = socket.recv_from(&mut buf).await?;
        let ip=peer.ip();
        let port=peer.port();
        let info=	String::from_utf8_lossy(&buf[..n]);
        dbg!((ip,port,info));
        socket.send_to(&buf[..n], &peer).await?;
    }
}