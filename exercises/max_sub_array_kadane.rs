pub fn max_subarray(array: &Vec<i32>) -> Vec<i32> {
    let n = array.len();

    if n == 1 {
        return vec![array[0]];
    }

    let mut max_subarr = vec![array[0]];
    let mut max_sum = array[0];
    let mut max_global_sum = max_sum;
    let mut max_global_subarr = vec![array[0]];

    for i in 1..array.len() {
        if array[i] > (max_sum + array[i]) {
            max_sum = array[i];
            max_subarr = vec![array[i]];
        } else {
            max_sum += array[i];
            max_subarr.push(array[i]);
        }
        if max_sum > max_global_sum {
            max_global_sum = max_sum;
            max_global_subarr = max_subarr.clone();
        }
    }

    max_global_subarr
}

