//! `connect` is a crate that makes [UDP hole punching](https://en.wikipedia.org/wiki/UDP_hole_punching) easier.
//! For use in a client, take a look at the `HolePunchConnect` trait.
//! For use on a server, take a look at the `server` module.
//!
//! One beautiful day, I or someone else might also add [TCP hole punching](https://en.wikipedia.org/wiki/TCP_hole_punching)
//! support to this crate. (Which is why the crate name doesn't contain UDP)
//! 
//! # Examples
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;
pub mod p2s;
use p2s::callee::*;
use p2s::caller::*;

pub async fn listen(remote: &str) -> anyhow::Result<()> {
    _listen(remote).await
}



pub async fn init_caller(localhost: &str) -> anyhow::Result<()> {
    _init_caller(localhost).await?;
    Ok(())
}
pub async fn connect(host: &str, uuid: &str) -> anyhow::Result<()> {
    loop{
        ask_connect(host, uuid).await?;
    }
}

