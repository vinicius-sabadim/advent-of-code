use crate::utils;

pub fn execute() -> usize {
    if let Ok(lines) = utils::read_lines("./src/input.txt") {
        let lines: Vec<String> = utils::get_content_as_string_vector(lines);
        let number_of_items = lines.len();
        let mut number_of_copies_of_each: Vec<usize> = vec![1; number_of_items];
    
        for (i, line) in lines.iter().enumerate() {
            let there_is_another_line = i + 1 < number_of_items;
            if there_is_another_line {
                let game: Vec<&str> = line.splitn(2, ':').collect();

                if let Some(card) = game.get(1) {
                    let mut card_parts = card.trim().splitn(2, '|').map(|s| s.trim());

                    if let (Some(winning_numbers), Some(my_numbers)) = (card_parts.next(), card_parts.next()) {
                        let winning_numbers: Vec<&str> = winning_numbers.split_whitespace().collect();
                        let my_numbers: Vec<&str> = my_numbers.split_whitespace().collect();

                        let number_of_matches = utils::get_number_of_matches(winning_numbers, my_numbers);
                        let current_number_of_cards = number_of_copies_of_each[i];

                        for j in 1..=number_of_matches {
                            number_of_copies_of_each[i + j] += current_number_of_cards;
                        }
                    }
                }
            }
        };

        return number_of_copies_of_each.iter().sum();
    }

    0
}