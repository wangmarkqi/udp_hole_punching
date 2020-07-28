use async_std::task::block_on;

fn main() {
    // let host = "0.0.0.0:4222";
    // block_on(punching_server::make_match(host)).unwrap();

    let remote = "39.96.40.177:4222";
    block_on(punching_client::listen(remote )).unwrap_or(());
    //
    //
    //  let loc2="127.0.0.1:9997";
    // let uuid = "b997dbac-e919-4e44-a8b5-9f7017381e30";
    //  block_on(punching_client::init_caller(loc2)).unwrap();
    // block_on(punching_client::connect(host, uuid)).unwrap();
}
