impl Solution {
pub fn convert(s: String, num_rows: i32) -> String {
    let result: String = String::new();
    let length = s.len();
    if length == 0 {
        return "".to_string();
    }
    let mut string_parts: Vec<String> = vec!["".to_string(); num_rows as usize];
    let chars: Vec<_> = s.chars().collect();
    let mut curr_cha = 0;
    loop {
        for i in 0..num_rows {
            if curr_cha >= length {
                break;
            }
            string_parts[i as usize].push(chars[curr_cha]);
            curr_cha += 1;
        }
        for i in (0..num_rows - 2).rev() {
            if curr_cha >= length {
                break;
            }
            string_parts[(i + 1) as usize].push(chars[curr_cha]);
            curr_cha += 1;
        }
        if curr_cha >= length {
            break;
        }
    }
    string_parts.join("")
}
}