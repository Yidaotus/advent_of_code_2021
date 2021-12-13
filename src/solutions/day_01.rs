// This is the better solution
// Since the input is sorted the
#[allow(dead_code)]
pub fn calculate_depth_b_optimized(measurements: Vec<u64>) -> u64 {
    let number_of_increases = measurements
        .windows(3)
        .map(|window| window.into_iter().sum())
        .collect::<Vec<u64>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count();
    return number_of_increases as u64;
}

// This is the primitive solution
#[allow(dead_code)]
pub fn calculate_depth_b_iterative(measurements: Vec<u64>) -> u64 {
    let mut number_of_increases = 0;
    let mut sums: Vec<u64> = Vec::new();
    // - 3 because our window size is 3
    // + 1 because the range is exclusive, but we use it as a starting index
    // which is inclusive
    for index in 0..measurements.len() - 3 + 1 {
        let current_window = &measurements[index..(index + 3)];
        let window_sum = current_window.into_iter().sum::<u64>();
        sums.push(window_sum);
    }
    for index in 1..sums.len() {
        let previous = sums[index - 1];
        let current = sums[index];
        if current > previous {
            number_of_increases += 1;
        }
    }
    return number_of_increases as u64;
}

// This is the primitive solution
#[allow(dead_code)]
pub fn calculate_depth_3(measurements: Vec<u64>) -> u64 {
    let number_of_increases = measurements
        .windows(3)
        .map(|window| window.into_iter().sum())
        .collect::<Vec<u64>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count();
    return number_of_increases as u64;
}

// This is the best solution I have found on GitHub
#[allow(dead_code)]
pub fn calculate_depth_windows(measurements: Vec<u64>) -> u64 {
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

#[cfg(test)]
mod tests {
    use crate::solutions::day_01::{
        calculate_depth, calculate_depth_3, calculate_depth_b_iterative, calculate_depth_windows,
    };

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

    #[test]
    fn test_known_answer_b_iterative() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(calculate_depth_b_iterative(input), 5);
    }
}
