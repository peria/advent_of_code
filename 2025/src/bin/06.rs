use std::io::BufRead;

use itertools::multizip;

fn main() {
    let data = get_input();
    let answer1 = solve1(&data);
    let answer2 = solve2(&data);
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn get_input() -> Vec<Vec<String>> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_ascii_whitespace()
                .map(|x| x.to_string())
                .collect()
        })
        .collect()
}

struct Value {
    nums: Vec<i64>,
    operator: char,
}

fn solve1(data: &Vec<Vec<String>>) -> i64 {
    let ops = data.last().unwrap();
    let nums = &data[..(data.len() - 1)];
    let mut values: Vec<Value> = ops
        .iter()
        .map(|op| Value {
            nums: Vec::new(),
            operator: op.chars().nth(0).unwrap(),
        })
        .collect();
    for line in data.iter().take(data.len() - 1) {
        values.iter_mut().zip(line.iter()).for_each(|(v, n)| {
            v.nums.push(n.parse::<i64>().unwrap());
        });
    }

    values
        .iter()
        .map(|v| match v.operator {
            '+' => v.nums.iter().sum(),
            '*' => v.nums.iter().product(),
            _ => 0,
        })
        .sum()
}

fn solve2(data: &Vec<Vec<String>>) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";

    #[test]
    fn test_solve1() {
        let input: Vec<Vec<_>> = SAMPLE_INPUT
            .trim()
            .split('\n')
            .map(|s| s.split_ascii_whitespace().map(|x| x.to_string()).collect())
            .collect();
        assert_eq!(solve1(&input), 4277556);
    }

    #[test]
    fn test_solve2() {
        let input: Vec<Vec<_>> = SAMPLE_INPUT
            .trim()
            .split('\n')
            .map(|s| s.split_ascii_whitespace().map(|x| x.to_string()).collect())
            .collect();
        assert_eq!(solve2(&input), 13);
    }
}
