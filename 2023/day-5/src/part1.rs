use std::cmp;
use crate::utils;

pub fn execute() -> usize {
    let mut lowest_location = i64::MAX;

    if let Ok(blocks) = utils::read_blocks_from_file("./resources/input.txt") {
        let seeds: Vec<&str> = blocks[0].split_whitespace().skip(1).collect();
        let seed_to_soil_map: Vec<&str> = blocks[1].split('\n').skip(1).collect();
        let soil_to_fertilizer: Vec<&str> = blocks[2].split('\n').skip(1).collect();
        let fertilizer_to_water: Vec<&str> = blocks[3].split('\n').skip(1).collect();
        let water_to_light: Vec<&str> = blocks[4].split('\n').skip(1).collect();
        let light_to_temperature: Vec<&str> = blocks[5].split('\n').skip(1).collect();
        let temperature_to_humidity: Vec<&str> = blocks[6].split('\n').skip(1).collect();
        let humidity_to_location: Vec<&str> = blocks[7].split('\n').skip(1).collect();

        for seed in seeds {
            if let Ok(seed) = seed.parse::<i64>() {
                let soil_required = utils::get_destination(seed_to_soil_map.clone(), seed);
                let fertilizer_required = utils::get_destination(soil_to_fertilizer.clone(), soil_required);
                let water_required = utils::get_destination(fertilizer_to_water.clone(), fertilizer_required);
                let light_required = utils::get_destination(water_to_light.clone(), water_required);
                let temperature_required = utils::get_destination(light_to_temperature.clone(), light_required);
                let humidity_required = utils::get_destination(temperature_to_humidity.clone(), temperature_required);
                let location_required = utils::get_destination(humidity_to_location.clone(), humidity_required);

                // println!("Seed {}, soil {}, fertilizer {}, water {}, light {}, temperature {}, humidity {}, location {}", seed, soil_required, fertilizer_required, water_required, light_required, temperature_required, humidity_required, location_required);

                lowest_location = cmp::min(lowest_location, location_required);
            }
        }

        return lowest_location as usize
    }

    0
}


