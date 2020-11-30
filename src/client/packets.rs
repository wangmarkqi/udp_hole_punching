use super::packet::Packet;

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
        let max = self[0].max;
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
    let mut v=vec![];
    for i in 0..2000{
        v.push(97);
    }
    let (u,mut pacs) = Packet::new_pacs_from_send_bytes(&v);
    dbg!(&pacs[0].body.len());
    let pac=pacs[0].clone();
    let s1=Packet::new_from_save_db(&pac.to_bytes());
    dbg!(s1.body.len());

}