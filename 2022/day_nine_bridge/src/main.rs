use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file");
    let mut head_max_row = -1000000;
    let mut head_min_row = 1000000;
    let mut head_max_col = -1000000;
    let mut head_min_col = 1000000;
    let mut tail_visits: HashSet<(i32, i32)> = HashSet::new();
    // part 1 change the initial size to 2 instead of 10, arbitrary rope with init length n+1
    // +1 for the head, so part 2, 9 knots is 10
    let mut knots = [(0,0); 10];

    for line in contents.lines() {
        let mut operation = line.split(" ");
        let direction = operation.next().unwrap();
        let steps = operation.next().unwrap();
        let mut head_row = knots[0].0;
        let mut head_col = knots[0].1;

        for _ in 0..steps.parse::<i32>().unwrap() {
            tail_visits.insert((knots[knots.len()-1].0, knots[knots.len()-1].1));
            // head movement
            match direction {
                "U" => head_row += 1,
                "D" => head_row -= 1,
                "L" => head_col -= 1,
                "R" => head_col += 1,
                _ => panic!("Invalid opeartion in input")
            }
            knots[0] = (head_row, head_col);

            head_max_row = std::cmp::max(head_max_row, head_row);
            head_max_col = std::cmp::max(head_max_col, head_col);
            head_min_row = std::cmp::min(head_min_row, head_row);
            head_min_col = std::cmp::min(head_min_col, head_col);

            for i in 1..knots.len() {
                let mut first = knots[i-1].clone();
                let mut second = knots[i].clone();
                let (new_row, new_col) = move_knot(&mut first.0, &mut first.1, &mut second.0, &mut second.1);
                knots[i] = (new_row, new_col);
            }

        }
        tail_visits.insert((knots[knots.len()-1].0, knots[knots.len()-1].1));
    }

    let mut row_ptr = head_max_row;
    let mut col_ptr = 0;
    println!("row min {} row max {}, col min {} col max {}", head_min_row, head_max_row, head_min_col, head_max_col);

    /*
    for _row in head_min_row..head_max_row+1 {
        for _col in head_min_col..head_max_col+1 {
            if (row_ptr, col_ptr) == (head_row, head_col) {
                print!("H ")
            } else if (row_ptr, col_ptr) == (tail_row, tail_col) {
                print!("T ")
            } else if tail_visits.contains(&(row_ptr, col_ptr)) && !(col_ptr == 0 && row_ptr == 0) {
                print!("# ");
            } else if col_ptr == 0 && row_ptr == 0 {
                print!("s ");
            } else {
               // print!("({}, {})", row_ptr, col_ptr);
               print!(". ")
            }
            col_ptr += 1;
        }
        col_ptr = head_min_col;
        row_ptr -= 1;
        print!("\n");
    }
    */
    println!("Tail unique visits are {}", tail_visits.len());
}

fn move_knot(preceeding_row: &mut i32, preceeding_col: &mut i32, succeeding_row: &mut i32, succeeding_col: &mut i32) -> (i32, i32) {
    let mut head_row = (*preceeding_row).clone();
    let mut head_col = (*preceeding_col).clone();
    let mut tail_row = (*succeeding_row).clone();
    let mut tail_col = (*succeeding_col).clone();

    // head is 2 greater left or right on same row
    if head_row == tail_row && head_col == tail_col + 2 {
        tail_col += 1;
        return (tail_row, tail_col);
    }
    if head_row == tail_row && head_col == tail_col - 2 {
        tail_col -= 1;
        return (tail_row, tail_col);
    }

    // head is 2 greater up or down but same column
    if head_col == tail_col && head_row == tail_row + 2 {
        tail_row += 1;
        return (tail_row, tail_col);
    }
    if head_col == tail_col && head_row == tail_row - 2 {
        tail_row -= 1;
        return (tail_row, tail_col);
    }

    // not in the same row or column
    if head_row > tail_row && head_col > tail_col && (head_row, head_col) != (tail_row+1, tail_col+1) {
        // up right
        tail_row += 1;
        tail_col += 1;
        return (tail_row, tail_col);
    } else if head_row < tail_row && head_col > tail_col && (head_row, head_col) != (tail_row-1, tail_col+1) {
        // down right
        tail_row -= 1;
        tail_col += 1;
        return (tail_row, tail_col);
    } else if head_row > tail_row && head_col < tail_col && (head_row, head_col) != (tail_row+1, tail_col-1) {
        // up left
        tail_row += 1;
        tail_col -= 1;
        return (tail_row, tail_col);
    } else if head_row < tail_row && head_col < tail_col && (head_row, head_col) != (tail_row-1, tail_col-1) {
        // down left
        tail_row -= 1;
        tail_col -= 1;
        return (tail_row, tail_col);
    } else {
        return (tail_row, tail_col);
    }
}
