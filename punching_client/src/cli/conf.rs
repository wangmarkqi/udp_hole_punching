use async_std::net::{SocketAddr};

#[derive(PartialEq, Debug, Clone)]
pub struct Conf {
    pub id: [u8; 2],
    pub size: u16,
    pub swap_server: String,
}

impl Conf {
    pub fn default() -> Self {
        let mut rng = rand::thread_rng();
        let n1: u8 = rng.gen();
        let n2: u8 = rng.gen();
        let rnd_id = [n1, n2];
        Conf {
            id: rnd_id,
            size: 1024 as u16,
            swap_server:String::from("127.0.0.1:4222"),
        }
    }
}