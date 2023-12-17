use std::num::ParseIntError;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct Range {
    source_start: i64,
    source_end: i64,
    destination_start: i64,
    destination_end: i64
}

pub fn read_blocks_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut blocks = Vec::new();
    let mut current_block = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            if !current_block.is_empty() {
                blocks.push(current_block.trim().to_string());
                current_block.clear();
            }
        } else {
            current_block.push_str(&line);
            current_block.push('\n');
        }
    }

    if !current_block.is_empty() {
        blocks.push(current_block.trim().to_string());
    }

    Ok(blocks)
}

pub fn get_range(value: &str) -> Result<Range, ParseIntError> {
    let parts: Vec<&str> = value.split_whitespace().collect();
    let destination = parts[0].parse::<i64>()?;
    let source = parts[1].parse::<i64>()?;
    let length = parts[2].parse::<i64>()?;

    Ok(Range {
        source_start: source,
        source_end: source + length - 1,
        destination_start: destination,
        destination_end: destination + length - 1,
    })
}

pub fn find_range_that_includes_the_source(values: Vec<&str>, source: i64) -> Option<Range> {
    for value in values {
        if let Ok(range) = get_range(&value) {
            if source >= range.source_start && source <= range.source_end {
                return Some(range);
            }
        }
    }
    None
}

pub fn find_range_that_includes_the_destination(values: Vec<&str>, destination: i64) -> Option<Range> {
    for value in values {
        if let Ok(range) = get_range(&value) {
            if destination >= range.destination_start && destination <= range.destination_end {
                return Some(range);
            }
        }
    }
    None
}

pub fn get_destination_for_source(result: Option<Range>, source: i64) -> i64 {
    match result {
        Some(range) => (range.destination_start) + (source - range.source_start),
        None    => source
    }
}

pub fn get_source_for_destination(result: Option<Range>, destination: i64) -> i64 {
    match result {
        Some(range) => (range.source_start) + (destination - range.destination_start),
        None    => destination
    }
}

pub fn get_destination(almanac_map: Vec<&str>, source: i64) -> i64 {
    let result = find_range_that_includes_the_source(almanac_map, source);
    get_destination_for_source(result, source)
}

pub fn get_source(almanac_map: Vec<&str>, destination: i64) -> i64 {
    let result = find_range_that_includes_the_destination(almanac_map, destination);
    get_source_for_destination(result, destination)
}

#[test]
fn first_range() {
    let value = "50 98 2";
    let range = get_range(value).unwrap();
    assert_eq!(range.destination_start, 50);
    assert_eq!(range.source_start, 98);
    assert_eq!(range.source_end, 99);
}

#[test]
fn second_range() {
    let value = "52 50 48";
    let range = get_range(value).unwrap();
    assert_eq!(range.destination_start, 52);
    assert_eq!(range.source_start, 50);
    assert_eq!(range.source_end, 97);
}

#[test]
fn find_range_with_source_found() {
    let values = vec!["50 98 2", "52 50 48"];
    let source = 53;

    let result = find_range_that_includes_the_source(values, source);
    assert!(result.is_some());

    let range = result.unwrap();
    assert!(source >= range.source_start && source <= range.source_end);
}

#[test]
fn find_range_with_source_not_found() {
    let values = vec!["50 98 2", "52 50 48"];
    let source = 30;

    let result = find_range_that_includes_the_source(values, source);
    assert!(result.is_none());
}

#[test]
fn find_range_with_destination_found() {
    let values = vec!["50 98 2", "52 50 48"];
    let destination = 53;

    let result = find_range_that_includes_the_destination(values, destination);
    assert!(result.is_some());

    let range = result.unwrap();
    assert!(destination >= range.destination_start && destination <= range.destination_end);
}

#[test]
fn find_range_with_destination_not_found() {
    let values = vec!["50 98 2", "52 50 48"];
    let destination = 30;

    let result = find_range_that_includes_the_destination(values, destination);
    assert!(result.is_none());
}

#[test]
fn get_destination_for_source_found() {
    let values = vec!["50 98 2", "52 50 48"];
    let source = 53;

    let result = find_range_that_includes_the_source(values, source);
    let destination = get_destination_for_source(result, source);
    assert_eq!(destination, 55);
}

#[test]
fn get_destination_for_source_not_found() {
    let values = vec!["50 98 2", "52 50 48"];
    let source = 30;

    let result = find_range_that_includes_the_source(values, source);
    let destination = get_destination_for_source(result, source);
    assert_eq!(destination, 30);
}
