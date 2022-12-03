use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
enum MatchOutcome {
    Win,
    Loss,
    Draw
}

fn main() {
    /*
    // tests part 1
    // loss
    assert!(process_data(String::from("B X")) == 1);
    assert!(process_data(String::from("C Y")) == 2);
    assert!(process_data(String::from("A Z")) == 3);
    // ties
    assert!(process_data(String::from("A X")) == 4);
    assert!(process_data(String::from("B Y")) == 5);
    assert!(process_data(String::from("C Z")) == 6);
    // wins
    assert!(process_data(String::from("C X")) == 7);
    assert!(process_data(String::from("A Y")) == 8);
    assert!(process_data(String::from("B Z")) == 9);
    // tests part 2
    // loss
    assert!(process_by_outcome(String::from("A X")) == 3);
    assert!(process_by_outcome(String::from("B X")) == 1);
    assert!(process_by_outcome(String::from("C X")) == 2);
    // ties
    assert!(process_by_outcome(String::from("A Y")) == 4);
    assert!(process_by_outcome(String::from("B Y")) == 5);
    assert!(process_by_outcome(String::from("C Y")) == 6);
    //wins
    assert!(process_by_outcome(String::from("A Z")) == 8);
    assert!(process_by_outcome(String::from("B Z")) == 9);
    assert!(process_by_outcome(String::from("C Z")) == 7);
    */

    let contents = fs::read_to_string("rps_strat.txt").expect("Error reading file"); 
    //process_data(contents);
    let score = process_by_outcome(contents);
    println!("Final score is {}", score);
}

fn process_data(input: String) -> i32 {
    let choice_score = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
                                                            //      R .       P .       S
    let player_translate = HashMap::from([("X", 0), ("Y", 1), ("Z", 2)]);
    let oppo_translate = HashMap::from([("A", 0), ("B", 1), ("C", 2)]);
    let mut score = 0;

    for line in input.lines() {
        let options = line.split(" ").collect::<Vec<&str>>();
        // expecting a panic if not two but trusting the in put will be valid since I know it is
        let oppo_choice = *oppo_translate.get(options[0]).unwrap();
        let play = *player_translate.get(options[1]).unwrap();
        let result = get_match_result(oppo_choice, play);
        let result_points = match result {
            MatchOutcome::Win => 6,
            MatchOutcome::Loss => 0,
            MatchOutcome::Draw => 3
        };
        let choice_points = choice_score.get(options[1]).unwrap();
        let round_pts = result_points + choice_points;
        println!("Round {} has result {:?} and result_pts {} and choice_pts {} and round_pts {}", line, result, result_points, choice_points, round_pts);
        score = score + round_pts
    }
    println!("output score is {}", score);
    score
}

fn process_by_outcome(input: String) -> i32{

    let player_translate = HashMap::from([("X", 0), ("Y", 1), ("Z", 2)]);
    let oppo_translate = HashMap::from([("A", 0), ("B", 1), ("C", 2)]);
    let mut score = 0;
    for line in input.lines() {

        let options = line.split(" ").collect::<Vec<&str>>();
        // expecting a panic if not two but trusting the in put will be valid since I know it is
        let oppo_choice: i32 = *oppo_translate.get(options[0]).unwrap();
        let outcome = options[1];
        let result_points = match outcome{
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("Invalid outcome choice")
        };
        let choice_points = match result_points {
            0 => (oppo_choice-1).rem_euclid(3) + 1,
            3 => oppo_choice + 1,
            6 => (oppo_choice+1).rem_euclid(3)+1,
            _ => panic!("Invalid result points ")
        };
        println!("line {} result pts {} and choice pts {}", line, result_points, choice_points);
        let tot = choice_points + result_points;
        score += tot;
    }
    score
}

fn get_match_result(oppo_play: i32, self_play: i32) -> MatchOutcome {
    println!("opp {} self {}", oppo_play, self_play);
    let self_win = (self_play-1).rem_euclid(3);
    let self_loss = (self_play+1).rem_euclid(3);
    println!("self win is {} . and loss is {}", self_win, self_loss);
    if oppo_play == self_play {
        MatchOutcome::Draw
    } else if oppo_play == self_loss{
        MatchOutcome::Loss
    } else {
    // represents else if oppo_play == (self_play-1%2){
        MatchOutcome::Win
    }
}
