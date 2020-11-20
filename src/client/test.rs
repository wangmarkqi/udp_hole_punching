use super::conf::Conf;
use super::api::*;
use async_std::task::block_on;
use super::listen::listen;

pub async fn test_callee_listen() -> anyhow::Result<()> {
    let mut conf = Conf::default();
    conf.swap_server = "39.96.40.177:4222".to_string();
    conf.id = "wq".to_string();
    conf.set();
    init_udp().await?;
    std::thread::spawn(|| {
        block_on(listen());
    });

    loop {
        let (addr, v) = rec_from();
        if v.len() > 0 {
            let s = String::from_utf8_lossy(&v);
            dbg!("callee rec res");
            dbg!(s);
            let back = "i got you".as_bytes().to_vec();
            send(&back, addr).await?;
        }
    };
    Ok(())
}

pub async fn test_caller_api() -> anyhow::Result<()> {
    let mut conf = Conf::default();
    conf.swap_server = "39.96.40.177:4222".to_string();
    conf.set();
    init_udp().await?;

    let addr = get_peer_address("wq").await?;
    dbg!(addr);
    std::thread::spawn(|| {
        block_on(listen());
    });
    dbg!("begin");


    loop {
        let msg = "i send you".as_bytes().to_vec();
        send(&msg, addr).await?;

        let (addr, v) = rec_from();
        if v.len() > 0 {
            let s = String::from_utf8_lossy(&v);
            dbg!("caller  rec res");
            dbg!(s);
        }
    }
    Ok(())
}