use crate::utils;

pub fn execute() -> usize {
    if let Ok(lines) = utils::read_lines("./src/input.txt") {
        let sum_of_possible_game_ids: usize = lines
            .filter_map(|line| line.ok())
            .enumerate()
            .filter_map(|(i, line)| {
                let game_records = utils::get_game_records(&line);
                let game_records_individually = utils::get_game_records_individually(game_records);
                let game_id = i + 1;

                is_possible(game_records_individually.clone()).then(|| game_id)
        })
        .sum();

        return sum_of_possible_game_ids;
    }

    0
}

fn is_possible(entries: Vec<String>) -> bool {
    let (red_over_12, green_over_13, blue_over_14) = entries.iter().fold(
        (false, false, false),
        |(red, green, blue), entry| {
            let parts: Vec<&str> = entry.split_whitespace().collect();
            let color = parts[1];
            if let Ok(value) = parts[0].parse::<usize>() {
                match color {
                    "red" => (red || value > 12, green, blue),
                    "green" => (red, green || value > 13, blue),
                    "blue" => (red, green, blue || value > 14),
                    _ => (red, green, blue),
                }
            } else {
                (red, green, blue)
            }
        },
    );

    !(red_over_12 || green_over_13 || blue_over_14)
}