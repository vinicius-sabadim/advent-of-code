use crate::utils;

pub fn execute() -> usize {
    if let Ok(lines) = utils::read_lines("./src/input.txt") {
        let lines: Vec<String> = utils::get_content_as_string_vector(lines);

        lines
            .iter()
            .enumerate()
            .flat_map(|(i, current_line)| {
                let previous_line = if i > 0 { Some(&lines[i - 1]) } else { None };
                let next_line = if i < lines.len() - 1 { Some(&lines[i + 1]) } else { None };

                utils::get_number_matches_in_line(&current_line)
                    .into_iter()
                    .filter_map(move |(matched_value, start_index, end_index)| {
                        let above = has_symbol_in_the_right_place_in_line(previous_line, start_index, end_index);
                        let current = has_symbol_in_the_right_place_in_line(Some(&current_line), start_index, end_index);
                        let below = has_symbol_in_the_right_place_in_line(next_line, start_index, end_index);

                        if above || current || below {
                            Some(matched_value)
                        } else {
                            None
                        }
                    })
            })
            .sum()
    } else {
        0
    }
}

fn has_symbol_in_the_right_place_in_line(line: Option<&String>, start_index: usize, end_index: usize) -> bool {
    line.map(|l| get_chars(l, start_index, end_index))
        .map_or(false, |chars| chars.chars().any(|c| !c.is_digit(10) && c != '.'))
}

fn get_chars(line: &String, start_index: usize, end_index: usize) -> &str {
    let start = if start_index == 0 { 0 } else { start_index - 1 };
    let end = if end_index + 1 > line.len() { line.len() - 1 } else { end_index };

    return &line[start..=end];
}