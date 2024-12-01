use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let rucksacks: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect();

    let score: i64 = rucksacks.iter().map(|x| get_score(x)).sum();
    println!("{}", score);
    let score: i64 = rucksacks.chunks(3).map(|x| get_group_score(&x)).sum();
    println!("{}", score);
}

fn get_score(x: &String) -> i64 {
    let n = x.len();
    let (x, y) = (&x[..(n / 2)], &x[(n / 2)..]);
    let xs: HashSet<char> = HashSet::from_iter(x.chars());
    let x = y.chars().find(|c| xs.contains(&c)).unwrap();
    x.to_digit(36).unwrap() as i64 - 9 + if x.is_lowercase() { 0 } else { 26 }
}

fn get_group_score(xs: &[String]) -> i64 {
    let char_sets: Vec<HashSet<char>> = xs
        .iter()
        .map(|x| HashSet::<char>::from_iter(x.chars().into_iter()))
        .collect();
    let cs = &(&char_sets[0] & &char_sets[1]) & &char_sets[2];
    let c = cs.iter().next().unwrap();
    c.to_digit(36).unwrap() as i64 - 9 + if c.is_lowercase() { 0 } else { 26 }
}
