use async_std::io;
use super::define::*;
use std::time::{Duration};
use async_trait::async_trait;
#[async_trait]
// this trait if for packet with body_len>0
impl Receiver for Packet {
    fn get_cached(&self, me: Who) -> Vec<(u16, Vec<u8>)> {
        if self.body_len == 0 {
            return vec![];
        }
        // find right map
        let  map = {
            match me {
                Who::Callee => REC_CALLEE.lock().unwrap(),
                Who::Caller => REC_CALLER.lock().unwrap(),
            }
        };
        let k=(self.session,self.max);
        let mut v = map.get(&k).unwrap().to_owned();
        v.sort_by(|a, b| a.0.cmp(&b.0));
        v
    }
    fn is_done(&self, me: Who) -> bool {
        if self.body_len == 0 {
            return true;
        }
        let v = self.get_cached(me);
        self.max as usize == v.len()
    }
    fn clear_cached(&self, me: Who) {
        if self.body_len == 0 { return; };

        let mut map = {
            match me {
                Who::Callee => REC_CALLEE.lock().unwrap(),
                Who::Caller => REC_CALLER.lock().unwrap(),
            }
        };
        map.remove(&(self.session, self.max)).unwrap().to_owned();
    }
    // 拿到成功后就删除了数据
    fn assembly(&self, me: Who) -> anyhow::Result<Vec<u8>> {
        // body没有，要传递的就是header
        if self.body_len == 0 {
            return Ok(self.pack());
        }
        if !self.is_done(me) {
            return Err(anyhow!("this session has not been done"));
        }
        let data = self.get_cached(me);
        if data.len()-1 != self.max as usize {
            panic!("data len should equall to max");
        }

        let mut start = vec![];
        for (i, msg) in data.iter().enumerate() {
            let (order,  ms) = msg;
            if i != *order as usize {
                panic!("order size error");
            }
            let mut parts=ms.to_owned();
            start.append(&mut parts);
        }
        self.clear_cached(me);
        Ok(start)
    }
}


// 如果收取的包长度小于header，说明msg就在包里面，如果大于，说明msg在body里面，需要把body转移到msg字段
pub async fn rec_single_pac(me: Who) -> anyhow::Result<Packet> {
    // 先收进来
    let socket = {
        match me {
            Who::Callee => SOC.get().unwrap(),
            Who::Caller => CONN.get().unwrap(),
        }
    };
    let mut buf = vec![0u8; PAC_SIZE];
    let (n, peer) = io::timeout(Duration::from_secs(4), async {
        socket.recv_from(&mut buf).await
    }).await?;

    if n == 0 {
        return Err(anyhow!("receive no data from server"));
    }
    if n > PAC_SIZE {
        return Err(anyhow!("max pack size:{},actual rec{}",PAC_SIZE,n));
    }
// 没有body的就返回，有body的就存起来，然后返回
    let mut header = Packet::unpack(&buf[0..n].to_vec())?;
    // 只有p2p改地址，服务器的不改
    match header.cmd{
        CMD::P2P=> header.address = peer,
        _=>(),
    }
    if header.body_len == 0 {
        // 出去的可能是p2p，open等
        return Ok(header);
    };


    // find right map
    let mut map = {
        match me {
            Who::Callee => REC_CALLEE.lock().unwrap(),
            Who::Caller => REC_CALLER.lock().unwrap(),
        }
    };

    let k = (header.session, header.max);
    let body = unpack_body(&buf, header.body_len as usize);
    // 注意存储的是body
    let v = (header.order, body);
    if map.contains_key(&k) {
        let mut origin = map.get(&k).unwrap().to_owned();
        origin.push(v);
        map.insert(k, origin);
    } else {
        let mut _v = vec![];
        _v.push(v);
        map.insert(k, _v);
    };
    Ok(header)
}

fn unpack_body(enc: &Vec<u8>, body_len: usize) -> Vec<u8> {
    let header_len = u16::from_be_bytes([enc[0], enc[1]]) as usize;
    let mut body = vec![0; body_len];
    let body_start = 2 + header_len;
    for i in 0..body_len {
        body[i] = enc[body_start + i];
    }
    body.to_vec()
}