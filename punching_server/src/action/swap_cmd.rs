#[derive(Copy, Clone)]
pub enum SwapCmd {
    None=0,
    Save=1,
    Ask=2,
    Open=3,
    P2P=4,
    Err=5,
}

impl SwapCmd {
    pub fn int2enum(i: u8) -> Self {
        match i {
            1 => SwapCmd::Save,
            2 => SwapCmd::Ask,
            3 => SwapCmd::Open,
            4 => SwapCmd::P2P,
            5 => SwapCmd::Err,
            _ => SwapCmd::None,
        }
    }
    pub fn enum2int( &self) -> u8 {
        *self as u8
    }
}
