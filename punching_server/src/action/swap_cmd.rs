#[derive(Copy, Clone)]
pub enum SwapCmd {
    None = 0,
    Save = 1,
    Ask = 2,
    Open = 3,
    Req = 4,
    ServerErr = 5,
    Got = 6,
    PeerErr = 7,
}

fn gen_cmd(id: &str, cmd: SwapCmd) -> Vec<u8> {
    let mut v = vec![];
    v.push(cmd.enum2int());
    let b = id.as_bytes();
    for i in b.iter() {
        v.push(*i);
    }
    v
}

impl SwapCmd {
    pub fn int2enum(i: u8) -> Self {
        match i {
            1 => SwapCmd::Save,
            2 => SwapCmd::Ask,
            3 => SwapCmd::Open,
            4 => SwapCmd::Req,
            5 => SwapCmd::ServerErr,
            6 => SwapCmd::Got,
            7 => SwapCmd::PeerErr,
            _ => SwapCmd::None,
        }
    }
    pub fn enum2int(&self) -> u8 {
        *self as u8
    }
    pub fn ask(id: &str) -> Vec<u8> {
        gen_cmd(id, SwapCmd::Ask)
    }
    pub fn save(id: &str) -> Vec<u8> {
        gen_cmd(id, SwapCmd::Save)
    }
    pub fn open(id: &str) -> Vec<u8> {
        gen_cmd(id, SwapCmd::Open)
    }
    pub fn from_server(i:u8) -> bool{
    let c = SwapCmd::int2enum(i);
        match c{
            SwapCmd::Save=>true,
            SwapCmd::Ask=>true,
            SwapCmd::Open=>true,
            SwapCmd::ServerErr=>true,
            _=>false,
        }
    }
}
