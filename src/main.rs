use async_std::task::block_on;
use punching_server as serv;
use puching_client as cli;

    msg.to_vec()
}

fn main() {
    // let host= "39.96.40.177:4222";
    // let host= "0.0.0.0:4222";
    // block_on(serv::make_match(host)).unwrap()
    // dbg!("run main");
   // let remote = "39.96.40.177:4222";
        let body="adfsasdfasdfadfad".as_bytes();
    let res=cli::protocal::Packet::segment_bytes(body,10);
}
