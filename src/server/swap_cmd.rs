#[derive(Copy, Clone,Debug,PartialEq, Eq, Hash)]
pub enum SwapCmd {
    None = 0,
    Save = 1,
    Ask = 2,
    Open = 3,
    ServerErr = 4,
    P2P = 5,
    Finish = 6,
    Hello= 7,
    Resend=8,
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
            4 => SwapCmd::ServerErr,
            5 => SwapCmd::P2P,
            6 => SwapCmd::Finish,
            7 => SwapCmd::Hello,
            8 => SwapCmd::Resend,
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
    pub fn from_server(&self) -> bool{
        match self{
            SwapCmd::Save=>true,
            SwapCmd::Ask=>true,
            SwapCmd::Open=>true,
            SwapCmd::ServerErr=>true,
            _=>false,
        }
    }
}
