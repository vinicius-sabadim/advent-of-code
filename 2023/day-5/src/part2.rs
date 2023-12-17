use crate::utils;

pub fn execute() -> usize {
    if let Ok(blocks) = utils::read_blocks_from_file("./resources/input.txt") {
        let seeds: Vec<&str> = blocks[0].split_whitespace().skip(1).collect();
        let seed_to_soil_map: Vec<&str> = blocks[1].split('\n').skip(1).collect();
        let soil_to_fertilizer: Vec<&str> = blocks[2].split('\n').skip(1).collect();
        let fertilizer_to_water: Vec<&str> = blocks[3].split('\n').skip(1).collect();
        let water_to_light: Vec<&str> = blocks[4].split('\n').skip(1).collect();
        let light_to_temperature: Vec<&str> = blocks[5].split('\n').skip(1).collect();
        let temperature_to_humidity: Vec<&str> = blocks[6].split('\n').skip(1).collect();
        let humidity_to_location: Vec<&str> = blocks[7].split('\n').skip(1).collect();

        let mut location = 1;

        loop {
            let humidity_required = utils::get_source(humidity_to_location.clone(), location);
            let temperature_required = utils::get_source(temperature_to_humidity.clone(), humidity_required);
            let light_required = utils::get_source(light_to_temperature.clone(), temperature_required);
            let water_required = utils::get_source(water_to_light.clone(), light_required);
            let fertilizer_required = utils::get_source(fertilizer_to_water.clone(), water_required);
            let soil_required = utils::get_source(soil_to_fertilizer.clone(), fertilizer_required);
            let seed_required = utils::get_source(seed_to_soil_map.clone(), soil_required);
            let there_is_the_seed = is_seed_available_for_the_soil(seeds.clone(), seed_required);

            if there_is_the_seed {
                break
            }

            location = location + 1;
        }

        return location as usize
    }

    0
}

fn is_seed_available_for_the_soil(seeds: Vec<&str>, seed: i64) -> bool {
    seeds
        .chunks(2)
        .map(|chunk| (chunk[0].parse::<i64>(), chunk[1].parse::<i64>()))
        .any(|(result1, result2)| match (result1, result2) {
            (Ok(num1), Ok(num2)) => seed >= num1 && seed <= num1 + num2,
            _ => false,
        })
}