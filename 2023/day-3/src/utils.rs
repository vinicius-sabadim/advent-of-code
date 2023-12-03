use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

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

pub fn get_number_matches_in_line(line: &String) -> Vec<(usize, usize, usize)> {
    let digit_pattern = Regex::new(r"\d+").unwrap();

    digit_pattern
        .find_iter(line)
        .filter_map(|m| Some((m.as_str().parse().ok()?, m.start(), m.end())))
        .collect()
}

pub fn get_asterisk_matches_in_line(line: &String) -> Vec<usize> {
    let asterisk_pattern = Regex::new(r"\*").unwrap();

    asterisk_pattern
        .find_iter(line)
        .filter_map(|m| Some(m.start()))
        .collect()
}
