use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn load_dataset<T>(dataset: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let file = File::open(dataset).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<T>().unwrap())
        .collect()
}
