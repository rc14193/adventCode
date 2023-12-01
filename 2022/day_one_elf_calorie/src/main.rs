use std::fs;

fn main() {
    let file_path = "elf_calorie.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut elves = Vec::new();

    let mut elf = Vec::new();
    for line in contents.lines() {
        match line {
            "" => {
                elves.push(elf.clone());
                elf = Vec::new();
            },
            _ => {
                let calories = line.parse::<i32>().unwrap();
                elf.push(calories);
            }
        }
    }

    let mut elves_calories = elves.iter().map(|x| x.iter().sum()).collect::<Vec<i32>>();
    elves_calories.sort();
    println!("max calories is {}", elves_calories[elves_calories.len()-1]);

    let top_three = elves_calories[elves_calories.len()-1]+elves_calories[elves_calories.len()-2]+elves_calories[elves_calories.len()-3];
    println!("top 3 total is {}", top_three);
}
