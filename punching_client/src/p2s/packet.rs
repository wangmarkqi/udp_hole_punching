use punching_server::action::packet::Packet;

pub trait CMD{
    fn callee_registry(&self) -> bool;
    fn if_need_to_open(&self) -> Packet;
    // calle confirm open for paticular caller.
    fn confirm_conn_open(&self) -> bool;
}
impl CMD for Packet{
     fn callee_registry(&self) ->Packet{
         let pac=Packet::default();

     }
    fn if_need_to_open(&self) -> Packet;
    // calle confirm open for paticular caller.
    fn confirm_conn_open(&self) -> bool;
}

