use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("couldn't read file");
    let mut set: HashSet<char> = HashSet::new();
    let mut q: VecDeque<char> = VecDeque::new();
    let mut idx = 0;
    for char in contents.chars() {
        q.push_back(char);
        idx += 1;
        if q.len() == 14 {
            set.extend(q.clone().iter());
            if set.len() == 14 {
                println!("unique is at idx {}", idx);
                return;
            } else {
                set.clear();
                q.pop_front();
            }
        }
    }
}
