use std::fs;

fn main() {
    /*
    I recognize the inefficiencies of my solutions and there are better ones.
    I however am using the advent to learn rust and not produce the most performant solution
    */
    let mut forest: Vec<Vec<i8>> = vec![];
    let contents = fs::read_to_string("input.txt").expect("couldn't read");
    for line in contents.lines() {
        let mut row: Vec<i8> = vec![];
        let nums = line.chars();
        for num in nums {
            row.push(num.to_string().parse::<i8>().unwrap());
        }
        forest.push(row);
    }
    let mut visibility = vec![vec![false; forest[0].len()]; forest.len()];
    //println!("Forest is {:?}", forest);
    let mut row_ctr = 0;
    for row in forest.iter() {
        let mut left_ptr: usize = 0;
        let mut right_ptr: usize = (row.len()-1).try_into().unwrap();

        let mut max_height_left = -1;
        let mut max_height_right = -1;

        for _col in 0..row.len() {
            //println!("on row {} column {} left ptr {} and right ptr {}", row_ctr, i, left_ptr, right_ptr);
            let left_visible = row[left_ptr] > max_height_left;
            let right_visible = row[right_ptr] > max_height_right;

            max_height_left = std::cmp::max(row[left_ptr], max_height_left);
            max_height_right = std::cmp::max(row[right_ptr], max_height_right);

            visibility[row_ctr][left_ptr] = visibility[row_ctr][left_ptr] || left_visible; 
            visibility[row_ctr][right_ptr] = visibility[row_ctr][right_ptr] || right_visible; 
            if right_ptr != 0 {
                right_ptr = right_ptr - 1;
                left_ptr = left_ptr + 1;
            }
        }
        row_ctr += 1;
    }

    for col in 0..forest[0].len() {
        let mut top_ptr: usize = 0;
        let mut bottom_ptr: usize = (forest.len()-1).try_into().unwrap();

        let mut max_height_top = -1;
        let mut max_height_bottom = -1;
        for _row in 0..forest.len() {
            //println!("on row {} column {} top ptr {} and bottom ptr {}", row, col, top_ptr, bottom_ptr);
            let top_visible = forest[top_ptr][col] > max_height_top;
            let bottom_visible = forest[bottom_ptr][col] > max_height_bottom;

            max_height_top = std::cmp::max(forest[top_ptr][col], max_height_top);
            max_height_bottom = std::cmp::max(forest[bottom_ptr][col], max_height_bottom);

            visibility[top_ptr][col] = visibility[top_ptr][col] || top_visible; 
            visibility[bottom_ptr][col] = visibility[bottom_ptr][col] || bottom_visible; 

            if bottom_ptr != 0 {
                bottom_ptr = bottom_ptr - 1;
                top_ptr = top_ptr + 1;
            }
        }
    }

    let mut tot_visible = 0;
    for row in visibility {
        for col in row {
            if col {
                tot_visible += 1;
            }
        }
    }

    // part 2
    let mut score = vec![vec![0; forest[0].len()]; forest.len()];
    println!("total visible is {}", tot_visible);
    for i in 0..forest.len() {
        for j in 0..forest[0].len() {
            score[i][j] = calculate_view(&forest, i, j, forest[i][j].clone());
        }
    }

    let mut max_score = -1; //works because actual scores cannot be negative, so don't need negative inf

    for row in score {
        for col in row {
            if col > max_score {
                max_score = col;
            }
        }
    }
    println!("Max scenic score is {}", max_score);


}

fn calculate_view(forest: &Vec<Vec<i8>>, row: usize, col: usize, house_height: i8) -> i32 {
    let mut row_ptr = row.clone();
    let mut col_ptr = col.clone();

    let mut up_tree_count = 0;
    let mut down_tree_count = 0;
    let mut left_tree_count = 0;
    let mut right_tree_count = 0;
    //move up
    while row_ptr != 0 {
        row_ptr -= 1;
        if house_height > forest[row_ptr][col_ptr] {
            up_tree_count += 1;
        } else if house_height == forest[row_ptr][col_ptr] {
            up_tree_count += 1;
            row_ptr = 0;
        }else {
            row_ptr = 0;
        }
    }

    row_ptr = row.clone();
    //move down
    while row_ptr != forest.len()-1 {
        row_ptr += 1;
        if house_height > forest[row_ptr][col_ptr] {
            down_tree_count += 1;
        } else if house_height == forest[row_ptr][col_ptr]{
            down_tree_count += 1;
            row_ptr = forest.len()-1;
        } else {
            row_ptr = forest.len()-1;
        }
    }
    row_ptr = row.clone();
    //move left
    while col_ptr != 0 {
        col_ptr -= 1;
        //println!("for {} checking that {} at {}, {} is shorter", house_height, forest[row_ptr][col_ptr], row_ptr, col_ptr);
        if house_height > forest[row_ptr][col_ptr] {
            left_tree_count += 1;
        } else if house_height == forest[row_ptr][col_ptr] {
            left_tree_count += 1;
            col_ptr = 0;
        }else {
        }
    }
    //println!();
    col_ptr = col.clone();
    //move right
    while col_ptr != forest[0].len()-1 {
        col_ptr += 1;
        if house_height > forest[row_ptr][col_ptr] {
            right_tree_count += 1;
        } else if house_height == forest[row_ptr][col_ptr] {
            right_tree_count += 1;
            col_ptr = forest[0].len()-1;
        }else {
            col_ptr = forest[0].len()-1;
        }
    }

    println!("for row {} and col {}, and height {} got scores of u, d, l, r: {}, {}, {}, {}", row, col, house_height, up_tree_count, down_tree_count, left_tree_count, right_tree_count);

    up_tree_count*down_tree_count*left_tree_count*right_tree_count

}
