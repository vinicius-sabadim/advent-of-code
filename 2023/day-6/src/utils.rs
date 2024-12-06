use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_number_of_ways_to_beat_record(time: u64, distance: u64) -> usize {
    let mut can_beat_record = 0;

    for speed in 1..time {
        let available_time = time - speed;
        if available_time * speed > distance {
            can_beat_record += 1;
        }
    }

    can_beat_record
}

#[test]
fn find_options_for_time_7_and_distance_9() {
    let time = 7;
    let distance = 9;

    let number_of_ways_to_beat_record = get_number_of_ways_to_beat_record(time, distance);

    assert_eq!(number_of_ways_to_beat_record, 4);
}

#[test]
fn find_options_for_time_30_and_distance_200() {
    let time = 30;
    let distance = 200;

    let number_of_ways_to_beat_record = get_number_of_ways_to_beat_record(time, distance);

    assert_eq!(number_of_ways_to_beat_record, 9);
}
