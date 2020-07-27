
use async_std::task::block_on;
use std::thread;
fn main() {
    let host="127.0.0.1:9999";
    thread::spawn( move ||{
        block_on(punching_server::run(host)).unwrap();
    });

    block_on(punching_client::start(host)).unwrap();
}
