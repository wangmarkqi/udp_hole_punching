use async_std::net::{SocketAddr};
use punching_server::{PAC_SIZE, Packet, SPACE_SIZE,HEADER_SIZE};
use super::callee::SOC;
use super::caller::CONN;
use async_trait::async_trait;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Who {
    Callee,
    Caller,
}

#[async_trait]
pub trait P2P {
    async fn send_pac(&mut self, me: Who, addr: SocketAddr) -> anyhow::Result<usize>;
}

#[async_trait]
impl P2P for Packet {
    async fn send_pac(&mut self, me: Who, addr: SocketAddr) -> anyhow::Result<usize> {
        let socket = {
            match me {
                Who::Callee => SOC.get().unwrap(),
                Who::Caller => CONN.get().unwrap(),
            }
        };
        // 小包立刻解决
        let all = self.pack().len();
        let data_len = self.msg.len();
        if all <= HEADER_SIZE {
            let n = socket.send_to(&self.pack(), addr).await?;
            return Ok(n);
        };

        // 大包,把msg提取出来，然后把结构体msg置空，然后json
        let data = &self.msg.clone();
        self.msg = vec![0];

        let body_len = PAC_SIZE - HEADER_SIZE-SPACE_SIZE;
        let remainder = data_len % body_len;
        let times = data_len / body_len;
        // 改max属性,max从0开始
        if remainder != 0 {
            self.max = times;
        } else {
            self.max = times - 1;
        }
        // 分批发送，取整和余数
        for i in 0..times {
            let start =  body_len * i;
            let end =  body_len * (i+1);
            // 改order属性
            self.order = i;
            let body = &data[start..end].to_vec();
            dbg!(body.len());
           let pac=gen_pac(self,body);
            dbg!(pac.len());
            socket.send_to(&pac, addr).await?;
        }
        // 分批发送，取整和余数
        if remainder != 0 {
            self.order = times;
            let body = &data[data_len-remainder..data_len].to_vec();
            let pac=gen_pac(self,body);
            socket.send_to(&pac, addr).await?;
        }

        Ok(all)
    }
}

fn gen_pac(pac:&Packet, body: &Vec<u8>) -> Vec<u8> {
    let mut header = gen_header(pac.pack());
    let mut msg = body.to_owned();
    header.append(&mut msg);
    header
}

fn gen_header(header: Vec<u8>) -> Vec<u8> {
    dbg!(header.len());
    if header.len() > HEADER_SIZE {
        panic!("header beyond limit");
    }
    let mut header_part = vec![];
    for i in 0..HEADER_SIZE {
        if i < header.len() {
            header_part.push(header[i]);
        }
        header_part.push(0);
    }
    dbg!(header_part.len());
    header_part
}


