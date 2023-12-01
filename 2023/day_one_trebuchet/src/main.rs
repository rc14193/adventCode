use core::num;
use std::fs;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input");
    let mut total = 0;

    for line in contents.lines() {
        total = total + part_two_line_process(line);
    }
    println!("final total is {}", total);
}

fn part_one_line_process(line: &str) -> i32{
    let mut numbers = line
        .chars()
        .filter(|c| {"123456789".contains(&c.to_string())});
    let firstNum = numbers.next().expect(&format!("Expect a first number on {}", line));
    let lastNum = numbers.last().unwrap_or(firstNum);
    let mut amount = String::new();
    amount.push(firstNum);
    amount.push(lastNum);
    println!("amount for line {} is {}", line, amount);
    amount.parse::<i32>().unwrap()
}

fn part_two_line_process(line: &str) -> i32 {
    let mut numerals = String::new();
    let mut head = String::new();
    let mut tail = String::new();
    let possible_values = HashMap::from([
        ("1",'1'),
        ("2",'2'),
        ("3",'3'),
        ("4",'4'),
        ("5",'5'),
        ("6",'6'),
        ("7",'7'),
        ("8",'8'),
        ("9",'9'),
        ("one",'1'),
        ("two",'2'),
        ("three",'3'),
        ("four",'4'),
        ("five",'5'),
        ("six",'6'),
        ("seven",'7'),
        ("eight",'8'),
        ("nine",'9')
    ]);
    for char in line.clone().chars() {
        head.push(char);
        let mut value_present = possible_values.keys().map(|k| (head.contains(k), k));
        //println!("vec of present {:?}", value_present.clone().collect::<Vec<(bool, &&str)>>());
        if(value_present.clone().any(|x| x.0)) {
            let value = value_present.skip_while(|x| x.0 == false).next().unwrap().1;
            println!("head {} and value {:?}", head, value);
            numerals.push(possible_values.get(value).unwrap().clone());
            break;
        }
    }
    for char in line.chars().rev() {
        tail.insert(0, char);
        let mut value_present = possible_values.keys().map(|k| (tail.contains(k), k));
        if(value_present.clone().any(|x| x.0)) {
            let value = value_present.skip_while(|x| x.0 == false).next().unwrap().1;
            println!("tail {} and value {:?}", tail, value);
            numerals.push(possible_values.get(value).unwrap().clone());
            break;
        }
    }
    println!("head number for line {} is {}", line, numerals);
    numerals.parse::<i32>().unwrap()
}