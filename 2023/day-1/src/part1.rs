use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    if let Ok(lines) = read_lines("./src/input.txt") {
        let pattern = Regex::new(r"(\d)").unwrap();
        let mut sum = 0;

        for line in lines {
            if let Ok(ip) = line {
                let mut matched_integers: Vec<i32> = Vec::new();

                for mat in pattern.find_iter(&ip) {
                    if let Ok(matched_value) = mat.as_str().parse::<i32>() {
                        matched_integers.push(matched_value);
                    }
                }

                if let Some(first) = matched_integers.first() {
                    if let Some(last) = matched_integers.last() {
                        sum += first * 10 + last;
                    } else {
                        sum += first * 10 + first;
                    }
                }
            }
        }

        println!("{}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}