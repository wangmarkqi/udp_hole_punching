use async_std::net::UdpSocket;
use punching_server::serv;
pub async fn _listen(host:&str)->anyhow::Result<()> {
    const THE_MERCHANT_OF_VENICE: &[u8] = b"
    If you prick us, do we not bleed?
    If you tickle us, do we not laugh?
    If you poison us, do we not die?
    And if you wrong us, shall we not revenge?
";

    let socket = UdpSocket::bind("127.0.0.1:0").await?;

    loop{

        let m= socket.send_to(THE_MERCHANT_OF_VENICE, host).await?;
        dbg!(m);
           let (n, me) = socket.recv_from(&mut buf).await?;
        if n == 0 {
            continue;
        }
        let data = String::from_utf8_lossy(&buf[0..n]);
        let mut income: Packet = serde_json::from_str(&data)?;
        dbg!(&data);
    }
    Ok(())
}
