pub fn longest_common_subsequence(s1: &str, s2: &str) -> i32 {
    let mut lcs = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1.chars().nth(i-1) == s2.chars().nth(j-1) {
                lcs[i][j] = 1 + lcs[i - 1][j - 1];
            } else {
                let lcs_i = lcs[i - 1][j];
                let lcs_j = lcs[i][j - 1];

                lcs[i][j] = max(lcs_i, lcs_j);
            }
        }
    }

    return lcs[s1.len()][s2.len()];
}
