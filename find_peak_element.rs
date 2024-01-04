fn find_peak_element(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    if nums.len() == 1 { 
        return 0;
    }
    if nums[0] > nums[1] { 
        return 0;
    }
    if nums[n-1] > nums[n-2] { 
        return (n-1) as i32;
    }
		
	  let mut start = 1;
    let mut end = n-2;
        
    while start <= end {
        let mid = start + (end - start)/2;
        if nums[mid] > nums[mid-1] && nums[mid] > nums[mid+1] {
            return mid as i32;
        }
        else if nums[mid] < nums[mid-1] {
            end = mid - 1;
        }
        else if nums[mid] < nums[mid+1] {
            start = mid + 1;
        }
    }

    -1
}
