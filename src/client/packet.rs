use crate::server::swap_cmd::SwapCmd;
use std::mem::size_of;
use super::packets::Packets;
use super::conf::Conf;

#[derive(PartialEq, Debug, Clone)]
pub struct Packet {
    pub cmd: u8,
    // 1 is over ,0 is not over
    pub over: u8,
    // start from 0
    pub order: u32,
    pub body: Vec<u8>,
}

// cli cmd session over oder body
impl Packet {
    pub fn empty() -> Self {
        Packet {
            cmd: SwapCmd::None.enum2int(),
            over: 0,
            order: 0,
            body: vec![],
        }
    }
    pub fn header_len() -> usize {
        2 * size_of::<u8>() + size_of::<u32>()
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v = vec![];
        v.push(self.cmd);
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

    pub fn new_pacs_from_send_bytes( body: &Vec<u8>) -> Vec<Packet> {
        let command=SwapCmd::P2P;
        let conf_size=Conf::get().size;
        let data = segment_bytes(body, conf_size, Packet::header_len());
        let mut res = vec![];
        let total = &data.len();
        for (i, v) in data.iter().enumerate() {
            let ov = if i == total - 1 { 1 } else { 0 };
            let p = Packet {
                cmd:command.enum2int() ,
                order: i as u32,
                over: ov as u8,
                body: v.to_owned(),
            };
            res.push(p);
        }
        res
    }
    pub fn new_from_rec_bytes(total: usize, buf: &Vec<u8>) -> Self {
        let ord_b: [u8; 4] = [buf[2],buf[3], buf[4], buf[5]];
        let ord_u = u32::from_be_bytes(ord_b);
        Packet {
            cmd: buf[0],
            over: buf[1],
            order: ord_u,
            body: buf[6..total].to_vec(),
        }
    }
}

fn segment_bytes(body: &Vec<u8>, conf_size: usize,header_len:usize) -> Vec<Vec<u8>> {

    // if msg is empty
    let task_total_len = body.len();
    if task_total_len == 0 {
        let  res = vec![];
        return res;
    }

    // calculate max
    let real_capacity_len = conf_size - header_len;
    let remainder = task_total_len % real_capacity_len;
    let times = task_total_len / real_capacity_len;
    // 改max属性,max从0开始
    let max = if remainder != 0 { times } else { times - 1 };

    let mut queue = vec![];
    let mut task_done_len = 0;
    let mut order = 0;

    while task_done_len < task_total_len {
        let task_left_len = task_total_len - task_done_len;
        let this_done_len = {
            if task_left_len >= real_capacity_len {
                real_capacity_len as usize
            } else {
                task_left_len as usize
            }
        };

        let mut this_body = vec![0; this_done_len];
        for i in task_done_len..task_done_len + this_done_len {
            this_body[i - task_done_len] = body[i];
        }

        task_done_len = task_done_len + this_done_len;

        order = order + 1;

        queue.push(this_body);
    }
    queue
}
