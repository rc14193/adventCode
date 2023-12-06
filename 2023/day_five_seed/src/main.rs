use std::fs;

fn main() {
    let file = "example_input.txt";
    let contents = fs::read_to_string(file).expect("couldn't open input file");
    let mut lines = contents.lines();
    // parse seeds
    let seed_line = lines.next().expect("Couldn't get seeds line");
    let seeds: Vec<_> = nums_line_to_vec(seed_line);
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
    println!("{:?}", parsed_maps);
    // translate seeds over different maps
    let seeds: Vec<_> = seeds.into_iter().map(|seed| seed_translator(seed, parsed_maps.clone())).collect();
    println!("translated seeds are {:?}", seeds)
}

fn seed_translator(mut seed: u64, parsed_maps: Vec<Vec<Vec<u64>>>) -> u64 {
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
        println!("After range seed {} becomes {}", init_seed, seed);
    }
    println!();
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
