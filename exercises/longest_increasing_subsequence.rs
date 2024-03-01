pub fn longest_increasing_subsequence(seq: Vec<i32>) -> usize {
    let mut v = vec![0];

    for i in 1..seq.len() {
        if seq[i] > seq[v[v.len() - 1]] {
            v.push(i);
            continue;
        }
        if seq[i] < seq[v[0]] {
            v[0] = i;
            continue;
        }

        // bin search
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

    v.len()
}

