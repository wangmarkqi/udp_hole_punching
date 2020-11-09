pub fn segment_bytes(body: &Vec<u8>, conf_size: usize,header_len:usize) -> Vec<Vec<u8>> {

    // if msg is empty
    let task_total_len = body.len();
    if task_total_len == 0 {
        let  res = vec![];
        return res;
    }

    // calculate max
    let real_capacity_len = conf_size - header_len;
    let remainder = task_total_len % real_capacity_len;
    let times = task_total_len / real_capacity_len;
    // 改max属性,max从0开始
    let max = if remainder != 0 { times } else { times - 1 };

    let mut queue = vec![];
    let mut task_done_len = 0;
    let mut order = 0;

    while task_done_len < task_total_len {
        let task_left_len = task_total_len - task_done_len;
        let this_done_len = {
            if task_left_len >= real_capacity_len {
                real_capacity_len as usize
            } else {
                task_left_len as usize
            }
        };

        let mut this_body = vec![0; this_done_len];
        for i in task_done_len..task_done_len + this_done_len {
            this_body[i - task_done_len] = body[i];
        }

        task_done_len = task_done_len + this_done_len;

        order = order + 1;

        queue.push(this_body);
    }
    queue
}
