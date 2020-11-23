# UDP_HOLE_PUNCHING
This crate is aimed to be rust p2p communication framework.

## What is p2p? 

 When a socket has been "connected" to the other client, the "connection" is directly to the client, not the server. The server is not used to pass through packages, only to pair the two clients together. As you should know, since it's the whole purpose of [UDP hole punching](https://en.wikipedia.org/wiki/UDP_hole_punching).
 To implement a p2p framework, the crate provide 2 sub function:
 - the server: Use this function if you want to run a standard server which waits for client-callee to send the identification for registry, then client-caller send "Ask" command with client-callee id.  The server will sends the each clients' IP address and external port number to each other. It also has some basic protection against receive overtime etc. If you want something more customisable, take a look at `make_match`,
 - the client(includes the caller and callee): The single udp transfer packet size in this crate is defined by "pub const Conf.size = 1024;". However,the crate will split data beyond size automatically when sending, assembly to integrate automatically when receiving, asking resending when packetes is not complete. 
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
  - test_callee_listen
  
 
```
    use super::conf::Conf;
    use super::api::*;
    use async_std::task::block_on;
    use super::listen::listen;
    use std::time::Duration;
    pub async fn test_callee_listen() -> anyhow::Result<()> {
        let mut conf = Conf::default();
        conf.swap_server = "<swap server ip: port>".to_string();
        conf.id = "wq".to_string();
        conf.set();
        init_udp().await?;
        std::thread::spawn(|| {
            block_on(listen());
        });

        loop {
            let (addr, v) = rec_from();
            if v.len() > 0 {
                let s = String::from_utf8_lossy(&v);
                dbg!("callee rec res");
                dbg!(s.len());
                let back = "callee got you".as_bytes().to_vec();
                send(&back, addr).await?;
            }
        };
        Ok(())
    }


```

  - test_caller_api 
  
  ```
    pub async fn test_caller_api() -> anyhow::Result<()> {
        let mut conf = Conf::default();
        conf.swap_server = "<swap server ip: port>".to_string();
        conf.set();
        init_udp().await?;
        std::thread::spawn(|| {
            block_on(listen());
        });
        let addr = get_peer_address("wq").await?;
        dbg!(addr);
        dbg!("begin");

//test resend assembly when pac size beyond conf size
        let msg={
            let mut v=vec![];
            for i in 0..1024*10{
                v.push(8 as u8);
            }
            v
        };

        loop {
            send(&msg, addr).await?;
            let (addr, v) = rec_from();
            if v.len() > 0 {
                let s = String::from_utf8_lossy(&v);
                dbg!("caller  rec res");
            }
        }
        Ok(())
    }

  ```

