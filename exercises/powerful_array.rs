pub fn powerful_array(arr: &Vec<u32>, queries: &mut Vec<(usize, usize)>) {
    let n = arr.len();

    let mut map = HashMap::new();
    let mut cur_l = 0;
    let mut cur_r = 0;

    // mo's algorithm 

    let mut sorted_queries: Vec<_> = queries.iter().cloned().collect();
    let mut permutation: Vec<usize> = (0..queries.len()).collect();

    let sqrt_n = (n as f64).sqrt() as usize + 1;
    sorted_queries.sort_by_key(|&(l, r)| (l / sqrt_n, r));
    permutation.sort_by_key(|&i| (queries[i].0 / sqrt_n, queries[i].1));

    // execution

    let mut answers = vec![];
    let mut power = 0;

    for query in queries {
     
        // closure must be defined inside the loop...
        let mut edit = |idx: usize, incr: bool| {
            if incr {
                if map.contains_key(&arr[idx]) {
                    let prev_occ = map.get_mut(&arr[idx]).unwrap();
                    power -= &arr[idx] * (*prev_occ) * (*prev_occ);
                    *prev_occ += 1;
                    power += &arr[idx] * (*prev_occ) * (*prev_occ);
                } else {
                    map.insert(&arr[idx], 1);
                    power += (&arr[idx] * 1 * 1);
                }
            } else {
                let prev_occ = map.get_mut(&arr[idx]).unwrap();
                power -= &arr[idx] * (*prev_occ) * (*prev_occ);
                *prev_occ -= 1;
                power += &arr[idx] * (*prev_occ) * (*prev_occ);
            }
        };

        let (mut l, mut r) = query;

        l -= 1; // queries are 1-indexed

        while cur_l < l {
            edit(cur_l, false);
            cur_l += 1;
        }

        while cur_l > l {
            cur_l -= 1;
            edit(cur_l, true);
        }

        while cur_r < r {
            edit(cur_r, true);
            cur_r += 1;
        }

        while cur_r > r {
            cur_r -= 1;
            edit(cur_r, false);
        }

        answers.push(power);
    }

    let mut permuted_answers = vec![0; answers.len()];
    for (i, answer) in permutation.into_iter().zip(answers) {
        permuted_answers[i] = answer;
    }

    println!("{:?}", permuted_answers);
}
