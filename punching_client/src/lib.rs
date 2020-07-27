//! `hole-punch-connect` is a crate that makes [UDP hole punching](https://en.wikipedia.org/wiki/UDP_hole_punching) easier.
//! For use in a client, take a look at the `HolePunchConnect` trait.
//! For use on a server, take a look at the `server` module.
//!
//! One beautiful day, I or someone else might also add [TCP hole punching](https://en.wikipedia.org/wiki/TCP_hole_punching)
//! support to this crate. (Which is why the crate name doesn't contain UDP)
//! 
//! # Examples

pub mod p2s;
use p2s::callee::_listen;
pub async fn listen(host:&str) -> anyhow::Result<()> {
    _listen(host).await
}

