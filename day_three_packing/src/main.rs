use std::{fs, 
    iter::zip,
    collections::HashSet,
    collections::HashMap};

fn main() {
    /*
    //tests
    let contents = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
*/
    let contents = fs::read_to_string("input.txt").expect("Failed to read input");
    let lower_translate: HashMap<char, i32> = HashMap::from_iter(zip('a'..='z', 1..27)); // remember non inclusive bounds
    let upper_translate: HashMap<char, i32> = HashMap::from_iter(zip('A'..='Z', 27..53));
    let translator = lower_translate.into_iter().chain(upper_translate).collect::<HashMap<char, i32>>();
    let packages = contents.lines().map(|line| split_containers(line));
    let unions = packages.into_iter().map(|container| find_union(container));
    let priorities: i32 = unions.map(|char| translator.get(&char).unwrap()).sum();
    println!("total is {}", priorities);
    // part 2
    let mut acc = vec![];
    let mut curr_group = vec![];
    println!("doing part 2, acc is {:?}", acc);
    for line in contents.lines() {
        println!("doing line {}", line);
        if curr_group.len() < 3{
            curr_group.push(String::from(line))
        } else {
            acc.push(curr_group.clone());
            curr_group = Vec::new();
            curr_group.push(String::from(line));
        }
    }
    acc.push(curr_group);
    println!("doing part 2, acc is {:?}", acc);
    let group_unions = acc.into_iter().map(|groups| find_union(groups));
    let group_badge: i32 = group_unions.map(|char| translator.get(&char).unwrap()).sum();
    println!("group badge result is {}", group_badge)
}

fn find_union(container: Vec<String>) -> char{
    let mut sets:Vec<HashSet<char>> = Vec::new();
    for item in container.clone() {
        sets.push(HashSet::from_iter(item.chars()));
    }

    let mut intersect = sets[0].clone(); // Intersection of the first set is the first set, wasted computation but will be correct
    for set in sets {
        // Need help on if there is a better way to get HashSet<char> from &char items
        intersect = intersect.intersection(&set).into_iter().map(|x| x.clone()).collect::<HashSet<char>>();
    }
    let inter = intersect.into_iter().collect::<Vec<char>>()[0].clone();
    println!("for string items {:?}, intersection of {:?}", container, inter);
    inter
}

fn split_containers(line: &str) -> Vec<String>{
    let char_nums = line.chars().count();
    vec![line.chars().take(char_nums/2).collect::<String>(), line.chars().skip(char_nums/2).collect::<String>()]
}
