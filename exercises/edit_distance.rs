use std::cmp::min;

pub fn edit_distance(word1: String, word2: String) -> i32 {
    let len1 = word1.chars().count();
    let len2 = word2.chars().count();

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        for j in 0..=len2 {
            if i == 0 {
                matrix[i][j] = j as i32;
            } else if j == 0 {
                matrix[i][j] = i as i32;
            } else if word1.chars().nth(i - 1) == word2.chars().nth(j - 1) {
                matrix[i][j] = matrix[i - 1][j - 1];
            } else {
                matrix[i][j] = 1 + min(
                    min(matrix[i][j - 1], matrix[i - 1][j]),
                    matrix[i - 1][j - 1],
                );
            }
        }
    }

    matrix[len1][len2]
}

