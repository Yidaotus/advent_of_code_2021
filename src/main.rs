mod utility;
mod solutions;

fn main() {
    let dataset = utility::load_dataset::<u64>("input.txt");
    let result = solutions::day_01::calculate_depth_3_optimized(dataset);
    println!("Result: {}", result);
}
