use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input");
    let mut total = 0;

    for line in contents.lines() {
        let mut numbers = line
            .chars()
            .filter(|c| {"123456789".contains(&c.to_string())});
        let firstNum = numbers.next().expect(&format!("Expect a first number on {}", line));
        let lastNum = numbers.last().unwrap_or(firstNum);
        let mut amount = String::new();
        amount.push(firstNum);
        amount.push(lastNum);
        println!("amount for line {} is {}", line, amount);
        total = total + amount.parse::<i32>().unwrap();
    }
    println!("final total is {}", total);
}
