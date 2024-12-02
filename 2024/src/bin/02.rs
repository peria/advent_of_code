use std::io::BufRead;

fn main() {
    let data = input();
    println!("{}", solve1(&data));
    println!("{}", solve2(&data));
}

fn input() -> Vec<Vec<i64>> {
    let mut data = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.ok().unwrap();
        let nums = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        data.push(nums);
    }
    data
}

fn solve1(data: &Vec<Vec<i64>>) -> usize {
    data.iter().filter(|&xs| is_safe(xs)).count()
}

fn solve2(data: &Vec<Vec<i64>>) -> usize {
    data.iter()
        .filter(|&xs| is_safe(xs) || is_safe_with_take(xs))
        .count()
}

fn is_safe_with_take(xs: &Vec<i64>) -> bool {
    let n = xs.len();
    for i in 0..n {
        let mut ys: Vec<_> = xs[..i].iter().collect();
        ys.extend(xs[(i + 1)..].iter());
        let ys = ys.iter().map(|&y| *y).collect();
        if is_safe(&ys) {
            return true;
        }
    }
    false
}

fn is_safe(xs: &Vec<i64>) -> bool {
    let diff: Vec<_> = xs
        .iter()
        .zip(xs[1..].iter())
        .map(|(x0, x1)| x1 - x0)
        .collect();

    diff.iter().all(|&d| -3 <= d && d <= -1) || diff.iter().all(|&d| 1 <= d && d <= 3)
}
