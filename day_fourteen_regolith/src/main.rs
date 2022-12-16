use std::fs;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read.");
    let mut walls: HashSet<(isize,isize)> = HashSet::new();
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;
    for line in contents.lines() {
        let coords = line.split(" -> ").collect::<Vec<&str>>();
        let mut first_x = coords[0].split(",").collect::<Vec<&str>>()[0].parse::<isize>().unwrap();
        let mut first_y = coords[0].split(",").collect::<Vec<&str>>()[1].parse::<isize>().unwrap();
        for coord in 1..coords.len() {
            let mut start_x = 0;
            let mut start_y = 0;
            let mut end_x = 0;
            let mut end_y = 0;
            let second_x =  coords[coord].split(",").collect::<Vec<&str>>()[0].parse::<isize>().unwrap();
            let second_y =  coords[coord].split(",").collect::<Vec<&str>>()[1].parse::<isize>().unwrap();
            let change = (first_x-second_x, first_y-second_y);
            if change.0 < 0 && change.1 == 0 {
                start_x = first_x;
                end_x = second_x;
                start_y = first_y;
                end_y = first_y;
            } else if change.0 > 0 && change.1 == 0 {
                start_x = second_x;
                end_x = first_x;
                start_y = first_y;
                end_y = first_y;
            } else if  change.0 == 0 && change.1 < 0 {
                start_x = first_x;
                end_x = first_x;
                start_y = first_y;
                end_y = second_y;
            } else if  change.0 == 0 && change.1 > 0 {
                start_x = first_x;
                end_x = first_x;
                start_y = second_y;
                end_y = first_y;
            } else {
                println!("point {},{}", change.0, change.1);
                panic!("hand non 0 for both parts of change and cannot have diagonal line")
            }
            max_x = std::cmp::max(max_x, start_x);
            max_x = std::cmp::max(max_x, end_x);
            min_x = std::cmp::min(min_x, start_x);
            min_x = std::cmp::min(min_x, end_x);

            max_y = std::cmp::max(max_y, end_y);
            max_y = std::cmp::max(max_y, start_y);
            for i in start_x..=end_x {
                for j in start_y..=end_y {
                    walls.insert((i,j));
                }
            }
            first_x = second_x;
            first_y = second_y;

        }
    }
    let mut sand_drops = 0;
    let mut sand_pos: (isize, isize) = (500,0);
    let mut exit = false;
    while !exit {
        //println!("Sand pos {}, {}", sand_pos.0, sand_pos.1);
        //println!("Area bounds, max: {}, min: {}, y {}", min_x, max_x, max_y);
        let next_open = look_ahead(&sand_pos, &walls);
        sand_drops += match next_open {
            Some(pos) => { 
                if sand_pos.1 == max_y + 1 {
                    walls.insert(sand_pos);
                    sand_pos = (500,0);
                    1
                } else {
                    sand_pos = pos;
                    0
                }
            },
            None => {
                if sand_pos == (500,0) {
                    exit = true;
                    1
                } else {
                    walls.insert(sand_pos);
                    sand_pos = (500,0);
                    1
                }
            }
        }
    }
    println!("Sand drops is {}", sand_drops);

}

fn display_wall(walls: &HashSet<(isize, isize)>, min_x: isize, max_x: isize, max_y: isize) {
    for y in 0..=max_y {
        for x in 0..=(max_x-min_x) {
            if walls.contains(&(x+min_x, y)) {
                print!("# ")
            }
            else {
                print!(". ")
            }
        }
        print!("\n")
    }
}

fn look_ahead(pos: &(isize, isize), walls: &HashSet<(isize, isize)>) -> Option<(isize, isize)> {
    let mut x = pos.0.clone();
    let mut y = pos.1.clone();

    if !(walls.contains(&(x, y+1))) {
        Some((x, y+1))
    } else if !(walls.contains(&(x-1, y+1))) {
        Some((x-1, y+1))
    } else if !(walls.contains(&(x+1, y+1))) {
        Some((x+1, y+1))
    } else {
        None
    }
}
