use core::panic;
fn print_seedrange(maps: &Vec<Vec<Vec<u64>>>) {
    for m in maps {
        for n in m {
            println!("start:{} end:{}", n[1], n[1] + n[2] - 1);
        }
    }
}

fn minimum_location(maps: &Vec<Vec<Vec<u64>>>, seeds: Vec<u64>) -> u64 {
    let mut locations = Vec::new();
    for seed in seeds {
        locations.push(location_of_seed(maps, seed));
    }
    *locations.iter().min().unwrap()
}

fn location_of_seed(maps: &Vec<Vec<Vec<u64>>>, mut seed: u64) -> u64 {
    // println!("looking for seed {}", seed);
    let start = seed;
    for (i, m) in maps.iter().enumerate() {
        // m.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut mapping = None;
        for row in m {
            let source = row[1];
            let dest = row[0];
            let len = row[2];
            if seed >= source && seed < source + len {
                mapping = Some(dest + (seed - source));
                // println!(
                //     "found mapping for {} its {}, in map {} ",
                //     seed,
                //     mapping.unwrap(),
                //     i
                // );
                break;
            }
        }
        if mapping.is_some() {
            seed = mapping.unwrap();
        }
    }
    // println!("mapping for seed {} is {} ", start, seed);
    seed
}

fn location_to_seed(inverted_maps: &Vec<Vec<Vec<u64>>>, mut location: u64) -> u64 {
    //println!("looking for location {}", location);
    let _start = location;
    for (_i, m) in inverted_maps.iter().enumerate() {
        // m.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut mapping = None;
        for row in m {
            let source = row[0];
            let dest = row[1];
            let len = row[2];
            if location >= source && location < source + len {
                mapping = Some(dest + (location - source));
                // println!(
                //     "found mapping for {} its {}, in map {} ",
                //     seed,
                //     mapping.unwrap(),
                //     i
                // );
                break;
            }
        }
        if mapping.is_some() {
            location = mapping.unwrap();
        }
    }
    // println!("mapping for seed {} is {} ", start, seed);
    location
}
fn main() {
    let seeds = include_str!("../../inputs/day05seeds.txt");
    let seeds = seeds
        .split_whitespace()
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut seed_to_soil = include_str!("../../inputs/day05seed-to-soil.txt")
        .lines()
        .map(|x| {
            x.split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    seed_to_soil.sort_by(|a, b| a[1].cmp(&b[1]));

    let mut last_entry = seed_to_soil.last().unwrap();

    let mut last_source = last_entry[1];

    let soil_to_fertilizer = include_str!("../../inputs/day05soil-to-fertilizer.txt")
        .lines()
        .map(|x| {
            x.split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let fertilizer_to_water = include_str!("../../inputs/day05fertilizer-to-water.txt")
        .lines()
        .map(|x| {
            x.split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let water_to_light = include_str!("../../inputs/day05water-to-light.txt")
        .lines()
        .map(|x| {
            x.split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let light_to_temperature = include_str!("../../inputs/day05light-to-temperature.txt")
        .lines()
        .map(|x| {
            x.split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let temperature_to_humidity = include_str!("../../inputs/day05temperature-to-humidity.txt")
        .lines()
        .map(|x| {
            x.split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let humidity_to_location = include_str!("../../inputs/day05humidity-to-location.txt")
        .lines()
        .map(|x| {
            x.split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let maps = vec![
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ];

    let maps = maps
        .iter()
        .cloned()
        .map(|mut x| {
            x.sort_by(|a, b| a[1].cmp(&b[1]));
            x
        })
        .collect::<Vec<_>>();

    let inverted_maps = maps
        .iter()
        .cloned()
        .map(|mut x| {
            x.sort_by(|a, b| a[0].cmp(&b[0]));
            x
        })
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<_>>();

    let min = minimum_location(&maps, seeds.clone());
    //let min_seedpairs = minimum_location_seedpairs(&maps, seeds);
    //let min = minimum_seedpairs(&maps, &seeds.clone());
    println!("part 1 min location: {}", min);
    //println!("min location seedpairs: {}", min_seedpairs);
    // print_seedrange(&maps);

    // solution is to invert the mappings so we can go from location to seed,
    // then then check if the seed is in our seed/len pairs
    //
    let mut min = None;
    'finished: for i in 0.. {
        let seed = location_to_seed(&inverted_maps, i);
        for s in seeds.chunks_exact(2) {
            let start_seed = s[0];
            let end_seed = s[0] + s[1] - 1;
            if seed >= start_seed && seed <= end_seed {
                println!("part2 minimum location {}", i);
                min = Some(i);
                break 'finished;
            }
        }
    }
    if min.is_none() {
        panic!("no min found");
    }
}
