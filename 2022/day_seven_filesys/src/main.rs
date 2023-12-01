use std::{fs, rc::Rc, rc::Weak, cell::RefCell};

struct Node {
    size: i64,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
    name: String,
    object: ObjectType,
}

enum ObjectType {
    File,
    Dir
}


fn main() {
    let nodes: Vec<Node> = vec![]; 
    let mut root_node = Rc::new(RefCell::new((Node {
        size: 0,
        parent: None,
        children: vec![],
        name: String::from("/"),
        object: ObjectType::Dir,
    })));
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file");
    let mut acc: i64 = 0;
    let mut acc2: Vec<i64> = vec![];
    parse_tree(contents, &root_node);
    recurse_tree(&root_node, &mut acc, &mut acc2);
    println!("Found sum of {}", acc);
    acc2.sort();
    let unused_space = 70000000-acc2[acc2.len()-1];
    println!("Unused space is {}", unused_space);
    let will_be_greater = acc2.iter().filter(|val| **val+unused_space > 30_000_000).collect::<Vec<&i64>>();
    println!("will be greater {:?}", will_be_greater)
}

fn recurse_tree(node: &Rc<RefCell<Node>>, acc: &mut i64, acc2: &mut Vec<i64>) -> i64{
    println!("processing node {}", node.borrow().name);
    match node.borrow().object {
        ObjectType::Dir => {
            println!("Hit dir {} so iter children", node.borrow().name);
            let mut sum = 0;
            for child in node.borrow().children.iter() {
                let sub_size = recurse_tree(child, acc, acc2);
                sum += sub_size;
            }
            let out = format!("for dir {} and children {:?} sum is {}", node.borrow().name, node.borrow().children.iter().map(|n| n.borrow().name.clone()).collect::<Vec<String>>(), sum.clone());
            println!("{}", out);
            fs::write("scratch.txt", out).expect("couldn't write");
            acc2.push(sum);
            if sum < 100000 {
                *acc += sum
            }
            sum
        },
        ObjectType::File => {
            println!("Hit file so handling file");
            node.borrow().size
        }
    }

}

fn parse_tree(contents: String, root_node: &Rc<RefCell<Node>>) {
    let mut lines = contents.lines();
    lines.next();
    let mut cur_node = Rc::clone(root_node);

    for line in lines {
        let mut parts = line.split(" ");
        let first = parts.next().unwrap();
        if first == "$" {
            let command = parts.next().unwrap();
            if command == "ls" {
                continue;
            } else if command == "cd" {
                let second = String::from(parts.next().unwrap());
                if second != ".." {
                    let mut tmp = cur_node.clone();
                    for child in cur_node.borrow().children.iter() {
                        if child.borrow().name == second {
                            tmp = child.clone();
                        }
                    }
                    cur_node = tmp;
                }else if second == ".." {
                    let tmp = cur_node.borrow().parent.clone();
                    cur_node = tmp.as_ref().unwrap().upgrade().unwrap();
                }
            }

        } else {
            if first == "dir" {
                let new_node = Rc::new(RefCell::new((Node {
                    size: 0,
                    parent: Some(Rc::downgrade(&cur_node)),
                    children: vec![],
                    name: String::from(parts.next().unwrap()),
                    object: ObjectType::Dir,
                })));
                cur_node.borrow_mut().children.push(new_node);
            } else if first.parse::<i64>().is_ok() {
                let new_node = Rc::new(RefCell::new((Node {
                    size: first.parse::<i64>().unwrap(),
                    parent: Some(Rc::downgrade(&cur_node)),
                    children: vec![],
                    name: String::from(parts.next().unwrap()),
                    object: ObjectType::File
                })));
                cur_node.borrow_mut().children.push(new_node);
            }
        }
    }

}
