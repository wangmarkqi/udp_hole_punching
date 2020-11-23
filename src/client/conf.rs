use std::sync:: Mutex;
use once_cell::sync::Lazy;

#[derive(PartialEq, Debug, Clone)]
pub struct Conf {
    pub id: String,
    pub size: usize,
    pub msg_queue_len:usize,
    pub ask_resend_more:usize,
    // micro secs
    pub single_rec_timeout: i32,
    pub ask_resend_elapse:i32,
    pub ask_resend_interval:i32,
    pub swap_server: String,
    // 秒
    pub ask_address_elapse: i32,
    pub heart_beat_interval:i32,
    pub send_cache_timeout: i32,
    pub rec_cache_timeout:i32,
}

pub static CONF: Lazy<Mutex<Conf>> = Lazy::new(|| {
    let mut m = Conf::default();
    Mutex::new(m)
});

impl Conf {
    pub fn default() -> Self {
        let _id = "test";
        let conf = Conf {
            id: _id.to_string(),
            size: 1400,
            ask_resend_more:10,
            msg_queue_len:400,
            single_rec_timeout: 100,
            ask_resend_elapse:400,
            ask_resend_interval:200,
            swap_server: String::from("127.0.0.1:4222"),
            ask_address_elapse:9,
            heart_beat_interval:40,
            rec_cache_timeout:4,
            send_cache_timeout: 4*2,
        };
        conf
    }
    pub fn set(&self) {
        let mut m = CONF.lock().unwrap();
        *m = self.clone();
    }
    pub fn get() -> Self {
        let m = &*CONF.lock().unwrap();
        m.clone()
    }
}