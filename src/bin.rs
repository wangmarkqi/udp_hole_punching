use async_std::task::block_on;
pub mod server;
pub mod client;
#[macro_use]
extern crate anyhow;
use client::test::*;
use server::process::test_swap_server;
fn main() {

    // block_on(test_callee_listen());

    // block_on(test_caller_api());
    // block_on(test_swap_server());
}
