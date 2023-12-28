fn kadane_algorithm(array: &[i32]) -> &[i32] {
    let mut max_ending_here = array[0];
    let mut max_so_far = array[0];
    let mut start_index = 0;
    let mut end_index = 0;
    let mut temp_start_index = 0;

    for (i, &num) in array.iter().enumerate().skip(1) {
        if num > max_ending_here + num {
            max_ending_here = num;
            temp_start_index = i;
        } else {
            max_ending_here += num;
        }

        if max_ending_here > max_so_far {
            start_index = temp_start_index;
            end_index = i;
            max_so_far = max_ending_here;
        }
    }

    &array[start_index..=end_index]
}

fn main() {
    let array = vec![4,3,-10,3,-1,2,0,-3,5,7,-4,-8,-10,4,7,-30,-2,-6,4,7];
    let max_subarray = kadane_algorithm(&array);
    println!("Max_Array: {:?}; Sum: {}", max_subarray, max_subarray.iter().fold(0, |x,y| x+y));
}