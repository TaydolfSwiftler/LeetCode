pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut chars = strs.iter().map(|x| x.chars()).collect::<Vec<_>>();
    let length = strs.len();
    let mut candidate = "".to_string();
    let mut chars_index = 0;
    let mut kill_flag = false;
    let mut curr_char;
    loop {
        let count: u16 = 0;
        match chars[0].next() {
            None => break,
            Some(x) => {
                curr_char = x;
            }
        }
        for i in 1..length {
            match chars[i].next() {
                None => {
                    kill_flag = true;
                    break;
                }
                Some(x) => {
                    if x == curr_char {
                        continue;
                    } else {
                        kill_flag = true;
                        break;
                    }
                }
            }
        }
        if kill_flag {
            break;
        }
        chars_index += 1;
        candidate.push(curr_char);
    }

    candidate
}
