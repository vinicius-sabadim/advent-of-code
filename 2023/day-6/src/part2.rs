use crate::utils;

pub fn execute() -> usize {
    if let Ok(lines) = utils::read_lines("./resources/input.txt") {
        let content = lines
            .filter_map(|line| line.ok())
            .collect::<Vec<String>>();

        let (time, distance) = get_time_and_distance(content);

        return utils::get_number_of_ways_to_beat_record(time, distance);
    }

    0
}

fn get_time_and_distance(content: Vec<String>) -> (u64, u64) {
    let combined_time: String = content[0]
        .split_whitespace()
        .skip(1) // Skip the "Time:" label
        .collect();

    let combined_distance: String = content[1]
        .split_whitespace()
        .skip(1) // Skip the "Time:" label
        .collect();

    let time: u64 = combined_time.parse().unwrap_or_default();
    let distance: u64 = combined_distance.parse().unwrap_or_default();

    (time, distance)
}
