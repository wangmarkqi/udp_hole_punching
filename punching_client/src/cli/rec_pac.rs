use async_std::io;
use punching_server::{PAC_SIZE,HEADER_SIZE, Packet, CMD};
use std::time::Duration;
use super::callee::SOC;
use super::caller::CONN;
use super::p2p::Who;
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

static PACS_CALLEE: Lazy<Mutex<HashMap<i32, Vec<Packet>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let mut v = vec![];
    let pac = Packet::default();
    v.push(pac);
    map.insert(0, v);
    Mutex::new(map)
});
static PACS_CALLER: Lazy<Mutex<HashMap<i32, Vec<Packet>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let mut v = vec![];
    let pac = Packet::default();
    v.push(pac);
    map.insert(0, v);
    Mutex::new(map)
});

pub async fn rec_pac(me: Who) -> anyhow::Result<Packet> {
    let pac = rec_single_pac(me).await?;
    if pac.max == 0 {
        return Ok(pac);
    }

    // find right map
    let mut map = {
        match me {
            Who::Callee => PACS_CALLEE.lock().unwrap(),
            Who::Caller => PACS_CALLER.lock().unwrap(),
        }
    };

    // save new pac to map
    let session = pac.session;
    if map.contains_key(&session) {
        let mut origin = map.get(&session).unwrap().to_owned();
        origin.push(pac);
        map.insert(session, origin);
    } else {
        let mut _v = vec![];
        _v.push(pac);
        map.insert(session, _v);
    };

    // find pacs done

    let map_clone = map.clone();
    for (k, v) in map_clone {
        if v.len() > 0 {
            let max = v[0].max;
            if max == v.len() - 1 {
                map.remove(&k);
                let res = assembly_pacs(v)?;
                return Ok(res);
            }
        }
    }
    Err(anyhow!("no complete data pack received"))
}

fn assembly_pacs(mut v: Vec<Packet>) -> anyhow::Result<Packet> {
    v.sort_by(|a, b| a.order.cmp(&b.order));
    let mut res = v[0].clone();
    if res.order != 0 {
        panic!("the packs order wrong");
    }
    let mut start = vec![];
    for p in v.iter() {
        start.append(&mut p.msg.to_owned());
    }
    res.msg = start;
    Ok(res)
}
// 如果收取的包长度小于header，说明msg就在包里面，如果大于，说明msg在body里面，需要把body转移到msg字段
pub async fn rec_single_pac(me: Who) -> anyhow::Result<Packet> {
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

    // header截去
    let header = {
        if n <= HEADER_SIZE {
            &buf[0..n]
        } else {
            // header size控制不住
            &buf[0..HEADER_SIZE]
        }
    };

    dbg!(n,header);
    let data = String::from_utf8_lossy(header);
    let mut income: Packet = serde_json::from_str(&data)?;

    //补充body msg
    if n > HEADER_SIZE {
        let body= &buf[HEADER_SIZE..n].to_vec();
        income.msg=body.to_owned();
    }
    // 说明这个包从server过来的
    if income.cmd != CMD::Open {
        match me {
            Who::Caller => income.callee_address = peer,
            Who::Callee => income.caller_address = peer,
        };
    }

    Ok(income)
}
