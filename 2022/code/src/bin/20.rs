use std::io::BufRead;

fn main() {
    let numbers = get_input();

    println!("{}", get_grove(&numbers, 1, 1));
    println!("{}", get_grove(&numbers, 811589153, 10));
}

fn get_grove(numbers: &Vec<i64>, dec_key: i64, n: usize) -> i64 {
    let mut indexed: Vec<(usize, i64)> = numbers.iter().map(|x| x * dec_key).enumerate().collect();
    for _ in 0..n {
        mix_seq(&mut indexed);
    }
    let numbers: Vec<i64> = indexed.iter().map(|(_i, x)| *x).collect();
    let zero = numbers.iter().position(|x| x == &0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| numbers[(zero + i) % numbers.len()])
        .sum()
}

fn mix_seq(indexed: &mut Vec<(usize, i64)>) {
    let n = indexed.len();
    let m = (n - 1) as i64;
    for i in 0..n {
        let j = indexed.iter().position(|(id, _v)| id == &i).unwrap();
        let val = indexed.remove(j);
        let (_i, v) = val;
        let j = (j as i64 + (v % m + m) % m) % m;
        indexed.insert(j as usize, val);
    }
}

fn get_input() -> Vec<i64> {
    let mut numbers = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        numbers.push(line.parse().unwrap());
    }
    numbers
}
