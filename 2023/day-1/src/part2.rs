use crate::utils;

pub fn execute() -> u32 {
    if let Ok(lines) = utils::read_lines("./src/input.txt") {
        let sum: u32 = lines
            .filter_map(|line| line.ok())
            .map(|ip| {
                let new_line = replace_words_with_number(&ip);
                let matched_integers = utils::get_first_and_last_integers(&new_line);
                utils::get_sum_of_integers(matched_integers)
            })
            .fold(0, |acc, val| acc + val);

        return sum;
    }

    0
}

fn replace_words_with_number(line: &str) -> String {
    let new_line = line
        .replace("oneight", "18")
        .replace("twone", "21")
        .replace("threeight", "38")
        .replace("fiveight", "58")
        .replace("sevenine", "79")
        .replace("eightwo", "82")
        .replace("eighthree", "83")
        .replace("nineight", "98")
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");

    return new_line
}
