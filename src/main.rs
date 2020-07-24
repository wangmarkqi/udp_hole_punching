
use async_std::task::block_on;
fn main() {
    let host="127.0.0.1:9999";
    block_on(punching_server::run(host)).unwrap();
}
