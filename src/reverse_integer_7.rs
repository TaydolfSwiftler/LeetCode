impl Solution {
pub fn reverse(x: i32) -> i32 {
    const RADIX:i64 = 10;
    let mut input = x as i64;
    let mut rev: i64 = 0;
    while input != 0 {
        rev = rev * RADIX + input % RADIX;
        input /= 10;
    }
    if rev > i32::MAX as i64 || rev < i32::MIN as i64 {
        return 0;
    }
    rev as i32
}
}
