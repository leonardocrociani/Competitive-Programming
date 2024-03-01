use hands_on_2::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

fn main() {
    test_first_solution();
    test_second_solution();
}


fn test_first_solution () {
    let folder_path = "./tests/1/";

    for file_num in 0..=10 {

        let input_file_path = format!("{}input{}.txt", folder_path, file_num);
        let output_file_path = format!("{}output{}.txt", folder_path, file_num);

        if let Ok(input_file) = File::open(&input_file_path) {
            if let Ok(output_file) = &File::open(&output_file_path) {

                let reader = BufReader::new(input_file);
                let mut output_reader = BufReader::new(output_file);
                let mut output_lines = output_reader.lines();

                let mut array: Vec<i32> = Vec::new();
                let mut tree = SegmentTree::from(vec![-1]);
                let mut max_queries = 0;
                let mut array_len = 0;

                for (index, line) in reader.lines().enumerate() {
                    let line = line.unwrap();
                    let numbers: Vec<i32> = line
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    if index == 0 {
                        array_len = numbers[0] as usize;
                        max_queries = numbers[1] as usize;
                    } else if index == 1 {
                        array = numbers;
                        tree = SegmentTree::from(array);
                    } else {
                        if numbers.len() > 0 {
                            if numbers[0] == 0 {
                                tree.Update(numbers[1] as usize, numbers[2] as usize, numbers[3] as i32);
                            } 
                            else if index > array_len + 1 {
                                let result = tree.Max(numbers[1] as usize, numbers[2] as usize);
                                let expected_output = output_lines.nth((index - array_len + 1) as usize).unwrap().unwrap();
                                let expected_result: i32 = expected_output.parse().unwrap();
        
                                assert_eq!(result, expected_result);
                            }
                        }
                    }
                }
            } else {
                panic!("Errore nell'apertura del file di output: {}", output_file_path);
            }
        } else {
            panic!("Errore nell'apertura del file di input: {}", input_file_path);
        }
    }

    println!("Tests from set 1 passed!");
}

fn test_second_solution() {
    let folder_path = "./tests/2/";

    let mut segments: Vec<Vec<(usize, usize)>> = vec![];
    let mut queries: Vec<Vec<Vec<u32>>> = vec![];
    let mut results: Vec<Vec<u32>> = vec![];

    for file_num in 0..=7 {
        let input_file_path = format!("{}input{}.txt", folder_path, file_num);
        let output_file_path = format!("{}output{}.txt", folder_path, file_num);

        if let Ok(input_file) = File::open(&input_file_path) {
            if let Ok(output_file) = File::open(&output_file_path) {
                let reader = BufReader::new(input_file);
                let output_reader = BufReader::new(output_file);
                let mut output_lines = output_reader.lines();

                let mut array: Vec<i32> = Vec::new();
                let mut m = 0;
                let mut n = 0;
                let mut local_segment = vec![];
                let mut local_results = vec![];
                let mut local_queries = vec![];

                for (index, line) in reader.lines().enumerate() {
                    let line = line.unwrap();
                    let numbers: Vec<u32> = line
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    if index == 0 {
                        n = numbers[0] as usize;
                        m = numbers[1] as usize;
                    } else if index < n + 1 {
                        if numbers.len() > 0 {
                            local_segment.push((numbers[0] as usize, numbers[1] as usize));
                        }
                    } else {
                        if numbers.len() > 0 {
                            local_queries.push(vec![numbers[0], numbers[1], numbers[2]]);

                            if let Some(expected_output) = output_lines.next() {
                                let expected_result: u32 = expected_output.unwrap().parse().unwrap();
                                local_results.push(expected_result);
                            } else {
                                panic!("Not enough output lines for input file {}", file_num);
                            }
                        }
                    }
                }

                queries.push(local_queries);
                results.push(local_results);
                segments.push(local_segment);
            } else {
                panic!("Errore nell'apertura del file di output: {}", output_file_path);
            }
        } else {
            panic!("Errore nell'apertura del file di input: {}", input_file_path);
        }
    }

    for i in 0..=7 {
        let segment = segments[i].clone();
        let res = results[i].clone();
        let qs = queries[i].clone();
        let mut tree = S2Tree::from_segments(segment);

        for j in 0..qs.len() {
            let expected = res[j];
            let q = qs[j].clone();
            let actual = tree.IsThere(q[0] as usize, q[1] as usize, q[2] as i32);

            assert_eq!(expected as i32, actual);
        }
    }

    println!("Tests from set 2 passed!");
}