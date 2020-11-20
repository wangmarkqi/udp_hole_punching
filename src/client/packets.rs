use super::packet::Packet;
use super::conf::Conf;

pub trait Packets {
    fn sort(&mut self);
    fn assembly(&mut self) -> Vec<u8>;
    fn lacks(&mut self) -> Vec<u32>;
}

impl Packets for Vec<Packet> {
    // alway call sort when update packets
    fn sort(&mut self) {
        self.sort_by(|a, b| a.order.partial_cmp(&b.order).unwrap());
    }

    fn lacks(&mut self) -> Vec<u32> {
        let conf = Conf::get();
        let add_more = conf.ask_resend_more as u32;
        let mut res = vec![];
        let total = self.len();
        if total == 0 {
            res.push(0);
            return res;
        }
        self.sort();

        let end = &self[total - 1];
        let max_order = end.order;
        if end.over != 1 {
            for i in max_order + 1..max_order + 1 + add_more {
                res.push(i);
            }
        }
        let orders: Vec<u32> = self.iter().map(|e| e.order).collect();

        for i in 0..total as u32 {
            if !orders.contains(&i) {
                res.push(i);
            }
        }
        res
    }
    fn assembly(&mut self) -> Vec<u8> {
        let mut res = vec![];
        if self.len() == 0 {
            return res;
        };
        self.sort();
        for i in self.iter() {
            for ii in i.body.iter() {
                res.push(*ii);
            }
        }
        res
    }
}

#[test]
fn test_new_send() {
    let body = &"abcdefghij".as_bytes().to_vec();
    let res = Packet::new_pacs_from_send_bytes(body, 12);

}