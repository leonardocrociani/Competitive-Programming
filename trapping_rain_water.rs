fn trapping_rain_water (array:&Vec<i32>) -> i32 {
    let width = array.len();
    let height = *array.iter().max().unwrap();
    let mut sum = 0;

    for i in 0..height {
        for j in 0..width {
            if array[j] <= i {

                let mut left_present = false;
                let mut right_present = false;

                for k in (0..j).rev() {
                    if array[k] > i {
                        left_present = true;
                        break;
                    }
                }

                for k in (j+1)..width {
                    if array[k] > i {
                        right_present = true;
                        break;
                    }
                }

                if left_present && right_present {
                    sum = sum + 1;
                }
            }
        }
    }

    return sum;
}

fn main () {
    let array = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    println!("Trapped water cells: {}", trapping_rain_water(&array));
}