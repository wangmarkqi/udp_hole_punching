use crate::protocal::packet::Packet;
use punching_server::SwapCmd;
pub trait Command{

    fn got(sess:u8) ->Self;
}

impl Check for Packet{
    fn ask(sess:u8)->Packets{

    }
}
