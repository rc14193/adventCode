use std::{fs, collections::VecDeque};

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

    let d = parsed_maps.iter().map(|v| v.iter().map(|i| i.iter().nth(1).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    println!("d is {:?}", d.clone());
    //let mut dq: VecDeque<_> = VecDeque::from(d);
    let mut tr_pts = Vec::new();
    for (idx, pt) in d.clone().iter().enumerate() {
        let map = parsed_maps.clone().into_iter().skip(idx+1).collect::<Vec<Vec<_>>>();
        if map.len() != 0 {
            for p in pt {
                tr_pts.push(seed_translator(**p, &map))
            }
        }
        }
    println!("tr_pts is {:?}", tr_pts);

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
