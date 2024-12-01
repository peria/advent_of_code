use std::io::BufRead;

fn main() {
    let segments = get_input();

    let covered = segments
        .iter()
        .filter(|(a, b, c, d)| (a - c) * (b - d) <= 0)
        .count();
    println!("{}", covered);
    let overlap = segments
        .iter()
        .filter(|(a, b, c, d)| !(d < a || b < c))
        .count();
    println!("{}", overlap);
}

fn get_input() -> Vec<(i64, i64, i64, i64)> {
    let mut pairs = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let x = line.unwrap();
        let x: Vec<&str> = x.split(',').collect();
        let x: Vec<Vec<i64>> = x
            .iter()
            .map(|&x| {
                x.split('-')
                    .map(|y| y.to_string().parse::<i64>().unwrap())
                    .collect()
            })
            .collect();
        pairs.push((x[0][0], x[0][1], x[1][0], x[1][1]));
    }
    pairs
}
