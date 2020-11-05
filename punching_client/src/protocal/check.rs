use crate::protocal::packets::Packets;
use super::cache::{Cache, Store};
pub trait Check{
    fn not_send(sess:u8) ->Self;
}
impl Check for Packets{
    fn not_send(sess:u8)->Packets{
       let send_pacs=Cache::get(sess,Store::Send) ;
        let back_pacs:Self=Cache::get(sess,Store::Back) ;

        if back_pacs.pacs.len()==0{
            return send_pacs;
        }
        let finish:Vec<u32>=back_pacs.pacs.iter().map(|e|e.order).collect();

        let mut v=vec![];
        for i in send_pacs.pacs.iter(){
            if !finish.contains(&i.order){
                v.push(i.clone());
            }
        }
        Packets{
            session:sess,
            pacs:v,
        }
    }
}
