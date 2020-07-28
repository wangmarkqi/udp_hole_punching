use crate::action::packet::Packet;
use std::sync::Mutex;
use std::collections::HashMap;

use std::net::{SocketAddr};
lazy_static! {
    pub static ref ADDRESS: Mutex<HashMap<String,SocketAddr>> = Mutex::new(HashMap::new());
}

pub trait Process {
    fn callee_registry(& self) -> bool;
    fn make_pair(&mut self) -> bool;
}

impl Process for Packet {
    fn callee_registry(&self) -> bool {
        let mut dic = ADDRESS.lock().unwrap();
        let id = &self.callee_uuid;
        dic.insert(id.to_owned(), self.callee_address);
        true
    }
    fn make_pair(&mut self) -> bool {
        let callee_id = &self.callee_uuid;
        let dic = ADDRESS.lock().unwrap();
        if !dic.contains_key(callee_id) {
            self.success=false;
            self.err="can not find peer ip to call".to_string();
            return false;
        }
        self.success=true;
        self.callee_address = *dic.get(callee_id).unwrap();
        true
    }
}
