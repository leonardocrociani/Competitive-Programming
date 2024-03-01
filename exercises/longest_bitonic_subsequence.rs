use std::cmp::max;

pub fn longest_bitonic_subsequence(seq: Vec<i32>) -> usize {
    let _lis = lis(&seq);

    let mut cl = seq.clone();
    cl.reverse();
    let _lds = lis(&cl);

    let n = seq.len();
    let mut _max = 0;

    for i in 0..n {
        _max = max(_max, _lis[i] + _lds[n - 1 - i] - 1);
    }

    _max
}

fn lis(seq: &Vec<i32>) -> Vec<usize> {
    let mut v = vec![0];
    let mut lis = vec![1];

    for i in 1..seq.len() {
        if seq[i] > seq[v[v.len() - 1]] {
            v.push(i);
        } else if seq[i] < seq[v[0]] {
            v[0] = i;
        } else {
            let mut l = 0;
            let mut r = v.len();

            while l < r {
                let mid = l + (r - l) / 2;

                if seq[v[mid]] > seq[i] {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }

            v[l] = i;
        }

        lis.push(v.len())
    }

    lis
}
