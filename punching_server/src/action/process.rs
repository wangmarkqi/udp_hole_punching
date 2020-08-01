use crate::action::packet::Packet;
use std::sync::Mutex;
use std::collections::HashMap;

use std::net::{SocketAddr};
lazy_static! {
    pub static ref ADDRESS: Mutex<HashMap<String,SocketAddr>> = Mutex::new(HashMap::new());
}

pub trait Process {
    fn callee_registry(& self) -> bool;
    fn make_pair(&self) -> (Packet,Packet);
}

impl Process for Packet {
    fn callee_registry(&self) -> bool {
        let mut dic = ADDRESS.lock().unwrap();
        let id = &self.callee;
        dic.insert(id.to_owned(), self.address);
        true
    }

    fn make_pair(&self) -> (Packet,Packet) {
        let mut pac2caller=Packet::caller_open_default(&self.callee);
        let mut pac2callee=Packet::caller_open_default(&self.callee);
        let mut pac2fail=Packet::caller_open_default(&self.callee);

        let dic = ADDRESS.lock().unwrap();
        if !dic.contains_key(&self.callee) {
            pac2fail.success=false;
            pac2fail.err="can not find peer ip to call".to_string();
            return (pac2fail.clone(),pac2fail);
        }
        pac2callee.address= self.address;
        pac2caller.address=*dic.get(&self.callee).unwrap();
        (pac2caller,pac2callee)
    }
}
