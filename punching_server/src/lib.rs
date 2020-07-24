use async_std::net::UdpSocket;

pub mod action;

use crate::action::cmds::*;
use crate::action::packet::Packet;

#[macro_use]
extern crate lazy_static;

pub const SAVE: &str = "0";
// caller ask peer to open,query if open,ask peer to close
pub const OPEN: &str = "1";
pub const QUERY: &str = "2";
pub const CLOSE: &str = "3";
pub const CONFIRM: &str = "4";


pub const SUCCESS: &'static [u8] = "success".as_bytes();
pub const FAIL: &'static [u8] = "fail".as_bytes();


pub async fn run(host: &str) -> anyhow::Result<()> {
    let socket = UdpSocket::bind(host).await?;
    let mut buf = vec![0u8; 512 * 8];
    loop {
        let (n, me) = socket.recv_from(&mut buf).await?;
        if n == 0 {
            continue;
        }
        let data = String::from_utf8_lossy(&buf[0..n]);
        let mut income: Packet = serde_json::from_str(&data)?;


        let mut default_packet = Packet::default();
        let default_res = Packet::default().pack();
        match &income.cmd.as_str() {
            &SAVE => {
                // 先把以前没有的ip登记了
                income.callee_address = me;
                &income.callee_registry();
                // 我来包地址的，没想到你让我open，检查这个登记的id有没有人叫他
                let p = &income.if_need_to_open();
                // 这种情况就是没有发送需求
                if p.caller_address == Packet::localhost() {
                    socket.send_to(&default_res, &me).await?;
                } else {
                    // 有人请求，confirm回应
                    let res = p.pack();
                    socket.send_to(&res, &me).await?;
                }
            }
            &CONFIRM => {
                income.callee_address = me;
                &income.confirm_conn_open();
                socket.send_to(&default_res, &me).await?;
            }
            &OPEN => {
                income.caller_address = me;
                let p=&income.caller_ask_open();
                socket.send_to(&p.pack(), &me).await?;
            }
            &QUERY => {
                let p = &income.caller_query_conn();

                if p.caller_address == Packet::localhost() {
                    default_packet.success = false;
                    default_packet.err = "not in ready list".to_string();
                    let res = default_packet.pack();
                    socket.send_to(&res, &me).await?;
                } else {
                    let res = p.pack();
                    socket.send_to(&res, &me).await?;
                }
            }
            _ => {
                default_packet.err = "cmd not match".to_string();
                default_packet.success = false;
                let res = default_packet.pack();
                socket.send_to(&res, &me).await?;
            }
        }
    }
}