use std::io::BufRead;

const ROCK: i64 = 0;
const PAPER: i64 = 1;
const SCISSORS: i64 = 2;

fn main() {
    let hands = get_hands();

    println!("{}", hands.iter().map(|&x| get_score(x)).sum::<i64>());
    println!("{}", hands.iter().map(|&x| follow(x)).sum::<i64>());
}

fn get_hands() -> Vec<(char, char)> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|x| {
            let y: Vec<char> = x.unwrap().chars().collect();
            (y[0], y[2])
        })
        .collect()
}

fn get_score(xs: (char, char)) -> i64 {
    let (x, y) = xs;
    let x = match x {
        'A' => ROCK,
        'B' => PAPER,
        'C' => SCISSORS,
        _ => -1,
    };
    let y = match y {
        'X' => ROCK,
        'Y' => PAPER,
        'Z' => SCISSORS,
        _ => -1,
    };
    let win = match (x - y + 3) % 3 {
        0 => 3,
        1 => 0,
        2 => 6,
        _ => -1,
    };
    win + y + 1
}

fn follow(xs: (char, char)) -> i64 {
    let (x, y) = xs;
    let x = match x {
        'A' => ROCK,
        'B' => PAPER,
        'C' => SCISSORS,
        _ => -1,
    };
    let win: i64 = match y {
        'X' => 0,
        'Y' => 1,
        'Z' => 2,
        _ => -1,
    };
    let y = (x + win + 2) % 3;
    win * 3 + y + 1
}
