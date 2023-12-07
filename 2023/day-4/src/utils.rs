use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_content_as_string_vector(lines: io::Lines<io::BufReader<File>>) -> Vec<String> {
    lines
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>()
}

pub fn get_number_of_matches(winning_numbers: Vec<&str>, my_numbers: Vec<&str>) -> usize {
    my_numbers.iter().filter(|&num| winning_numbers.contains(num)).count()
}