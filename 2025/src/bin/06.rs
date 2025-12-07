use std::io::BufRead;

fn main() {
    let data = get_input();
    let answer1 = solve1(&data);
    let answer2 = solve2(&data);
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn get_input() -> Vec<Vec<char>> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

struct Value {
    nums: Vec<i64>,
    operator: char,
}

fn solve1(data: &Vec<Vec<char>>) -> i64 {
    let ops = data.last().unwrap().iter().collect::<String>();
    let ops = ops.split_ascii_whitespace();
    let mut values: Vec<Value> = ops
        .map(|op| Value {
            nums: Vec::new(),
            operator: op.chars().nth(0).unwrap(),
        })
        .collect();
    for line in data.iter().take(data.len() - 1) {
        let nums = line.iter().collect::<String>();
        values
            .iter_mut()
            .zip(nums.split_ascii_whitespace())
            .for_each(|(v, n)| {
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

fn solve2(data: &Vec<Vec<char>>) -> i64 {
    let n = data.len() - 1;
    let (num_lines, operation_line) = (&data[..n], &data[n]);

    let mut total = 0i64;
    let mut nums = vec![0i64; n];
    for i in (0..(data[0].len())).rev() {
        for (l, val) in num_lines.iter().zip(nums.iter_mut()) {
            let c = l[i];
            if c == ' ' {
                continue;
            }
            *val = *val * 10 + (c as u8 - b'0') as i64;
        }
        if operation_line[i] == ' ' {
            continue;
        }

        let operator = operation_line[i];
        let line_total: i64 = match operator {
            '+' => nums.iter().sum(),
            '*' => nums.iter().product(),
            _ => 0,
        };
        total += line_total;
        for val in nums.iter_mut() {
            *val = 0;
        }
    }
    total
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
            .split('\n')
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(solve1(&input), 4277556);
    }

    #[test]
    fn test_solve2() {
        let input: Vec<Vec<_>> = SAMPLE_INPUT
            .split('\n')
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(solve2(&input), 3263827);
    }
}
