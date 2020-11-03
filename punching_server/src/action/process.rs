use std::collections::HashMap;
use std::net::{SocketAddr};
use async_std::net::UdpSocket;
use super::swap_cmd::SwapCmd;
use super::swap_protocal::Swap;


const SERVER_SIZE: usize = 128;

pub async fn make_match(host: &str) -> anyhow::Result<()> {
    dbg!("server=====",host);
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
            let resp = Swap::pack_err("beyond isze");
            socket.send_to(&resp, me).await?;
            continue;
        }
        let swap = Swap::new(&buf, me, n);
        dbg!(&swap);
        let id=swap.id.clone();
        if &id == "" {
            let resp = Swap::pack_err("no id");
            socket.send_to(&resp, me).await?;
            continue;
        }
        let cmd = SwapCmd::int2enum(swap.cmd);


        let mut resp_me = vec![];

        match cmd {
            // callee sent to registry
            SwapCmd::Save => {

                store.insert(id.clone(), swap.address);
                resp_me = swap.pack("success".as_bytes());
            }
            SwapCmd::Ask => {
                if store.contains_key(&id) {
                    let peer = store[&id].to_string();
                    dbg!(&peer);
                    resp_me = swap.pack(&peer.as_bytes().to_vec());
                } else {
                    let resp = Swap::pack_err("no registry");
                    socket.send_to(&resp, me).await?;
                    continue;
                };
            }
            SwapCmd::Open => {
                if store.contains_key(&id) {
                    let peer = store[&id];
                    let my_addr = swap.address.to_string();
                    let pack_peer = swap.pack(my_addr.as_bytes());
                    // 给peer，把自己的add发过去
                    socket.send_to(&pack_peer, peer).await?;
                    // 给自己，发成功
                    resp_me = swap.pack("success".as_bytes());
                } else {
                    let resp = Swap::pack_err("no registry");
                    socket.send_to(&resp, me).await?;
                    continue;
                };
            }
            _ => {
                resp_me = swap.pack("no match cmd".as_bytes());
            }
        }
        socket.send_to(&resp_me, swap.address).await?;
    }
}

