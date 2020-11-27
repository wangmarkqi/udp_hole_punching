use super::packet::Packet;
use super::conf::Conf;

pub trait Packets {
    fn is_complete(&self) -> bool;
    fn sort(&mut self);
    fn assembly(&mut self) -> Vec<u8>;
}

impl Packets for Vec<Packet> {
    fn is_complete(&self) -> bool {
        if self.len() == 0 {
            return false;
        }
        let pac = self[0];
        let max = pac.max;
        if self.len() != max as usize + 1 {
            return false;
        }
        // let mut res = vec![];
        let orders: Vec<u32> = self.iter().map(|e| e.order).collect();
        for i in 0..max + 1 as u32 {
            if !orders.contains(&i) {
                return false;
                // res.push(i);
            }
        }
        true
    }
    // alway call sort when update packets
    fn sort(&mut self) {
        self.sort_by(|a, b| a.order.partial_cmp(&b.order).unwrap());
    }
    fn assembly(&mut self) -> Vec<u8> {
        let mut res = vec![];
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