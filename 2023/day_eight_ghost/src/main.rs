use std::{fs, collections::HashMap, iter::Map};

fn main() {
    let file = "input.txt";
    let contents = fs::read_to_string(file).expect("Couldn't read file");
    let mut lines = contents.lines();
    let instructions = lines.clone().next().expect("Should have instruction line");
    let nodes_lines = lines.clone().skip(2);
    let graph = parse_nodes(nodes_lines);
    let mut location = String::from("AAA");
    let mut end = String::from("AAA");
    let mut instructions = instructions.chars().cycle();
    let mut step_count: u64 = 0;
    /*
    while end != "ZZZ" {
        let motion = instructions.next().expect("No instruction in cycle");
        location = perform_motion(&location, motion, &graph); 
        step_count = step_count + 1;
        if location == "ZZZ" {
            end = "ZZZ".to_string();
        }
    }
    */
    println!("Reached ZZZ in {}", step_count);
    // part 2
    let locations = graph.clone().into_keys().filter(|v| v.ends_with("A"));
    let mut locations = locations.collect::<Vec<_>>();
    let instructions = lines.next().expect("Should have instruction line");
    let mut instructions = instructions.chars().cycle();
    let mut all_at_end = false;
    let mut step_count: u64 = 0;
    while !all_at_end {
        let motion = instructions.next().expect("No instruction in cycle");
        locations = locations.into_iter().map(|l| {
            perform_motion(&l, motion, &graph)
        }).collect::<Vec<String>>();
        step_count += 1;
        all_at_end = locations.clone().into_iter().all(|v| v.ends_with("Z"));
    }
    println!("Reached end of many in {}", step_count)

}

fn perform_motion(location: &String, instruction: char, graph: &HashMap<String, (String, String)>) -> String {
        let neighbors = graph.get(location).expect("Node should have neighbors").clone(); 
        match instruction {
            'L' => {
                neighbors.0
            },
            'R' => {
                neighbors.1
            },
            _ => panic!("Invlaid motion instruction")
        }.trim().to_string()
}

fn parse_nodes<'a>(nodes_lines: impl Iterator<Item = &'a str>) -> HashMap<String, (String, String)> {

    let mut graph = HashMap::new();
    for line in nodes_lines {
        println!("Parsing line {}", line);
        let mut labels = line.split(" = ");
        let local = labels.next().expect("Should have a local start").trim();
        let neighbors = labels.next().expect("Should have neighbors");
        let neighbors = neighbors.replace("(", "").replace(")", "");
        let mut neighbors = neighbors.split(",");
        let left = neighbors.next().expect("Error finding left");
        let right = neighbors.next().expect("Error finding right");
        graph.insert(local.to_string(), (left.to_string(), right.to_string()));
    }
    graph
}