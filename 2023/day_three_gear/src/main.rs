use std::collections::HashMap;
use std::fs;

fn main() {
    let file = "input.txt";
    let contents = fs::read_to_string(file).expect("Couldn't parse file");
    // parse contents
    let char_grid = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    // step through char grid
    // parse a number, keeping track of total digits and indexes, once done parsing number check all neighbors for a symbol
    let mut num_window = String::new();
    let mut digit_idxs = Vec::new();
    let mut part_nums = Vec::new();
    let mut gear_ratios: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    for row in 0..char_grid.len() {
        for col in 0..char_grid[row].len() {
            match char_grid[row][col] {
                '0'..='9' => {
                    num_window.push(char_grid[row][col]);
                    digit_idxs.push((row, col))
                }
                _ => {
                    if num_window.len() != 0 {
                        let has_symbol_neighbor = check_neighbors_for_symbols_and_track_gear_ratio(
                            &char_grid,
                            digit_idxs.clone(),
                            char_grid[0].len() as isize,
                            char_grid.len() as isize,
                            num_window.clone(),
                            &mut gear_ratios,
                        );
                        if has_symbol_neighbor {
                            part_nums.push(
                                num_window
                                    .parse::<u64>()
                                    .expect("Couldn't parse num window"),
                            )
                        }
                    }
                    num_window.clear();
                    digit_idxs.clear();
                }
            }
        }
    }
    let gear_ratios: Vec<_> = gear_ratios
        .values()
        .filter(|gear| gear.len() == 2)
        .collect();
    println!("part nums of {:?}", part_nums);
    let part_num_sum = part_nums.into_iter().fold(0, |acc, val| acc + val);
    let gear_products = gear_ratios
        .clone()
        .into_iter()
        .map(|gear| gear.into_iter().product::<u64>())
        .collect::<Vec<_>>();
    println!("part nums sum is {}", part_num_sum);
    println!("Gear ratios are {:?}", gear_ratios);
    println!("Gear ratio product is {:?}", gear_products);
    println!(
        "Gear ratios sum is {}",
        gear_products.into_iter().sum::<u64>()
    )
}

fn check_neighbors_for_symbols_and_track_gear_ratio(
    char_grid: &Vec<Vec<char>>,
    digit_idxs: Vec<(usize, usize)>,
    max_x: isize,
    max_y: isize,
    num_val: String,
    gear_ratios: &mut HashMap<(usize, usize), Vec<u64>>,
) -> bool {
    let num_val = num_val
        .parse::<u64>()
        .expect("Couldn't parse num window in neighbor check");
    let mut has_neighbor = false;
    // top, top right, right, bottom right, bottom, bottom left, left, top left (clockwise from top)
    let neighbor_values = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    let mut found_gears: Vec<(usize, usize)> = Vec::new(); // track if we already incremented this gear for this number so we don't double inc
    for idx in digit_idxs {
        let neighbors = neighbor_values.map(|dir| (idx.0 as isize + dir.0, idx.1 as isize + dir.1));
        let neighbors = neighbors
            .into_iter()
            .filter(|val| val.0 >= 0 && val.1 >= 0 && val.0 < max_x && val.1 < max_y);

        for neighbor in neighbors {
            let neighbor_x = usize::try_from(neighbor.0).expect("Couldn't convert row");
            let neighbor_y = usize::try_from(neighbor.1).expect("Couldn't convert row");
            let neighbor_val = char_grid[neighbor_x][neighbor_y];
            // In part one we can return early, part 2 we check all neighbors
            if !neighbor_val.is_digit(10) && neighbor_val != '.' {
                has_neighbor = true;
            }
            if neighbor_val == '*' && !found_gears.contains(&(neighbor_x, neighbor_y)) {
                let mut updated_gear = gear_ratios
                    .get(&(neighbor_x, neighbor_y))
                    .unwrap_or(&Vec::new())
                    .to_vec();
                updated_gear.push(num_val);
                gear_ratios.insert((neighbor_x, neighbor_y), updated_gear);
                found_gears.push((neighbor_x, neighbor_y));
            }
        }
    }
    has_neighbor
}
