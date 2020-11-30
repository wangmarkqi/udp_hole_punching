use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(PartialEq, Debug, Clone)]
pub struct Conf {
    pub id: String,
    pub db_path: String,
    pub size: usize,
    pub min_retry_len: i32,
    pub retry_send_times: i32,
    // micro secs
    pub single_rec_timeout: i32,
    pub swap_server: String,
    // 秒
    pub heart_beat_interval: i32,
}

static CONF: Lazy<Mutex<Conf>> = Lazy::new(|| {
    let  m = Conf::default();
    Mutex::new(m)
});

impl Conf {
    pub fn default() -> Self {
        let conf = Conf {
            id: "".to_string(),
            db_path: "./db".to_string(),
            size: 1420,
            min_retry_len: 10,
            retry_send_times: 4,
            single_rec_timeout: 50,
            swap_server: String::from("127.0.0.1:4222"),
            heart_beat_interval: 14,
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