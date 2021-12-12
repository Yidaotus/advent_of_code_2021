use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[allow(dead_code)]
// This is the better solution
// Since the input is sorted the
fn calculate_depth_3_optimized(measurements: Vec<u64>) -> u64 {
    let number_of_increases = measurements
        .windows(3)
        .map(|window| window.into_iter().sum())
        .collect::<Vec<u64>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count();
    return number_of_increases as u64;
}

#[allow(dead_code)]
// This is the primitive solution
fn calculate_depth_3(measurements: Vec<u64>) -> u64 {
    let number_of_increases = measurements
        .windows(3)
        .map(|window| window.into_iter().sum())
        .collect::<Vec<u64>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count();
    return number_of_increases as u64;
}

#[allow(dead_code)]
// This is the best solution I have found on GitHub
fn calculate_depth_windows(measurements: Vec<u64>) -> u64 {
    let number_of_increases = measurements.windows(2).filter(|x| x[0] < x[1]).count();
    return number_of_increases as u64;
}

#[allow(dead_code)]
// This is my initial primitive solution
fn calculate_depth(measurements: Vec<u64>) -> u64 {
    let mut number_of_increases = 0;
    for index in 1..measurements.len() {
        let previous_value = measurements[index - 1];
        let current_value = measurements[index];
        if current_value > previous_value {
            number_of_increases += 1;
        }
    }
    return number_of_increases;
}

fn main() {
    let file = File::open("input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let input: Vec<u64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect();
    let result = calculate_depth_3(input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_known_answer() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(calculate_depth(input), 7);
    }

    #[test]
    fn test_known_answer_window() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(calculate_depth_windows(input), 7);
    }

    #[test]
    fn test_known_answer_b() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(calculate_depth_3(input), 5);
    }
}

