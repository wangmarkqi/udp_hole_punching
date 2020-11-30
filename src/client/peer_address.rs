use once_cell::sync::Lazy;
use std::sync::Mutex;

static PeerAddress: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new("".to_string())
});

pub fn update_peer_address(address: String) {
    let mut store = PeerAddress.lock().unwrap();
    *store = address;
}

pub fn get_peer_address() -> String {
    let store = PeerAddress.lock().unwrap();
    store.clone()
}
