pub mod server;
pub use server::swap_cmd::SwapCmd;
pub mod client;
#[macro_use]
extern crate anyhow;
/// # Examples
/// A server to make match
/// ```
///use async_std::task::block_on;

/// fn main() {
///  let host = "0.0.0.0:9292";
///     block_on(punching_server::make_match(host)).unwrap();
/// ```
pub use server::process::make_match;
