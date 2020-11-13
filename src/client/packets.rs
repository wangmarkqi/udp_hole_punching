use super::packet::Packet;

pub trait Packets {
    fn empty() -> Self;
    fn min(&self) -> u32;
    fn max(&self) -> u32;
    fn has_begin(&self) -> bool;
    fn has_end(&self) -> bool;
    fn lack(&self) -> Vec<u32>;
    fn is_continue(&self) -> bool;
    fn assembly(&self) -> Vec<u8>;
    fn add_no_duplicate(&mut self, pac: Packet)->bool;
}

impl Packets for Vec<Packet> {
    fn empty() -> Vec<Packet> {
        vec![]
    }
    fn min(&self) -> u32 {
        if self.len() == 0 {
            return 0;
        }
        self.iter().map(|e| e.order).min().unwrap()
    }
    fn max(&self) -> u32 {
        if self.len() == 0 {
            return 0;
        }
        self.iter().map(|e| e.order).max().unwrap()
    }
    fn has_begin(&self) -> bool {
        for i in self.iter() {
            if i.order == 0 {
                return true;
            }
        }
        false
    }
    fn has_end(&self) -> bool {
        for i in self.iter() {
            if i.over == 1 {
                return true;
            }
        }
        false
    }
    fn lack(&self) -> Vec<u32> {
        let mut orders: Vec<u32> = self.iter().map(|e| e.order).collect();
        let max = self.max();
        let mut v = vec![];
        for i in 0..max + 1 {
            if !orders.contains(&i) {
                v.push(i);
            }
        }
        if !self.has_end() {
            v.push(max + 1);
        }
        v
    }

    fn is_continue(&self) -> bool {
        let mut orders: Vec<u32> = self.iter().map(|e| e.order).collect();
        orders.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let total = orders.len();
        for i in 0..total - 1 {
            if i < total - 1 {
                let cur = orders[i] as i32;
                let next = orders[i + 1] as i32;
                let differ = next - cur;
                if differ != 1 {
                    return false;
                }
            }
        }
        true
    }
    fn assembly(&self) -> Vec<u8> {
        let mut res = vec![];
        if self.len() == 0 {
            return res;
        };
        let pac = &mut self.clone();
        pac.sort_by(|a, b| a.order.partial_cmp(&b.order).unwrap());
        for i in pac.iter() {
            for ii in i.body.iter() {
                res.push(*ii);
            }
        }
        res
    }

    fn add_no_duplicate(&mut self, pac: Packet)->bool {
        let orders: Vec<u32> = self.iter().map(|e| e.order).collect();
        if !orders.contains(&pac.order) {
            self.push(pac);
            return true;
        }
        false
    }
}

