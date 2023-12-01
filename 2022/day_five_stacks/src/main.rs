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
            let mut segments = line.split(" ");
            let pops = segments.nth(1).unwrap().parse::<i32>().unwrap();
            let pop_idx = segments.nth(1).unwrap().parse::<usize>().unwrap()-1;
            let push_idx = segments.nth(1).unwrap().parse::<usize>().unwrap()-1;
            let mut grabbed_chars: Vec<char> = vec![];
            for i in 0..pops {
                let top_crate = stacks[pop_idx].pop().unwrap().clone(); 
                grabbed_chars.push(top_crate);
            }
            stacks[push_idx].extend(grabbed_chars.iter().rev());
        }
    }
    println!("Final boxes were: ");
    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }

}

fn display_stacks(stacks: Vec<Vec<char>>) -> () {
    let mut i = 0;
    for stack in stacks {
        print!("{} ", i);
        for chara in stack {
            print!("{} ", chara);
        }
        print!("\n");
         i += 1;
    }
}

fn parse_box_line(line: &str, mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>>{
    let chars = line.chars().clone().collect::<Vec<char>>();
    let mut num_idx = 0;
    for i in 0..9 {
        num_idx = 4*i +1;
        if chars[num_idx] != ' ' && !String::from(chars[num_idx]).parse::<i32>().is_ok() {
            stacks[i].push(chars[num_idx].clone());
        }
    };
    stacks
}
