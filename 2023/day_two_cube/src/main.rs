use std::{fs, iter::zip};

enum CubeColors {
    Red,
    Green,
    Blue
}

struct Cube {
    color: CubeColors,
    value: u32
}

fn main() {
    let file = "input.txt";
    let contents = fs::read_to_string(file).expect("Couldn't get contents");
    let mut valid_sums: u32 = 0;
    let max_valid = [12u32,13,14];
    for line in contents.lines() {
        // separate game id and game
        let game_title = line.split(':').next().expect("Game should have title");
        let game_id = game_title.split(' ').last().expect("Game title should have number");
        // break up game rounds
        let rounds_list = line.split(':').last().expect("Should have rounds list");
        let rounds = rounds_list.split(';');
        // parse rounds to cubes struct
        let parsed_rounds = rounds.flat_map(|round| {
            let cubes = round.split(',');
            cubes.map(|cube_str| cube_str_to_cube(cube_str))
        });
        // find max of each color
        let color_maxes = parsed_rounds.fold([0,0,0], |mut acc, cube| {
            let color = cube.color as usize;
            if acc[color] < cube.value {
                acc[color] = cube.value;
            }
            acc
        });
        // find out if impossible game
        let mut impossible = zip(max_valid, color_maxes).map(|maxes| maxes.0 >= maxes.1);
        let is_valid = impossible.all(|v| v);
        if is_valid {
            println!("Game id {} is valid", game_id);
            valid_sums = valid_sums + game_id.parse::<u32>().expect("Couldn't parse game id value");
        }
    }
    println!("Sum of valid games is {}", valid_sums)
}

// for more robust could str lower but won't cause advent of code :)
fn cube_str_to_cube(cube_string: &str) -> Cube {
    let mut string_split = cube_string.trim().split(' ');
    let cube_value = string_split.next().expect("No cube value found").parse::<u32>().expect("Couldn't parse cube value");
    let cube_color = match string_split.next().expect("No color string found") {
        "red" => CubeColors::Red,
        "green" => CubeColors::Green,
        "blue" => CubeColors::Blue,
        _ => panic!("Non-matching color string found")
    };
    Cube { color: cube_color, value: cube_value }
}
