use std::collections::HashMap;

// Optimal solution -> O(n) with Hasing
pub fn k_good_segment(arr: &Vec<u32>, k: usize) -> (i32, i32) {
    let mut map: HashMap<u32, u32> = HashMap::new();
    let mut diff_count = 0;
    let mut best_range = (0, 0);
    let mut l = 0;
    let mut r = 0;

    while l <= r {

        if l == arr.len() { 
            break
        }

        if diff_count > k || r == arr.len() {

            // remove el in l +  update diff_count
            let mut remove = false;
            match map.get_mut(&arr[l]) {
                Some(val) => {
                    if *val > 1 {
                        *val -= 1;
                    } else {
                        diff_count -= 1;
                        remove = true;
                    }
                }
                _ => (),
            }
            if remove {
                map.remove(&arr[l]);
            }

            l += 1;
        } else {
            // insert el in r + update diff_count
            match map.get_mut(&arr[r]) {
                Some(val) => {
                    *val += 1;
                }
                None => {
                    map.insert(arr[r], 1);
                    diff_count += 1;
                }
            }

            r += 1;
        }

        if diff_count == k {
            if r as i32 - 1 - l as i32 > best_range.1 - best_range.0 {
                best_range = (l as i32, r  as i32 - 1);
            }
        }
    }

    best_range
}