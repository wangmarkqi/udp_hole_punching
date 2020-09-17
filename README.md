# UDP_HOLE_PUNCHING
This crate is aimed to be rust p2p communication framework.

## What is p2p? 

 When a socket has been "connected" to the other client, the "connection" is directly to the client, not the server. The server is not used to pass through packages, only to pair the two clients together. As you should know, since it's the whole purpose of [UDP hole punching](https://en.wikipedia.org/wiki/UDP_hole_punching).
 To implement a p2p framework, the crate provide 3 sub function:
 - the server: Use this function if you want to run a standard server which waits for client-callee to send the uuid identification for registry, then client-caller send "Open" command with client-callee uuid.  The server will sends the each clients' IP address and external port number to each other. It also has some basic protection against receive overtime etc. If you want something more customisable, take a look at `make_match`,
 - the client(includes the caller and callee): The single udp transfer packet size in this crate is defined by "pub const PAC_SIZE: usize = 1472;". However,the p2p trait will split data beyond size automatically when sending, assembly to integrate automatically when receiving, withing single thread. See  `P2P trait`for more.

  ## Quick Start 
  - Server 
  
  see main.rs
```
// the code should be run in the server with public ip. 

use async_std::task::block_on;
fn main() {
     let host = "0.0.0.0:9292";
     block_on(punching_server::make_match(host)).unwrap();
}
```
  - Client:Callee 
  
  see main.rs. 
```
// the code should be run in the client you want to waite to be called. the callee will create uuid file if not exist,and send to server for registry purpose. the user can create uuid file manually, uuid should not duplicate among callees. 

use async_std::task::block_on;
// all your route and handler here.msg is incoming data and return result.
 fn echo(msg:&Vec<u8>)->Vec<u8>{
     msg.to_vec()
}

 fn main() {
     let remote = "xx.xx.xx.xx:xxxxx";
     block_on(punching_client::listen(remote ,&echo)).unwrap_or(());
}
```

  - Client:Caller 
  
  see main.rs
  ```

// the code should be run in the client you want to send commands to callee. the uuid is id for callee. 
use async_std::task::block_on;
 fn main() {
  block_on(
      punching_client::cli::caller::test()
  ).unwrap()
}
  ```
see punching_client/cli/caller.rs
   ```

pub async fn test() -> anyhow::Result<()> {
    let uuid = "b997dbac-e919-4e44-a8b5-9f7017381e30";
    let remote = "xx.xx.xx.xx:xxxx";
    let address = connect(remote, uuid).await?;
    dbg!(&address);
    let mut msg = vec![];
    msg.push(1);
    msg.push(2);
    for _ in 0..1099 {
        for u in 0..10 {
            msg.push(u);
        }
    }
    // let msg="this is just a test".as_bytes().to_vec();
    // dbg!(&msg.len());
    let session = send(&msg, address).await?;
    dbg!(session);
    let res = rec(session,1000).await?;
    let back=res.1;
   // let back= std::str::from_utf8(&res.1).unwrap();
    dbg!(back.len());
    Ok(())
}

  ```
 
