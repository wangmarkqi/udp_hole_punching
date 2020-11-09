use super::packet::Packet;
use super::packets::Packets;
use super::store::Store;
use crate::server::swap_cmd::SwapCmd;
use super::conf::Conf;
use std::net::SocketAddr;
pub trait Command{
    fn req(addr:SocketAddr,body:Vec<u8>) ->Self;
    fn peer_err(body:Vec<u8>,sess:u8) ->Self;
    fn resp(body:Vec<u8>,sess:u8) ->Self;
    fn got(sess:u8) ->Self;
    fn resend(sess:u8,order:u32) ->Self;

}
impl Command for Vec<Packet>{
    fn req(addr:SocketAddr,body:Vec<u8>)->Self{
        let sess=Store::Send.new_sess(addr);
        let mut pac=Packet::empty();
        pac.session=sess;
        pac.cmd=SwapCmd::Req.enum2int();
        let conf=Conf::get();
        pac.new_pacs_from_send_bytes(conf.size,&body)
    }
    fn peer_err(body:Vec<u8>,sess:u8)->Self{
        let mut pac=Packet::empty();
        pac.session=sess;
        pac.cmd=SwapCmd::PeerErr.enum2int();
        let conf=Conf::get();
        pac.new_pacs_from_send_bytes(conf.size,&body)
    }
    fn resp(body:Vec<u8>,sess:u8)->Self{
        let mut pac=Packet::empty();
        pac.session=sess;
        pac.cmd=SwapCmd::Resp.enum2int();
        let conf=Conf::get();
        pac.new_pacs_from_send_bytes(conf.size,&body)
    }
    fn got(sess:u8)->Self{
        let mut pac=Packet::empty();
        pac.session=sess;
        pac.cmd=SwapCmd::Got.enum2int();
        let conf=Conf::get();
        pac.new_pacs_from_send_bytes(conf.size,&vec![])
    }

    fn resend(sess:u8,order:u32) ->Self{
        let body=order.to_be_bytes().to_vec();
        let mut pac=Packet::empty();
        pac.session=sess;
        pac.cmd=SwapCmd::Resend.enum2int();
        let conf=Conf::get();
        pac.new_pacs_from_send_bytes(conf.size,&body)
    }

}
