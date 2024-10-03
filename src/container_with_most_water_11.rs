use std::cmp;

//Working but too slow of a Solution
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area: i32 = 0;
    let mut curr_area: i32;
    for i in 0..height.len() {
        for j in i..height.len() {
            curr_area = cmp::min(height[i], height[j]) * (j - i) as i32;
            if curr_area > max_area {
                max_area = curr_area;
            }
        }
    }
    max_area
}

//This Solution uses the fact that the array length has to be 10_000 or less;
pub fn max_area_improved(height: Vec<i32>) -> i32 {
    let mut max_area: i32 = 0;
    let mut curr_area: i32;
    let arr_length = height.len();
    let mut end_distance = arr_length as i32 - 5000;
    if end_distance < 0 {
        end_distance = 0;
    }
    for i in 0..cmp::min(height.len(), 5000) {
        for j in (cmp::max(i, end_distance as usize)..height.len()).rev() {
            curr_area = cmp::min(height[i], height[j]) * (j - i) as i32;
            if curr_area > max_area {
                max_area = curr_area;
            }
        }
    }
    max_area
}
