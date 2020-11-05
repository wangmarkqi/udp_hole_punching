pub mod action;
pub use action::process::make_match;
pub use action::swap_cmd::SwapCmd;
use async_std::task::block_on;

/// # Examples
/// A server to make match
/// ```
///use async_std::task::block_on;

/// fn main() {
///  let host = "0.0.0.0:9292";
///     block_on(punching_server::make_match(host)).unwrap();
/// ```

#[test]
pub fn run_server() {
    let host = "127.0.0.1:4222";
    block_on(make_match(host)).unwrap();
    // let remote = "39.96.40.177:4222";
    // block_on(punching_client::listen(remote ,&echo)).unwrap_or(());
  // block_on(
  //     punching_client::protocal::caller::test()
  // ).unwrap()
}
