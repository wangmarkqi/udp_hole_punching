use punching_server::{CMD,Packet};
use std::fs::{read_to_string, File};
use std::io::prelude::*;

pub trait P2S {
    fn callee_report(&mut self) -> anyhow::Result<()> ;
    fn caller_ask_open(&mut self,uuid:&str) -> anyhow::Result<()> ;
}

impl P2S for Packet {
    fn callee_report(&mut self) -> anyhow::Result<()> {
        let uuid=get_uuid()?;
        self.callee_uuid=uuid;
        self.cmd=CMD::Save;
        Ok(())
    }
    fn caller_ask_open(&mut self,uuid:&str) -> anyhow::Result<()>{
        self.cmd=CMD::Open;
        self.callee_uuid=uuid.to_string();
        Ok(())
    }
}

fn get_uuid() -> anyhow::Result<String> {
    let path="./uuid";
    let p = std::path::Path::new(path);
    if p.exists() {
        let res = read_to_string(p)?;
        if res != "" {
            return Ok(res);
        }
    }
    let my_uuid = uuid::Uuid::new_v4();
    let mut output: File = File::create(p)?;
    write!(output, "{}", my_uuid)?;
    let content = format!("{}", my_uuid);
    Ok(content)
}

