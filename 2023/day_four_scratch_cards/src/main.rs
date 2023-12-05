use std::{fs, collections::HashSet};
use std::collections::HashMap;

fn main() {
    let file_name = "input.txt";
    let contents = fs::read_to_string(file_name).expect("Couldn't parse file");
    let mut sum_pts = 0;
    let mut copies: HashMap<u32, u32> = HashMap::new();
    for line in contents.lines()  {
        let mut card_title = line.split(':').next().unwrap(); 
        let card_id = card_title.split(' ').last().unwrap().parse::<u32>().unwrap();
        let mut cards = line.split(':').last().unwrap().trim().split('|');
        let winners = cards.next().expect("Couldn't get winner cards").trim().split(' ').filter(|val| val != &"").map(|val| val.trim().parse::<u32>().expect(format!("Couldn't parse val {}", val).as_str()));
        let yours = cards.next().expect("Couldn't get next part after |").trim().split(' ').filter(|val| val != &"").map(|val| val.trim().parse::<u32>().expect(format!("failed trying to parse {}", val).as_str()));
        let winners = winners.collect::<HashSet<_>>();
        let yours = yours.collect::<HashSet<_>>();
        let matching = winners.intersection(&yours).collect::<Vec<_>>();
        println!("for card {} set is {:?}", card_id, matching);
        let init_copy_count = copies.get(&(card_id)).unwrap_or(&0).clone();
        copies.insert(card_id, init_copy_count.clone() + 1);
        if matching.len() == 0 {
            continue;
        }
        sum_pts = sum_pts + 2u32.pow((matching.len() - 1) as u32);
        for id_num in 1..=matching.len() {
            let mut copy_id_count = copies.get(&(card_id+id_num as u32)).unwrap_or(&0) + init_copy_count;
            println!("For card {} setting copy count {}", card_id+id_num as u32, copy_id_count);
            copies.insert(card_id+id_num as u32, copy_id_count + 1);
        }
    }
    println!("final sum is {}", sum_pts);
    println!("copies is {:?}", copies);
    println!("copies sum is {:?}", copies.values().into_iter().sum::<u32>());
}
