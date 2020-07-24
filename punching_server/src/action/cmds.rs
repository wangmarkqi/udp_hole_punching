use crate::action::packet::Packet;
use std::sync::Mutex;
use std::collections::HashMap;

use std::net::{SocketAddr};
lazy_static! {
    pub static ref ADDRESS: Mutex<HashMap<String,SocketAddr>> = Mutex::new(HashMap::new());
    pub static ref ASK: Mutex<Vec<Packet>> = Mutex::new(vec![]);
    pub static ref READY: Mutex<Vec<Packet>> = Mutex::new(vec![]);
}

pub trait CMD {
    // callee report id and address
    // if need to open,return caller adress,else success
    fn callee_registry(&self) -> bool;
    fn if_need_to_open(&self) -> Packet;
    // calle confirm open for paticular caller.
    fn confirm_conn_open(&self) -> bool;
    fn caller_ask_open(&self) -> Packet;
    fn caller_query_conn(&self) -> Packet;
}

impl CMD for Packet {
    fn callee_registry(&self) -> bool {
        let mut  dic=ADDRESS.lock().unwrap();
        let id=&self.callee_uuid;
        dic.insert(id.to_owned(),self.callee_address);
        true
    }

    // self.info==me id me is callee
    fn if_need_to_open(&self) -> Packet {
        let mut lis = ASK.lock().unwrap();
        let lis2 = lis.clone();

        for (i, v) in lis2.iter().enumerate() {
            if self.callee_uuid == v.callee_uuid {
                lis.remove(i);
                return v.clone();
            }
        }
        Packet::default()
    }
    // callee send
    fn confirm_conn_open(&self) -> bool {
        let mut lis = READY.lock().unwrap();
        lis.push(self.clone());
        true
    }

    // 要在上面查看是否有通讯需求，这样send,用arc mutex hash map
    // 客户端第一次发申请，第二次发查询是否打开
    fn caller_ask_open(&self) -> Packet {
        let mut default=Packet::default();
        let callee_id = &self.callee_uuid;
        let dic=ADDRESS.lock().unwrap();
        if !dic.contains_key(callee_id){
            default.success = false;
            default.err = "peer address not exist".to_string();
            return default;
        }
        let mut lis = ASK.lock().unwrap();
        lis.push(self.clone());
        default.msg = "ask enqueue".to_string();
        default
    }

    fn caller_query_conn(&self) -> Packet {
        let mut lis = READY.lock().unwrap();
        let lis2 = lis.clone();

        for (i, v) in lis2.iter().enumerate() {
            if self.callee_uuid == v.callee_uuid && self.caller_address == v.caller_address {
                lis.remove(i);
                return v.clone();
            }
        }
        Packet::default()
    }
}
