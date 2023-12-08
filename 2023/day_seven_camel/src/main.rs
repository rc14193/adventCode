use std::{cmp::Ordering, collections::HashMap, fs};

#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOf,
    FullHouse,
    FourOf,
    FiveOf,
}

#[derive(PartialEq, PartialOrd, Eq, Clone, Debug)]
struct Card<'a> {
    str_input: &'a str,
    hand_type: HandType,
    bid: u64,
}

impl<'a> Card<'a> {
    fn new(input: &'a str, hand: HandType, bid: u64) -> Card<'a> {
        Card {
            str_input: input,
            hand_type: hand,
            bid: bid,
        }
    }
}

impl<'a> Ord for Card<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_chars = self.str_input.chars();
        let other_chars = other.str_input.chars();
        let pairs = self_chars.zip(other_chars);
        let first_different = pairs.skip_while(|(a, b)| a == b).next().unwrap();
        // using ascii to compare, shift JQKA to be in right order
        let mut cards_vals = vec![];
        for c in vec![first_different.0, first_different.1] {
            let mut val = c as u64;
            val = match val {
                // A is 65 Dec
                75 => val - 11, // K 75 => 64
                81 => val - 18, // Q 81 => 63
                // part 2 J becomes weakes, 2 in ASCII is 50
                74 => val - 35, // J 74 => 62, p2 74 => 49
                84 => val - 23, // T 84 => 61
                _ => val,
            };
            cards_vals.push(val);
        }
        println!(
            "In compare checking {}: {} vs {}: {}",
            first_different.0, cards_vals[0], first_different.1, cards_vals[1]
        );
        if cards_vals[0] > cards_vals[1] {
            Ordering::Greater
        } else if cards_vals[0] < cards_vals[1] {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

fn main() {
    // 248372045
    let file = "input.txt";
    let contents = fs::read_to_string(file).expect("Couldn't read file");
    let lines = contents.lines();
    let hand_types = [
        HandType::HighCard,
        HandType::OnePair,
        HandType::TwoPair,
        HandType::ThreeOf,
        HandType::FullHouse,
        HandType::FourOf,
        HandType::FiveOf,
    ];
    let cards = lines.map(|l| parse_card(l));
    let card_vec = cards.clone().collect::<Vec<_>>();
    let x = 0;
    let card_groups = cards.fold(HashMap::new(), |mut acc, card| {
        let mut card_list = acc.get(&card.hand_type).unwrap_or(&Vec::new()).clone();
        card_list.push(card.clone());
        acc.insert(card.hand_type, card_list);
        acc
    });
    let mut sorted_cards = vec![];
    for hand_type in hand_types {
        let mut card_in_type = card_groups.get(&hand_type);
        match card_in_type {
            Some(v) => {
                let mut out = v.clone();
                out.sort_by(|a, b| a.cmp(b));
                println!("For hand type {:?} have cards {:?}", hand_type, out);
                sorted_cards.extend(out);
            }
            None => continue,
        }
    }
    let cards_scoring =
        sorted_cards
            .iter()
            .zip(1..=sorted_cards.len() as u64)
            .map(|(card, score)| {
                println!(
                    "Scoring card {} with bid {} and score {}",
                    card.str_input, card.bid, score
                );
                card.bid * score
            });
    let total_score: u64 = cards_scoring.sum();
    println!("Output total is {}", total_score);
}

fn parse_card(card_line: &str) -> Card {
    let card = card_line.split(' ').next().unwrap();
    let bid = card_line.split(' ').last().unwrap().parse::<u64>().unwrap();
    let card_chars = card.chars();
    let mut counter: HashMap<char, u64> = card_chars.fold(HashMap::new(), {
        |mut acc, c| {
            let current_count = acc.get(&c).unwrap_or(&0);
            acc.insert(c, current_count + 1);
            acc
        }
    });
    let jack_num = counter.remove(&'J');
    let card_max = counter.values().max();
    let hand_type = match card_max {
        Some(card_max) => parse_card_type(*card_max, counter),
        None => HandType::OnePair,
    };
    let hand_type = match jack_num {
        Some(n) => try_to_promote(hand_type, n),
        None => hand_type,
    };
    Card::new(card_line, hand_type, bid)
}

fn parse_card_type(count_max: u64, counter: HashMap<char, u64>) -> HandType {
    match count_max {
        5 => HandType::FiveOf,
        4 => HandType::FourOf,
        3 => {
            if counter.values().any(|v| v == &2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOf
            }
        }
        2 => {
            let pairs = counter.iter().fold(0, |mut acc, (c, occurr)| {
                if (occurr == &(2 as u64)) {
                    acc + 1
                } else {
                    acc
                }
            });
            if pairs > 1 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        1 => HandType::HighCard,
        _ => panic!("Invalid count amount in card line"),
    }
}

fn try_to_promote(hand_type: HandType, j_count: u64) -> HandType {
    if j_count == 0 || hand_type == HandType::FiveOf || hand_type == HandType::FullHouse {
        hand_type
    } else {
        let new_hand_type = match hand_type {
            HandType::HighCard => HandType::OnePair,
            HandType::OnePair => HandType::ThreeOf,
            HandType::TwoPair => HandType::FullHouse,
            HandType::ThreeOf => HandType::FourOf,
            HandType::FullHouse => hand_type,
            HandType::FourOf => HandType::FiveOf,
            HandType::FiveOf => hand_type,
        };
        try_to_promote(new_hand_type, j_count - 1)
    }
}
