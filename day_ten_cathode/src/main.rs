use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Can't read file");
    let mut register: isize = 1;
    let mut cycle_counter = 0;
    let mut register_statuses: Vec<(isize, isize)> = vec![];
    let mut interested_cycle_p1 = 20;
    let mut interested_cycle = 40;
    let mut cur_crt: Vec<String> = vec![];
    let mut last_interest = 0;

    for line in contents.lines() {
        let mut operation = line.split(" ");
        let command = operation.next().unwrap();
        let mut cycle_runs = 0;
        let mut addition = 0;
        match command {
            "addx" => {
                cycle_runs = 2;
                addition = operation.next().unwrap().parse::<isize>().unwrap();
            },
            "noop" => {
                cycle_runs = 1;
                addition = 0

            },
            _ => {panic!("invalid operation performed");}
        }

        for _ in 0..cycle_runs {
            cycle_counter += 1;
            println!("making display for cycle {} and register value {}", cycle_counter%40, register);
            if  cycle_counter%40 >= register && cycle_counter%40 < register+3 {
                cur_crt.push(String::from("#"));
            } else {
                cur_crt.push(String::from("."));
            }
            if cycle_counter == interested_cycle_p1 {
                interested_cycle_p1 += 40;
                register_statuses.push((cycle_counter, register));
            }
            if cycle_counter == interested_cycle && interested_cycle <= 220 {
                last_interest = interested_cycle;
                //register_statuses.push((cycle_counter, register));
                interested_cycle += 40;
                //let count = format!("{}", cycle_counter);
                //cur_crt.push(count);
                cur_crt.push(String::from("\n"));
            }
        }
        register += addition;
        /*
        let crt_len = TryInto::<isize>::try_into(cur_crt.len()).unwrap();
        if  crt_len >= register-1 && crt_len < register+1 {
            cur_crt.push(String::from("#"));
        } else {
            cur_crt.push(String::from("."));
        }*/
        if cycle_counter == interested_cycle_p1 {
            interested_cycle_p1 += 40;
            register_statuses.push((cycle_counter, register));
        }
        if cycle_counter == interested_cycle && interested_cycle <= 220 {
            last_interest = interested_cycle;
            //register_statuses.push((cycle_counter, register));
            interested_cycle += 40;
            //let count = format!("{}", cycle_counter);
            //cur_crt.push(count);
            cur_crt.push(String::from("\n"));
        }
    }
    println!("Register statuses are {:?}", register_statuses);
    let total: isize = register_statuses.iter().map(|(val, v)| val*v).sum();
    println!("Final total is {}", total);
    let final_str: String = cur_crt.into_iter().collect();
    println!("Final Output:\n{}", final_str);
}
