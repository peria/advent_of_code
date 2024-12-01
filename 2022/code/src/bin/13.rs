use std::cmp::Ordering;
use std::io::BufRead;

fn main() {
    let lists = get_input();

    println!("{}", solve_1(&lists));
    println!("{}", solve_2(&lists));
}

fn solve_1(lists: &Vec<(json::JsonValue, json::JsonValue)>) -> usize {
    let mut sum = 0usize;
    for (i, (left, right)) in lists.iter().enumerate() {
        if compare(left, right) == Ordering::Less {
            sum += i + 1;
        }
    }
    sum
}

fn solve_2(xs: &Vec<(json::JsonValue, json::JsonValue)>) -> usize {
    let key1 = json::parse("[[2]]").unwrap();
    let key2 = json::parse("[[6]]").unwrap();

    let mut list = Vec::new();
    for (x, y) in xs.iter() {
        list.push(x.clone());
        list.push(y.clone());
    }
    list.push(key1.clone());
    list.push(key2.clone());
    list.sort_by(|x, y| compare(&x, &y));

    let mut ret = 1;
    for (i, x) in list.iter().enumerate() {
        if x == &key1 || x == &key2 {
            eprintln!("{}", i + 1);
            ret *= i + 1;
        }
        eprintln!("{}", x.to_string());
    }
    ret
}

fn compare(x: &json::JsonValue, y: &json::JsonValue) -> Ordering {
    if x.is_number() && y.is_number() {
        return x.as_i32().unwrap().cmp(&y.as_i32().unwrap());
    }

    let mut x = x.clone();
    let mut y = y.clone();
    if x.is_number() {
        x = json::array![x.as_i32().unwrap()];
    }
    if y.is_number() {
        y = json::array![y.as_i32().unwrap()];
    }

    let n = usize::max(x.len(), y.len());
    for i in 0..n {
        if i == x.len() {
            return Ordering::Less;
        }
        if i == y.len() {
            return Ordering::Greater;
        }

        let c = compare(&x[i], &y[i]);
        if c != Ordering::Equal {
            return c;
        }
    }
    Ordering::Equal
}

fn get_input() -> Vec<(json::JsonValue, json::JsonValue)> {
    let mut lists = Vec::new();
    let mut left = None;
    let mut right = None;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if left.is_none() {
            left = Some(json::parse(&line).unwrap());
        } else if right.is_none() {
            right = Some(json::parse(&line).unwrap());
            lists.push((left.clone().unwrap(), right.clone().unwrap()));
        } else {
            left = None;
            right = None;
        }
    }
    lists
}
