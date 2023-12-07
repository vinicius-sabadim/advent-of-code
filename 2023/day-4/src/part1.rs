use crate::utils;

pub fn execute() -> usize {
    if let Ok(lines) = utils::read_lines("./src/input.txt") {
        let sum_of_possible_game_ids: usize = lines
            .filter_map(|line| line.ok())
            .map(|line| {
                let game: Vec<&str> = line.splitn(2, ':').collect();

                if let Some(card) = game.get(1) {
                    let mut card_parts = card.trim().splitn(2, '|').map(|s| s.trim());

                    if let (Some(winning_numbers), Some(my_numbers)) = (card_parts.next(), card_parts.next()) {
                        let winning_numbers: Vec<&str> = winning_numbers.split_whitespace().collect();
                        let my_numbers: Vec<&str> = my_numbers.split_whitespace().collect();

                        let number_of_matches = utils::get_number_of_matches(winning_numbers, my_numbers);
                        calculate_points(number_of_matches)
                        
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
            .sum();

        return sum_of_possible_game_ids;
    }

    0
}

fn calculate_points(number_of_matches: usize) -> usize {
    if number_of_matches == 0 { return 0; }

    (2..=number_of_matches).fold(1, |acc, _| acc * 2)
}