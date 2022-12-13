use std::fs;
use std::collections::VecDeque;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read string");
    let mut min_steps = 99999999;
    let mut start_point = (0,0);
    let mut end_point = (0,0);

    let pairs = ('a'..='z').into_iter().zip((0..26).into_iter());
    let mut string_translation: HashMap<char, i32> = pairs.collect();
    let mut terrain: Vec<Vec<char>> = vec![];
    string_translation.insert('S', 0);
    string_translation.insert('E', 25);

    // to vec chars
    for line in contents.lines() {
        let mut points = line.chars().collect::<Vec<char>>();
        terrain.push(points);
    }

    // convert chars to values
    let mut terrain_num: Vec<Vec<i32>> = vec![];
    for (i, line) in terrain.iter().enumerate() {
        let mut num_line = vec![];
        for (j,c) in line.iter().enumerate() {
            if *c == 'S' {
                start_point = (i,j);
                num_line.push(0);
            } else if *c == 'E' {
                end_point = (i,j);
                num_line.push(25);
            } else {
                let point_val = string_translation.get(c).unwrap();
                num_line.push(*point_val);
            }
        }
        terrain_num.push(num_line);
    }

    //println!("input translation is:\n {:?}", terrain_num);
    println!("Start is at {:?} and end is at {:?}", start_point, end_point);
    let max_row = terrain_num.len();
    let max_col = terrain_num[0].len();

    // path finding operation
    let mut q = VecDeque::new();
    q.push_back((start_point, 0usize));
    let mut visit_values: HashMap<(usize, usize), usize> = HashMap::new();

    // iterative rather than recursive
    while q.len() != 0 {
        let (current_point, cur_weight) = q.pop_front().unwrap();
        let neighbors = create_neighbors(current_point, max_row, max_col);
        visit_values.insert(current_point, cur_weight);
        min_steps = std::cmp::min(cur_weight+1,min_steps);
        if current_point == end_point {
            break;
        }
        for neighbor in neighbors {
            let prev_visit = visit_values.get(&neighbor);
            if !(terrain_num[neighbor.0][neighbor.1] <= terrain_num[current_point.0][current_point.1] + 1) {
                continue; // skip neighbors that don't have a height 1 more or less than the current point
            }
            match prev_visit {
                Some(old_weight) => {
                    // already visited so ignore
                },
                None => {
                    visit_values.insert(neighbor, cur_weight+1);
                    q.push_back((neighbor, cur_weight+1));
                }
            };
        }
    }
    println!("end value is {}", visit_values.get(&end_point).unwrap());
    // part 2
    // there is a better way to do this, many in fact but reiterate my other solns
    // I am not looking for the best but merely pracitce with Rust
    let start_point = end_point;
    // path finding operation
    let mut q = VecDeque::new();
    q.push_back((start_point, 0usize));
    let mut visit_values: HashMap<(usize, usize), usize> = HashMap::new();

    // iterative rather than recursive
    while q.len() != 0 {
        //println!("q is {:?}", q);
        let (current_point, cur_weight) = q.pop_front().unwrap();
        let neighbors = create_neighbors(current_point, max_row, max_col);
        visit_values.insert(current_point, cur_weight);
        min_steps = std::cmp::min(cur_weight+1,min_steps);
        for neighbor in neighbors {
            let prev_visit = visit_values.get(&neighbor);
            if !(terrain_num[neighbor.0][neighbor.1] >= terrain_num[current_point.0][current_point.1]-1) {
                continue; // if a neighbor is not greater or a step below than the current point then continue
            }
            match prev_visit {
                Some(old_weight) => {
                    // already visited so ignore
                },
                None => {
                    visit_values.insert(neighbor, cur_weight+1);
                    q.push_back((neighbor, cur_weight+1));
                }
            };
        }
    }
    let mut zero_points: Vec<usize> = vec![];
    for i in 0..terrain_num.len() {
        for j in 0..terrain_num.len() {
            if !(terrain_num[i][j] == 0) {
                continue;
            }
            let res = visit_values.get(&(i,j));
            match res {
                Some(v)=> {zero_points.push(*v)},
                None => {}
            }
        }
    }
    zero_points.sort();
    println!("zero points are {:?}", zero_points);

}

fn create_neighbors(input_point: (usize, usize), max_row: usize, max_col: usize) -> Vec<(usize,usize)> {
    let mut neighbors = vec![];
    if input_point.0 != 0 {
        let up = (input_point.0-1, input_point.1);
        neighbors.push(up);
    }
    if input_point.0 != max_row-1 {
        let down = (input_point.0+1, input_point.1);
        neighbors.push(down);
    }
    if input_point.1 != 0 {
        let left = (input_point.0, input_point.1-1);
        neighbors.push(left);
    }
    if input_point.1 != max_col-1 {
        let right = (input_point.0, input_point.1+1);
        neighbors.push(right);
    }
    neighbors
}
