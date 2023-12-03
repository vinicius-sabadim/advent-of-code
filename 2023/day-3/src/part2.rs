use crate::utils;

pub fn execute() -> usize {
    if let Ok(lines) = utils::read_lines("./src/input.txt") {
        let lines: Vec<String> = utils::get_content_as_string_vector(lines);

        let mut gear_ratios = Vec::new();

        for (i, current_line) in lines.iter().enumerate() {
            let previous_line = if i > 0 { Some(&lines[i - 1]) } else { None };
            let next_line = if i < lines.len() - 1 { Some(&lines[i + 1]) } else { None };

            for start_index in utils::get_asterisk_matches_in_line(&current_line) {
                let above = has_number_in_the_right_place_in_line(previous_line, start_index);
                let current = has_number_in_the_right_place_in_line(Some(&current_line), start_index);
                let below = has_number_in_the_right_place_in_line(next_line, start_index);

                let total_count = [&above, &current, &below]
                    .iter()
                    .map(|vec| vec.len())
                    .sum::<usize>();

                if total_count == 2 {
                    let gear_ratio = [above, current, below]
                        .iter()
                        .filter(|vec| !vec.is_empty())
                        .flat_map(|vec| vec)
                        .product::<usize>();

                    gear_ratios.push(gear_ratio);
                }
            }
        }

        return gear_ratios.iter().sum();
    } else {
        0
    }
}

fn has_number_in_the_right_place_in_line(line: Option<&String>, start_index: usize) -> Vec<usize> {
    let numbers = line.map(|l| utils::get_number_matches_in_line(l));

    let mut numbers_found: Vec<usize> = vec![];

    if let Some(entries) = numbers {
        for (value, start, end) in entries {
            let start = if start == 0 { 0 } else { start - 1 };

            let line_length = line.as_ref().unwrap().len();
            let end = if end + 1 > line_length { line_length - 1 } else { end };

            if (start..=end).contains(&start_index) {
                numbers_found.push(value);
            }
        }
    }

    numbers_found
}