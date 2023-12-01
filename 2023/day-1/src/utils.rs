use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_first_and_last_integers(line: &str) -> Vec<u32> {
    let pattern = Regex::new(r"(\d)").unwrap();

    pattern
        .find_iter(line)
        .filter_map(|mat| mat.as_str().parse().ok())
        .collect()
}

pub fn get_sum_of_integers(matched_integers: Vec<u32>) -> u32 {
    matched_integers.get(0).copied().unwrap_or(0) * 10
        + matched_integers.last().unwrap_or(&matched_integers[0])
}