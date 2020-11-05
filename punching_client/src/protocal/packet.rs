use punching_server::SwapCmd;
use std::mem::size_of;
use super::tools::segment_bytes;
use super::packets::Packets;

#[derive(PartialEq, Debug, Clone)]
pub struct Packet {
    pub cmd: u8,
    pub session: u8,
    pub over: u8,
    pub order: u32,
    pub body: Vec<u8>,
}

// protocal cmd session over oder body
impl Packet {
    pub fn empty() -> Self {
        Packet {
            cmd: SwapCmd::None.enum2int(),
            session: 0,
            over: 0,
            order: 0,
            body: vec![],
        }
    }
    pub fn header_len(&self) -> usize {
        3 * size_of::<u8>() + size_of::<u32>()
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v = vec![];
        v.push(self.cmd);
        v.push(self.session);
        v.push(self.over);
        let order = self.order.to_be_bytes();
        for i in order.iter() {
            v.push(*i);
        }
        for i in self.body.iter() {
            v.push(*i)
        }
        v
    }
    pub fn new_pacs_from_send_bytes(&self, conf_size: usize, body: &Vec<u8>) -> Packets {
        let data = segment_bytes(body, conf_size, self.header_len());
        let mut res = vec![];
        let total = &data.len();
        for (i, v) in data.iter().enumerate() {
            let ov = if i == total - 1 { 1 } else { 0 };
            let p = Packet {
                cmd: self.cmd,
                session: self.session,
                order: i as u32,
                over: ov as u8,
                body: v.to_owned(),
            };
            res.push(p);
        }
        Packets {
            session:self.session,
            pacs: res,
        }
    }
    pub fn new_from_rec_bytes(total: usize, buf: &Vec<u8>) -> Self {
        let ord_b :[u8;4]= [buf[3],buf[4],buf[5],buf[6]];
        let ord_u = u32::from_be_bytes(ord_b);
        Packet {
            cmd: buf[0],
            session: buf[1],
            over: buf[2],
            order: ord_u,
            body: buf[7..total - 1].to_vec(),
        }
    }
    pub pac_feed
}

#[test]
fn test_pac() {}