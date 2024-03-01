pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut map: HashMap<u32, usize> = HashMap::new();
    let mut sum = 0;

    for i in 0..nums.len() {
        sum += nums[i];
        let res = sum % k;

        if res == 0 && i >= 1 {
            return true;
        }

        match map.get(&(res as u32)) {
            Some(idx) => {
                if i - idx > 1 {
                    return true;
                }
            }
            None => {
                map.insert(res as u32, i);
            }
        }
    }

    false
}

