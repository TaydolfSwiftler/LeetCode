pub fn my_atoi(s: String) -> i32 {
    if s.trim().len() == 0 {
        return 0;
    }
    let mut trimmed = s.trim().chars();
    let mut chars = vec![];
    let mut sign: bool = true;
    let first_char = trimmed.next().unwrap();
    if first_char == '-' {
        sign = false;
    }
    else if first_char != '+' {
        trimmed = s.trim().chars()
    }
    for x in trimmed {
        if !x.is_numeric() {
            break;
        }
        chars.push(x);
    }
    let mut new_string = chars.into_iter().collect::<String>();
    new_string = new_string.trim_start_matches('0').to_string();
    if new_string.len() == 0 {
        return 0;
    }
    if new_string.len() > 10 {
        return if sign {
            i32::MAX
        } else {
            i32::MIN
        }
    }
    let mut big = new_string.parse::<i64>().unwrap();
    if !sign {
        big = big * -1;
    }


    if big > i32::MAX as i64 {
        return i32::MAX;
    }
    if big < i32::MIN as i64 {
        return i32::MIN;
    }
    big as i32
}