use std::io::BufRead;

fn main() {
    let (stacks, operations) = get_input();

    println!("{}", simulate(&stacks, &operations));
    println!("{}", simulate2(&stacks, &operations));
}

fn get_input() -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut stacks = vec![Vec::new(); 9];
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            break;
        }
        for i in (1..line.len()).step_by(4) {
            let c = line.chars().nth(i).unwrap();
            if c != ' ' {
                stacks[i / 4].push(c);
            }
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let mut operations = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let elems: Vec<&str> = line.split(' ').collect();
        let x = elems[1].parse().unwrap();
        let y = elems[3].parse().unwrap();
        let z = elems[5].parse().unwrap();
        operations.push((x, y, z));
    }
    (stacks, operations)
}

fn simulate(stacks: &Vec<Vec<char>>, operations: &Vec<(usize, usize, usize)>) -> String {
    let mut stacks = stacks.clone();
    for (n, from, to) in operations {
        let from = from - 1;
        let to = to - 1;
        for _ in 0..*n {
            if let Some(v) = stacks[from].pop() {
                stacks[to].push(v);
            }
        }
    }

    let mut ret = String::new();
    for s in stacks {
        if s.is_empty() {
            ret.push(' ');
        } else {
            ret.push(*s.last().unwrap());
        }
    }
    ret
}

fn simulate2(stacks: &Vec<Vec<char>>, operations: &Vec<(usize, usize, usize)>) -> String {
    let mut stacks = stacks.clone();
    for (n, from, to) in operations {
        let from = from - 1;
        let to = to - 1;
        let mut tmp = Vec::new();
        for _ in 0..*n {
            if let Some(v) = stacks[from].pop() {
                tmp.push(v);
            }
        }
        tmp.reverse();
        stacks[to].append(&mut tmp);
    }

    let mut ret = String::new();
    for s in stacks {
        if s.is_empty() {
            ret.push(' ');
        } else {
            ret.push(*s.last().unwrap());
        }
    }
    ret
}
