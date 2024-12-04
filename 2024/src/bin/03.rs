use regex::Regex;
use std::io::BufRead;

fn main() {
    let data = input();
    println!("{}", solve1(&data));
    println!("{}", solve2(&data));
}

fn input() -> Vec<String> {
    let mut data = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.ok().unwrap();
        data.push(line);
    }
    data
}

fn solve1(data: &Vec<String>) -> i64 {
    let instructions = parse(data);

    instructions
        .iter()
        .map(|x| match x {
            Instruction::MUL(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

fn solve2(data: &Vec<String>) -> i64 {
    let instructions = parse(data);
    let mut enabled = true;
    let mut sum = 0;
    for inst in instructions.iter() {
        match inst {
            Instruction::MUL(a, b) => {
                if enabled {
                    sum += a * b;
                }
            }
            Instruction::DO => {
                enabled = true;
            }
            Instruction::DONT => {
                enabled = false;
            }
            _ => {}
        }
    }
    sum
}

#[derive(Debug)]
enum Instruction {
    MUL(i64, i64),
    DO,
    DONT,
    ERROR,
}

fn parse(data: &Vec<String>) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let data = data.join(" ");
    let re = Regex::new(r"(mul|do|don't)\(([\d,]*)\)").unwrap();
    for (_, [inst, args]) in re.captures_iter(&data).map(|c| c.extract()) {
        let inst = match inst {
            "mul" => {
                let vals: Vec<&str> = args.split(',').collect();
                let inst = if vals.len() != 2 {
                    Instruction::ERROR
                } else if vals.iter().any(|v| v.len() == 0) {
                    Instruction::ERROR
                } else {
                    Instruction::MUL(vals[0].parse().unwrap(), vals[1].parse().unwrap())
                };
                inst
            }
            "do" => Instruction::DO,
            "don't" => Instruction::DONT,
            _ => Instruction::ERROR,
        };

        instructions.push(inst);
    }

    instructions
}
