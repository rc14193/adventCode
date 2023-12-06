use std::fs;

fn main() {
    let file = "example_input.txt";
    let contents = fs::read_to_string(file).expect("couldn't open input file");
    let mut lines = contents.lines();
    // parse seeds
    let seed_line = lines.next().expect("Couldn't get seeds line");
    let seeds: Vec<_> = nums_line_to_vec(seed_line.clone());
    println!("seeds are {:?}", seeds);
    lines.next();
    // parse each map
    let maps: Vec<_> = lines.collect();
    let mut parsed_maps: Vec<Vec<&str>> = vec![vec![]];
    // parse file to str vecs
    let parsed_maps = maps.into_iter().fold(parsed_maps, |mut acc, val| {
        let current_map = acc.len() - 1;
        if val.contains("-") {
            return acc;
        }
        match val {
            "" => {
                acc.push(Vec::new());
            }
            _ => {
                acc[current_map].push(val);
            }
        };
        acc
    });
    // parse str vecs to num lines vecs
    let parsed_maps = parsed_maps
        .into_iter()
        .map(|row| {
            let r = row.into_iter().map(|line| nums_line_to_vec(line)).collect::<Vec<Vec<_>>>();
            r
        }).collect::<Vec<Vec<Vec<_>>>>();
    println!("{:?}", parsed_maps.clone());
    // translate seeds over different maps
    let seeds: Vec<_> = seeds.into_iter().map(|seed| seed_translator(seed, &parsed_maps)).collect();
    println!("translated seeds are {:?}", seeds.clone());
    println!("seed min is {}", seeds.clone().into_iter().min().unwrap());
    // part 2 create ranges
    let seeds: Vec<_> = nums_line_to_vec(seed_line.clone());
    let even_seeds = seeds.clone().into_iter().step_by(2);
    let odd_seeds = seeds.clone().into_iter().skip(1).step_by(2);
    let seed_ranges = even_seeds.zip(odd_seeds).collect::<Vec<_>>();
    println!("seed ranges are {:?}", seed_ranges);
    let mut seeds_from_range = Vec::new();
    let mut min_range: (u64, u64) = (0,0);
    let mut min_translated_range: (u64, u64) = (0,0);
    let mut min_translated = u64::MAX;
   for range in seed_ranges {
        println!("checking range {:?}", range);
        let translated1 = seed_translator(range.0, &parsed_maps);
        let translated2 = seed_translator(range.0 + range.1, &parsed_maps);
        println!("Got values of {} and {}", translated1, translated2);
        if (translated1 < min_translated || translated2 < min_translated) {
            seeds_from_range = Vec::new();
            seeds_from_range.push(translated1);
            seeds_from_range.push(translated2);
            min_translated = [translated1, translated2].into_iter().min().unwrap();
            min_range = range;
            min_translated_range = (translated1, translated2)
        }
    } 
    println!("cheking bounds range is {:?}", seeds_from_range);
    println!("checking min translated range of {:?}", min_translated_range);
    let range_min = min_translated_range.0.min(min_translated_range.1);
    let range_max = min_translated_range.0.max(min_translated_range.1);
    println!("range min is {} and max is {}", range_min, range_max);
    let mut total_min = u64::MAX;
    let mut seeds_from_range = Vec::new();
    for seed in min_range.0..=(min_range.0+min_range.1) {
        let translated = seed_translator(seed, &parsed_maps);
        seeds_from_range.push(translated);
        if translated < total_min {
            total_min = translated;
        }
    }
    println!("output range is {:?}", seeds_from_range);
    println!("min range is {:?}", min_range);
    println!("total min is {}", total_min);
}

fn seed_translator_reversed(mut seed: u64, parsed_maps: &Vec<Vec<Vec<u64>>>) -> u64 {
    for map in parsed_maps.into_iter().rev() {
        let init_seed = seed.clone();
        for range in map.into_iter().rev() {
            if (seed >= range[1] && seed < range[1]+range[2]) {
                seed = (seed - range[1]) + range[0];
                break;
            }
            else {
                continue; // dont modify seed and go to next range, if no ranges match seed will map to same
            }
        }
    }
    seed
}

fn seed_translator(mut seed: u64, parsed_maps: &Vec<Vec<Vec<u64>>>) -> u64 {
    for map in parsed_maps {
        let init_seed = seed.clone();
        for range in map {
            if (seed >= range[1] && seed < range[1]+range[2]) {
                seed = (seed - range[1]) + range[0];
                break;
            }
            else {
                continue; // dont modify seed and go to next range, if no ranges match seed will map to same
            }
        }
    }
    seed
}

fn nums_line_to_vec(line: &str) -> Vec<u64> {
    let line = line
        .split(": ")
        .last()
        .expect("Couldn't get numbers for seeds")
        .split(" ")
        .map(|val| val.parse::<u64>().expect("Couldn't parse seed num"))
        .collect::<Vec<u64>>();
    line
}
