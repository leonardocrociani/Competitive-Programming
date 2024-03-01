use std::cmp::min;

pub fn min_jumps(arr: &Vec<i32>) -> i32 {
    let mut num_jumps = vec![i32::MAX; arr.len()];
    num_jumps[0] = 0;

    for i in 1..arr.len() {
        for j in 0..i {
            // can I reach i from position j?
            if arr[j] >= i as i32 - j as i32 {
                num_jumps[i] = min(num_jumps[i], num_jumps[j] + 1)
            }
        }
    }

    num_jumps[arr.len() - 1]
}

