pub fn search_peak_element(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = nums.len();
    let n = nums.len();

    if n == 1 {
        return 0;
    }

    if n == 2 {
        return if nums[0] > nums[1] { 0 } else { 1 };
    }

    while l < r {
        let mid = l + (r - l) / 2;

        if mid == 0 && nums[0] > nums[1] {
            return 0;
        }

        if mid == n - 1 && nums[n - 1] > nums[n - 2] {
            return (n - 1) as i32;
        }

        if nums[mid] > nums[mid + 1] && nums[mid] > nums[mid - 1] {
            return mid as i32;
        } else {
            if nums[mid] < nums[mid + 1] {
                l = mid;
            } else {
                r = mid;
            }
        }
    }

    -1 as i32
}

