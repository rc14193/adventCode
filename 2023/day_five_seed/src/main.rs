use std::{fs, collections::VecDeque};

fn main() {
    let file = "input.txt";
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
    println!();
    println!();
    // part 2 iterate translation of each range
    let seeds: Vec<_> = nums_line_to_vec(seed_line.clone());
    let even_seeds = seeds.clone().into_iter().step_by(2);
    let odd_seeds = seeds.clone().into_iter().skip(1).step_by(2);
    let seed_ranges = even_seeds.clone().zip(odd_seeds).collect::<Vec<_>>();
    let mut points: Vec<u64> = even_seeds.collect::<Vec<_>>();
    // iteratively step through getting the lowest start of each range at each map
    // the end of this is the start of all final ranges, which is the lowest
    for idx in 0..parsed_maps.len() {
        let map = parsed_maps.clone().into_iter().skip(idx).take(1).collect::<Vec<_>>();
        let mut translated = points.iter().map(|p| seed_translator(*p, &map)).collect::<Vec<_>>();
        translated.extend(map.first().unwrap().iter().map(|r| r[0]).collect::<Vec<_>>());
        points = translated;
    }
    println!("pts is {:?}", points);
    // work back from the output ranges to all possible inputs based on final ranges
    let points = points.iter().map(|p| seed_translator_reversed(p.clone(), &parsed_maps)).collect::<Vec<_>>(); 
    // filter the possible to only the valid based on the input ranges
    let points = points.iter().filter(|p| seed_ranges.iter().any(|r| r.0 <= **p && **p < r.0+r.1)).collect::<Vec<_>>();
    println!("pts is {:?}", points);
    // translate the filtered input ranges and get the min
    let points = points.iter().map(|p| seed_translator(**p, &parsed_maps)).collect::<Vec<_>>();
    println!("pts is {:?}", points);
    println!("min is {}", points.iter().min().unwrap());
}

fn seed_translator_reversed(mut seed: u64, parsed_maps: &Vec<Vec<Vec<u64>>>) -> u64 {
    for map in parsed_maps.into_iter().rev() {
        let init_seed = seed.clone();
        for range in map.into_iter().rev() {
            if (seed >= range[0] && seed < range[0]+range[2]) {
                seed = (seed - range[0]) + range[1];
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
