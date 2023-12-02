use crate::utils;

pub fn execute() -> usize {
    if let Ok(lines) = utils::read_lines("./src/input.txt") {
        let product_of_minimum_cubes_necessary: usize = lines
            .filter_map(|line| line.ok())
            .map(|line| {
                let game_records = utils::get_game_records(&line);
                let game_records_individually = utils::get_game_records_individually(game_records);
                let (min_red, min_green, min_blue) = get_minimum_cubes_necessary(game_records_individually.clone());

                min_red * min_green * min_blue
        })
        .sum();

        return product_of_minimum_cubes_necessary;
    }

    0
}

fn get_minimum_cubes_necessary(entries: Vec<String>) -> (usize, usize, usize) {
    let mut min_red: usize = 0;
    let mut min_green: usize = 0;
    let mut min_blue: usize = 0;

    for entry in &entries {
        let parts: Vec<&str> = entry.split_whitespace().collect();
        let color = parts[1];
        if let Ok(value) = parts[0].parse::<usize>() {
            match color {
                "red" => min_red = min_red.max(value),
                "green" => min_green = min_green.max(value),
                "blue" => min_blue = min_blue.max(value),
                _ => (),
            }
        }
    }

    (min_red, min_green, min_blue)
}