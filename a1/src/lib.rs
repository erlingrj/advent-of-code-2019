use std::fs;
use std::path::Path;

pub fn read_integer_file<P: AsRef<Path>>(file: P, sep: &str) -> Vec<i32> {
    fs::read_to_string(file).unwrap().split(sep).map(|line| line.parse::<i32>().unwrap()).collect()
}