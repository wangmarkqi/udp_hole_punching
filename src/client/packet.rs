use crate::server::swap_cmd::SwapCmd;
use std::mem::size_of;
use super::conf::Conf;
use rand::Rng;

#[derive(PartialEq, Debug, Clone)]
pub struct Packet {
    pub cmd: u8,
    pub sess: u32,
    pub max: u32,
    pub order: u32,
    pub body: Vec<u8>,
}

// cli cmd session over oder body
impl Packet {
    pub fn empty() -> Self {
        Packet {
            cmd: SwapCmd::None.enum2int(),
            sess: 0,
            max: 0,
            order: 0,
            body: vec![],
        }
    }
    pub fn random_sess() -> u32 {
        let mut rng = rand::thread_rng();
        let n: u32 = rng.gen();
        n
    }
    pub fn hello() -> Vec<u8> {
        let mut pac = Packet::empty();
        pac.cmd = SwapCmd::Hello.enum2int();
        pac.to_bytes()
    }
    pub fn got(p: &Packet) -> Vec<u8> {
        let mut pac = Packet::empty();
        pac.cmd = SwapCmd::Got.enum2int();
        pac.sess = p.sess;
        pac.max = p.max;
        pac.order = p.order;
        pac.to_bytes()
    }

    pub fn header_len() -> usize {
        1 * size_of::<u8>() + 3 * size_of::<u32>()
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v = vec![];
        v.push(self.cmd);
        let sess = self.sess.to_be_bytes();
        for i in sess.iter() {
            v.push(*i);
        }
        let max = self.max.to_be_bytes();
        for i in max.iter() {
            v.push(*i);
        }
        let order = self.order.to_be_bytes();
        for i in order.iter() {
            v.push(*i);
        }
        for i in self.body.iter() {
            v.push(*i)
        }
        v
    }
    pub fn new_pacs_from_send_bytes(body: &Vec<u8>) -> (u32,Vec<Packet>) {
        let ses=Packet::random_sess();
        let command = SwapCmd::P2P;
        let conf_size = Conf::get().size;
        let data = segment_bytes(body, conf_size, Packet::header_len());
        let mut res = vec![];
        let total = &data.len();
        for (i, v) in data.iter().enumerate() {
            let p = Packet {
                cmd: command.enum2int(),
                sess: ses,
                max: *total as u32 - 1,
                order: i as u32,
                body: v.to_owned(),
            };
            res.push(p);
        }
        (ses,res)
    }
    pub fn new_from_save_db( buf: &Vec<u8>) -> Self {
        let total=buf.len();
        Packet::new_from_rec_bytes(total,buf)
    }
    pub fn new_from_rec_bytes(total: usize, buf: &Vec<u8>) -> Self {
        let sess_b: [u8; 4] = [buf[1], buf[2], buf[3], buf[4]];
        let sess_u = u32::from_be_bytes(sess_b);
        let max_b: [u8; 4] = [buf[5], buf[6], buf[7], buf[8]];
        let max_u = u32::from_be_bytes(max_b);
        let ord_b: [u8; 4] = [buf[9], buf[10], buf[11], buf[12]];
        let ord_u = u32::from_be_bytes(ord_b);
        Packet {
            cmd: buf[0],
            sess: sess_u,
            max: max_u,
            order: ord_u,
            body: buf[13..total].to_vec(),
        }
    }
}

fn segment_bytes(body: &Vec<u8>, conf_size: usize, header_len: usize) -> Vec<Vec<u8>> {

    // if msg is empty
    let task_total_len = body.len();
    if task_total_len == 0 {
        let res = vec![];
        return res;
    }

    // calculate max
    let real_capacity_len = conf_size - header_len;
    let remainder = task_total_len % real_capacity_len;
    let times = task_total_len / real_capacity_len;

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
