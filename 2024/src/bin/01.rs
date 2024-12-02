use std::{collections::HashMap, io::prelude::*};

fn main() {
    let data = input();
    println!("{}", solve1(&data));
    println!("{}", solve2(&data));
}

fn input() -> (Vec<i64>, Vec<i64>) {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.ok().unwrap();
        let values: Vec<_> = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        lefts.push(values[0]);
        rights.push(values[1]);
    }
    (lefts, rights)
}

fn solve1(data: &(Vec<i64>, Vec<i64>)) -> i64 {
    let (mut lefts, mut rights) = data.clone();
    lefts.sort();
    rights.sort();
    lefts
        .iter()
        .zip(rights.iter())
        .map(|(&l, &r)| (l - r).abs())
        .sum()
}

fn solve2(data: &(Vec<i64>, Vec<i64>)) -> i64 {
    let left = count(&data.0);
    let right = count(&data.1);

    left.iter()
        .map(|(k, &v)| *k * *right.get(k).unwrap_or(&0) as i64 * v as i64)
        .sum()
}

fn count(xs: &Vec<i64>) -> HashMap<i64, usize> {
    let mut c = HashMap::new();
    for x in xs.iter() {
        if let Some(&n) = c.get(x) {
            c.get_mut(x).map(|val| {
                *val = n + 1;
            });
        } else {
            c.insert(*x, 1);
        }
    }
    c
}
