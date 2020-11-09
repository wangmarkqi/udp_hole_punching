use super::tools::segment_bytes;
use super::packet::Packet;
use super::packets::Packets;
#[test]
fn test_seg(){
     let body="abcdefgh".as_bytes().to_vec();
    let res=segment_bytes(&body,4,2);
    dbg!(res);
}
#[test]
fn test_new_send(){
    let mut pac=Packet::empty();
    pac.session=234;
    let body=&"abcdefghij".as_bytes().to_vec();
    let res=pac.new_pacs_from_send_bytes(10,body);
    dbg!(&res.max());
    dbg!(&res.min());
    dbg!(&res.is_over());
    dbg!(&res.assembly());
    dbg!(&res.is_continue());

}
