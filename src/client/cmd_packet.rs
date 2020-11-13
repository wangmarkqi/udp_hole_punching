use super::packet::Packet;
use super::packets::Packets;
use crate::server::swap_cmd::SwapCmd;
use super::conf::Conf;
use super::send_cache::*;
use std::net::SocketAddr;

pub trait Cmd2Pac {
    fn hello(&self) -> Packet;
    fn finish(&self,sess: u8) ->Packet;
    fn resend(&self,sess: u8, order: u32) ->Packet;
}

impl Cmd2Pac for SwapCmd {
    fn hello(&self) -> Packet{
        let mut pac = Packet::empty();
        pac.cmd = SwapCmd::Hello.enum2int();
        pac
    }
    fn finish(&self,sess: u8) -> Packet{
        let mut pac = Packet::empty();
        pac.cmd = SwapCmd::Finish.enum2int();
        pac.session=sess;
        pac
    }

    fn resend(&self,sess: u8, order: u32) -> Packet{
        let mut pac = Packet::empty();
        pac.cmd = SwapCmd::Resend.enum2int();
        pac.session=sess;
        pac.order=order;
        pac
    }
}
