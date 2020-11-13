use async_std::task::block_on;
pub mod server;
pub use server::process::make_match;
pub mod client;
#[macro_use]
extern crate anyhow;
use client::api_server::test_server_api;
use client::api_callee::test_callee_listen;
use server::process::test_swap_server;
use client::packet::Packet;
fn main() {

    block_on(test_callee_listen());

    // block_on(test_server_api());
    // block_on(test_swap_server());
}
