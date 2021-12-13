mod utility;
mod solutions;

fn main() {
    let dataset = utility::load_dataset::<String>("input_day2.txt");
    let result = solutions::day_02::calculate_aim_path(dataset.iter().map(String::as_str).collect()).unwrap();

    println!("Result: {}", result);
}
