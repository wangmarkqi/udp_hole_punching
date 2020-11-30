pub mod server;
pub mod client;
#[macro_use]
extern crate anyhow;
use client::test::*;
fn main() {

    // test_db();
    // test_callee_listen();
    test_caller_api();
    // test_swap_server();
}
