use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_game_records(line: &str) -> String {
    let pattern = Regex::new(r"Game \d+: ").unwrap();

    pattern.replace_all(line, "").to_string()
}

pub fn get_game_records_individually(records: String) -> Vec<String> {
    records
        .split(';')
        .flat_map(|part| part.split(',').map(|game| game.trim().to_string()))
        .collect()
}
