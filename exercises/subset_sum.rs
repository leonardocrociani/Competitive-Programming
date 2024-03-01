fn subset_sum(set: Vec<usize>, val: usize) -> bool {
    let mut start_row = vec![None; val + 1];
    start_row[0] = Some(true);

    let mut matrix = vec![start_row; set.len()];

    for j in 1..=val {
        matrix[0][j] = Some(set[0] == j);
    }

    for i in 1..set.len() {
        for j in 1..=val {
            if set[i] > j {
                matrix[i][j] = matrix[i - 1][j];
            } else {
                matrix[i][j] = Some(matrix[i - 1][j - set[i]].is_some() || matrix[i-1][j].is_some());
            }
        }
    }

    matrix[set.len() - 1][val].is_some()
}

