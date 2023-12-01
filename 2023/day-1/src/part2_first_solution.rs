use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn execute() -> u32 {
    if let Ok(lines) = read_lines("./src/input.txt") {
        let mut sum = 0;

        for line in lines {
            if let Ok(ip) = line {
                let mut matched_integers: Vec<u32> = Vec::new();
        
                for (i, current_char) in ip.char_indices() {
                    let substring = &ip[i..];
                    if current_char.is_digit(10) {
                        matched_integers.push(current_char.to_digit(10).unwrap());
                    } else {
                        let (result, word) = is_text_a_number_match(substring);
                        if result {
                            matched_integers.push(convert_word_to_number(word));
                        }
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

        return sum;
    }

    0
}

fn is_text_a_number_match(substring: &str) -> (bool, &str) {
    let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    
    for word in &words {
        if substring.starts_with(word) {
            return (true, word)
        }
    }

    (false, "")
}

fn convert_word_to_number(word: &str) -> u32 {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}