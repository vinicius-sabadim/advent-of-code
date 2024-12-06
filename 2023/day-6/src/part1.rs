use crate::utils;

pub fn execute() -> usize {
    if let Ok(lines) = utils::read_lines("./resources/input.txt") {
        let content = lines
            .filter_map(|line| line.ok())
            .collect::<Vec<String>>();

        let (time, distance) = get_all_times_and_distances(content);

        let mut result = 1;
        for i in 0..time.len() {
            let number_of_ways_to_beat_record = utils::get_number_of_ways_to_beat_record(time[i], distance[i]);
            result *= number_of_ways_to_beat_record;
        }

        return result;
    }

    0
}

fn get_all_times_and_distances(content: Vec<String>) -> (Vec<u64>, Vec<u64>) {
    let time: Vec<u64> = content[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let distance: Vec<u64> = content[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    (time, distance)
}

