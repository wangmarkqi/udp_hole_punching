use std::collections::HashMap;
use std::net::{SocketAddr};
use async_std::net::UdpSocket;
use super::swap_cmd::SwapCmd;
use super::swap_protocal::Swap;

const SERVER_SIZE: usize = 128;

pub async fn make_match(host: &str) -> anyhow::Result<()> {
    dbg!("swap server=====",host);
    let socket = UdpSocket::bind(host).await?;
    let mut store: HashMap<String, SocketAddr> = HashMap::new();

    loop {
        let mut buf = vec![0u8; SERVER_SIZE];
        let (n, me) = socket.recv_from(&mut buf).await?;
        // let me = address_me.to_string();
        if n == 0 {
            let resp = Swap::pack_err("no data");
            socket.send_to(&resp, me).await?;
            continue;
        }
        if n > SERVER_SIZE {
            let resp = Swap::pack_err("beyond size");
            socket.send_to(&resp, me).await?;
            continue;
        }
        let swap = Swap::new(&buf, me, n);
        dbg!(&swap);
        let id = swap.id.clone();
        if &id == "" {
            let resp = Swap::pack_err("no id");
            socket.send_to(&resp, me).await?;
            continue;
        }
        let cmd = SwapCmd::int2enum(swap.cmd);



        match cmd {
            // callee sent to registry
            SwapCmd::Save => {
                store.insert(id.clone(), swap.address);
                dbg!("rec save and not send  to saver");
            }
            SwapCmd::Ask => {
                if store.contains_key(&id) {
                    let peer = store[&id];
                    // 给peer，把自己的add发过去,换成open指令
                    dbg!("send open to peer");
                    let pack_peer = swap.pack_open();
                    // 发送两次
                    socket.send_to(&pack_peer, peer).await?;
                    socket.send_to(&pack_peer, peer).await?;
                    // 给自己，发peer address
                    // 给peer，把自己的add发过去,换成open指令
                    dbg!("send address to asker");
                    let peer_address = peer.to_string();
                    let resp_me = swap.pack(&peer_address.as_bytes().to_vec());
                    socket.send_to(&resp_me, swap.address).await?;

                } else {
                    dbg!("send err to reqer");
                    let resp_me = swap.pack("no registry".as_bytes());
                    socket.send_to(&resp_me, swap.address).await?;
                };
            }
            _ => {}
        }
    }
}

