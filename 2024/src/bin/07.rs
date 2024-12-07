use std::{collections::HashSet, io::BufRead};

fn main() {
    let equations = input();
    println!("{}", solve1(&equations));
    println!("{}", solve2(&equations));
}

fn input() -> Vec<Equation> {
    let mut equations = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let equation = Equation::from(&line.ok().unwrap() as &str);
        equations.push(equation);
    }
    equations
}

fn solve1(equations: &Vec<Equation>) -> i64 {
    let mul = |x: i64, y: i64| x * y;
    let add = |x: i64, y: i64| x + y;
    let operations = vec![mul, add];

    equations
        .iter()
        .filter(|eq| eq.is_valid_with_operations(&operations))
        .map(|eq| eq.answer)
        .sum()
}

fn solve2(equations: &Vec<Equation>) -> i64 {
    let mul = |x: i64, y: i64| x * y;
    let add = |x: i64, y: i64| x + y;
    let concat = |x: i64, y: i64| {
        let mut shift = 1;
        while shift <= y {
            shift *= 10;
        }
        let (z, overflow) = x.overflowing_mul(shift);
        if overflow {
            return std::i64::MAX;
        }
        let (z, overflow) = z.overflowing_add(y);
        if overflow {
            return std::i64::MAX;
        }
        z
    };
    let operations = vec![mul, add, concat];

    equations
        .iter()
        .filter(|eq| eq.is_valid_with_operations(&operations))
        .map(|eq| eq.answer)
        .sum()
}

struct Equation {
    answer: i64,
    numbers: Vec<i64>,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let xs: Vec<&str> = value.split(':').collect();
        let answer = xs[0].parse().unwrap();
        let numbers = xs[1]
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Equation { answer, numbers }
    }
}

impl Equation {
    fn is_valid_with_operations(&self, operations: &Vec<fn(i64, i64) -> i64>) -> bool {
        let mut values = HashSet::new();
        values.insert(self.numbers[0]);
        for &number in self.numbers[1..].iter() {
            let mut next_values = HashSet::new();
            let mut add_if_valid = |x| {
                if x <= self.answer {
                    next_values.insert(x);
                }
            };

            for &v in values.iter() {
                for operation in operations.iter() {
                    add_if_valid(operation(v, number));
                }
            }
            values = next_values;
        }

        values.contains(&self.answer)
    }
}
