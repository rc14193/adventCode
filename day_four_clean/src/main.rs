use std::{fs, ops::Index};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file.");
    let pairs = contents.lines().map(|line| split_pairs(line));
    let contains = pairs.map(|pair| pair_ranges(pair)).fold(0, |mut acc, v|{
        if v {
            acc += 1;
        }
        acc
    });
    println!("The containing pairs count is {}", contains);
}

fn split_pairs(line: &str) -> Vec<&str>{
    line.split(',').collect::<Vec<&str>>()
}

fn pair_ranges(pair: Vec<&str>) -> bool{
    let first_pair = pair[0];
    let second_pair = pair[1];

    let first_nums = first_pair.split('-').collect::<Vec<&str>>();
    let second_nums = second_pair.split('-').collect::<Vec<&str>>();

    let first_min = first_nums[0].parse::<u32>().unwrap();
    let first_max = first_nums[1].parse::<u32>().unwrap();

    let second_min = second_nums[0].parse::<u32>().unwrap();
    let second_max = second_nums[1].parse::<u32>().unwrap();

    let mut contains = false;

    /* part 1 check */
    if first_min >= second_min && first_min <= second_max && first_max <= second_max && first_max >= second_min {
        contains = true;
    }
    if second_min >= first_min && second_min <= first_max && second_max <= first_max && second_max >= first_min {
        contains = true;
    }
    /* part 2 check just replace the center && with an || */
    contains
}
