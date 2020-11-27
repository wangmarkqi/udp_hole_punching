use std::sync:: Mutex;
use once_cell::sync::Lazy;

#[derive(PartialEq, Debug, Clone)]
pub struct Conf {
    pub id: String,
    pub size: usize,
    pub retry_send_times:i32,
    // micro secs
    pub single_rec_timeout: i32,
    pub swap_server: String,
    // 秒
    pub heart_beat_interval:i32,
}

static CONF: Lazy<Mutex<Conf>> = Lazy::new(|| {
    let mut m = Conf::default();
    Mutex::new(m)
});

impl Conf {
    pub fn default() -> Self {
        let _id = "test";
        let conf = Conf {
            id: _id.to_string(),
            size: 1420,
            retry_send_times:3,
            single_rec_timeout: 10,
            swap_server: String::from("127.0.0.1:4222"),
            heart_beat_interval:14,
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