use async_trait::async_trait;
use super::define::*;
#[async_trait]
impl Sender for Packet {
    fn segmentation(&self,task_total:&Vec<u8>) -> Vec<Vec<u8>> {
        // give a template for header
        let mut template = self.clone();

        // if msg is empty
        let task_total_len = task_total.len();
        if task_total_len == 0 {
            let mut res = vec![];
            res.push(template.pack());
            return res;
        }


        // calculate max
        let header_len = template.pack().len();
        let real_capacity_len = PAC_SIZE - header_len;
        let remainder = task_total_len % real_capacity_len;
        let times = task_total_len / real_capacity_len;
        // 改max属性,max从0开始
        let max = if remainder != 0 { times } else { times - 1 } as u16;
        template.max = max;


        let mut queue = vec![];
        let mut task_done_len = 0;
        let mut order = 0;

        while task_done_len < task_total_len {
            let task_left_len = task_total_len - task_done_len;
            let this_done_len = {
                if task_left_len >= real_capacity_len {
                    real_capacity_len
                } else {
                    task_left_len
                }
            };

            template.order = order;
            template.body_len = this_done_len as u16;
            let header = template.pack();
            if header.len() != header_len {
                panic!("impossible happens");
            }
            let mut msg = vec![0; header_len + this_done_len];
            for i in 0..header_len {
                msg[i] = header[i];
            }
            for i in header_len..header_len + this_done_len {
                msg[i] = task_total[task_done_len + i - header_len];
            }

            queue.push(msg);
            task_done_len = task_done_len + this_done_len;
            order = order + 1;
        }
        if order != max {
            panic!("one of max or order is wrong")
        }
        queue
    }

    async fn send_pac(&self, me: Who,msg:&Vec<u8>) -> anyhow::Result<u16> {
        let socket = {
            match me {
                Who::Callee => SOC.get().unwrap(),
                Who::Caller => CONN.get().unwrap(),
            }
        };
        let queue = self.segmentation(msg);
        for q in queue.iter() {
            socket.send_to(q, self.address).await?;
        }
        Ok(self.session)
    }
}





