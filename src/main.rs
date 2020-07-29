use async_std::task::block_on;


pub fn echo(msg:&Vec<u8>)->Vec<u8>{
    msg.to_vec()
}
fn main() {
    // let host = "0.0.0.0:4222";
    // block_on(punching_server::make_match(host)).unwrap();

    // let remote = "39.96.40.177:4222";
    // block_on(punching_client::listen(remote ,&echo)).unwrap_or(());
  block_on(
      punching_client::cli::caller::test()
  ).unwrap()
}
