pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let mut sub_length = 0;
    let length = nums.len();
    let num_sum: i64 = nums.iter().fold(0i64, |sum, i| sum + (*i as i64));
    let remainder = num_sum % p as i64;
    if remainder == 0 {
        return 0;
    }
    let mut sub_sums_remainders = vec![0i64; length];
    loop {
        for i in 0..(length - sub_length) {
            sub_sums_remainders[i] = (sub_sums_remainders[i] + nums[i + sub_length] as i64) % p as i64;
            if sub_sums_remainders[i] == remainder {
                return (sub_length + 1) as i32
            }
        }
        sub_length += 1;
        if sub_length + 1 >= length {
            break;
        }
    }
    -1
}