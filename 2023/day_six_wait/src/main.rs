use core::time;
use std::fs;

fn main() {
    let file = "input.txt";
    let contents = fs::read_to_string(file).expect("Couldn't open input");
    let mut lines = contents.lines();
    let times = lines
        .next()
        .expect("No time line present")
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .filter(|t| t != &"")
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .expect("No time line present")
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .filter(|d| d != &"")
        .collect::<Vec<_>>();
    println!("times {:?}", times);
    println!("distances {:?}", distances);
    // part 1
    solve_time(times.clone(), distances.clone());
    // part 2
    let mut combined_times: Vec<&str> = Vec::new();
    let new_time = times.iter().fold(String::new(), |mut acc, v| {acc.push_str(v); return acc;});
    combined_times.push(&new_time.as_str());
    let mut combined_distances: Vec<&str> = Vec::new();
    let new_time = distances.iter().fold(String::new(), |mut acc, v| {acc.push_str(v); return acc;});
    combined_distances.push(&new_time.as_str());
    solve_time(combined_times, combined_distances);
}

fn solve_time(times: Vec<&str>, distances: Vec<&str>) {
    // quadratic formula
    let times = times.iter().map(|d| d.parse::<u64>().unwrap());
    let distances = distances.iter().map(|d| d.parse::<u64>().unwrap());
    let eqns = times.zip(distances);
    let plus_roots = eqns.clone().map(|(time, distance)|(time as f64+((time*time-(4*distance)) as f64).sqrt())/2.0);
    let negative_roots = eqns.map(|(time, distance)|(time as f64-((time*time-(4*distance)) as f64).sqrt())/2.0);
    println!("plus roots {:?}", plus_roots.clone().collect::<Vec<_>>());
    println!("negative roots {:?}", negative_roots.clone().collect::<Vec<_>>());
    let max_press_times = plus_roots.into_iter().map(|t| (t-1.0).ceil() as u64).collect::<Vec<_>>();
    let min_press_times = negative_roots.into_iter().map(|t| (t+1.0).floor() as u64).collect::<Vec<_>>();
    println!("max times {:?}", max_press_times.clone());
    println!("min times {:?}", min_press_times.clone());
    let valid_press_counts = max_press_times.iter().zip(min_press_times.iter()).map(|(max, min)| (max-min)+1);
    println!("valid press count {:?}", valid_press_counts.clone().collect::<Vec<_>>());
    println!("product {}", valid_press_counts.product::<u64>());
}
