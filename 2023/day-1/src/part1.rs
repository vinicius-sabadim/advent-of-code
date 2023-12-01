use crate::utils;

pub fn execute() -> u32 {
    if let Ok(lines) = utils::read_lines("./src/input.txt") {
        let sum: u32 = lines
            .filter_map(|line| line.ok())
            .map(|ip| {
                let matched_integers = utils::get_first_and_last_integers(&ip);
                utils::get_sum_of_integers(matched_integers)
            })
            .fold(0, |acc, val| acc + val);

        return sum;
    }

    0
}