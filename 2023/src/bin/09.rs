use std::{io::BufRead, sync::LockResult};

fn main() {
    let oasis = read_oasis();
    // eprintln!("{:?}", &oasis);
    let val1: i64 = oasis.iter().map(|x| extrapolate_next(x)).sum();
    println!("{}", val1);
    let val2: i64 = oasis.iter().map(|x| extrapolate_back(x)).sum();
    println!("{}", val2);
}

fn extrapolate_next(xs: &Vec<i64>) -> i64 {
    let mut xs = xs.clone();
    let mut lasts = Vec::new();
    while !xs.is_empty() {
        for i in 1..xs.len() {
            xs[i - 1] = xs[i] - xs[i - 1];
        }
        lasts.push(xs.pop().unwrap());
    }
    lasts.iter().sum()
}

fn extrapolate_back(xs: &Vec<i64>) -> i64 {
    let mut xs = xs.clone();
    let mut firsts = Vec::new();
    while !xs.is_empty() {
        firsts.push(xs[0]);
        for i in 1..xs.len() {
            xs[i - 1] = xs[i] - xs[i - 1];
        }
        xs.pop().unwrap();
    }
    firsts.iter().rev().fold(0, |s, x| x - s)
}

fn read_oasis() -> Vec<Vec<i64>> {
    let mut seqs = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let seq: Vec<i64> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        seqs.push(seq);
    }
    seqs
}
