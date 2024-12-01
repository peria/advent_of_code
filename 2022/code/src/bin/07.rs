use std::io::BufRead;

fn main() {
    let sizes = read_input();
    println!("{}", choose_smalls(&sizes));
    println!("{}", choose_best(&sizes));
}

fn choose_smalls(sizes: &Vec<i64>) -> i64 {
    const SIZE_LIMIT: i64 = 100000;
    sizes.iter().filter(|&s| s <= &SIZE_LIMIT).sum()
}

fn choose_best(sizes: &Vec<i64>) -> i64 {
    const SPACE: i64 = 70000000;
    const NEED: i64 = 30000000;
    const MAX: i64 = SPACE - NEED;

    let current = sizes.iter().max().unwrap();
    let to_be_freed = current - MAX;
    *sizes.iter().filter(|&s| s >= &to_be_freed).min().unwrap()
}

fn read_input() -> Vec<i64> {
    let mut sizes = Vec::new();
    let mut stack = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let terms: Vec<&str> = line.split(' ').collect();
        if terms[0] == "$" {
            if terms[1] == "cd" {
                if terms[2] == ".." {
                    sizes.push(stack.pop().unwrap());
                } else {
                    stack.push(0i64);
                }
            }
        } else {
            match terms[0] {
                "dir" => {}
                _ => {
                    let s: i64 = terms[0].parse().unwrap();
                    for i in 0..stack.len() {
                        stack[i] += s;
                    }
                }
            }
        }
    }
    while !stack.is_empty() {
        sizes.push(stack.pop().unwrap());
    }
    sizes
}
