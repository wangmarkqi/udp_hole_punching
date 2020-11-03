use async_std::net::{SocketAddr};
use super::tools::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Packets {
    pub session: [u8; 2],
    pub max: u16,
    pub order: u16,
    pub body: Vec<u8>,
}


impl Packet {
    pub fn rand_new() -> Self {
        let mut rng = rand::thread_rng();
        let n1: u8 = rng.gen();
        let n2: u8 = rng.gen();
        let sess_rand = [n1, n2];
        Packet {
            session: sess_rand,
            order: 0,
            max: 0,
            body: vec![],
        }
    }
    pub fn toBytes(&self) -> Vec<u8> {
        let mut v = vec![];
        v.push(self.session[0]);
        v.push(self.session[1]);
        let order = self.order.to_be_bytes();
        v.push(order[0]);
        v.push(order[1]);
        let max = self.max.to_be_bytes();
        v.push(max[0]);
        v.push(max[1]);
        for i in self.body.iter() {
            v.push(i)
        }
        v
    }
    pub fn segment_bytes(body: &Vec<u8>, conf_size: u16) -> Vec<Self> {
        let mut pac = Packet::rand_new();

        // if msg is empty
        let task_total_len = body.len();
        if task_total_len == 0 {
            let mut res = vec![];
            res.push(pac);
            return res;
        }

        // calculate max
        let header_len = pac.session.len() + + pac.order.len() + pac.max.len() as usize;
        let real_capacity_len = conf_size as usize - header_len;
        let remainder = task_total_len % real_capacity_len;
        let times = task_total_len / real_capacity_len;
        // 改max属性,max从0开始
        let max = if remainder != 0 { times } else { times - 1 };
        pac.max = max;

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

            pac.body = this_body;
            task_done_len = task_done_len + this_done_len;

            pac.order = order;
            order = order + 1;

            queue.push(pac);
        }
        if order != max + 1 {
            panic!("one of max or order is wrong")
        }
        queue
    }


}

