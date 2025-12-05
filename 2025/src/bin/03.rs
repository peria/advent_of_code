use std::io::BufRead;

fn main() {
    let batteries = get_input();
    let answer1 = solve1(&batteries);
    let answer2 = solve2(&batteries);
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn get_input() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().to_string())
        .collect()
}

fn get_joltage(battery: &str, n: usize) -> i64 {
    let mut vals = vec![0; n + 1];

    for (j, c) in battery.chars().enumerate() {
        let d = c.to_digit(10).unwrap() as i64;
        for i in (0..n.min(j + 1)).rev() {
            let v = vals[i] * 10 + d;
            if v >= vals[i + 1] {
                vals[i + 1] = v;
            }
        }
    }
    vals[n]
}

fn solve1(batteries: &Vec<String>) -> i64 {
    batteries.iter().map(|b| get_joltage(b, 2)).sum()
}

fn solve2(batteries: &Vec<String>) -> i64 {
    batteries.iter().map(|b| get_joltage(b, 12)).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_solve1() {
        let input = SAMPLE_INPUT
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(solve1(&input), 357);
    }

    #[test]
    fn test_solve2() {
        let input = SAMPLE_INPUT
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(solve2(&input), 3121910778619);
    }
}
