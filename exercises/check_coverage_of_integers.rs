pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {

    let max = ranges.iter().max_by(|a, b| a[1].cmp(&b[1])).unwrap()[1];
    let mut line = vec![0; max as usize + 2];

    for range in ranges {
        line[range[0] as usize] += 1;
        line[range[1] as usize + 1] -= 1;
    }

    let mut overlaps = 0;

    for i in 1..right as usize {
        overlaps += line[i];
        if i >= left as usize && overlaps == 0 {
            return false;
        }
    }

    return true
}

