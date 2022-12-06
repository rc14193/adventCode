use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("couldn't read file");
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut reading_stacks: bool = true;
    for i in 1..10 {
        stacks.push(vec![]);
    }
    for line in contents.lines() {
        if line.is_empty() {
            stacks = stacks.into_iter().map(|stack| {stack.into_iter().rev().collect::<Vec<char>>()}).collect::<Vec<Vec<char>>>();
            reading_stacks = false;
        } else if reading_stacks {
            stacks = parse_box_line(line, stacks);
        } else {

        }
    }
    println!("stacks were {:?}", stacks)
}

fn parse_box_line(line: &str, mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>>{
    let chars = line.chars().clone().collect::<Vec<char>>();
    let mut num_idx = 0;
    for i in 0..9 {
        num_idx = 4*i +1;
        println!("checking idx {} for chars of len {}", num_idx, chars.len());
        if chars[num_idx] != ' ' && !String::from(chars[num_idx]).parse::<i32>().is_ok() {
            stacks[i].push(chars[num_idx].clone());
        }
    };
    stacks
}
