use std::io::BufRead;

fn main() {
    let instructions = get_input();
    let answer1 = solve1(&instructions);
    let answer2 = solve2(&instructions);
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

fn solve1(instructions: &Vec<String>) -> i64 {
    let mut count = 0;
    let mut dial = 50;
    for inst in instructions {
        let dir = inst.chars().nth(0).unwrap();
        let amount = &inst[1..];
        let amount = amount.parse::<i64>().unwrap();

        dial = match dir {
            'R' => dial + amount,
            'L' => dial - amount + 100,
            _ => panic!("Invalid direction"),
        };
        while dial < 0 {
            dial += 100;
        }
        while dial >= 100 {
            dial -= 100;
        }

        if dial == 0 {
            count += 1;
        }
    }

    count
}

fn solve2(instructions: &Vec<String>) -> i64 {
    let mut count = 0;
    let mut dial = 50;
    for inst in instructions {
        let dir = inst.chars().nth(0).unwrap();
        let amount = &inst[1..];
        let amount = amount.parse::<i64>().unwrap();
        let sign = match dir {
            'R' => 1,
            'L' => -1,
            _ => panic!("Invalid direction"),
        };

        for _ in 0..amount {
            dial += sign;
            if dial < 0 {
                dial += 100;
            }
            if dial >= 100 {
                dial -= 100;
            }
            if dial == 0 {
                count += 1;
            }
        }
    }

    count
}

mod test {
    const TEST_INPUT: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_solve1() {
        let instructions = TEST_INPUT
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect();
        let answer1 = super::solve1(&instructions);
        let expect1 = 3;
        assert_eq!(answer1, expect1);
    }

    #[test]
    fn test_solve2() {
        let instructions = TEST_INPUT
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect();
        let answer2 = super::solve2(&instructions);
        let expect2 = 6;
        assert_eq!(answer2, expect2);
    }
}
