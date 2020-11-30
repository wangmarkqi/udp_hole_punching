# UDP_HOLE_PUNCHING
This crate is aimed to be rust p2p communication framework.

## What is p2p? 

 When a socket has been "connected" to the other client, the "connection" is directly to the client, not the server. The server is not used to pass through packages, only to pair the two clients together. As you should know, since it's the whole purpose of [UDP hole punching](https://en.wikipedia.org/wiki/UDP_hole_punching).
 To implement a p2p framework, the crate provide 2 sub function:
 - the server: Use this function if you want to run a standard server which waits for client-callee to send the identification for registry, then client-caller send "Ask" command with client-callee id.  The server will sends the each clients' IP address and external port number to each other. It also has some basic protection against receive overtime etc. If you want something more customisable, take a look at `make_match`,
 - the client(includes the caller and callee): The single udp transfer packet size in this crate is defined by "pub const Conf.size = 1024;". However,the crate will split data beyond size automatically when sending, assembly to integrate automatically when receiving, ask resending when packets is not complete. 
  ## Quick Start 
  
  - main.rs
```
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


```
  - test_swap_server 
  
```
    pub async fn test_swap_server() {
        let host = "0.0.0.0:xxxx";
        let res= make_match(host).await;
        match res{
            Ok(())=>dbg!("everything ok"),
            Err(e)=>dbg!(&e.to_string()),
        };
}
```
  - test_callee/caller_listen
  
 
```
 
fn read_file_as_u8(inputfile: &str) -> anyhow::Result<Vec<u8>> {
    let mut _inputfile = File::open(inputfile)?;
    let mut v: Vec<u8> = Vec::new();
    _inputfile.read_to_end(&mut v)?;
    Ok(v)
}

fn write_file_as_u8(path_str: &str, binary: &Vec<u8>) -> anyhow::Result<String> {
    let p = std::path::Path::new(path_str);
    std::fs::write(p, binary)?;
    Ok(format!("保存成功,地址：{}", path_str))
}


pub fn test_callee_listen() -> anyhow::Result<()> {
    let mut conf = Conf::default();
    conf.swap_server = "x.x.x.x:xxxx".to_string();
    conf.id = "xx".to_string();
    conf.db_path="./data/callee".to_string();
    conf.set();

    init_udp();
    std::thread::spawn(|| {
        listen();
    });
    loop {
        std::thread::sleep(Duration::from_secs(10));
        let (addr, v) = rec_from();
        if &v.len() > &0 {
            dbg!("callee rec res");
            write_file_as_u8("/home/b.exe", &v)?;
            let back = "callee got you".as_bytes().to_vec();
            send(&back, addr);
        }
    };
    Ok(())
}

pub fn test_caller_api() -> anyhow::Result<()> {
    let mut conf = Conf::default();
    conf.swap_server = "x.x.x.x:xxxx".to_string();
    conf.db_path="./data/caller".to_string();
    conf.set();
    init_udp();
    std::thread::spawn(|| {
        listen();
    });
    ask_peer_address("xx");
    std::thread::sleep(Duration::from_secs(9));
    let addr = read_peer_address()?;
    dbg!(&addr);
    dbg!("begin");

    let msg = read_file_as_u8("D://a.exe")?;
    let sess=send(&msg, addr);

    loop {
        let  v = rec_one(addr,sess);
        if v.len() > 0 {
            let s = String::from_utf8_lossy(&v);
            dbg!("caller  rec res");
        }
        std::thread::sleep(Duration::from_secs(4));
    }
    Ok(())
}


```

## Who use this crate ? 
-  remote_shell: https://github.com/wangmarkqi/remote_shell
