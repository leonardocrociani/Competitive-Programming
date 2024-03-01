use hands_on_3::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

fn main() {
    test_first_solution();
    test_second_solution();
}

fn test_first_solution () {
    let number_of_tests = 5;
    let folder_path = "./tests/1/";

    for file_num in 0..number_of_tests {

        let input_file_path = format!("{}input{}.txt", folder_path, file_num);
        let output_file_path = format!("{}output{}.txt", folder_path, file_num);

        if let Ok(input_file) = File::open(&input_file_path) {
            if let Ok(output_file) = &File::open(&output_file_path) {

                let reader = BufReader::new(input_file);
                let mut output_reader = BufReader::new(output_file);
                let mut output_lines = output_reader.lines();

                let mut D = 0;
                let mut matrix :Vec<Vec<u32>> = vec![];

                for (index, line) in reader.lines().enumerate() {
                    let line = line.unwrap();
                    let numbers: Vec<u32> = line.trim()
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    if index == 0 {
                        D = numbers[1] as usize;
                    } else {
                        if numbers.len() > 0 {
                            matrix.push(numbers);
                        }
                    }
                }

                let expected_result: u32 = output_lines.nth(0).unwrap().unwrap().parse().unwrap();
                assert_eq!(plan_holiday(matrix, D), expected_result);

            } else {
                panic!("Errore nell'apertura del file di output: {}", output_file_path);
            }
        } else {
            panic!("Errore nell'apertura del file di input: {}", input_file_path);
        }
    }

    println!("Tests from set 1 passed!");
}

fn test_second_solution () {
    let number_of_tests = 11;
    let folder_path = "./tests/2/";

    for file_num in 0..number_of_tests {

        let input_file_path = format!("{}input{}.txt", folder_path, file_num);
        let output_file_path = format!("{}output{}.txt", folder_path, file_num);

        if let Ok(input_file) = File::open(&input_file_path) {
            if let Ok(output_file) = &File::open(&output_file_path) {

                let reader = BufReader::new(input_file);
                let mut output_reader = BufReader::new(output_file);
                let mut output_lines = output_reader.lines();

                let mut n = 0;
                let mut topics :Vec<(i32,i32)> = vec![];

                for (index, line) in reader.lines().enumerate() {
                    let line = line.unwrap();
                    let numbers: Vec<u32> = line.trim()
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    if index == 0 {
                        n = numbers[0] as usize;
                    } else {
                        if numbers.len() > 0{
                            topics.push((numbers[0] as i32, numbers[1] as i32));
                        }
                    }
                }

                let expected_result: usize = output_lines.nth(0).unwrap().unwrap().parse().unwrap();
                assert_eq!(design_course(topics), expected_result);

            } else {
                panic!("Errore nell'apertura del file di output: {}", output_file_path);
            }
        } else {
            panic!("Errore nell'apertura del file di input: {}", input_file_path);
        }
    }

    println!("Tests from set 2 passed!");
}